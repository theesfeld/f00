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

fn cmp_entry(a: &Entry, b: &Entry, opts: &ListOptions) -> std::cmp::Ordering {
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
    };

    if opts.reverse {
        primary.reverse()
    } else {
        primary
    }
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
    use crate::entry::{Entry, EntryKind, GitStatus, TimeField};
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
        }
    }

    #[test]
    fn sort_by_name_case_insensitive() {
        let mut entries = vec![
            entry("Banana", EntryKind::File, 0, None),
            entry("apple", EntryKind::File, 0, None),
            entry("Cherry", EntryKind::File, 0, None),
        ];
        sort_entries(&mut entries, &ListOptions::default());
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["apple", "Banana", "Cherry"]);
    }

    #[test]
    fn sort_by_size_largest_first() {
        let mut entries = vec![
            entry("big", EntryKind::File, 100, None),
            entry("small", EntryKind::File, 1, None),
            entry("mid", EntryKind::File, 50, None),
        ];
        let opts = ListOptions {
            sort_by: SortBy::Size,
            ..Default::default()
        };
        sort_entries(&mut entries, &opts);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["big", "mid", "small"]);
    }

    #[test]
    fn sort_by_time_newest_first() {
        let mut entries = vec![
            entry("old", EntryKind::File, 0, Some(10)),
            entry("new", EntryKind::File, 0, Some(100)),
            entry("mid", EntryKind::File, 0, Some(50)),
        ];
        let opts = ListOptions {
            sort_by: SortBy::Time,
            ..Default::default()
        };
        sort_entries(&mut entries, &opts);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["new", "mid", "old"]);
    }

    #[test]
    fn reverse_name_sort() {
        let mut entries = vec![
            entry("a", EntryKind::File, 0, None),
            entry("b", EntryKind::File, 0, None),
            entry("c", EntryKind::File, 0, None),
        ];
        let opts = ListOptions {
            reverse: true,
            ..Default::default()
        };
        sort_entries(&mut entries, &opts);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["c", "b", "a"]);
    }

    #[test]
    fn dirs_first() {
        let mut entries = vec![
            entry("zfile", EntryKind::File, 0, None),
            entry("adir", EntryKind::Directory, 0, None),
            entry("bfile", EntryKind::File, 0, None),
        ];
        let opts = ListOptions {
            dirs_first: true,
            ..Default::default()
        };
        sort_entries(&mut entries, &opts);
        assert_eq!(entries[0].name, "adir");
    }

    #[test]
    fn sort_by_extension() {
        let mut entries = vec![
            entry("a.txt", EntryKind::File, 0, None),
            entry("b.rs", EntryKind::File, 0, None),
            entry("c.md", EntryKind::File, 0, None),
            entry("noext", EntryKind::File, 0, None),
        ];
        let opts = ListOptions {
            sort_by: SortBy::Extension,
            ..Default::default()
        };
        sort_entries(&mut entries, &opts);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["noext", "c.md", "b.rs", "a.txt"]);
    }

    #[test]
    fn cmp_name_strips_dot_for_key() {
        assert!(cmp_name(".foo", "foo").is_le());
        assert!(cmp_name("apple", "banana").is_lt());
    }

    #[test]
    fn gnu_name_sort_is_bytewise() {
        assert_eq!(cmp_name_with_mode("B", "a", true), std::cmp::Ordering::Less);
        // 'B' < 'a' in ASCII
    }

    #[test]
    fn time_field_access() {
        let mut e = entry("x", EntryKind::File, 0, Some(50));
        e.accessed = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(10));
        assert_eq!(
            e.time_for(TimeField::Accessed),
            Some(SystemTime::UNIX_EPOCH + Duration::from_secs(10))
        );
    }
}
