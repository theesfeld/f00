use crate::entry::Entry;
use crate::options::ListOptions;

/// Return true if `entry` should be shown under `opts`.
pub fn should_show(entry: &Entry, opts: &ListOptions) -> bool {
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
}
