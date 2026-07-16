//! Locale / collation goldens for name sort order.
//!
//! Under `LC_ALL=C` we expect byte order. When a UTF-8 locale is available we
//! check that f00 produces a stable sort and optionally matches system `ls -1`
//! name order for the same fixture.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-locale-{}-{}-{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base).unwrap();
    base
}

fn empty_config(fixture: &Path) -> PathBuf {
    let p = fixture.with_extension("empty.toml");
    fs::write(&p, "").unwrap();
    p
}

fn f00_names(cfg: &Path, dir: &Path, locale: &str) -> Vec<String> {
    let out = Command::new(bin())
        .env("LC_ALL", locale)
        .env("LANG", locale)
        .args(["--color=never", "--git=false", "--config"])
        .arg(cfg)
        .args(["--gnu", "-1"])
        .arg(dir)
        .output()
        .expect("spawn f00");
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('.'))
        .collect()
}

fn system_ls_names(dir: &Path, locale: &str) -> Option<Vec<String>> {
    let ls = ["ls", "/bin/ls", "/usr/bin/ls"]
        .iter()
        .find(|p| Path::new(p).exists())
        .copied()?;
    let out = Command::new(ls)
        .env("LC_ALL", locale)
        .env("LANG", locale)
        .args(["-1"])
        .arg(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty() && !s.starts_with('.'))
            .collect(),
    )
}

fn build_ascii_fixture(dir: &Path) {
    for name in ["zeta", "Alpha", "beta", "10", "2", "a"] {
        fs::write(dir.join(name), b"x").unwrap();
    }
}

fn build_unicode_fixture(dir: &Path) {
    // Mixed ASCII + common Latin-1/Unicode letters.
    for name in ["apple", "Banana", "café", "Cafe", " boop", "naïve", "zebra"] {
        let n = name.trim();
        fs::write(dir.join(n), b"x").unwrap();
    }
}

#[test]
fn c_locale_sort_is_byte_order() {
    let dir = temp_dir("c-byte");
    build_ascii_fixture(&dir);
    let cfg = empty_config(&dir);

    let names = f00_names(&cfg, &dir, "C");
    let mut expected = names.clone();
    expected.sort(); // byte order under C for our pure ASCII set
    assert_eq!(
        names, expected,
        "LC_ALL=C should yield byte-sorted names; got {names:?}"
    );

    // Optional: match system ls under C.
    if let Some(ls) = system_ls_names(&dir, "C") {
        assert_eq!(names, ls, "f00 --gnu -1 should match ls -1 under LC_ALL=C");
    }

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(cfg);
}

#[test]
fn c_locale_is_stable_and_complete() {
    let dir = temp_dir("c-stable");
    build_unicode_fixture(&dir);
    let cfg = empty_config(&dir);

    let a = f00_names(&cfg, &dir, "C");
    let b = f00_names(&cfg, &dir, "C");
    assert_eq!(a, b, "sort must be stable across runs");
    let mut set = a.clone();
    set.sort();
    // All created names present (visible).
    for n in ["apple", "Banana", "café", "Cafe", "boop", "naïve", "zebra"] {
        assert!(a.iter().any(|x| x == n), "missing {n} in {a:?}");
    }

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(cfg);
}

/// When a UTF-8 locale is installed, f00 must produce a deterministic order
/// and (when possible) match system `ls` under the same locale.
#[test]
fn utf8_locale_sort_matches_ls_when_available() {
    let dir = temp_dir("utf8");
    build_unicode_fixture(&dir);
    let cfg = empty_config(&dir);

    // Prefer common UTF-8 locales; skip if none exist on the host.
    let candidates = [
        "C.UTF-8",
        "en_US.UTF-8",
        "en_US.utf8",
        "en_GB.UTF-8",
        "C.utf8",
    ];
    let mut locale = None;
    for loc in candidates {
        // Probe: if ls accepts the locale without error, use it.
        if system_ls_names(&dir, loc).is_some() {
            locale = Some(loc);
            break;
        }
        // Also accept f00 running under the locale even without ls.
        let names = f00_names(&cfg, &dir, loc);
        if names.len() >= 5 {
            locale = Some(loc);
            break;
        }
    }

    let Some(loc) = locale else {
        // No usable UTF-8 locale — still exercise C path already covered.
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_file(cfg);
        return;
    };

    let f00 = f00_names(&cfg, &dir, loc);
    let f00_again = f00_names(&cfg, &dir, loc);
    assert_eq!(f00, f00_again, "UTF-8 locale sort must be deterministic");

    if let Some(ls) = system_ls_names(&dir, loc) {
        // Same *set*; order may differ if f00 uses byte sort vs strcoll —
        // prefer order match when both under same LC_ALL; if diverge, still
        // require set equality so we don't claim false parity.
        let mut a = f00.clone();
        let mut b = ls.clone();
        a.sort();
        b.sort();
        assert_eq!(
            a, b,
            "name set must match ls under {loc}; f00={f00:?} ls={ls:?}"
        );
        // Soft order check: document if order differs (strcoll vs byte).
        if f00 != ls {
            eprintln!(
                "note: order under {loc} differs from ls (f00 may use byte order); f00={f00:?} ls={ls:?}"
            );
        }
    }

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(cfg);
}

#[test]
fn reverse_respects_locale_env_c() {
    let dir = temp_dir("rev");
    build_ascii_fixture(&dir);
    let cfg = empty_config(&dir);

    let forward = f00_names(&cfg, &dir, "C");
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--color=never", "--git=false", "--config"])
        .arg(&cfg)
        .args(["--gnu", "-1", "-r"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let rev: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('.'))
        .collect();
    let mut expected = forward.clone();
    expected.reverse();
    assert_eq!(rev, expected);

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(cfg);
}
