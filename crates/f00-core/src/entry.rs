use std::fs::{FileType, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local};

use crate::error::{Error, Result};

/// High-level file kind used for display and sorting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

impl EntryKind {
    pub fn from_file_type(ft: FileType) -> Self {
        if ft.is_dir() {
            Self::Directory
        } else if ft.is_symlink() {
            Self::Symlink
        } else if ft.is_file() {
            Self::File
        } else {
            Self::Other
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::Symlink => "symlink",
            Self::Other => "other",
        }
    }
}

/// Optional git status annotation (filled by f00-git when enabled).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GitStatus {
    #[default]
    Clean,
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Ignored,
    Conflicted,
    Unknown,
}

impl GitStatus {
    pub fn as_char(self) -> Option<char> {
        match self {
            Self::Clean => None,
            Self::Modified => Some('M'),
            Self::Added => Some('A'),
            Self::Deleted => Some('D'),
            Self::Renamed => Some('R'),
            Self::Untracked => Some('?'),
            Self::Ignored => Some('!'),
            Self::Conflicted => Some('U'),
            Self::Unknown => Some(' '),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Modified => "modified",
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Renamed => "renamed",
            Self::Untracked => "untracked",
            Self::Ignored => "ignored",
            Self::Conflicted => "conflicted",
            Self::Unknown => "unknown",
        }
    }
}

/// A single filesystem entry ready for formatting.
#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub kind: EntryKind,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    /// Permission mode bits (unix) or 0 on platforms without them.
    pub mode: u32,
    pub readonly: bool,
    pub symlink_target: Option<PathBuf>,
    pub depth: usize,
    pub git_status: GitStatus,
    /// True when this entry is a directory listing header (for recursive mode).
    pub is_dir_header: bool,
}

impl Entry {
    pub fn from_path(path: impl AsRef<Path>, depth: usize) -> Result<Self> {
        let path = path.as_ref();
        let meta = std::fs::symlink_metadata(path).map_err(|source| Error::Metadata {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_path_and_meta(path, &meta, depth)
    }

    pub fn from_path_and_meta(path: &Path, meta: &Metadata, depth: usize) -> Result<Self> {
        let file_type = meta.file_type();
        let kind = EntryKind::from_file_type(file_type);
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());

        let symlink_target = if file_type.is_symlink() {
            std::fs::read_link(path).ok()
        } else {
            None
        };

        let mode = file_mode(meta);

        Ok(Self {
            path: path.to_path_buf(),
            name,
            kind,
            size: meta.len(),
            modified: meta.modified().ok(),
            created: meta.created().ok(),
            accessed: meta.accessed().ok(),
            mode,
            readonly: meta.permissions().readonly(),
            symlink_target,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: false,
        })
    }

    /// Create a synthetic directory header for recursive listings.
    pub fn dir_header(path: impl AsRef<Path>, depth: usize) -> Self {
        let path = path.as_ref().to_path_buf();
        let name = path.to_string_lossy().into_owned();
        Self {
            path,
            name,
            kind: EntryKind::Directory,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            mode: 0,
            readonly: false,
            symlink_target: None,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: true,
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }

    pub fn modified_datetime(&self) -> Option<DateTime<Local>> {
        self.modified.map(DateTime::<Local>::from)
    }

    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.name).extension().and_then(|e| e.to_str())
    }
}

#[cfg(unix)]
fn file_mode(meta: &Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode()
}

#[cfg(not(unix))]
fn file_mode(_meta: &Metadata) -> u32 {
    0
}
