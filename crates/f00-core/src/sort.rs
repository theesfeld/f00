use crate::entry::{Entry, EntryKind};
use crate::options::{ListOptions, SortBy};

/// Case-insensitive name comparison.
///
/// In GNU mode, compare raw byte/string order without stripping leading dots.
pub fn cmp_name(a: &str, b: &str) -> std::cmp::Ordering {
    cmp_name_with_mode(a, b, false)
}

pub fn cmp_name_with_mode(a: &str, b: &str, gnu: bool) -> std::cmp::Ordering {
    if gnu {
        // Approximate LC_COLLATE=C: byte-wise, case-sensitive.
        return a.cmp(b);
    }
    // Friendly default: case-insensitive, interleave hidden files.
    let a_key = a.trim_start_matches('.').to_ascii_lowercase();
    let b_key = b.trim_start_matches('.').to_ascii_lowercase();
    match a_key.cmp(&b_key) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    }
}

/// strverscmp-like natural / version comparison (`ls -v`).
///
/// Digit runs compare numerically (with leading-zero special-case similar to glibc);
/// non-digit runs compare byte-wise. `~` sorts before every other character and
/// before end-of-string (glibc `strverscmp`).
pub fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut i = 0usize;
    let mut j = 0usize;

    while i < ab.len() || j < bb.len() {
        // glibc: `~` sorts before every other character *and* before end-of-string.
        if i >= ab.len() {
            return if bb[j] == b'~' {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
        if j >= bb.len() {
            return if ab[i] == b'~' {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        let a_digit = ab[i].is_ascii_digit();
        let b_digit = bb[j].is_ascii_digit();

        if a_digit && b_digit {
            // Leading zeros: all-zero prefix is "fractional" in glibc strverscmp.
            let a_zeros = count_leading(ab, i, b'0');
            let b_zeros = count_leading(bb, j, b'0');
            let i0 = i + a_zeros;
            let j0 = j + b_zeros;
            let a_digits = count_digits(ab, i0);
            let b_digits = count_digits(bb, j0);

            // Compare digit values first by length then lexicographically.
            match a_digits.cmp(&b_digits) {
                Ordering::Equal => {
                    let acmp = ab[i0..i0 + a_digits].cmp(&bb[j0..j0 + b_digits]);
                    if acmp != Ordering::Equal {
                        return acmp;
                    }
                    // Equal numbers: more leading zeros → less (glibc).
                    if a_zeros != b_zeros {
                        return b_zeros.cmp(&a_zeros);
                    }
                }
                ord => return ord,
            }
            i = i0 + a_digits;
            j = j0 + b_digits;
            continue;
        }

        if a_digit != b_digit {
            return cmp_nondigit_byte(ab[i], bb[j]);
        }

        // Both non-digit: compare until digit or end.
        while i < ab.len() && j < bb.len() && !ab[i].is_ascii_digit() && !bb[j].is_ascii_digit() {
            match cmp_nondigit_byte(ab[i], bb[j]) {
                Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
                ord => return ord,
            }
        }
        if i < ab.len() && j < bb.len() {
            // One hit a digit boundary.
            if ab[i].is_ascii_digit() != bb[j].is_ascii_digit() {
                return cmp_nondigit_byte(ab[i], bb[j]);
            }
        }
    }
    Ordering::Equal
}

/// glibc `strverscmp`: `~` sorts before every other character, including end-of-string.
fn cmp_nondigit_byte(a: u8, b: u8) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    if a == b {
        return Ordering::Equal;
    }
    if a == b'~' {
        return Ordering::Less;
    }
    if b == b'~' {
        return Ordering::Greater;
    }
    a.cmp(&b)
}

fn count_leading(bytes: &[u8], start: usize, ch: u8) -> usize {
    let mut n = 0;
    while start + n < bytes.len() && bytes[start + n] == ch {
        n += 1;
    }
    n
}

fn count_digits(bytes: &[u8], start: usize) -> usize {
    let mut n = 0;
    while start + n < bytes.len() && bytes[start + n].is_ascii_digit() {
        n += 1;
    }
    n
}

pub(crate) fn cmp_entry(a: &Entry, b: &Entry, opts: &ListOptions) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    if opts.dirs_first {
        let a_dir = a.kind == EntryKind::Directory;
        let b_dir = b.kind == EntryKind::Directory;
        match (a_dir, b_dir) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }
    }

    let name_cmp = || cmp_name_with_mode(&a.name, &b.name, opts.gnu_mode);

    let primary = match opts.sort_by {
        SortBy::None => Ordering::Equal,
        SortBy::Name => name_cmp(),
        // GNU `-S`: largest first.
        SortBy::Size => b.size.cmp(&a.size).then_with(name_cmp),
        SortBy::Time => {
            let ta = a.time_for(opts.time_field);
            let tb = b.time_for(opts.time_field);
            // Newest first.
            tb.cmp(&ta).then_with(name_cmp)
        }
        SortBy::Extension => {
            let ea = a.extension().unwrap_or("");
            let eb = b.extension().unwrap_or("");
            if opts.gnu_mode {
                ea.cmp(eb).then_with(name_cmp)
            } else {
                ea.to_ascii_lowercase()
                    .cmp(&eb.to_ascii_lowercase())
                    .then_with(name_cmp)
            }
        }
        SortBy::Version => cmp_version(&a.name, &b.name).then_with(name_cmp),
        // GNU `--sort=width`: shortest display width first, then name.
        SortBy::Width => display_width(&a.name)
            .cmp(&display_width(&b.name))
            .then_with(name_cmp),
    };

    if opts.reverse {
        primary.reverse()
    } else {
        primary
    }
}

/// Approximate printed width for `--sort=width` (ASCII = 1; wide CJK ≈ 2).
fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let u = c as u32;
            if c == '\0' {
                0
            } else if u < 0x1100 {
                1
            } else if (0x1100..=0x115f).contains(&u)
                || u == 0x2329
                || u == 0x232a
                || (0x2e80..=0xa4cf).contains(&u)
                || (0xac00..=0xd7a3).contains(&u)
                || (0xf900..=0xfaff).contains(&u)
                || (0xfe10..=0xfe19).contains(&u)
                || (0xfe30..=0xfe6f).contains(&u)
                || (0xff00..=0xff60).contains(&u)
                || (0xffe0..=0xffe6).contains(&u)
            {
                2
            } else {
                1
            }
        })
        .sum()
}

/// Sort entries according to options. Directory headers keep relative order.
pub fn sort_entries(entries: &mut [Entry], opts: &ListOptions) {
    if opts.sort_by == SortBy::None {
        if opts.reverse {
            entries.reverse();
        }
        return;
    }

    entries.sort_by(|a, b| match (a.is_dir_header, b.is_dir_header) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => cmp_entry(a, b, opts),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    fn entry(name: &str, kind: EntryKind, size: u64, secs: Option<u64>) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind,
            size,
            modified: secs.map(|s| SystemTime::UNIX_EPOCH + Duration::from_secs(s)),
            created: None,
            accessed: None,
            changed: secs.map(|s| SystemTime::UNIX_EPOCH + Duration::from_secs(s)),
            mode: 0,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: 0,
            owner: "u".into(),
            group: "g".into(),
            author: "u".into(),
            context: String::new(),
        }
    }

    #[test]
    fn version_tilde_before_digits() {
        use std::cmp::Ordering;
        assert_eq!(cmp_version("file~", "file1"), Ordering::Less);
        assert_eq!(cmp_version("file1", "file2"), Ordering::Less);
        assert_eq!(cmp_version("file2", "file10"), Ordering::Less);
        assert_eq!(cmp_version("file~", "file10"), Ordering::Less);
    }

    #[test]
    fn width_sort_shorter_first() {
        let mut opts = ListOptions::default();
        opts.sort_by = SortBy::Width;
        let a = entry("a", EntryKind::File, 0, None);
        let b = entry("bbbb", EntryKind::File, 0, None);
        assert_eq!(cmp_entry(&a, &b, &opts), std::cmp::Ordering::Less);
    }
}
