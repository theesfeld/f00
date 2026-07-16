use std::collections::HashMap;
use std::fs::{FileType, Metadata};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use chrono::{DateTime, Local};

use crate::error::{Error, Result};

/// Which expensive metadata fields to populate when building an [`Entry`].
#[derive(Debug, Clone, Copy, Default)]
pub struct MetaFill {
    /// Resolve uid/gid via NSS (getpwuid/getgrgid). Cached process-wide.
    pub resolve_names: bool,
    /// Read `security.selinux` xattr (Linux).
    pub read_context: bool,
}

impl MetaFill {
    /// Full long-format fill (names + optional SELinux when requested separately).
    pub fn rich(read_context: bool) -> Self {
        Self {
            resolve_names: true,
            read_context,
        }
    }

    /// Minimal fill for short listings / machine formats that only need path+size+kind.
    pub fn cheap() -> Self {
        Self::default()
    }
}

/// Process-wide caches for uid/gid → name (avoids repeated NSS in long mode).
fn owner_cache() -> &'static Mutex<HashMap<u32, String>> {
    use std::sync::OnceLock;
    static CACHE: OnceLock<Mutex<HashMap<u32, String>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn group_cache() -> &'static Mutex<HashMap<u32, String>> {
    use std::sync::OnceLock;
    static CACHE: OnceLock<Mutex<HashMap<u32, String>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

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

/// Which timestamp is primary for display / sort (`ls --time`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeField {
    #[default]
    Modified,
    Accessed,
    Changed,
    Birth,
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
    /// Status-change time (`st_ctime`) when available.
    pub changed: Option<SystemTime>,
    /// Permission mode bits (unix) or 0 on platforms without them.
    pub mode: u32,
    pub readonly: bool,
    pub symlink_target: Option<PathBuf>,
    pub depth: usize,
    pub git_status: GitStatus,
    /// True when this entry is a directory listing header (for recursive mode).
    pub is_dir_header: bool,
    /// Hard link count (`st_nlink`).
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub inode: u64,
    /// Allocated blocks in 512-byte units (GNU `ls -s` style) when known.
    pub blocks: u64,
    /// Owner name (or numeric string).
    pub owner: String,
    /// Group name (or numeric string).
    pub group: String,
    /// Author (GNU `--author`); typically same as owner on Linux.
    pub author: String,
    /// SELinux security context (`-Z`); empty if unavailable.
    pub context: String,
}

impl Entry {
    pub fn from_path(path: impl AsRef<Path>, depth: usize) -> Result<Self> {
        Self::from_path_with(path, depth, MetaFill::default())
    }

    pub fn from_path_with(path: impl AsRef<Path>, depth: usize, fill: MetaFill) -> Result<Self> {
        let path = path.as_ref();
        let meta = std::fs::symlink_metadata(path).map_err(|source| Error::Metadata {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_path_and_meta_with(path, &meta, depth, fill)
    }

    /// Like [`from_path`] but follow the final symlink target for metadata (`-L`).
    pub fn from_path_follow(path: impl AsRef<Path>, depth: usize) -> Result<Self> {
        Self::from_path_follow_with(path, depth, MetaFill::default())
    }

    pub fn from_path_follow_with(
        path: impl AsRef<Path>,
        depth: usize,
        fill: MetaFill,
    ) -> Result<Self> {
        let path = path.as_ref();
        let meta = std::fs::metadata(path).map_err(|source| Error::Metadata {
            path: path.to_path_buf(),
            source,
        })?;
        let mut entry = Self::from_path_and_meta_with(path, &meta, depth, fill)?;
        // Keep symlink name but drop "-> target" when showing dereference.
        if entry.kind == EntryKind::Symlink {
            entry.kind = EntryKind::from_file_type(meta.file_type());
            entry.symlink_target = None;
        }
        Ok(entry)
    }

    pub fn from_path_and_meta(path: &Path, meta: &Metadata, depth: usize) -> Result<Self> {
        Self::from_path_and_meta_with(path, meta, depth, MetaFill::default())
    }

    pub fn from_path_and_meta_with(
        path: &Path,
        meta: &Metadata,
        depth: usize,
        fill: MetaFill,
    ) -> Result<Self> {
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
        let (nlink, uid, gid, inode, blocks) = meta_ids(meta);
        let (owner, group) = if fill.resolve_names {
            (resolve_owner_cached(uid), resolve_group_cached(gid))
        } else {
            (String::new(), String::new())
        };
        let changed = ctime_of(meta);
        let context = if fill.read_context {
            read_selinux_context(path)
        } else {
            String::new()
        };

        Ok(Self {
            path: path.to_path_buf(),
            name,
            kind,
            size: meta.len(),
            modified: meta.modified().ok(),
            created: meta.created().ok(),
            accessed: meta.accessed().ok(),
            changed,
            mode,
            readonly: meta.permissions().readonly(),
            symlink_target,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink,
            uid,
            gid,
            inode,
            blocks,
            author: owner.clone(),
            owner,
            group,
            context,
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
            changed: None,
            mode: 0,
            readonly: false,
            symlink_target: None,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: true,
            nlink: 0,
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

    pub fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }

    pub fn modified_datetime(&self) -> Option<DateTime<Local>> {
        self.modified.map(DateTime::<Local>::from)
    }

    pub fn accessed_datetime(&self) -> Option<DateTime<Local>> {
        self.accessed.map(DateTime::<Local>::from)
    }

    pub fn created_datetime(&self) -> Option<DateTime<Local>> {
        self.created.map(DateTime::<Local>::from)
    }

    pub fn changed_datetime(&self) -> Option<DateTime<Local>> {
        self.changed.map(DateTime::<Local>::from)
    }

    pub fn time_for(&self, field: TimeField) -> Option<SystemTime> {
        match field {
            TimeField::Modified => self.modified,
            TimeField::Accessed => self.accessed,
            TimeField::Changed => self.changed.or(self.modified),
            TimeField::Birth => self.created.or(self.modified),
        }
    }

    pub fn datetime_for(&self, field: TimeField) -> Option<DateTime<Local>> {
        self.time_for(field).map(DateTime::<Local>::from)
    }

    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.name).extension().and_then(|e| e.to_str())
    }

    /// Owner string for display (`-n` forces numeric).
    pub fn owner_display(&self, numeric: bool) -> String {
        if numeric {
            self.uid.to_string()
        } else {
            self.owner.clone()
        }
    }

    pub fn group_display(&self, numeric: bool) -> String {
        if numeric {
            self.gid.to_string()
        } else {
            self.group.clone()
        }
    }

    pub fn author_display(&self, numeric: bool) -> String {
        if numeric {
            self.uid.to_string()
        } else if !self.author.is_empty() {
            self.author.clone()
        } else {
            self.owner_display(numeric)
        }
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

#[cfg(unix)]
fn meta_ids(meta: &Metadata) -> (u64, u32, u32, u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (
        meta.nlink(),
        meta.uid(),
        meta.gid(),
        meta.ino(),
        meta.blocks(),
    )
}

#[cfg(not(unix))]
fn meta_ids(meta: &Metadata) -> (u64, u32, u32, u64, u64) {
    let size = meta.len();
    let blocks = size.div_ceil(512);
    (1, 0, 0, 0, blocks)
}

#[cfg(unix)]
fn ctime_of(meta: &Metadata) -> Option<SystemTime> {
    use std::os::unix::fs::MetadataExt;
    use std::time::Duration;
    // st_ctime is seconds since epoch; st_ctime_nsec for subsecond.
    let secs = meta.ctime();
    if secs < 0 {
        return None;
    }
    let nsec = meta.ctime_nsec();
    let nsec = if (0..1_000_000_000).contains(&nsec) {
        nsec as u32
    } else {
        0
    };
    Some(SystemTime::UNIX_EPOCH + Duration::new(secs as u64, nsec))
}

#[cfg(not(unix))]
fn ctime_of(meta: &Metadata) -> Option<SystemTime> {
    // Windows has no ctime-as-status-change; fall back to modified.
    meta.modified().ok()
}

/// Best-effort SELinux context via `security.selinux` xattr.
fn read_selinux_context(path: &Path) -> String {
    #[cfg(unix)]
    {
        read_selinux_context_unix(path).unwrap_or_default()
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        String::new()
    }
}

#[cfg(target_os = "linux")]
fn read_selinux_context_unix(path: &Path) -> Option<String> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let name = CString::new("security.selinux").ok()?;
    // Linux: getxattr(path, name, value, size)
    let size = unsafe { libc::getxattr(c_path.as_ptr(), name.as_ptr(), std::ptr::null_mut(), 0) };
    if size <= 0 {
        return None;
    }
    let mut buf = vec![0u8; size as usize];
    let n = unsafe {
        libc::getxattr(
            c_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr().cast(),
            buf.len(),
        )
    };
    if n <= 0 {
        return None;
    }
    buf.truncate(n as usize);
    while buf.last() == Some(&0) {
        buf.pop();
    }
    String::from_utf8(buf).ok()
}

/// macOS/BSD: different getxattr arity; SELinux xattr is typically absent — skip.
#[cfg(all(unix, not(target_os = "linux")))]
fn read_selinux_context_unix(_path: &Path) -> Option<String> {
    None
}

fn resolve_owner_cached(uid: u32) -> String {
    if let Ok(cache) = owner_cache().lock() {
        if let Some(name) = cache.get(&uid) {
            return name.clone();
        }
    }
    let name = resolve_owner(uid);
    if let Ok(mut cache) = owner_cache().lock() {
        cache.insert(uid, name.clone());
    }
    name
}

fn resolve_group_cached(gid: u32) -> String {
    if let Ok(cache) = group_cache().lock() {
        if let Some(name) = cache.get(&gid) {
            return name.clone();
        }
    }
    let name = resolve_group(gid);
    if let Ok(mut cache) = group_cache().lock() {
        cache.insert(gid, name.clone());
    }
    name
}

#[cfg(unix)]
fn resolve_owner(uid: u32) -> String {
    // SAFETY: getpwuid returns a static pointer for the duration of the call.
    unsafe {
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            return uid.to_string();
        }
        let name = std::ffi::CStr::from_ptr((*pw).pw_name);
        name.to_string_lossy().into_owned()
    }
}

#[cfg(unix)]
fn resolve_group(gid: u32) -> String {
    unsafe {
        let gr = libc::getgrgid(gid);
        if gr.is_null() {
            return gid.to_string();
        }
        let name = std::ffi::CStr::from_ptr((*gr).gr_name);
        name.to_string_lossy().into_owned()
    }
}

#[cfg(not(unix))]
fn resolve_owner(_uid: u32) -> String {
    String::from("user")
}

#[cfg(not(unix))]
fn resolve_group(_gid: u32) -> String {
    String::from("group")
}

#[cfg(test)]
mod meta_fill_tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file() -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "f00-meta-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::write(&p, b"x").unwrap();
        p
    }

    #[test]
    fn cheap_fill_skips_owner_names() {
        let p = temp_file();
        let e = Entry::from_path_with(&p, 0, MetaFill::cheap()).unwrap();
        assert!(e.owner.is_empty(), "cheap path should not resolve owner");
        assert!(e.context.is_empty());
        assert_eq!(e.size, 1);
        let _ = fs::remove_file(&p);
    }

    #[test]
    fn rich_fill_resolves_owner_on_unix() {
        let p = temp_file();
        let e = Entry::from_path_with(&p, 0, MetaFill::rich(false)).unwrap();
        #[cfg(unix)]
        assert!(!e.owner.is_empty() || e.uid > 0);
        #[cfg(not(unix))]
        let _ = e;
        let _ = fs::remove_file(&p);
    }
}
