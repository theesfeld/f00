use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use walkdir::WalkDir;

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
        let entry = if follow {
            Entry::from_path_follow(path, 0)?
        } else {
            Entry::from_path_and_meta(path, &meta, 0)?
        };
        return Ok(Listing::new(path.to_path_buf(), meta.is_dir(), vec![entry]));
    }

    if opts.recursive {
        list_recursive(path, opts)
    } else {
        list_directory(path, opts)
    }
}

/// Build an [`Entry`] from a readdir item.
fn entry_from_dir_entry(item: &fs::DirEntry, follow_links: bool) -> Option<Entry> {
    let entry_path = item.path();
    if follow_links {
        Entry::from_path_follow(&entry_path, 0).ok()
    } else {
        let meta = item
            .metadata()
            .or_else(|_| fs::symlink_metadata(&entry_path));
        match meta {
            Ok(m) => Entry::from_path_and_meta(&entry_path, &m, 0).ok(),
            Err(_) => None,
        }
    }
}

/// Stat directory children, optionally in parallel with rayon.
fn stat_dir_entries(dir_entries: &[fs::DirEntry], opts: &ListOptions) -> Vec<Entry> {
    let follow = opts.follow_links;
    let map_one = |item: &fs::DirEntry| entry_from_dir_entry(item, follow);

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

    // Synthesize `.` and `..` sequentially (must not race with parallel stat).
    if opts.all {
        if let Ok(dot) = Entry::from_path(path, 0) {
            let mut e = dot;
            e.name = ".".to_string();
            entries.push(e);
        }
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            if let Ok(mut e) = Entry::from_path(parent, 0) {
                e.name = "..".to_string();
                e.path = parent.to_path_buf();
                entries.push(e);
            }
        } else if let Ok(mut e) = Entry::from_path(path, 0) {
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

/// Basic recursive listing using walkdir.
pub fn list_recursive(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let collect_timing = opts.collect_timing;
    let mut timing = ListTiming::default();
    let t_all = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    let mut entries = Vec::new();
    let mut minor_errors = 0usize;
    let max_depth = opts.max_depth.unwrap_or(usize::MAX);

    let root_ignore = if opts.use_ignore_files {
        Some(load_ignore_set(path))
    } else {
        None
    };
    let mut ignore_by_dir: Vec<(PathBuf, IgnoreSet)> = Vec::new();

    // Group by parent directory: emit header then children for each dir.
    // First, collect all walkdir results.
    // Recursive walk stays sequential: headers and section order must be stable.
    // (Parallelism lives in non-recursive `list_directory` where readdir is flat.)
    let walker = WalkDir::new(path)
        .follow_links(opts.follow_links)
        .max_depth(max_depth)
        .sort_by_file_name();

    // Track which directories we've seen to emit headers.
    let mut current_dir: Option<PathBuf> = None;

    for item in walker {
        let item = match item {
            Ok(i) => i,
            Err(_) => {
                minor_errors += 1;
                continue;
            }
        };

        let depth = item.depth();
        let entry_path = item.path();

        // Skip the root itself as an entry; we'll list its children under a header if needed.
        if depth == 0 {
            current_dir = Some(entry_path.to_path_buf());
            // Emit header for root
            entries.push(Entry::dir_header(entry_path, 0));
            continue;
        }

        let parent = entry_path.parent().map(|p| p.to_path_buf());
        if let Some(ref p) = parent {
            if current_dir.as_ref() != Some(p) {
                // New directory section
                current_dir = Some(p.clone());
            }
        }

        // When we visit a directory (depth > 0), emit a header before its children.
        // Walkdir yields the directory node first.
        if item.file_type().is_dir() && depth > 0 {
            if let Ok(meta) = item.metadata() {
                if let Ok(e) = Entry::from_path_and_meta(entry_path, &meta, depth) {
                    let show = crate::filter::should_show(&e, opts)
                        && !ignored_by_sets(&e, opts, root_ignore.as_ref(), &mut ignore_by_dir);
                    if show {
                        entries.push(e);
                    }
                }
            }
            entries.push(Entry::dir_header(entry_path, depth));
            continue;
        }

        let meta = match item.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if let Ok(e) = Entry::from_path_and_meta(entry_path, &meta, depth) {
            if crate::filter::should_show(&e, opts)
                && !ignored_by_sets(&e, opts, root_ignore.as_ref(), &mut ignore_by_dir)
            {
                entries.push(e);
            }
        }
    }

    if let Some(t0) = t_all {
        // Attribute walk+stat wall time primarily to readdir/stat combined.
        let ms = t0.elapsed().as_millis();
        timing.readdir_ms = ms / 2;
        timing.stat_ms = ms.saturating_sub(timing.readdir_ms);
    }

    let t_sort = if collect_timing {
        Some(Instant::now())
    } else {
        None
    };

    // For recursive mode, sort within sections delimited by headers.
    sort_recursive_sections(&mut entries, opts);

    if let Some(t0) = t_sort {
        timing.sort_ms = t0.elapsed().as_millis();
    }

    let mut listing = Listing::new(path.to_path_buf(), true, entries)
        .with_timing(if collect_timing { Some(timing) } else { None });
    listing.minor_errors = minor_errors;
    Ok(listing)
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
