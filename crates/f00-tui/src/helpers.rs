//! Pure helpers for the directory browser (unit-testable without a TTY).

use std::path::{Path, PathBuf};

use f00_core::Entry;

/// Case-insensitive substring filter over entry names.
///
/// Empty `query` returns every entry. Matching is on [`Entry::name`] only.
pub fn filter_entries<'a>(entries: &'a [Entry], query: &str) -> Vec<&'a Entry> {
    if query.is_empty() {
        return entries.iter().collect();
    }
    let q = query.to_ascii_lowercase();
    entries
        .iter()
        .filter(|e| e.name.to_ascii_lowercase().contains(&q))
        .collect()
}

/// Resolve the parent directory of `cwd` for browser navigation.
///
/// - Empty / `.` → `..` (relative parent)
/// - Root (`/`) → itself
/// - Otherwise → parent path (preserving absolute/relative style of `cwd`)
pub fn parent_dir(cwd: &Path) -> PathBuf {
    if cwd.as_os_str().is_empty() || cwd == Path::new(".") {
        return PathBuf::from("..");
    }
    if cwd == Path::new("/") {
        return PathBuf::from("/");
    }
    match cwd.parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
        Some(_) => {
            // Relative path like `foo` with empty parent → `.`
            if cwd.is_absolute() {
                PathBuf::from("/")
            } else {
                PathBuf::from(".")
            }
        }
        None => PathBuf::from("."),
    }
}

/// Join `cwd` with a child name, handling `.` and `..` specially.
pub fn join_child(cwd: &Path, name: &str) -> PathBuf {
    match name {
        "." => cwd.to_path_buf(),
        ".." => parent_dir(cwd),
        _ => cwd.join(name),
    }
}

/// Clamp selection index into `0..len` (or 0 when empty).
pub fn clamp_index(selected: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        selected.min(len - 1)
    }
}

/// Move selection by `delta`, clamping to the filtered list length.
pub fn move_selection(selected: usize, len: usize, delta: isize) -> usize {
    if len == 0 {
        return 0;
    }
    let cur = selected.min(len - 1) as isize;
    let next = (cur + delta).clamp(0, (len - 1) as isize);
    next as usize
}

/// Format selected paths for stdout on quit (one path per line).
pub fn format_selected_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether the terminal is interactive enough for a fullscreen TUI.
pub fn is_interactive_tty() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind};

    fn fake_entry(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind: EntryKind::File,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: 0,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: Default::default(),
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: 0,
            owner: String::new(),
            group: String::new(),
            author: String::new(),
            context: String::new(),
        }
    }

    #[test]
    fn filter_empty_query_keeps_all() {
        let entries = vec![fake_entry("a"), fake_entry("b")];
        assert_eq!(filter_entries(&entries, "").len(), 2);
    }

    #[test]
    fn filter_case_insensitive_substring() {
        let entries = vec![
            fake_entry("Cargo.toml"),
            fake_entry("README.md"),
            fake_entry("src"),
        ];
        let hits = filter_entries(&entries, "cargo");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Cargo.toml");

        let hits = filter_entries(&entries, "MD");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "README.md");
    }

    #[test]
    fn parent_of_dot_is_dotdot() {
        assert_eq!(parent_dir(Path::new(".")), PathBuf::from(".."));
    }

    #[test]
    fn parent_of_root_is_root() {
        assert_eq!(parent_dir(Path::new("/")), PathBuf::from("/"));
    }

    #[test]
    fn parent_of_nested() {
        assert_eq!(
            parent_dir(Path::new("/home/glenda/lsr")),
            PathBuf::from("/home/glenda")
        );
        assert_eq!(
            parent_dir(Path::new("crates/f00-tui")),
            PathBuf::from("crates")
        );
    }

    #[test]
    fn join_child_handles_dotdot() {
        assert_eq!(
            join_child(Path::new("/tmp/foo"), ".."),
            PathBuf::from("/tmp")
        );
        assert_eq!(
            join_child(Path::new("/tmp/foo"), "bar"),
            PathBuf::from("/tmp/foo/bar")
        );
    }

    #[test]
    fn clamp_and_move_selection() {
        assert_eq!(clamp_index(5, 0), 0);
        assert_eq!(clamp_index(5, 3), 2);
        assert_eq!(move_selection(0, 5, -1), 0);
        assert_eq!(move_selection(0, 5, 1), 1);
        assert_eq!(move_selection(4, 5, 1), 4);
        assert_eq!(move_selection(2, 0, 1), 0);
    }

    #[test]
    fn format_selected_one_per_line() {
        let paths = vec![PathBuf::from("a"), PathBuf::from("b/c")];
        assert_eq!(format_selected_paths(&paths), "a\nb/c");
    }
}
