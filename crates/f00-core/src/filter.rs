use crate::entry::Entry;
use crate::options::ListOptions;

/// Simple shell-style pattern match for `--ignore` / `-I` (supports `*` and `?`).
pub fn glob_match(pattern: &str, name: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), name.as_bytes())
}

fn glob_match_inner(pat: &[u8], name: &[u8]) -> bool {
    let (mut pi, mut ni) = (0usize, 0usize);
    let mut star_p = None;
    let mut star_n = 0usize;
    while ni < name.len() {
        if pi < pat.len() && (pat[pi] == b'?' || pat[pi] == name[ni]) {
            pi += 1;
            ni += 1;
        } else if pi < pat.len() && pat[pi] == b'*' {
            star_p = Some(pi);
            star_n = ni;
            pi += 1;
        } else if let Some(sp) = star_p {
            pi = sp + 1;
            star_n += 1;
            ni = star_n;
        } else {
            return false;
        }
    }
    while pi < pat.len() && pat[pi] == b'*' {
        pi += 1;
    }
    pi == pat.len()
}

/// Return true if `entry` should be shown under `opts`.
pub fn should_show(entry: &Entry, opts: &ListOptions) -> bool {
    if opts.ignore_backups && entry.name.ends_with('~') {
        return false;
    }

    for pat in &opts.ignore_patterns {
        if glob_match(pat, &entry.name) {
            return false;
        }
    }

    if opts.all {
        return true;
    }

    if entry.name == "." || entry.name == ".." {
        // `-A` (almost_all) hides `.` and `..` even though other dotfiles show.
        return false;
    }

    if entry.is_hidden() {
        return opts.almost_all;
    }

    true
}

/// Filter a list of entries in place according to options.
pub fn filter_entries(entries: &mut Vec<Entry>, opts: &ListOptions) {
    entries.retain(|e| should_show(e, opts));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn make_entry(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind: EntryKind::File,
            size: 0,
            modified: None,
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
    fn hides_dotfiles_by_default() {
        let opts = ListOptions::default();
        assert!(!should_show(&make_entry(".hidden"), &opts));
        assert!(should_show(&make_entry("visible"), &opts));
    }

    #[test]
    fn all_shows_everything_including_dot_and_dotdot() {
        let opts = ListOptions {
            all: true,
            ..Default::default()
        };
        assert!(should_show(&make_entry("."), &opts));
        assert!(should_show(&make_entry(".."), &opts));
        assert!(should_show(&make_entry(".git"), &opts));
    }

    #[test]
    fn almost_all_shows_dotfiles_but_not_dot_dotdot() {
        let opts = ListOptions {
            almost_all: true,
            ..Default::default()
        };
        assert!(!should_show(&make_entry("."), &opts));
        assert!(!should_show(&make_entry(".."), &opts));
        assert!(should_show(&make_entry(".config"), &opts));
        assert!(should_show(&make_entry("readme"), &opts));
    }

    #[test]
    fn filter_entries_retains_only_visible() {
        let opts = ListOptions::default();
        let mut entries = vec![
            make_entry("a"),
            make_entry(".b"),
            make_entry("c"),
            make_entry("."),
        ];
        filter_entries(&mut entries, &opts);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["a", "c"]);
    }

    #[test]
    fn ignore_backups() {
        let opts = ListOptions {
            ignore_backups: true,
            ..Default::default()
        };
        assert!(!should_show(&make_entry("file~"), &opts));
        assert!(should_show(&make_entry("file"), &opts));
    }

    #[test]
    fn ignore_patterns() {
        let opts = ListOptions {
            ignore_patterns: vec!["*.o".into(), "tmp*".into()],
            ..Default::default()
        };
        assert!(!should_show(&make_entry("a.o"), &opts));
        assert!(!should_show(&make_entry("tmp123"), &opts));
        assert!(should_show(&make_entry("a.rs"), &opts));
    }

    #[test]
    fn glob_star_and_question() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(!glob_match("*.rs", "main.toml"));
        assert!(glob_match("a?c", "abc"));
        assert!(!glob_match("a?c", "ac"));
    }
}
