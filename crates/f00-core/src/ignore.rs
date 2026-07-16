//! Lightweight `.gitignore` / `.f00ignore` pattern loading and matching.
//!
//! Not a full gitignore engine: supports `#` comments, blank lines, leading `/`
//! (anchored to the directory containing the ignore file), trailing `/`
//! (directories only), `!` negation, and shell-style `*` / `?` globs via
//! [`crate::filter::glob_match`].

use std::fs;
use std::path::{Path, PathBuf};

use crate::entry::Entry;
use crate::filter::glob_match;

/// Names of ignore files consulted when [`crate::ListOptions::use_ignore_files`] is set.
pub const IGNORE_FILE_NAMES: &[&str] = &[".f00ignore", ".gitignore"];

#[derive(Debug, Clone)]
struct IgnorePattern {
    /// Glob text without leading `/` or trailing `/`.
    pattern: String,
    /// When true, pattern only matches from the start of the relative path.
    anchored: bool,
    /// When true, only directory entries match.
    dir_only: bool,
    /// When true, a match *un-ignores* (git `!` rules).
    negated: bool,
}

/// Compiled ignore rules for one directory tree root (the dir that held the file).
#[derive(Debug, Clone, Default)]
pub struct IgnoreSet {
    /// Directory that contained the ignore file(s); patterns are relative to this.
    pub base: PathBuf,
    patterns: Vec<IgnorePattern>,
}

impl IgnoreSet {
    /// Load `.f00ignore` then `.gitignore` from `dir` (later files append; last match wins
    /// for negation, same as layered gitignore sources in one directory).
    pub fn load_for_dir(dir: &Path) -> Self {
        let mut set = Self {
            base: dir.to_path_buf(),
            patterns: Vec::new(),
        };
        for name in IGNORE_FILE_NAMES {
            let path = dir.join(name);
            if let Ok(text) = fs::read_to_string(&path) {
                set.patterns.extend(parse_ignore_text(&text));
            }
        }
        set
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Return true if `entry` should be hidden by this ignore set.
    ///
    /// Matching is performed against the path relative to [`Self::base`], using
    /// the entry name when the entry lives directly under `base`.
    pub fn ignores(&self, entry: &Entry) -> bool {
        if self.patterns.is_empty() {
            return false;
        }
        // Never hide the ignore files' parent synthetic `.` / `..`.
        if entry.name == "." || entry.name == ".." {
            return false;
        }

        let rel = relative_path(&self.base, &entry.path).unwrap_or_else(|| entry.name.clone());
        let rel = rel.replace('\\', "/");
        let rel = rel.trim_start_matches("./");
        if rel.is_empty() {
            return false;
        }

        let is_dir = entry.is_dir();
        let mut ignored = false;
        for pat in &self.patterns {
            if pat.dir_only && !is_dir {
                continue;
            }
            if pattern_matches(pat, rel) {
                ignored = !pat.negated;
            }
        }
        ignored
    }
}

/// Load ignore sets for `dir` (nearest directory only — one level).
pub fn load_ignore_set(dir: &Path) -> IgnoreSet {
    IgnoreSet::load_for_dir(dir)
}

fn parse_ignore_text(text: &str) -> Vec<IgnorePattern> {
    let mut out = Vec::new();
    for line in text.lines() {
        // Trim BOM/CRLF leftovers for Windows-friendly ignore files.
        let line = line.trim().trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (negated, rest) = if let Some(r) = line.strip_prefix('!') {
            (true, r)
        } else {
            (false, line)
        };
        if rest.is_empty() {
            continue;
        }
        let (anchored, rest) = if let Some(r) = rest.strip_prefix('/') {
            (true, r)
        } else {
            (false, rest)
        };
        let (dir_only, pattern) = if let Some(r) = rest.strip_suffix('/') {
            (true, r.to_string())
        } else {
            (false, rest.to_string())
        };
        if pattern.is_empty() {
            continue;
        }
        out.push(IgnorePattern {
            pattern,
            anchored,
            dir_only,
            negated,
        });
    }
    out
}

fn pattern_matches(pat: &IgnorePattern, rel: &str) -> bool {
    let pattern = pat.pattern.as_str();
    // Basename-only convenience: `*.o` matches anywhere in the relative path's final component
    // and also the full relative path (git-like for unanchored patterns without `/`).
    if pat.anchored || pattern.contains('/') {
        return glob_match(pattern, rel)
            || (rel.ends_with('/') && glob_match(pattern, rel.trim_end_matches('/')));
    }

    // Unanchored, no slash: match against any path segment and full rel.
    if glob_match(pattern, rel) {
        return true;
    }
    for segment in rel.split('/') {
        if glob_match(pattern, segment) {
            return true;
        }
    }
    false
}

fn relative_path(base: &Path, path: &Path) -> Option<String> {
    if let Ok(p) = path.strip_prefix(base) {
        return Some(p.to_string_lossy().replace('\\', "/"));
    }
    // Windows can fail strip_prefix across different path prefixes; fall back to
    // file name when both live under the same parent.
    if path.parent() == Some(base) {
        return path.file_name().map(|n| n.to_string_lossy().into_owned());
    }
    None
}

/// Filter entries in place using an optional ignore set.
pub fn apply_ignore_set(entries: &mut Vec<Entry>, set: &IgnoreSet) {
    if set.is_empty() {
        return;
    }
    entries.retain(|e| !set.ignores(e));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Entry, EntryKind, GitStatus};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let base = std::env::temp_dir().join(format!(
            "f00-core-ignore-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&base).unwrap();
        base
    }

    fn entry_in(dir: &Path, name: &str, is_dir: bool) -> Entry {
        Entry {
            path: dir.join(name),
            name: name.to_string(),
            kind: if is_dir {
                EntryKind::Directory
            } else {
                EntryKind::File
            },
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
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
    fn parse_and_match_basic_globs() {
        let dir = temp_dir();
        fs::write(
            dir.join(".gitignore"),
            "*.o\n# comment\n/build\ntmp/\n!keep.o\n",
        )
        .unwrap();
        let set = IgnoreSet::load_for_dir(&dir);
        assert!(set.ignores(&entry_in(&dir, "foo.o", false)));
        assert!(!set.ignores(&entry_in(&dir, "foo.rs", false)));
        assert!(
            set.ignores(&entry_in(&dir, "build", false)),
            "anchored /build should match name build"
        );
        assert!(set.ignores(&entry_in(&dir, "tmp", true)));
        assert!(!set.ignores(&entry_in(&dir, "tmp", false))); // dir_only
                                                              // Negation: last matching rule wins
        assert!(!set.ignores(&entry_in(&dir, "keep.o", false)));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn f00ignore_and_gitignore_both_loaded() {
        let dir = temp_dir();
        fs::write(dir.join(".f00ignore"), "secret.txt\n").unwrap();
        fs::write(dir.join(".gitignore"), "*.log\n").unwrap();
        let set = IgnoreSet::load_for_dir(&dir);
        assert!(set.ignores(&entry_in(&dir, "secret.txt", false)));
        assert!(set.ignores(&entry_in(&dir, "a.log", false)));
        assert!(!set.ignores(&entry_in(&dir, "a.txt", false)));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nested_path_unanchored_glob() {
        let dir = temp_dir();
        fs::write(dir.join(".gitignore"), "*.o\n").unwrap();
        let set = IgnoreSet::load_for_dir(&dir);
        let mut e = entry_in(&dir, "sub/x.o", false);
        e.path = dir.join("sub").join("x.o");
        e.name = "x.o".into();
        assert!(set.ignores(&e));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn apply_ignore_set_filters() {
        let dir = temp_dir();
        fs::write(dir.join(".gitignore"), "hide_me\n").unwrap();
        let set = IgnoreSet::load_for_dir(&dir);
        let mut entries = vec![
            entry_in(&dir, "keep", false),
            entry_in(&dir, "hide_me", false),
        ];
        apply_ignore_set(&mut entries, &set);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "keep");
        let _ = fs::remove_dir_all(&dir);
    }
}
