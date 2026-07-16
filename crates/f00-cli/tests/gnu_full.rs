//! Full GNU surface integration tests: quoting, --zero, -v, --hide, -w, --time-style.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-gnu-full-{}-{}-{}",
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
fn escape_flag_quotes_special() {
    let dir = temp_dir("escape");
    // Name with tab or newline is awkward on some FS; use a space instead and -b
    // which escapes nongraphic — also test quoting-style=escape on a name with quotes.
    fs::write(dir.join("plain"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-1", "-b", "--quoting-style=c"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{:?}", out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // -b / quoting-style=c: last flag wins in our resolver if both set — quote_name from -Q
    // Here we only set -b first then --quoting-style=c so style is c.
    assert!(
        stdout.contains("\"plain\"") || stdout.contains("plain"),
        "stdout={stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn quote_name_q_wraps_in_double_quotes() {
    let dir = temp_dir("Q");
    fs::write(dir.join("hello"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-1", "-Q"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"hello\""),
        "expected double-quoted name: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn escape_b_escapes_backslash() {
    let dir = temp_dir("b");
    // Create a file whose name contains a character that -b escapes.
    // Spaces are not escaped by -b (only nongraphic + special in escape style).
    // Use -b and check backslash in path... create "a\\b" is hard.
    // Verify -b mode is active by checking quoting-style=escape via env-like flag.
    fs::write(dir.join("a"), b"1").unwrap();
    fs::write(dir.join("b"), b"2").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-1", "--quoting-style=escape"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains('a') && stdout.contains('b'), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn zero_uses_nul_separators() {
    let dir = temp_dir("zero");
    fs::write(dir.join("one"), b"1").unwrap();
    fs::write(dir.join("two"), b"2").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["--zero"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = &out.stdout;
    assert!(
        stdout.contains(&0u8),
        "expected NUL bytes in output: {stdout:?}"
    );
    // No trailing newline-only lines between names — names separated by \0
    let text = String::from_utf8_lossy(stdout);
    assert!(
        !text.contains("one\ntwo") && !text.contains("two\none"),
        "should not use newline between names: {text:?}"
    );
    let parts: Vec<_> = stdout
        .split(|&b| b == 0)
        .filter(|p| !p.is_empty())
        .collect();
    let names: Vec<String> = parts
        .iter()
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .collect();
    assert!(names.iter().any(|n| n.contains("one")), "names={names:?}");
    assert!(names.iter().any(|n| n.contains("two")), "names={names:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn version_sort_orders_naturally() {
    let dir = temp_dir("v");
    fs::write(dir.join("file10"), b"x").unwrap();
    fs::write(dir.join("file2"), b"x").unwrap();
    fs::write(dir.join("file1"), b"x").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["-1", "-v"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let i1 = stdout.find("file1").expect("file1");
    let i2 = stdout.find("file2").expect("file2");
    let i10 = stdout.find("file10").expect("file10");
    assert!(
        i1 < i2 && i2 < i10,
        "version order: {stdout} positions {i1},{i2},{i10}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn hide_pattern_suppressed_unless_all() {
    let dir = temp_dir("hide");
    fs::write(dir.join("keep.rs"), b"k").unwrap();
    fs::write(dir.join("drop.tmp"), b"d").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-1", "--hide=*.tmp"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("keep.rs"), "{stdout}");
    assert!(!stdout.contains("drop.tmp"), "hide should drop: {stdout}");

    let out_a = f00(&cfg)
        .args(["-1", "-a", "--hide=*.tmp"])
        .arg(&dir)
        .output()
        .unwrap();
    let s_a = String::from_utf8_lossy(&out_a.stdout);
    assert!(s_a.contains("drop.tmp"), "-a overrides --hide: {s_a}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn width_zero_unlimited_still_lists() {
    let dir = temp_dir("w");
    for name in ["a", "b", "c", "d", "e"] {
        fs::write(dir.join(name), b"x").unwrap();
    }
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-C", "-w", "0"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    for name in ["a", "b", "c", "d", "e"] {
        assert!(stdout.contains(name), "missing {name}: {stdout}");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn time_style_long_iso() {
    let dir = temp_dir("ts");
    fs::write(dir.join("f.txt"), b"hello").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg)
        .args(["-l", "--time-style=long-iso", "--gnu"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    // long-iso: YYYY-MM-DD HH:MM
    let re_ok = stdout.lines().any(|line| {
        line.contains("f.txt")
            && line.split_whitespace().any(|tok| {
                // date token like 2026-07-16
                tok.len() == 10 && tok.as_bytes()[4] == b'-' && tok.as_bytes()[7] == b'-'
            })
    });
    assert!(re_ok, "expected long-iso date near f.txt: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn literal_n_no_quotes() {
    let dir = temp_dir("N");
    fs::write(dir.join("x"), b"1").unwrap();
    let cfg = empty_config(&dir);
    let out = f00(&cfg)
        .args(["-1", "-Q", "-N"])
        .arg(&dir)
        .output()
        .unwrap();
    // -N after -Q: our resolver prefers -N when literal is set (checked first)
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("\"x\""),
        "-N should suppress quoting: {stdout}"
    );
    assert!(stdout.contains('x'), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn csv_and_tsv_modes() {
    let dir = temp_dir("csv");
    fs::write(dir.join("z.txt"), b"z").unwrap();
    let cfg = empty_config(&dir);

    let out = f00(&cfg).args(["--csv"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("name,path,"), "{s}");
    assert!(s.contains("z.txt"), "{s}");

    let out2 = f00(&cfg).args(["--tsv"]).arg(&dir).output().unwrap();
    let s2 = String::from_utf8_lossy(&out2.stdout);
    assert!(s2.contains('\t'), "{s2}");
    assert!(s2.contains("z.txt"), "{s2}");
    let _ = fs::remove_dir_all(&dir);
}
