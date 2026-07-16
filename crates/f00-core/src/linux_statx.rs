//! Linux `statx(2)` for multi-field metadata in one syscall (path C).
//!
//! Used when [`crate::ListOptions::linux_statx`] is set and we are not
//! following symlinks. On failure, callers fall back to `std::fs`.

#![cfg(target_os = "linux")]

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::entry::{Entry, EntryKind, GitStatus, MetaFill};
use crate::error::{Error, Result};

/// Build an entry via `statx` (no symlink follow).
pub fn entry_from_statx(path: &Path, depth: usize, fill: MetaFill) -> Result<Entry> {
    let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|_| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path contains NUL",
        ))
    })?;

    let mut buf: libc::statx = unsafe { std::mem::zeroed() };
    let mask = libc::STATX_TYPE
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

    let rc = unsafe {
        libc::statx(
            libc::AT_FDCWD,
            c_path.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
            mask,
            &mut buf as *mut libc::statx,
        )
    };
    if rc != 0 {
        return Err(Error::Metadata {
            path: path.to_path_buf(),
            source: std::io::Error::last_os_error(),
        });
    }

    // stx_mode is u16; promote for bitwise ops with S_IF* (u32 on Linux).
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

    let uid = buf.stx_uid;
    let gid = buf.stx_gid;

    // For name resolution / SELinux, reuse the std-based builder (still cheaper
    // when those flags are off — the common short-list path).
    if fill.resolve_names || fill.read_context {
        return Entry::from_path_with(path, depth, fill);
    }

    Ok(Entry {
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
        mode: mode_full & 0o7777,
        readonly: mode_full & 0o200 == 0,
        symlink_target,
        depth,
        git_status: GitStatus::Clean,
        is_dir_header: false,
        nlink: u64::from(buf.stx_nlink),
        uid,
        gid,
        inode: buf.stx_ino,
        blocks: buf.stx_blocks,
        owner: String::new(),
        group: String::new(),
        author: String::new(),
        context: String::new(),
    })
}

fn stx_time(t: libc::statx_timestamp) -> Option<SystemTime> {
    if t.tv_sec < 0 {
        return None;
    }
    Some(SystemTime::UNIX_EPOCH + Duration::new(t.tv_sec as u64, t.tv_nsec))
}
