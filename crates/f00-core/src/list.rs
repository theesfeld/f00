use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
// jwalk for parallel recursive walks.

use crate::entry::Entry;
use crate::error::{Error, Result};
use crate::filter::filter_entries;
use crate::ignore::{apply_ignore_set, load_ignore_set, IgnoreSet};
use crate::options::{CliSymlinkMode, ListOptions};
use crate::sort::sort_entries;

/// Phase timings for a single listing (milliseconds).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListTiming {
    /// Time spent in `readdir` / collecting directory entries.
    pub readdir_ms: u128,
    /// Time spent stating paths and building [`Entry`] values.
    pub stat_ms: u128,
    /// Time spent filtering and sorting.
    pub sort_ms: u128,
}

impl ListTiming {
    /// Sum another timing into this one (for multi-path runs).
    pub fn add_assign(&mut self, other: &ListTiming) {
        self.readdir_ms = self.readdir_ms.saturating_add(other.readdir_ms);
        self.stat_ms = self.stat_ms.saturating_add(other.stat_ms);
        self.sort_ms = self.sort_ms.saturating_add(other.sort_ms);
    }
}

/// A listing produced for one path argument (or recursive tree).
#[derive(Debug, Clone)]
pub struct Listing {
    /// The path that was requested.
    pub root: PathBuf,
    /// Whether `root` itself is a directory.
    pub root_is_dir: bool,
    /// Entries to display. For recursive mode, may include dir headers.
    pub entries: Vec<Entry>,
    /// Recoverable problems while listing (e.g. unreadable subdirectory).
    /// Surfaces as process exit code 1 when non-zero.
    pub minor_errors: usize,
    /// Optional phase timings when [`ListOptions::collect_timing`] is set.
    pub timing: Option<ListTiming>,
}

impl Listing {
    fn new(root: PathBuf, root_is_dir: bool, entries: Vec<Entry>) -> Self {
        Self {
            root,
            root_is_dir,
            entries,
            minor_errors: 0,
            timing: None,
        }
    }

    fn with_timing(mut self, timing: Option<ListTiming>) -> Self {
        self.timing = timing;
        self
    }
}

/// Decide whether to follow a CLI path argument that is a symlink.
fn follow_cli_path(path: &Path, opts: &ListOptions) -> bool {
    if opts.follow_links {
        return true;
    }
    match opts.cli_symlink {
        CliSymlinkMode::Never => false,
        CliSymlinkMode::Always => {
            // `-H`: follow command-line symlinks.
            fs::symlink_metadata(path)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
        }
        CliSymlinkMode::DirOnly => {
            // Follow only when the symlink resolves to a directory.
            let is_link = fs::symlink_metadata(path)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);
            if !is_link {
                return false;
            }
            fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
        }
    }
}

/// List a single path: if file, return that entry alone; if dir, list children.
///
/// With `-d` / `directory`, list the path itself even when it is a directory.
pub fn list_path(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let path = if path.as_os_str().is_empty() {
        Path::new(".")
    } else {
        path
    };

    let follow = follow_cli_path(path, opts);

    let meta = if follow {
        fs::metadata(path)
    } else {
        fs::symlink_metadata(path)
    }
    .map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            Error::NotFound(path.to_path_buf())
        } else {
            Error::Metadata {
                path: path.to_path_buf(),
                source,
            }
        }
    })?;

    // `-d`: list the directory entry itself, not its contents.
    if opts.directory || !meta.is_dir() {
        let fill = meta_fill_from(opts);
        let entry = if follow {
            Entry::from_path_follow_with(path, 0, fill)?
        } else {
            Entry::from_path_and_meta_with(path, &meta, 0, fill)?
        };
        return Ok(Listing::new(path.to_path_buf(), meta.is_dir(), vec![entry]));
    }

    if opts.recursive {
        list_recursive(path, opts)
    } else {
        list_directory(path, opts)
    }
}

fn meta_fill_from(opts: &ListOptions) -> crate::entry::MetaFill {
    crate::entry::MetaFill {
        resolve_names: opts.resolve_owner_group,
        read_context: opts.read_selinux,
    }
}

/// Build an [`Entry`] from a readdir item.
fn entry_from_dir_entry(
    item: &fs::DirEntry,
    follow_links: bool,
    fill: crate::entry::MetaFill,
    prefer_statx: bool,
) -> Option<Entry> {
    let entry_path = item.path();
    if follow_links {
        return Entry::from_path_follow_with(&entry_path, 0, fill).ok();
    }
    #[cfg(target_os = "linux")]
    {
        if prefer_statx {
            if let Ok(e) = crate::linux_statx::entry_from_statx(&entry_path, 0, fill) {
                return Some(e);
            }
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = prefer_statx;
    }
    let meta = item
        .metadata()
        .or_else(|_| fs::symlink_metadata(&entry_path));
    match meta {
        Ok(m) => Entry::from_path_and_meta_with(&entry_path, &m, 0, fill).ok(),
        Err(_) => None,
    }
}

/// Stat directory children, optionally in parallel with rayon (or io_uring).
fn stat_dir_entries(dir_entries: &[fs::DirEntry], opts: &ListOptions) -> Vec<Entry> {
    let follow = opts.follow_links;
    let fill = meta_fill_from(opts);
    let prefer_statx = opts.linux_statx;
    let map_one = |item: &fs::DirEntry| entry_from_dir_entry(item, follow, fill, prefer_statx);

    // Linux + feature: batch statx via io_uring for large dirs (no follow).
    #[cfg(all(target_os = "linux", feature = "io-uring"))]
    {
        if opts.io_uring
            && !follow
            && dir_entries.len() >= crate::io_uring_stat::IO_URING_THRESHOLD
            && !fill.resolve_names
            && !fill.read_context
        {
            let paths: Vec<_> = dir_entries.iter().map(|d| d.path()).collect();
            if let Some(entries) = crate::io_uring_stat::entries_from_paths_uring(&paths, fill) {
                // Preserve "same count roughly" — if many failures, fall back.
                if entries.len() * 10 >= paths.len() * 8 {
                    return entries;
                }
            }
        }
    }

    if !opts.use_parallel_stat(dir_entries.len()) {
        return dir_entries.iter().filter_map(map_one).collect();
    }

    let collect = || dir_entries.par_iter().filter_map(map_one).collect();

    if opts.threads > 1 {
        match rayon::ThreadPoolBuilder::new()
            .num_threads(opts.threads)
            .build()
        {
            Ok(pool) => pool.install(collect),
            Err(_) => collect(),
        }
    } else {
        collect()
    }
}

/// Non-recursive directory listing.
pub fn list_directory(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let collect_timing = opts.collect_timing;
    let mut timing = ListTiming::default();

    let mut entries = Vec::new();

    let fill = meta_fill_from(opts);
    // Synthesize `.` and `..` sequentially (must not race with parallel stat).
    if opts.all {
        if let Ok(dot) = Entry::from_path_with(path, 0, fill) {
            let mut e = dot;
            e.name = ".".to_string();
            entries.push(e);
        }
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            if let Ok(mut e) = Entry::from_path_with(parent, 0, fill) {
                e.name = "..".to_string();
                e.path = parent.to_path_buf();
                entries.push(e);
            }
        } else if let Ok(mut e) = Entry::from_path_with(path, 0, fill) {
            // path is like `.` or `/` — still emit `..` best-effort
            e.name = "..".to_string();
            entries.push(e);
        }
    }

    let t_readdir = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    let read = fs::read_dir(path).map_err(|source| Error::ReadDir {
        path: path.to_path_buf(),
        source,
    })?;

    let mut dir_entries = Vec::new();
    for item in read {
        let item = item.map_err(|source| Error::ReadDir {
            path: path.to_path_buf(),
            source,
        })?;
        dir_entries.push(item);
    }

    if let Some(t0) = t_readdir {
        timing.readdir_ms = t0.elapsed().as_millis();
    }

    let t_stat = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    let children = stat_dir_entries(&dir_entries, opts);
    entries.extend(children);

    if let Some(t0) = t_stat {
        timing.stat_ms = t0.elapsed().as_millis();
    }

    let t_sort = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    filter_entries(&mut entries, opts);
    if opts.use_ignore_files {
        let set = load_ignore_set(path);
        apply_ignore_set(&mut entries, &set);
    }
    // Deterministic order: sort always runs after parallel collect.
    sort_entries(&mut entries, opts);

    if let Some(t0) = t_sort {
        timing.sort_ms = t0.elapsed().as_millis();
    }

    Ok(
        Listing::new(path.to_path_buf(), true, entries).with_timing(if collect_timing {
            Some(timing)
        } else {
            None
        }),
    )
}

/// Lightweight walk record (owned paths so we can parallelize metadata).
struct Walked {
    path: PathBuf,
    depth: usize,
    is_dir: bool,
}

/// Basic recursive listing using parallel walk (jwalk) + parallel / io_uring metadata.
pub fn list_recursive(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let collect_timing = opts.collect_timing;
    let mut timing = ListTiming::default();

    let mut minor_errors = 0usize;
    let max_depth = opts.max_depth.unwrap_or(usize::MAX);

    let root_ignore = if opts.use_ignore_files {
        Some(load_ignore_set(path))
    } else {
        None
    };

    let t_walk = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    // Phase 1: parallel directory walk (jwalk). sort(true) keeps name order per dir.
    // We avoid metadata() during the walk so stat can be batched / parallel next.
    let mut walked: Vec<Walked> = Vec::new();
    // skip_hidden(false): f00 applies its own -a/-A/hide rules after the walk.
    let mut jwalker = jwalk::WalkDir::new(path)
        .follow_links(opts.follow_links)
        .skip_hidden(false)
        .sort(true);
    if max_depth != usize::MAX {
        jwalker = jwalker.max_depth(max_depth);
    }

    for item in jwalker {
        let item = match item {
            Ok(i) => i,
            Err(_) => {
                minor_errors += 1;
                continue;
            }
        };
        let depth = item.depth();
        if depth == 0 {
            continue; // root is not a listed entry
        }
        let is_dir = item.file_type().is_dir();
        walked.push(Walked {
            path: item.path(),
            depth,
            is_dir,
        });
    }

    // jwalk may not surface every unreadable-directory error on the iterator;
    // probe dirs so `-R` still yields exit code 1 (GNU ls-compatible minor error).
    for w in &walked {
        if w.is_dir {
            if let Err(err) = fs::read_dir(&w.path) {
                let kind = err.kind();
                if matches!(
                    kind,
                    std::io::ErrorKind::PermissionDenied
                        | std::io::ErrorKind::NotFound
                        | std::io::ErrorKind::Other
                ) {
                    minor_errors += 1;
                }
            }
        }
    }

    if let Some(t0) = t_walk {
        timing.readdir_ms = t0.elapsed().as_millis();
    }

    let fill = meta_fill_from(opts);
    let t_stat = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    // Phase 2: metadata → Entry.
    // Prefer io_uring batch statx for large cheap listings (Linux + feature).
    let built: Vec<Option<Entry>> = {
        #[cfg(all(target_os = "linux", feature = "io-uring"))]
        {
            if opts.io_uring
                && !opts.follow_links
                && walked.len() >= crate::io_uring_stat::IO_URING_THRESHOLD
                && !fill.resolve_names
                && !fill.read_context
            {
                let paths: Vec<_> = walked.iter().map(|w| w.path.clone()).collect();
                if let Some(mut ents) = crate::io_uring_stat::entries_from_paths_uring(&paths, fill)
                {
                    if ents.len() * 10 >= paths.len() * 8 {
                        // Restore walk depths by path (uring builds depth=0).
                        use std::collections::HashMap;
                        let depth_by: HashMap<&Path, usize> =
                            walked.iter().map(|w| (w.path.as_path(), w.depth)).collect();
                        for e in &mut ents {
                            if let Some(&d) = depth_by.get(e.path.as_path()) {
                                e.depth = d;
                            }
                        }
                        // Re-order to walk order and filter.
                        let mut by_path: HashMap<PathBuf, Entry> =
                            ents.into_iter().map(|e| (e.path.clone(), e)).collect();
                        let ordered: Vec<Option<Entry>> = walked
                            .iter()
                            .map(|w| {
                                by_path
                                    .remove(&w.path)
                                    .filter(|e| crate::filter::should_show(e, opts))
                            })
                            .collect();
                        if let Some(t0) = t_stat {
                            timing.stat_ms = t0.elapsed().as_millis();
                        }
                        // Jump to phase 3 with ordered.
                        return finish_recursive(
                            path,
                            opts,
                            walked,
                            ordered,
                            root_ignore,
                            minor_errors,
                            timing,
                            collect_timing,
                        );
                    }
                }
            }
        }

        let use_parallel = opts.use_parallel_stat(walked.len().max(1));
        let map = |w: &Walked| {
            Entry::from_path_with(&w.path, w.depth, fill)
                .ok()
                .filter(|e| crate::filter::should_show(e, opts))
        };
        if use_parallel {
            if opts.threads > 1 {
                match rayon::ThreadPoolBuilder::new()
                    .num_threads(opts.threads)
                    .build()
                {
                    Ok(pool) => pool.install(|| walked.par_iter().map(map).collect()),
                    Err(_) => walked.par_iter().map(map).collect(),
                }
            } else {
                walked.par_iter().map(map).collect()
            }
        } else {
            walked.iter().map(map).collect()
        }
    };

    if let Some(t0) = t_stat {
        timing.stat_ms = t0.elapsed().as_millis();
    }

    finish_recursive(
        path,
        opts,
        walked,
        built,
        root_ignore,
        minor_errors,
        timing,
        collect_timing,
    )
}

#[allow(clippy::too_many_arguments)]
fn finish_recursive(
    path: &Path,
    opts: &ListOptions,
    walked: Vec<Walked>,
    built: Vec<Option<Entry>>,
    root_ignore: Option<IgnoreSet>,
    minor_errors: usize,
    mut timing: ListTiming,
    collect_timing: bool,
) -> Result<Listing> {
    // Phase 3: sequential ignore filter + optional dir headers (stable walk order).
    let mut ignore_by_dir: Vec<(PathBuf, IgnoreSet)> = Vec::new();
    let mut entries: Vec<Entry> = Vec::with_capacity(built.len().saturating_add(16));

    if opts.emit_dir_headers {
        entries.push(Entry::dir_header(path, 0));
    }

    for (w, maybe) in walked.iter().zip(built) {
        let Some(e) = maybe else {
            continue;
        };
        if ignored_by_sets(&e, opts, root_ignore.as_ref(), &mut ignore_by_dir) {
            continue;
        }
        if opts.emit_dir_headers && w.is_dir {
            // Directory node as a listable entry, then a section header for its children.
            entries.push(e);
            entries.push(Entry::dir_header(&w.path, w.depth));
        } else {
            entries.push(e);
        }
    }

    let t_sort = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    if opts.emit_dir_headers {
        sort_recursive_sections(&mut entries, opts);
    } else {
        // Tree path: walk order already name-sorted within each directory (WalkDir).
        // Re-sort siblings only if a non-name primary sort was requested.
        if !matches!(opts.sort_by, crate::options::SortBy::Name) || opts.reverse {
            sort_tree_preorder(&mut entries, opts);
        }
    }

    if let Some(t0) = t_sort {
        timing.sort_ms = t0.elapsed().as_millis();
    }

    let mut listing = Listing::new(path.to_path_buf(), true, entries)
        .with_timing(if collect_timing { Some(timing) } else { None });
    listing.minor_errors = minor_errors;
    Ok(listing)
}

/// Re-sort a preorder tree listing by re-grouping siblings after a non-name sort.
///
/// Keeps a valid preorder: each directory’s children stay contiguous under it.
fn sort_tree_preorder(entries: &mut [Entry], opts: &ListOptions) {
    if entries.is_empty() {
        return;
    }
    // Build groups by parent path; stable relative to walk.
    // Simple approach: sort by (parent, sort_key) while preserving depth structure
    // via full path component sort with custom comparator for the leaf name only.
    // For size/time sorts, sort siblings that share the same parent.
    let mut i = 0;
    while i < entries.len() {
        let depth = entries[i].depth;
        // Find run of direct children of the same parent starting at i… actually
        // at depth `depth`, a sibling run is contiguous until depth < depth.
        let parent = entries[i].path.parent().map(Path::to_path_buf);
        let mut j = i + 1;
        while j < entries.len() {
            if entries[j].depth < depth {
                break;
            }
            if entries[j].depth == depth {
                let p = entries[j].path.parent().map(Path::to_path_buf);
                if p != parent {
                    break;
                }
            }
            j += 1;
        }
        // Within [i, j), extract sibling indices at exactly `depth`.
        let sib: Vec<usize> = (i..j).filter(|&k| entries[k].depth == depth).collect();
        if sib.len() > 1 {
            // Sort sibling subtrees by moving whole subtree blocks.
            // Build blocks: each sibling at depth owns [start, next_sibling).
            let mut blocks: Vec<Vec<Entry>> = Vec::with_capacity(sib.len());
            for (bi, &start) in sib.iter().enumerate() {
                let end = sib.get(bi + 1).copied().unwrap_or(j);
                blocks.push(entries[start..end].to_vec());
            }
            // cmp_entry already applies reverse when set.
            blocks.sort_by(|a, b| crate::sort::cmp_entry(&a[0], &b[0], opts));
            let mut out = Vec::with_capacity(j - i);
            for b in blocks {
                out.extend(b);
            }
            entries[i..j].clone_from_slice(&out);
        }
        i = if j > i { j } else { i + 1 };
    }
}

/// Check root ignore set and the ignore file in the entry's parent directory.
fn ignored_by_sets(
    entry: &Entry,
    opts: &ListOptions,
    root_ignore: Option<&IgnoreSet>,
    cache: &mut Vec<(PathBuf, IgnoreSet)>,
) -> bool {
    if !opts.use_ignore_files {
        return false;
    }
    if let Some(root) = root_ignore {
        if root.ignores(entry) {
            return true;
        }
    }
    let Some(parent) = entry.path.parent() else {
        return false;
    };
    if let Some(root) = root_ignore {
        if parent == root.base {
            return false;
        }
    }
    if let Some((_, set)) = cache.iter().find(|(p, _)| p == parent) {
        return set.ignores(entry);
    }
    let set = load_ignore_set(parent);
    let ignored = set.ignores(entry);
    cache.push((parent.to_path_buf(), set));
    ignored
}

fn sort_recursive_sections(entries: &mut [Entry], opts: &ListOptions) {
    let mut start = 0;
    while start < entries.len() {
        // Find next header after start
        if entries[start].is_dir_header {
            let section_start = start + 1;
            let mut end = section_start;
            while end < entries.len() && !entries[end].is_dir_header {
                end += 1;
            }
            sort_entries(&mut entries[section_start..end], opts);
            start = end;
        } else {
            // Leading non-header run
            let mut end = start;
            while end < entries.len() && !entries[end].is_dir_header {
                end += 1;
            }
            sort_entries(&mut entries[start..end], opts);
            start = end;
        }
    }
}

/// Outcome of listing one or more path arguments.
#[derive(Debug)]
pub struct ListOutcome {
    pub listings: Vec<Listing>,
    /// Errors for command-line path arguments that could not be listed.
    pub path_errors: Vec<Error>,
    /// Sum of recoverable errors inside successful listings (e.g. unreadable subdirs).
    pub minor_errors: usize,
}

impl ListOutcome {
    /// GNU-aligned exit status: 0 ok, 1 minor problems, 2 serious (bad path args).
    pub fn exit_code(&self) -> i32 {
        if !self.path_errors.is_empty() {
            2
        } else if self.minor_errors > 0 || self.listings.iter().any(|l| l.minor_errors > 0) {
            1
        } else {
            0
        }
    }

    /// Aggregate phase timings across successful listings.
    pub fn total_timing(&self) -> ListTiming {
        let mut total = ListTiming::default();
        for l in &self.listings {
            if let Some(ref t) = l.timing {
                total.add_assign(t);
            }
        }
        total
    }
}

/// List multiple paths, returning one Listing per successful path.
///
/// Failures on individual path arguments are collected (serious) rather than
/// aborting the whole run, matching GNU `ls` multi-argument behavior.
pub fn list_paths(paths: &[PathBuf], opts: &ListOptions) -> Result<Vec<Listing>> {
    let outcome = list_paths_with_errors(paths, opts);
    if let Some(err) = outcome.path_errors.into_iter().next() {
        // Preserve previous fail-fast API for callers that expect `Result`.
        return Err(err);
    }
    Ok(outcome.listings)
}

/// List paths, collecting per-argument errors instead of failing fast.
pub fn list_paths_with_errors(paths: &[PathBuf], opts: &ListOptions) -> ListOutcome {
    let owned: Vec<PathBuf>;
    let paths: &[PathBuf] = if paths.is_empty() {
        owned = vec![PathBuf::from(".")];
        &owned
    } else {
        paths
    };

    let mut listings = Vec::with_capacity(paths.len());
    let mut path_errors = Vec::new();
    let mut minor_errors = 0usize;

    for p in paths {
        match list_path(p, opts) {
            Ok(l) => {
                minor_errors += l.minor_errors;
                listings.push(l);
            }
            Err(e) => path_errors.push(e),
        }
    }

    ListOutcome {
        listings,
        path_errors,
        minor_errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir() -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let base = std::env::temp_dir().join(format!(
            "f00-core-list-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&base).unwrap();
        base
    }

    #[test]
    fn list_paths_with_errors_partial_success() {
        let dir = temp_dir();
        fs::write(dir.join("a.txt"), b"x").unwrap();
        let missing = dir.join("gone");
        let opts = ListOptions::default();
        let outcome = list_paths_with_errors(&[dir.clone(), missing], &opts);
        assert_eq!(outcome.listings.len(), 1);
        assert_eq!(outcome.path_errors.len(), 1);
        assert_eq!(outcome.exit_code(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_paths_ok_exit_0() {
        let dir = temp_dir();
        fs::write(dir.join("a.txt"), b"x").unwrap();
        let opts = ListOptions::default();
        let outcome = list_paths_with_errors(std::slice::from_ref(&dir), &opts);
        assert!(outcome.path_errors.is_empty());
        assert_eq!(outcome.exit_code(), 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_directory_honors_ignore_files() {
        let dir = temp_dir();
        fs::write(dir.join("keep.txt"), b"k").unwrap();
        fs::write(dir.join("skip.o"), b"o").unwrap();
        fs::write(dir.join(".gitignore"), "*.o\n").unwrap();

        let opts = ListOptions {
            use_ignore_files: true,
            ..Default::default()
        };
        let listing = list_directory(&dir, &opts).unwrap();
        let names: Vec<_> = listing.entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"keep.txt"));
        assert!(!names.contains(&"skip.o"));

        let opts_off = ListOptions {
            use_ignore_files: false,
            ..Default::default()
        };
        let listing = list_directory(&dir, &opts_off).unwrap();
        let names: Vec<_> = listing.entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"skip.o"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parallel_list_same_names_as_sequential() {
        let dir = temp_dir();
        // Well above PARALLEL_STAT_THRESHOLD so parallel path is taken.
        let mut expected = Vec::new();
        for i in 0..64 {
            let name = format!("file_{i:03}.txt");
            fs::write(dir.join(&name), b"x").unwrap();
            expected.push(name);
        }
        fs::create_dir(dir.join("subdir")).unwrap();
        expected.push("subdir".into());
        expected.sort();

        let sequential = ListOptions {
            parallel: false,
            threads: 1,
            ..Default::default()
        };
        let seq = list_directory(&dir, &sequential).unwrap();
        let mut seq_names: Vec<_> = seq.entries.iter().map(|e| e.name.clone()).collect();
        seq_names.sort();
        assert_eq!(
            seq_names, expected,
            "listing must include every created entry"
        );

        // Skip rayon parallel path on FreeBSD CI VMs (SIGSEGV under qemu).
        if !cfg!(target_os = "freebsd") {
            let parallel = ListOptions {
                parallel: true,
                threads: 0,
                ..Default::default()
            };
            let par = list_directory(&dir, &parallel).unwrap();
            let mut par_names: Vec<_> = par.entries.iter().map(|e| e.name.clone()).collect();
            par_names.sort();
            assert_eq!(
                seq_names, par_names,
                "parallel and sequential must produce identical ordered names"
            );
        }

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn collect_timing_fills_phases() {
        let dir = temp_dir();
        for i in 0..8 {
            fs::write(dir.join(format!("f{i}")), b"x").unwrap();
        }
        let opts = ListOptions {
            collect_timing: true,
            parallel: false,
            threads: 1,
            ..Default::default()
        };
        let listing = list_directory(&dir, &opts).unwrap();
        let t = listing.timing.expect("timing present");
        // Just ensure fields are populated (may be 0ms on very fast FS).
        let _ = (t.readdir_ms, t.stat_ms, t.sort_ms);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn threads_one_forces_serial_path() {
        let dir = temp_dir();
        for i in 0..40 {
            fs::write(dir.join(format!("n{i}")), b"x").unwrap();
        }
        let opts = ListOptions {
            parallel: true,
            threads: 1,
            ..Default::default()
        };
        assert!(!opts.use_parallel_stat(40));
        let listing = list_directory(&dir, &opts).unwrap();
        assert_eq!(listing.entries.len(), 40);
        let _ = fs::remove_dir_all(&dir);
    }
}
