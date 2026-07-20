//! Batch `statx` via **io_uring** for large directories (Linux, feature `io-uring`).
//!
//! Uses an open directory fd + relative names so the kernel avoids repeated path
//! resolution. Owner/group resolution is **not** performed here — callers use
//! [`crate::entry::Entry::fill_expensive`] when needed.

#![cfg(all(target_os = "linux", feature = "io-uring"))]

use std::ffi::CString;
use std::os::fd::{AsRawFd, BorrowedFd};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use io_uring::{opcode, types, IoUring};

use crate::entry::{Entry, MetaFill};
use crate::linux_statx;

/// Minimum children before io_uring batching is attempted.
pub const IO_URING_THRESHOLD: usize = 48;

/// Max SQEs per submit window (power-of-two friendly).
const RING_QD: u32 = 1024;

/// Stat many directory children with io_uring-submitted relative `statx` ops.
///
/// `dir` is an open `O_DIRECTORY` handle for `parent`. `names` are bare entry
/// names; `full_paths[i]` is the absolute (or relative) path for entry `i`.
///
/// Returns `None` if the ring cannot be created so callers can fall back.
/// Names/SELinux are **not** resolved (pass cheap `MetaFill`).
pub fn entries_from_dir_uring(
    dir: BorrowedFd<'_>,
    names: &[std::ffi::OsString],
    full_paths: &[PathBuf],
    fill: MetaFill,
) -> Option<Vec<Entry>> {
    if names.is_empty() {
        return Some(Vec::new());
    }
    debug_assert_eq!(names.len(), full_paths.len());

    // Never resolve NSS / xattrs inside the batch — do that after.
    let cheap = MetaFill {
        resolve_names: false,
        read_context: false,
    };
    let _ = fill; // callers fill expensive fields after

    let qd = (names.len().clamp(16, RING_QD as usize) as u32).next_power_of_two();
    let mut ring = IoUring::new(qd).ok()?;
    let dir_fd = types::Fd(dir.as_raw_fd());

    let mut c_names: Vec<CString> = Vec::with_capacity(names.len());
    for n in names {
        c_names.push(CString::new(n.as_bytes()).ok()?);
    }
    let mut buffers: Vec<libc::statx> = (0..names.len())
        .map(|_| unsafe { std::mem::zeroed() })
        .collect();

    let mut out: Vec<Option<Entry>> = vec![None; names.len()];
    let mut base = 0usize;
    let chunk = qd as usize;

    while base < names.len() {
        let end = (base + chunk).min(names.len());
        let mut submitted = 0usize;

        {
            let mut sq = ring.submission();
            for (off, (c_name, buf)) in c_names[base..end]
                .iter()
                .zip(buffers[base..end].iter_mut())
                .enumerate()
            {
                let user_data = (base + off) as u64;
                let sqe =
                    opcode::Statx::new(dir_fd, c_name.as_ptr(), buf as *mut libc::statx as *mut _)
                        .flags(libc::AT_SYMLINK_NOFOLLOW)
                        .mask(linux_statx::STATX_MASK)
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
                if let (Some(path), Some(buf)) = (full_paths.get(idx), buffers.get(idx)) {
                    if let Ok(e) = linux_statx::entry_from_statx_buf(path, 0, buf, cheap) {
                        out[idx] = Some(e);
                    }
                }
            }
        }
        base += submitted;
    }

    Some(out.into_iter().flatten().collect())
}

/// Legacy absolute-path batch (kept for recursive path lists without a shared dirfd).
pub fn entries_from_paths_uring(paths: &[PathBuf], fill: MetaFill) -> Option<Vec<Entry>> {
    if paths.is_empty() {
        return Some(Vec::new());
    }

    let cheap = MetaFill {
        resolve_names: false,
        read_context: false,
    };
    let _ = fill;

    let qd = (paths.len().clamp(16, RING_QD as usize) as u32).next_power_of_two();
    let mut ring = IoUring::new(qd).ok()?;

    let mut c_paths: Vec<CString> = Vec::with_capacity(paths.len());
    for p in paths {
        c_paths.push(CString::new(p.as_os_str().as_bytes()).ok()?);
    }
    let mut buffers: Vec<libc::statx> = (0..paths.len())
        .map(|_| unsafe { std::mem::zeroed() })
        .collect();

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
                .mask(linux_statx::STATX_MASK)
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
                    if let Ok(e) = linux_statx::entry_from_statx_buf(path, 0, buf, cheap) {
                        out[idx] = Some(e);
                    }
                }
            }
        }
        base += submitted;
    }

    Some(out.into_iter().flatten().collect())
}

/// Helper for tests / callers that only have paths under one parent.
#[allow(dead_code)]
pub fn parent_of(paths: &[PathBuf]) -> Option<&Path> {
    paths.first()?.parent()
}
