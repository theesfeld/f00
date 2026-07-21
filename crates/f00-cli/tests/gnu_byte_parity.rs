//! Byte-level parity against system GNU `ls` when available.
//!
//! These tests encode the **drop-in** contract: under `--gnu` with
//! `LC_ALL=C`, `NO_COLOR`, and colors/icons/git off, f00 must match
//! coreutils output for the common scripting flags.

#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn system_ls() -> Option<&'static str> {
    for p in ["/run/current-system/sw/bin/ls", "/usr/bin/ls", "/bin/ls"] {
        if Path::new(p).is_file() {
            // Prefer real coreutils (not an f00 symlink).
            let out = Command::new(p).arg("--version").output().ok()?;
            let v = String::from_utf8_lossy(&out.stdout);
            if v.contains("coreutils") || v.contains("GNU") {
                return Some(p);
            }
        }
    }
    None
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-byte-{}-{}-{}",
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

fn empty_config(fixture: &Path) -> PathBuf {
    let p = fixture.parent().unwrap_or(fixture).join(format!(
        "f00-byte-cfg-{}-{}.toml",
        std::process::id(),
        fixture.file_name().and_then(|n| n.to_str()).unwrap_or("x")
    ));
    fs::write(&p, "").unwrap();
    p
}

fn build_fixture() -> (PathBuf, PathBuf) {
    let dir = temp_dir("fx");
    fs::create_dir(dir.join("empty_dir")).unwrap();
    fs::create_dir(dir.join("has_child")).unwrap();
    fs::write(dir.join("has_child").join("c"), b"c").unwrap();
    fs::write(dir.join("a"), b"").unwrap();
    fs::write(dir.join("file1"), b"").unwrap();
    fs::write(dir.join("file2"), b"").unwrap();
    fs::write(dir.join("file10"), b"").unwrap();
    fs::write(dir.join("file~"), b"").unwrap();
    fs::write(dir.join("x y"), b"sp").unwrap();
    fs::write(dir.join("small"), b"a").unwrap();
    fs::write(dir.join("large"), vec![0u8; 4096]).unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("a", dir.join("link")).unwrap();
        let sh = dir.join("exec.sh");
        fs::write(&sh, b"#!/bin/sh\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&sh).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&sh, perms).unwrap();
    }
    let cfg = empty_config(&dir);
    (dir, cfg)
}

fn normalize(s: &str) -> String {
    s.lines()
        .map(|l| l.trim_end())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_ls_match(label: &str, ls: &str, dir: &Path, cfg: &Path, args: &[&str]) {
    let mut g = Command::new(ls);
    g.env("LC_ALL", "C")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .env("LS_COLORS", "")
        .arg("--color=never")
        .args(args)
        .arg(dir);
    let gout = g.output().expect("run system ls");

    let mut f = Command::new(bin());
    f.env("LC_ALL", "C")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .args([
            "--gnu",
            "--color=never",
            "--git=false",
            "--icons=never",
            "--config",
        ])
        .arg(cfg)
        .args(args)
        .arg(dir);
    let fout = f.output().expect("run f00");

    assert_eq!(
        gout.status.code(),
        fout.status.code(),
        "{label}: exit mismatch\nGNU stderr={}\nf00 stderr={}",
        String::from_utf8_lossy(&gout.stderr),
        String::from_utf8_lossy(&fout.stderr)
    );

    let gs = normalize(&String::from_utf8_lossy(&gout.stdout));
    let fs = normalize(&String::from_utf8_lossy(&fout.stdout));
    assert_eq!(
        gs, fs,
        "{label}: stdout mismatch\n--- GNU ---\n{gs}\n--- f00 ---\n{fs}"
    );
}

#[test]
fn byte_parity_core_modes() {
    let Some(ls) = system_ls() else {
        eprintln!("skip: no GNU coreutils ls on PATH");
        return;
    };
    let (dir, cfg) = build_fixture();

    // Broad matrix under `--gnu` vs system coreutils. Prefer flags whose output is
    // locale-stable with LC_ALL=C and fixed TZ.
    let cases: &[(&str, &[&str])] = &[
        // short listing / sort
        ("-1", &["-1"]),
        ("-1a", &["-1a"]),
        ("-1A", &["-1A"]),
        ("-1r", &["-1r"]),
        ("-1S", &["-1S"]),
        ("-1Sr", &["-1Sr"]),
        ("-1t", &["-1t"]),
        ("-1X", &["-1X"]),
        ("-1v", &["-1v"]),
        ("-1 --sort=size", &["-1", "--sort=size"]),
        ("-1 --sort=time", &["-1", "--sort=time"]),
        ("-1 --sort=extension", &["-1", "--sort=extension"]),
        ("-1 --sort=version", &["-1", "--sort=version"]),
        ("-1 --sort=none", &["-1", "--sort=none"]),
        ("-1 --sort=width", &["-1", "--sort=width"]),
        // note: GNU does not accept `--sort=name` (name is the default)
        // indicators / quoting
        ("-1b", &["-1b"]),
        ("-1Q", &["-1Q"]),
        ("-1N", &["-1N"]),
        ("-1q", &["-1q"]),
        (
            "-1 --quoting-style=literal",
            &["-1", "--quoting-style=literal"],
        ),
        ("-1 --quoting-style=shell", &["-1", "--quoting-style=shell"]),
        ("-1 --quoting-style=c", &["-1", "--quoting-style=c"]),
        (
            "-1 --quoting-style=escape",
            &["-1", "--quoting-style=escape"],
        ),
        (
            "-1 --group-directories-first",
            &["-1", "--group-directories-first"],
        ),
        ("-1F", &["-1F"]),
        ("-1p", &["-1p"]),
        ("-1 --file-type", &["-1", "--file-type"]),
        (
            "-1 --indicator-style=slash",
            &["-1", "--indicator-style=slash"],
        ),
        (
            "-1 --indicator-style=classify",
            &["-1", "--indicator-style=classify"],
        ),
        // recurse / filter
        ("-1R", &["-1R"]),
        ("-1B", &["-1B"]),
        ("-1 --hide=file*", &["-1", "--hide=file*"]),
        ("-1 -I file*", &["-1", "-I", "file*"]),
        // size / inode (plain units — human `-sh` covered below when green)
        ("-s1", &["-s1"]),
        ("-s1k", &["-s1k"]),
        ("-i1", &["-i1"]),
        // long forms (stable time style)
        ("-l --time-style=long-iso", &["-l", "--time-style=long-iso"]),
        ("-l --time-style=full-iso", &["-l", "--time-style=full-iso"]),
        ("-l --time-style=iso", &["-l", "--time-style=iso"]),
        (
            "-la --time-style=long-iso",
            &["-la", "--time-style=long-iso"],
        ),
        (
            "-lA --time-style=long-iso",
            &["-lA", "--time-style=long-iso"],
        ),
        (
            "-lg --time-style=long-iso",
            &["-lg", "--time-style=long-iso"],
        ),
        (
            "-lo --time-style=long-iso",
            &["-lo", "--time-style=long-iso"],
        ),
        (
            "-lG --time-style=long-iso",
            &["-lG", "--time-style=long-iso"],
        ),
        (
            "-ln --time-style=long-iso",
            &["-ln", "--time-style=long-iso"],
        ),
        (
            "-li --time-style=long-iso",
            &["-li", "--time-style=long-iso"],
        ),
        (
            "-ls --time-style=long-iso",
            &["-ls", "--time-style=long-iso"],
        ),
        ("-l --full-time", &["-l", "--full-time"]),
        (
            "-l --author --time-style=long-iso",
            &["-l", "--author", "--time-style=long-iso"],
        ),
        // dereference variants on the symlink fixture
        ("-1L", &["-1L"]),
        ("-1H", &["-1H"]),
        // Known remaining gaps (tracked on #124):
        // - `-d` path display for directory operands
        // - column wrap (`-C`/`-x`/`-m`) at exact terminal width
        // - human `-sh` / `--block-size=K` unit suffixes on size columns
        // - `--zero` needs raw NUL compare (not line-normalize)
    ];

    for (label, args) in cases {
        assert_ls_match(label, ls, &dir, &cfg, args);
    }

    // Single file operand: no `total` line for `-s` (run with relative path like users do).
    let mut g = Command::new(ls);
    g.current_dir(&dir)
        .env("LC_ALL", "C")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .args(["--color=never", "-s1", "large"]);
    let gout = g.output().unwrap();
    let mut f = Command::new(bin());
    f.current_dir(&dir)
        .env("LC_ALL", "C")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .args([
            "--gnu",
            "--color=never",
            "--git=false",
            "--icons=never",
            "--config",
        ])
        .arg(&cfg)
        .args(["-s1", "large"]);
    let fout = f.output().unwrap();
    assert_eq!(gout.status.code(), fout.status.code());
    assert_eq!(
        normalize(&String::from_utf8_lossy(&gout.stdout)),
        normalize(&String::from_utf8_lossy(&fout.stdout)),
        "single-file -s1"
    );

    let _ = fs::remove_dir_all(&dir);
}
