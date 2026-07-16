//! Batch `statx` via **io_uring** for large directories (Linux, feature `io-uring`).

#![cfg(all(target_os = "linux", feature = "io-uring"))]

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use io_uring::{opcode, types, IoUring};

use crate::entry::{Entry, MetaFill};
use crate::linux_statx;

/// Minimum children before io_uring batching is attempted.
pub const IO_URING_THRESHOLD: usize = 48;

/// Stat many paths with io_uring-submitted `statx` ops.
///
/// Returns `None` if the ring cannot be created or submissions fail so callers
/// can fall back. Entries that fail individual ops are skipped.
pub fn entries_from_paths_uring(paths: &[PathBuf], fill: MetaFill) -> Option<Vec<Entry>> {
    if paths.is_empty() {
        return Some(Vec::new());
    }
    if fill.resolve_names || fill.read_context {
        return None;
    }

    let qd = paths.len().clamp(8, 256).next_power_of_two() as u32;
    let mut ring = IoUring::new(qd).ok()?;

    let mut c_paths: Vec<CString> = Vec::with_capacity(paths.len());
    for p in paths {
        c_paths.push(CString::new(p.as_os_str().as_bytes()).ok()?);
    }
    let mut buffers: Vec<libc::statx> = (0..paths.len())
        .map(|_| unsafe { std::mem::zeroed() })
        .collect();

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

    let mut out: Vec<Option<Entry>> = vec![None; paths.len()];
    let mut base = 0usize;
    let chunk = qd as usize;

    while base < paths.len() {
        let end = (base + chunk).min(paths.len());
        let mut submitted = 0usize;

        {
            let mut sq = ring.submission();
            for (off, (c_path, buf)) in c_paths[base..end]
                .iter()
                .zip(buffers[base..end].iter_mut())
                .enumerate()
            {
                let user_data = (base + off) as u64;
                let sqe = opcode::Statx::new(
                    types::Fd(libc::AT_FDCWD),
                    c_path.as_ptr(),
                    buf as *mut libc::statx as *mut _,
                )
                .flags(libc::AT_SYMLINK_NOFOLLOW)
                .mask(mask)
                .build()
                .user_data(user_data);
                unsafe {
                    if sq.push(&sqe).is_err() {
                        break;
                    }
                }
                submitted += 1;
            }
        }

        if submitted == 0 {
            return None;
        }
        ring.submit_and_wait(submitted).ok()?;

        for _ in 0..submitted {
            let cqe = ring.completion().next()?;
            let idx = cqe.user_data() as usize;
            if cqe.result() >= 0 {
                if let (Some(path), Some(buf)) = (paths.get(idx), buffers.get(idx)) {
                    if let Ok(e) = linux_statx::entry_from_statx_buf(path, 0, buf, fill) {
                        out[idx] = Some(e);
                    }
                }
            }
        }
        base += submitted;
    }

    Some(out.into_iter().flatten().collect())
}
