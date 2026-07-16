use crate::entry::{Entry, EntryKind};
use crate::options::{ListOptions, SortBy};

/// Case-insensitive name comparison with a simple natural-ish fallback.
pub fn cmp_name(a: &str, b: &str) -> std::cmp::Ordering {
    // Strip a leading `.` for sorting so hidden files interleave like GNU ls often does
    // under `LC_COLLATE=C`-ish behavior we approximate with case-insensitive order.
    let a_key = a.trim_start_matches('.').to_ascii_lowercase();
    let b_key = b.trim_start_matches('.').to_ascii_lowercase();
    match a_key.cmp(&b_key) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    }
}

fn kind_rank(kind: EntryKind) -> u8 {
    match kind {
        EntryKind::Directory => 0,
        EntryKind::Symlink => 1,
        EntryKind::File => 2,
        EntryKind::Other => 3,
    }
}

fn cmp_entry(a: &Entry, b: &Entry, opts: &ListOptions) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    if opts.dirs_first {
        let ka = kind_rank(a.kind);
        let kb = kind_rank(b.kind);
        // Only prioritize pure directories first.
        let a_dir = a.kind == EntryKind::Directory;
        let b_dir = b.kind == EntryKind::Directory;
        match (a_dir, b_dir) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {
                let _ = (ka, kb);
            }
        }
    }

    let primary = match opts.sort_by {
        SortBy::None => Ordering::Equal,
        SortBy::Name => cmp_name(&a.name, &b.name),
        SortBy::Size => a.size.cmp(&b.size).then_with(|| cmp_name(&a.name, &b.name)),
        SortBy::Time => {
            // Newest first is common for `ls -t`; we sort ascending here and
            // let `reverse` flip — actually classic `ls -t` is newest first.
            // We implement newest-first as the default Time order.
            b.modified
                .cmp(&a.modified)
                .then_with(|| cmp_name(&a.name, &b.name))
        }
        SortBy::Extension => {
            let ea = a.extension().unwrap_or("");
            let eb = b.extension().unwrap_or("");
            ea.to_ascii_lowercase()
                .cmp(&eb.to_ascii_lowercase())
                .then_with(|| cmp_name(&a.name, &b.name))
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

    // Stable partition: keep dir headers in place by sorting only non-headers
    // within contiguous runs, but for MVP just sort everything with headers first.
    entries.sort_by(|a, b| {
        match (a.is_dir_header, b.is_dir_header) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => cmp_entry(a, b, opts),
        }
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
            mode: 0,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
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
    fn sort_by_size() {
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
        assert_eq!(names, vec!["small", "mid", "big"]);
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
        assert!(entries[1].name == "bfile" || entries[1].name == "zfile");
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
        // After stripping `.` for the collate key, equal keys fall back to full
        // string order so `.foo` sorts just before `foo`.
        assert!(cmp_name(".foo", "foo").is_le());
        assert!(cmp_name("apple", "banana").is_lt());
        assert!(cmp_name("zebra", "apple").is_gt());
    }
}
