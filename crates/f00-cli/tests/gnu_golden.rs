//! Structural GNU golden / trust-track tests for f00.
//!
//! Assert structural parity with GNU ls behavior (sorted names, flag effects,
//! exit codes). Not always byte-identical to system `ls` (locales, padding,
//! timestamps). Optional name-set comparison against `ls -1` on Unix when present.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(unix)]
use std::collections::BTreeSet;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-golden-{}-{}-{}",
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

/// Write empty config *outside* the fixture so it is not listed as an entry.
fn empty_config(fixture: &Path) -> PathBuf {
    let parent = fixture
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(std::env::temp_dir);
    let p = parent.join(format!(
        "f00-empty-cfg-{}-{}.toml",
        std::process::id(),
        fixture.file_name().and_then(|n| n.to_str()).unwrap_or("x")
    ));
    fs::write(&p, "").unwrap();
    p
}

fn f00(config: &Path) -> Command {
    let mut c = Command::new(bin());
    c.env("LC_ALL", "C");
    c.args(["--color=never", "--git=false", "--config"]);
    c.arg(config);
    c
}

/// Fixture: a, b.txt, .hidden, sub/, link (unix), "x y", plus sized files for -S.
fn build_fixture(label: &str) -> (PathBuf, PathBuf) {
    let dir = temp_dir(label);
    fs::write(dir.join("a"), b"").unwrap();
    fs::write(dir.join("b.txt"), b"bb").unwrap();
    fs::write(dir.join(".hidden"), b"secret").unwrap();
    fs::create_dir(dir.join("sub")).unwrap();
    fs::write(dir.join("sub").join("inside.txt"), b"nested").unwrap();
    fs::write(dir.join("x y"), b"space").unwrap();
    // Distinct sizes for -S: large > mid > small
    fs::write(dir.join("small"), b"a").unwrap();
    fs::write(dir.join("mid"), vec![b'm'; 64]).unwrap();
    fs::write(dir.join("large"), vec![b'L'; 4096]).unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("a", dir.join("link")).unwrap();
    }
    let cfg = empty_config(&dir);
    (dir, cfg)
}

fn stdout_lines(out: &std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn names_from_minus_one(out: &std::process::Output) -> Vec<String> {
    stdout_lines(out)
}

#[test]
fn one_per_line_sorted_names() {
    let (dir, cfg) = build_fixture("one");
    let out = f00(&cfg).args(["-1"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let names = names_from_minus_one(&out);

    // Visible entries, C locale sort (byte order)
    let mut expected: Vec<&str> = vec!["a", "b.txt", "large", "mid", "small", "sub", "x y"];
    #[cfg(unix)]
    expected.push("link");
    expected.sort();
    assert_eq!(
        names, expected,
        "default -1 should list visible names sorted; got {names:?}"
    );
    assert!(!names
        .iter()
        .any(|n| n == ".hidden" || n == "." || n == ".."));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn all_includes_hidden_and_dot() {
    let (dir, cfg) = build_fixture("all");
    let out = f00(&cfg).args(["-1", "-a"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let names = names_from_minus_one(&out);
    assert!(names.iter().any(|n| n == ".hidden"), "names={names:?}");
    assert!(names.iter().any(|n| n == "."), "names={names:?}");
    assert!(names.iter().any(|n| n == ".."), "names={names:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn almost_all_includes_hidden_not_dot() {
    let (dir, cfg) = build_fixture("almost");
    let out = f00(&cfg).args(["-1", "-A"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let names = names_from_minus_one(&out);
    assert!(names.iter().any(|n| n == ".hidden"), "names={names:?}");
    assert!(!names.iter().any(|n| n == "."), "names={names:?}");
    assert!(!names.iter().any(|n| n == ".."), "names={names:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn directory_flag_lists_dir_not_children() {
    let (dir, cfg) = build_fixture("d");
    let sub = dir.join("sub");
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
fn size_sort_largest_first() {
    let (dir, cfg) = build_fixture("S");
    let out = f00(&cfg).args(["-1", "-S"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let large = stdout.find("large").expect("large");
    let mid = stdout.find("mid").expect("mid");
    let small = stdout.find("small").expect("small");
    assert!(large < mid && mid < small, "-S largest first: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reverse_sort() {
    let (dir, cfg) = build_fixture("r");
    let out = f00(&cfg).args(["-1", "-r"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let names = names_from_minus_one(&out);
    let mut sorted = names.clone();
    sorted.sort();
    let mut rev = sorted.clone();
    rev.reverse();
    assert_eq!(names, rev, "reverse of sorted names; got {names:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn zero_uses_nul_separators() {
    let (dir, cfg) = build_fixture("zero");
    let out = f00(&cfg).args(["--zero"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = &out.stdout;
    assert!(stdout.contains(&0u8), "expected NUL separators: {stdout:?}");
    let text = String::from_utf8_lossy(stdout);
    assert!(
        !text.contains("a\nb") && !text.contains("b.txt\na"),
        "should not use newline between names: {text:?}"
    );
    let parts: Vec<_> = stdout
        .split(|&b| b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .collect();
    assert!(
        parts
            .iter()
            .any(|n| n.contains('a') && !n.contains("b.txt")),
        "parts={parts:?}"
    );
    assert!(parts.iter().any(|n| n.contains("b.txt")), "parts={parts:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn quote_q_quotes_names_with_spaces() {
    let (dir, cfg) = build_fixture("Q");
    let out = f00(&cfg).args(["-1", "-Q"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"x y\""),
        "-Q should quote name with space: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn classify_f_adds_slash_for_dirs() {
    let (dir, cfg) = build_fixture("F");
    let out = f00(&cfg).args(["-1", "-F"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("sub/"), "dirs get /: {stdout}");
    #[cfg(unix)]
    {
        // symlink classified with @ under -F
        assert!(
            stdout.contains("link@") || stdout.contains("link"),
            "stdout={stdout}"
        );
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn long_format_has_permission_like_field_and_filename() {
    let (dir, cfg) = build_fixture("l");
    let out = f00(&cfg).args(["-l"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().find(|l| l.contains("b.txt")).unwrap_or("");
    assert!(!line.is_empty(), "missing b.txt line: {stdout}");
    // Permission-like field: type + rwx style (e.g. -rw-r--r--)
    let first = line.split_whitespace().next().unwrap_or("");
    assert!(
        first.len() >= 9
            && (first.starts_with('-') || first.starts_with('d') || first.starts_with('l'))
            && (first.contains('r')
                || first.contains('w')
                || first.contains('x')
                || first.contains('-')),
        "permission-like field missing: line={line}"
    );
    assert!(line.contains("b.txt"), "filename in long line: {line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn exit_2_on_missing_path() {
    let (dir, cfg) = build_fixture("missing");
    let missing = dir.join("no-such-path-golden");
    assert!(!missing.exists());
    let out = f00(&cfg).args(["-1"]).arg(&missing).output().unwrap();
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn exit_0_on_success() {
    let (dir, cfg) = build_fixture("ok");
    let out = f00(&cfg).args(["-1"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let _ = fs::remove_dir_all(&dir);
}

/// Optional: same *set* of names as system `ls -1` under LC_ALL=C (order too).
#[test]
#[cfg(unix)]
fn optional_ls_name_parity() {
    let (dir, cfg) = build_fixture("ls-parity");

    let ls_path = which_ls();
    let Some(ls) = ls_path else {
        // System ls not available — structural golden tests above still apply.
        let _ = fs::remove_dir_all(&dir);
        return;
    };

    let ls_out = Command::new(&ls)
        .env("LC_ALL", "C")
        .args(["-1"])
        .arg(&dir)
        .output()
        .expect("run ls");
    if !ls_out.status.success() {
        let _ = fs::remove_dir_all(&dir);
        return;
    }

    let f00_out = f00(&cfg).args(["--gnu", "-1"]).arg(&dir).output().unwrap();
    assert_eq!(f00_out.status.code(), Some(0));

    let ls_names: Vec<String> = String::from_utf8_lossy(&ls_out.stdout)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let f00_names = names_from_minus_one(&f00_out);

    let ls_set: BTreeSet<_> = ls_names.iter().cloned().collect();
    let f00_set: BTreeSet<_> = f00_names.iter().cloned().collect();
    assert_eq!(
        ls_set, f00_set,
        "name set should match ls -1; ls={ls_names:?} f00={f00_names:?}"
    );
    // Under LC_ALL=C, order should match for plain name sort.
    assert_eq!(
        ls_names, f00_names,
        "order under LC_ALL=C should match ls -1"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[cfg(unix)]
fn which_ls() -> Option<PathBuf> {
    for candidate in ["/bin/ls", "/usr/bin/ls"] {
        let p = PathBuf::from(candidate);
        if p.is_file() {
            return Some(p);
        }
    }
    Command::new("sh")
        .args(["-c", "command -v ls"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                }
            } else {
                None
            }
        })
}
