//! Exit-code integration tests (GNU-aligned 0 / 1 / 2).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-exit-{}-{}-{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base).expect("mkdir");
    base
}

/// Neutral empty config so user/XDG config cannot affect tests.
fn empty_config(dir: &Path) -> PathBuf {
    let p = dir.join("empty-f00-config.toml");
    fs::write(&p, "").unwrap();
    p
}

fn f00_cmd(config: &Path) -> Command {
    let mut c = Command::new(bin());
    c.args(["--color=never", "--git=false", "--config"]);
    c.arg(config);
    c
}

#[test]
fn clean_listing_exits_0() {
    let dir = temp_dir("ok");
    fs::write(dir.join("file.txt"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let status = f00_cmd(&cfg).arg(&dir).status().expect("spawn");
    assert_eq!(status.code(), Some(0), "expected success");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_path_exits_2() {
    let dir = temp_dir("missing-base");
    let cfg = empty_config(&dir);
    let missing = dir.join("definitely-missing-path");
    assert!(!missing.exists());

    let output = f00_cmd(&cfg).arg(&missing).output().expect("spawn");
    assert_eq!(
        output.status.code(),
        Some(2),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("not found") || err.contains("No such") || err.contains("f00:"),
        "stderr={err}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn good_and_missing_still_lists_good_exits_2() {
    let dir = temp_dir("partial");
    fs::write(dir.join("ok.txt"), b"hi").unwrap();
    let missing = dir.join("nope-missing");
    let cfg = empty_config(&dir);

    let output = f00_cmd(&cfg)
        .arg("-1")
        .arg(&dir)
        .arg(&missing)
        .output()
        .expect("spawn");

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ok.txt"),
        "should still list the good path; stdout={stdout}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("f00:"), "stderr={stderr}");

    let _ = fs::remove_dir_all(&dir);
}

/// True when this process is root (euid 0). Permission-denial tests cannot work as root:
/// FreeBSD/Linux root bypasses mode bits, so `chmod 0` dirs remain listable.
#[cfg(unix)]
fn running_as_root() -> bool {
    // Avoid a libc crate dep for a single syscall used only in tests.
    extern "C" {
        fn geteuid() -> u32;
    }
    // SAFETY: geteuid has no preconditions and is always safe to call.
    unsafe { geteuid() == 0 }
}

/// After chmod 0o000, return whether `path` is still readable as a directory by this process.
#[cfg(unix)]
fn still_readable_dir(path: &Path) -> bool {
    fs::read_dir(path).is_ok()
}

#[test]
#[cfg(unix)]
fn unreadable_path_arg_exits_2() {
    use std::os::unix::fs::PermissionsExt;

    if running_as_root() {
        // Root can always readdir mode-0 directories; skip on FreeBSD CI VMs etc.
        return;
    }

    let dir = temp_dir("unreadable");
    fs::write(dir.join("secret.txt"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let mut perms = fs::metadata(&dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&dir, perms).unwrap();

    if still_readable_dir(&dir) {
        // Platform or mount does not honor mode bits for this process.
        let mut perms = fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dir, perms).unwrap();
        let _ = fs::remove_dir_all(&dir);
        return;
    }

    let output = f00_cmd(&cfg).arg(&dir).output().expect("spawn");

    // Restore perms before cleanup
    let mut perms = fs::metadata(&dir).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dir, perms).unwrap();

    assert_eq!(
        output.status.code(),
        Some(2),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
#[cfg(unix)]
fn unreadable_subdir_in_recursive_exits_1() {
    use std::os::unix::fs::PermissionsExt;

    if running_as_root() {
        return;
    }

    let dir = temp_dir("recurse");
    fs::write(dir.join("visible.txt"), b"v").unwrap();
    let sub = dir.join("locked");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("hidden.txt"), b"h").unwrap();
    let cfg = empty_config(&dir);

    let mut perms = fs::metadata(&sub).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&sub, perms).unwrap();

    if still_readable_dir(&sub) {
        let mut perms = fs::metadata(&sub).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&sub, perms).unwrap();
        let _ = fs::remove_dir_all(&dir);
        return;
    }

    let output = f00_cmd(&cfg).arg("-R").arg(&dir).output().expect("spawn");

    // Restore
    let mut perms = fs::metadata(&sub).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&sub, perms).unwrap();

    let code = output.status.code();
    // walkdir should report an error for the locked dir → exit 1
    assert_eq!(
        code,
        Some(1),
        "expected minor error exit 1; stderr={} stdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let _ = fs::remove_dir_all(&dir);
}
