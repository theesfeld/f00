use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::entry::Entry;
use crate::error::{Error, Result};
use crate::filter::filter_entries;
use crate::options::ListOptions;
use crate::sort::sort_entries;

/// A listing produced for one path argument (or recursive tree).
#[derive(Debug, Clone)]
pub struct Listing {
    /// The path that was requested.
    pub root: PathBuf,
    /// Whether `root` itself is a directory.
    pub root_is_dir: bool,
    /// Entries to display. For recursive mode, may include dir headers.
    pub entries: Vec<Entry>,
}

/// List a single path: if file, return that entry alone; if dir, list children.
pub fn list_path(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let path = if path.as_os_str().is_empty() {
        Path::new(".")
    } else {
        path
    };

    let meta = fs::symlink_metadata(path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            Error::NotFound(path.to_path_buf())
        } else {
            Error::Metadata {
                path: path.to_path_buf(),
                source,
            }
        }
    })?;

    if !meta.is_dir() {
        let entry = Entry::from_path_and_meta(path, &meta, 0)?;
        return Ok(Listing {
            root: path.to_path_buf(),
            root_is_dir: false,
            entries: vec![entry],
        });
    }

    if opts.recursive {
        list_recursive(path, opts)
    } else {
        list_directory(path, opts)
    }
}

/// Non-recursive directory listing.
pub fn list_directory(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let mut entries = Vec::new();

    if opts.all {
        // Synthesize `.` and `..` like traditional ls -a.
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

    let read = fs::read_dir(path).map_err(|source| Error::ReadDir {
        path: path.to_path_buf(),
        source,
    })?;

    for item in read {
        let item = item.map_err(|source| Error::ReadDir {
            path: path.to_path_buf(),
            source,
        })?;
        let entry_path = item.path();
        let meta = item.metadata().or_else(|_| fs::symlink_metadata(&entry_path));
        let meta = match meta {
            Ok(m) => m,
            Err(_) => continue,
        };
        match Entry::from_path_and_meta(&entry_path, &meta, 0) {
            Ok(e) => entries.push(e),
            Err(_) => continue,
        }
    }

    filter_entries(&mut entries, opts);
    sort_entries(&mut entries, opts);

    Ok(Listing {
        root: path.to_path_buf(),
        root_is_dir: true,
        entries,
    })
}

/// Basic recursive listing using walkdir.
pub fn list_recursive(path: &Path, opts: &ListOptions) -> Result<Listing> {
    let mut entries = Vec::new();
    let max_depth = opts.max_depth.unwrap_or(usize::MAX);

    // Group by parent directory: emit header then children for each dir.
    // First, collect all walkdir results.
    let walker = WalkDir::new(path)
        .follow_links(opts.follow_links)
        .max_depth(max_depth)
        .sort_by_file_name();

    // Track which directories we've seen to emit headers.
    let mut current_dir: Option<PathBuf> = None;

    for item in walker {
        let item = match item {
            Ok(i) => i,
            Err(_) => continue,
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
                // Only emit header if not already the one we just opened from depth==0 handling
                // and this is a directory we're entering... walkdir visits dir before children.
            }
        }

        // When we visit a directory (depth > 0), emit a header before its children.
        // Walkdir yields the directory node first.
        if item.file_type().is_dir() && depth > 0 {
            // Directory as an entry under parent listing, AND as a future header.
            if let Ok(meta) = item.metadata() {
                if let Ok(e) = Entry::from_path_and_meta(entry_path, &meta, depth) {
                    // Will be filtered/sorted later per-section; collect flat for now.
                    let show = crate::filter::should_show(&e, opts);
                    if show {
                        entries.push(e);
                    }
                }
            }
            // Header for this dir's children (walkdir will visit them next)
            // Insert header after the dir entry itself so tree-ish recursive ls looks right.
            // Classic `ls -R` prints `\n./subdir:\n` then contents.
            entries.push(Entry::dir_header(entry_path, depth));
            continue;
        }

        let meta = match item.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if let Ok(e) = Entry::from_path_and_meta(entry_path, &meta, depth) {
            if crate::filter::should_show(&e, opts) {
                entries.push(e);
            }
        }
    }

    // For recursive mode, sort within sections delimited by headers.
    sort_recursive_sections(&mut entries, opts);

    Ok(Listing {
        root: path.to_path_buf(),
        root_is_dir: true,
        entries,
    })
}

fn sort_recursive_sections(entries: &mut Vec<Entry>, opts: &ListOptions) {
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

/// List multiple paths, returning one Listing per path.
pub fn list_paths(paths: &[PathBuf], opts: &ListOptions) -> Result<Vec<Listing>> {
    if paths.is_empty() {
        return Ok(vec![list_path(Path::new("."), opts)?]);
    }

    let mut out = Vec::with_capacity(paths.len());
    for p in paths {
        out.push(list_path(p, opts)?);
    }
    Ok(out)
}
