//! Optional git status integration for **f00**.
//!
//! Uses `git status --porcelain` as a lightweight subprocess (no libgit2).
//! Results are cached per repo root and invalidated when `.git/index` mtimes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::SystemTime;

use f00_core::{Entry, GitStatus, Listing};

/// How aggressively to collect untracked files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UntrackedMode {
    /// `git status -uall` — every untracked path (default for flat listings).
    #[default]
    All,
    /// `git status -uno` — skip untracked (faster; good for `-R` / `--tree`).
    No,
}

/// Snapshot of git status for paths under a repository root.
#[derive(Debug, Default, Clone)]
pub struct GitIndex {
    /// Absolute or relative paths → status. Keys are normalized relative to repo root.
    statuses: HashMap<PathBuf, GitStatus>,
    repo_root: Option<PathBuf>,
    /// Index mtime used for cache invalidation (if available).
    #[allow(dead_code)]
    index_mtime: Option<SystemTime>,
    #[allow(dead_code)]
    untracked: UntrackedMode,
}

#[derive(Clone)]
struct CacheEntry {
    index: GitIndex,
    index_mtime: Option<SystemTime>,
    untracked: UntrackedMode,
}

fn global_cache() -> &'static Mutex<HashMap<PathBuf, CacheEntry>> {
    use std::sync::OnceLock;
    static CACHE: OnceLock<Mutex<HashMap<PathBuf, CacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn index_mtime(repo: &Path) -> Option<SystemTime> {
    let index = repo.join(".git/index");
    std::fs::metadata(index).and_then(|m| m.modified()).ok()
}

impl GitIndex {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn repo_root(&self) -> Option<&Path> {
        self.repo_root.as_deref()
    }

    pub fn status_for(&self, path: &Path) -> GitStatus {
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
        // Basename fallback (only single-component keys)
        if let Some(name) = path.file_name() {
            for (k, v) in &self.statuses {
                if k.file_name() == Some(name) && k.components().count() == 1 {
                    return *v;
                }
            }
        }
        GitStatus::Clean
    }

    /// Discover repo from `start` and load porcelain status (cached).
    pub fn discover(start: &Path) -> Self {
        Self::discover_with(start, UntrackedMode::All)
    }

    /// Discover with untracked policy (uses process-wide cache).
    pub fn discover_with(start: &Path, untracked: UntrackedMode) -> Self {
        let root = match find_repo_root(start) {
            Some(r) => r,
            None => return Self::empty(),
        };

        let mtime = index_mtime(&root);
        if let Ok(cache) = global_cache().lock() {
            if let Some(hit) = cache.get(&root) {
                if hit.untracked == untracked && hit.index_mtime == mtime {
                    return hit.index.clone();
                }
            }
        }

        let index = Self::load_fresh(&root, mtime, untracked);
        if let Ok(mut cache) = global_cache().lock() {
            cache.insert(
                root.clone(),
                CacheEntry {
                    index: index.clone(),
                    index_mtime: mtime,
                    untracked,
                },
            );
            // Soft cap so long-lived CLIs don't grow unbounded across many repos.
            if cache.len() > 32 {
                // Drop an arbitrary old entry (HashMap iter order is fine here).
                if let Some(k) = cache.keys().next().cloned() {
                    cache.remove(&k);
                }
            }
        }
        index
    }

    fn load_fresh(root: &Path, mtime: Option<SystemTime>, untracked: UntrackedMode) -> Self {
        let uflag = match untracked {
            UntrackedMode::All => "-uall",
            UntrackedMode::No => "-uno",
        };
        let output = Command::new("git")
            .args(["status", "--porcelain", "--ignored=no", uflag])
            .current_dir(root)
            .output();

        let output = match output {
            Ok(o) if o.status.success() => o,
            _ => {
                return Self {
                    statuses: HashMap::new(),
                    repo_root: Some(root.to_path_buf()),
                    index_mtime: mtime,
                    untracked,
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
            let path_str = if let Some((left, _right)) = rest.split_once(" -> ") {
                left.trim()
            } else {
                rest
            };
            let path_str = path_str.trim_matches('"');
            let rel = PathBuf::from(path_str);
            let st = parse_porcelain_code(code);
            statuses.insert(rel.clone(), st);
            statuses.insert(root.join(&rel), st);
        }

        Self {
            statuses,
            repo_root: Some(root.to_path_buf()),
            index_mtime: mtime,
            untracked,
        }
    }
}

/// Parse the two-character porcelain XY code into a single simplified status.
fn parse_porcelain_code(code: &str) -> GitStatus {
    let chars: Vec<char> = code.chars().collect();
    let x = chars.first().copied().unwrap_or(' ');
    let y = chars.get(1).copied().unwrap_or(' ');

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

/// Annotate many listings; **one** porcelain map per git repo root (reused + process cache).
///
/// When any listing is recursive (depth>0 entries or dir headers beyond root), use
/// [`UntrackedMode::No`] for that repo to avoid scanning every untracked path.
pub fn annotate_listings(listings: &mut [Listing]) {
    use std::collections::HashMap;

    let mut by_repo: HashMap<PathBuf, GitIndex> = HashMap::new();
    for listing in listings.iter_mut() {
        let recursive = listing
            .entries
            .iter()
            .any(|e| e.depth > 0 || e.is_dir_header);
        let mode = if recursive {
            UntrackedMode::No
        } else {
            UntrackedMode::All
        };
        let key = find_repo_root(&listing.root).unwrap_or_else(|| listing.root.clone());
        let index = by_repo
            .entry(key)
            .or_insert_with(|| GitIndex::discover_with(&listing.root, mode));
        annotate_listing(listing, index);
    }
}

/// Convenience: annotate a free list of entries using a starting path for discovery.
pub fn annotate_entries(entries: &mut [Entry], start: &Path) {
    let recursive = entries.iter().any(|e| e.depth > 0);
    let mode = if recursive {
        UntrackedMode::No
    } else {
        UntrackedMode::All
    };
    let index = GitIndex::discover_with(start, mode);
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
        let found_c = std::fs::canonicalize(&found).unwrap_or(found);
        let base_c = std::fs::canonicalize(&base).unwrap_or(base.clone());
        assert_eq!(found_c, base_c);
        let _ = std::fs::remove_dir_all(&base);
        let _ = find_repo_root(Path::new(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn cache_returns_same_for_second_discover() {
        let base = std::env::temp_dir().join(format!(
            "f00-git-cache-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(base.join(".git")).unwrap();
        // No real git repo — discover returns empty statuses but still caches root.
        let a = GitIndex::discover_with(&base, UntrackedMode::No);
        let b = GitIndex::discover_with(&base, UntrackedMode::No);
        assert_eq!(a.repo_root(), b.repo_root());
        let _ = std::fs::remove_dir_all(&base);
    }
}
