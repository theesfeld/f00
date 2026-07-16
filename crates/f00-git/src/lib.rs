//! Optional git status integration for **f00**.
//!
//! Uses `git status --porcelain` as a lightweight subprocess MVP (no libgit2).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use f00_core::{Entry, GitStatus, Listing};

/// Snapshot of git status for paths under a repository root.
#[derive(Debug, Default, Clone)]
pub struct GitIndex {
    /// Absolute or relative paths → status. Keys are normalized relative to repo root.
    statuses: HashMap<PathBuf, GitStatus>,
    repo_root: Option<PathBuf>,
}

impl GitIndex {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn repo_root(&self) -> Option<&Path> {
        self.repo_root.as_deref()
    }

    pub fn status_for(&self, path: &Path) -> GitStatus {
        // Try exact, then file name relative to repo.
        if let Some(st) = self.statuses.get(path) {
            return *st;
        }
        if let Some(root) = &self.repo_root {
            if let Ok(rel) = path.strip_prefix(root) {
                if let Some(st) = self.statuses.get(rel) {
                    return *st;
                }
            }
        }
        // Basename fallback
        if let Some(name) = path.file_name() {
            for (k, v) in &self.statuses {
                if k.file_name() == Some(name) && k.components().count() == 1 {
                    return *v;
                }
            }
        }
        GitStatus::Clean
    }

    /// Discover repo from `start` and load porcelain status.
    pub fn discover(start: &Path) -> Self {
        let root = match find_repo_root(start) {
            Some(r) => r,
            None => return Self::empty(),
        };

        let output = Command::new("git")
            .args(["status", "--porcelain", "-uall"])
            .current_dir(&root)
            .output();

        let output = match output {
            Ok(o) if o.status.success() => o,
            _ => {
                return Self {
                    statuses: HashMap::new(),
                    repo_root: Some(root),
                };
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut statuses = HashMap::new();

        for line in stdout.lines() {
            if line.len() < 3 {
                continue;
            }
            let code = &line[..2];
            let rest = line[3..].trim();
            // Handle renames: `R  old -> new`
            let path_str = if let Some((left, _right)) = rest.split_once(" -> ") {
                left.trim()
            } else {
                rest
            };
            // Strip optional quotes
            let path_str = path_str.trim_matches('"');
            let rel = PathBuf::from(path_str);
            let st = parse_porcelain_code(code);
            statuses.insert(rel.clone(), st);
            statuses.insert(root.join(&rel), st);
        }

        Self {
            statuses,
            repo_root: Some(root),
        }
    }
}

/// Parse the two-character porcelain XY code into a single simplified status.
fn parse_porcelain_code(code: &str) -> GitStatus {
    let chars: Vec<char> = code.chars().collect();
    let x = chars.first().copied().unwrap_or(' ');
    let y = chars.get(1).copied().unwrap_or(' ');

    // Prefer worktree (Y) then index (X).
    for c in [y, x] {
        match c {
            'M' => return GitStatus::Modified,
            'A' => return GitStatus::Added,
            'D' => return GitStatus::Deleted,
            'R' | 'C' => return GitStatus::Renamed,
            'U' => return GitStatus::Conflicted,
            '?' => return GitStatus::Untracked,
            '!' => return GitStatus::Ignored,
            _ => {}
        }
    }
    GitStatus::Unknown
}

/// Walk up from `start` looking for a `.git` directory/file.
pub fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let start = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };
    let abs = std::fs::canonicalize(&start).unwrap_or(start);
    let mut cur = abs.as_path();
    loop {
        let git = cur.join(".git");
        if git.exists() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
}

/// Annotate listing entries with git status in place.
pub fn annotate_listing(listing: &mut Listing, index: &GitIndex) {
    for entry in &mut listing.entries {
        if entry.is_dir_header {
            continue;
        }
        entry.git_status = index.status_for(&entry.path);
    }
}

/// Annotate many listings; **one** porcelain map per git repo root (reused).
pub fn annotate_listings(listings: &mut [Listing]) {
    use std::collections::HashMap;
    use std::path::PathBuf;

    // Cache GitIndex by discovered repo root (or by listing root when not in a repo).
    let mut by_repo: HashMap<PathBuf, GitIndex> = HashMap::new();
    for listing in listings {
        let key = find_repo_root(&listing.root).unwrap_or_else(|| listing.root.clone());
        let index = by_repo
            .entry(key)
            .or_insert_with(|| GitIndex::discover(&listing.root));
        annotate_listing(listing, index);
    }
}

/// Convenience: annotate a free list of entries using a starting path for discovery.
pub fn annotate_entries(entries: &mut [Entry], start: &Path) {
    let index = GitIndex::discover(start);
    for entry in entries {
        if !entry.is_dir_header {
            entry.git_status = index.status_for(&entry.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_modified() {
        assert_eq!(parse_porcelain_code(" M"), GitStatus::Modified);
        assert_eq!(parse_porcelain_code("M "), GitStatus::Modified);
        assert_eq!(parse_porcelain_code("MM"), GitStatus::Modified);
    }

    #[test]
    fn parse_untracked() {
        assert_eq!(parse_porcelain_code("??"), GitStatus::Untracked);
    }

    #[test]
    fn parse_added() {
        assert_eq!(parse_porcelain_code("A "), GitStatus::Added);
    }

    #[test]
    fn find_repo_from_workspace() {
        // Prefer a disposable repo so CARGO_MANIFEST_DIR path moves (e.g. rename)
        // never leave a baked-in path without a .git.
        let base = std::env::temp_dir().join(format!(
            "f00-git-find-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(base.join("nested/deep")).unwrap();
        std::fs::create_dir_all(base.join(".git")).unwrap();
        let found = find_repo_root(&base.join("nested/deep")).expect("repo root");
        // canonicalize both sides: macOS /var → /private/var, Windows \\?\ prefixes.
        let found_c = std::fs::canonicalize(&found).unwrap_or(found);
        let base_c = std::fs::canonicalize(&base).unwrap_or(base.clone());
        assert_eq!(found_c, base_c);
        let _ = std::fs::remove_dir_all(&base);

        // Also accept the live workspace when present (non-fatal if path moved).
        let _ = find_repo_root(Path::new(env!("CARGO_MANIFEST_DIR")));
    }
}
