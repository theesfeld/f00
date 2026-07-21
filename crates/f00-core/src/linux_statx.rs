//! Linux `statx(2)` for multi-field metadata in one syscall.
//!
//! Hot path: open the parent directory once and call `statx` with **dirfd +
//! relative names** (far fewer path walks than absolute `AT_FDCWD` each time).

#![cfg(target_os = "linux")]

use std::ffi::{CString, OsStr};
use std::os::fd::{AsRawFd, BorrowedFd};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::entry::{Entry, EntryKind, GitStatus, MetaFill};
use crate::error::{Error, Result};

/// Shared `statx` mask for full listing metadata.
pub const STATX_MASK: u32 = libc::STATX_TYPE
    | libc::STATX_MODE
    | libc::STATX_NLINK
    | libc::STATX_UID
    | libc::STATX_GID
    | libc::STATX_SIZE
    | libc::STATX_BLOCKS
    | libc::STATX_MTIME
    | libc::STATX_ATIME
    | libc::STATX_CTIME
    | libc::STATX_BTIME
    | libc::STATX_INO;

/// Build an entry via absolute-path `statx` (no symlink follow).
pub fn entry_from_statx(path: &Path, depth: usize, fill: MetaFill) -> Result<Entry> {
    let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|_| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path contains NUL",
        ))
    })?;
    let mut buf: libc::statx = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        libc::syscall(
            libc::SYS_statx,
            libc::AT_FDCWD,
            c_path.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
            STATX_MASK,
            &mut buf as *mut libc::statx,
        )
    };
    if rc != 0 {
        return Err(Error::Metadata {
            path: path.to_path_buf(),
            source: std::io::Error::last_os_error(),
        });
    }
    entry_from_statx_buf(path, depth, &buf, fill)
}

/// `statx` relative to an open directory fd (no symlink follow).
///
/// `name` is the directory entry name only (not a path with separators).
/// `full_path` is the display/storage path for the resulting [`Entry`].
pub fn entry_from_statx_at(
    dir: BorrowedFd<'_>,
    name: &OsStr,
    full_path: &Path,
    depth: usize,
    fill: MetaFill,
) -> Result<Entry> {
    let c_name = CString::new(name.as_bytes()).map_err(|_| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "name contains NUL",
        ))
    })?;
    let mut buf: libc::statx = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        libc::syscall(
            libc::SYS_statx,
            dir.as_raw_fd(),
            c_name.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
            STATX_MASK,
            &mut buf as *mut libc::statx,
        )
    };
    if rc != 0 {
        return Err(Error::Metadata {
            path: full_path.to_path_buf(),
            source: std::io::Error::last_os_error(),
        });
    }
    entry_from_statx_buf(full_path, depth, &buf, fill)
}

/// Convert a filled `statx` buffer into an [`Entry`].
///
/// When `fill.resolve_names` / `read_context` is set, names/context are filled
/// here. Callers that batch I/O may pass `MetaFill::default()` and call
/// [`Entry::fill_expensive`] afterward.
pub fn entry_from_statx_buf(
    path: &Path,
    depth: usize,
    buf: &libc::statx,
    fill: MetaFill,
) -> Result<Entry> {
    let mode_full = u32::from(buf.stx_mode);
    let file_type = mode_full & libc::S_IFMT;
    let kind = match file_type {
        x if x == libc::S_IFDIR => EntryKind::Directory,
        x if x == libc::S_IFLNK => EntryKind::Symlink,
        x if x == libc::S_IFREG => EntryKind::File,
        _ => EntryKind::Other,
    };

    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());

    let symlink_target = if kind == EntryKind::Symlink {
        std::fs::read_link(path).ok()
    } else {
        None
    };

    let dev = makdev(buf.stx_dev_major, buf.stx_dev_minor);
    let rdev = makdev(buf.stx_rdev_major, buf.stx_rdev_minor);
    let mut entry = Entry {
        path: path.to_path_buf(),
        name,
        kind,
        size: buf.stx_size,
        modified: stx_time(buf.stx_mtime),
        created: if buf.stx_mask & libc::STATX_BTIME != 0 {
            stx_time(buf.stx_btime)
        } else {
            None
        },
        accessed: stx_time(buf.stx_atime),
        changed: stx_time(buf.stx_ctime),
        // Keep type bits so FIFO/socket/device classification and JSON mode_full work.
        mode: mode_full,
        readonly: mode_full & 0o200 == 0,
        symlink_target,
        depth,
        git_status: GitStatus::Clean,
        is_dir_header: false,
        nlink: u64::from(buf.stx_nlink),
        uid: buf.stx_uid,
        gid: buf.stx_gid,
        inode: buf.stx_ino,
        blocks: buf.stx_blocks,
        dev,
        rdev,
        blksize: u64::from(buf.stx_blksize),
        owner: String::new(),
        group: String::new(),
        author: String::new(),
        context: String::new(),
    };

    if fill.resolve_names || fill.read_context {
        entry.fill_expensive(fill);
    }

    Ok(entry)
}

fn stx_time(t: libc::statx_timestamp) -> Option<SystemTime> {
    if t.tv_sec < 0 {
        return None;
    }
    Some(SystemTime::UNIX_EPOCH + Duration::new(t.tv_sec as u64, t.tv_nsec as u32))
}

fn makdev(major: u32, minor: u32) -> u64 {
    // Linux makedev: ((major & 0xfff) << 8) | (minor & 0xff) | ...
    // Prefer libc when available.
    #[cfg(target_os = "linux")]
    {
        libc::makedev(major, minor)
    }
    #[cfg(not(target_os = "linux"))]
    {
        ((u64::from(major) & 0xfff) << 8) | (u64::from(minor) & 0xff)
    }
}
