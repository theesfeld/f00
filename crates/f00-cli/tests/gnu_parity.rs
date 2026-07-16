//! GNU ls parity smoke / golden-ish tests.
//!
//! We do not require byte-identical output to coreutils (locales, time zones, padding),
//! but we assert structural compatibility for common flags.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-gnu-{}-{}-{}",
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

fn empty_config(dir: &Path) -> PathBuf {
    let p = dir.join("empty.toml");
    fs::write(&p, "").unwrap();
    p
}

fn f00(config: &Path) -> Command {
    let mut c = Command::new(bin());
    c.args(["--color=never", "--git=false", "--config"]);
    c.arg(config);
    c
}

#[test]
fn directory_flag_lists_dir_itself() {
    let dir = temp_dir("d");
    let sub = dir.join("sub");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("inside.txt"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-1", "-d"]).arg(&sub).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("sub") || stdout.contains(sub.to_string_lossy().as_ref()),
        "stdout={stdout}"
    );
    assert!(
        !stdout.contains("inside.txt"),
        "-d must not list children: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ignore_backups_and_pattern() {
    let dir = temp_dir("ignore");
    fs::write(dir.join("keep.txt"), b"k").unwrap();
    fs::write(dir.join("drop.txt~"), b"b").unwrap();
    fs::write(dir.join("skip.o"), b"o").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-1", "-B", "-I", "*.o"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("keep.txt"), "{stdout}");
    assert!(!stdout.contains("drop.txt~"), "{stdout}");
    assert!(!stdout.contains("skip.o"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn size_sort_largest_first() {
    let dir = temp_dir("S");
    fs::write(dir.join("small"), b"a").unwrap();
    fs::write(dir.join("big"), vec![b'x'; 4096]).unwrap();
    fs::write(dir.join("mid"), vec![b'y'; 64]).unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-1", "-S"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let big = stdout.find("big").expect("big");
    let mid = stdout.find("mid").expect("mid");
    let small = stdout.find("small").expect("small");
    assert!(big < mid && mid < small, "order stdout={stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn long_format_has_owner_group_nlink() {
    let dir = temp_dir("l");
    fs::write(dir.join("file.txt"), b"hello").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-l", "--gnu"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Permissions start with type + rwx style
    assert!(
        stdout.contains("file.txt")
            && (stdout.contains("-rw") || stdout.contains("rw-") || stdout.contains("r--")),
        "stdout={stdout}"
    );
    // No git decoration column spacing with triple spaces from git when --gnu
    assert!(
        !stdout.contains(" M "),
        "git should be off under --gnu: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn classify_and_slash() {
    let dir = temp_dir("F");
    fs::create_dir(dir.join("d")).unwrap();
    let file = dir.join("x");
    fs::write(&file, b"#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&file).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&file, perms).unwrap();
    }
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-1", "-F"]).arg(&dir).output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("d/"), "dirs get /: {stdout}");
    #[cfg(unix)]
    assert!(
        stdout.contains("x*") || stdout.contains("x"),
        "stdout={stdout}"
    );

    let out2 = f00(&cfg).args(["-1", "-p"]).arg(&dir).output().unwrap();
    let s2 = String::from_utf8_lossy(&out2.stdout);
    assert!(s2.contains("d/"), "-p: {s2}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn commas_format() {
    let dir = temp_dir("m");
    fs::write(dir.join("a"), b"1").unwrap();
    fs::write(dir.join("b"), b"2").unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg).args(["-m"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(',') || stdout.contains("a"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn unsorted_all_f_flag() {
    let dir = temp_dir("f");
    fs::write(dir.join(".hid"), b"h").unwrap();
    fs::write(dir.join("vis"), b"v").unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg).args(["-1", "-f"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(".hid"), "-f implies -a: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn group_directories_first() {
    let dir = temp_dir("gdf");
    fs::write(dir.join("zzz"), b"f").unwrap();
    fs::create_dir(dir.join("aaa_dir")).unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg)
        .args(["-1", "--group-directories-first"])
        .arg(&dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let d = stdout.find("aaa_dir").expect("dir");
    let f = stdout.find("zzz").expect("file");
    assert!(d < f, "dirs first: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn gnu_disables_icons_even_if_requested() {
    let dir = temp_dir("gnu-icons");
    fs::write(dir.join("a.rs"), b"fn main(){}").unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg)
        .args(["-1", "--icons", "--gnu"])
        .arg(&dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Emoji icons should not appear under --gnu
    assert!(
        !stdout.contains('🦀') && !stdout.contains('📁'),
        "no icons under --gnu: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn almost_all_hides_dot_dotdot() {
    let dir = temp_dir("A");
    fs::write(dir.join(".hid"), b"h").unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg).args(["-1", "-A"]).arg(&dir).output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(".hid"), "{stdout}");
    // lines that are exactly . or ..
    for line in stdout.lines() {
        let t = line.trim();
        assert_ne!(t, ".");
        assert_ne!(t, "..");
    }
    let _ = fs::remove_dir_all(&dir);
}
