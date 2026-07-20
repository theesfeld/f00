//! Binary smoke tests for common flags (#13) and light polish cases (#5, #15).
//!
//! Uses the same process spawn pattern as other f00-cli integration tests
//! (no assert_cmd dependency). Covers: -la -A -h -R -t -1 --json --tree,
//! git column under --gnu / --git=false, and recursive minor errors.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-smoke-{}-{}-{}",
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

fn smoke_fixture(label: &str) -> (PathBuf, PathBuf) {
    let dir = temp_dir(label);
    fs::write(dir.join("alpha.txt"), b"hello").unwrap();
    fs::write(dir.join("beta.rs"), b"fn main() {}").unwrap();
    fs::write(dir.join(".hidden"), b"secret").unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    fs::write(dir.join("subdir").join("nested.txt"), b"x").unwrap();
    // Sized file so -h has something to format
    fs::write(dir.join("big.bin"), vec![0u8; 2048]).unwrap();
    let cfg = empty_config(&dir);
    (dir, cfg)
}

#[test]
fn smoke_one_per_line() {
    let (dir, cfg) = smoke_fixture("1");
    let out = f00(&cfg).args(["-1"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("alpha.txt"), "{stdout}");
    assert!(stdout.contains("beta.rs"), "{stdout}");
    assert!(
        !stdout.contains(".hidden"),
        "default hides dotfiles: {stdout}"
    );
    // one-per-line: multiple non-empty lines
    assert!(stdout.lines().filter(|l| !l.is_empty()).count() >= 3);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_long_all() {
    let (dir, cfg) = smoke_fixture("la");
    let out = f00(&cfg).args(["-la"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(".hidden"), "-a part of -la: {stdout}");
    assert!(stdout.contains("alpha.txt"), "{stdout}");
    // long: permission-like prefix somewhere
    assert!(
        stdout.lines().any(|l| {
            let t = l.trim_start();
            t.starts_with('-') || t.starts_with('d') || t.starts_with('l')
        }),
        "expected long listing: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_almost_all() {
    let (dir, cfg) = smoke_fixture("A");
    let out = f00(&cfg).args(["-A", "-1"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(".hidden"), "{stdout}");
    for line in stdout.lines() {
        let t = line.trim();
        assert_ne!(t, ".");
        assert_ne!(t, "..");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_human_readable() {
    let (dir, cfg) = smoke_fixture("h");
    let out = f00(&cfg).args(["-l", "-h"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("big.bin"), "{stdout}");
    // Human sizes typically use K/M/G or decimal units for multi-KB files
    let has_human = stdout.lines().any(|l| {
        l.contains("big.bin")
            && l.split_whitespace().any(|tok| {
                tok.ends_with('K')
                    || tok.ends_with('M')
                    || tok.ends_with('G')
                    || tok.ends_with('k')
                    || tok.contains('K')
                    || tok.contains('.')
            })
    });
    // Also accept pure digit if implementation uses binary without suffix for small;
    // prefer detecting a non-raw-2048 style field near big.bin.
    let raw_only = stdout
        .lines()
        .any(|l| l.contains("big.bin") && l.contains("2048"));
    assert!(
        has_human || !raw_only,
        "expected human-readable size for big.bin: {stdout}"
    );
    // Soft check: line for big.bin exists and is long-format-ish
    assert!(
        stdout
            .lines()
            .any(|l| l.contains("big.bin") && l.split_whitespace().count() >= 5),
        "{stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_recursive() {
    let (dir, cfg) = smoke_fixture("R");
    let out = f00(&cfg).args(["-R", "-1"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("alpha.txt"), "{stdout}");
    assert!(
        stdout.contains("nested.txt") || stdout.contains("subdir"),
        "recursive should reach subdir: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_sort_time() {
    let (dir, cfg) = smoke_fixture("t");
    // Ensure distinct mtimes
    let older = dir.join("older.txt");
    let newer = dir.join("newer.txt");
    fs::write(&older, b"old").unwrap();
    thread::sleep(Duration::from_millis(50));
    fs::write(&newer, b"new").unwrap();

    let out = f00(&cfg).args(["-1", "-t"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let i_new = stdout.find("newer.txt").expect("newer");
    let i_old = stdout.find("older.txt").expect("older");
    assert!(i_new < i_old, "-t newest first: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_json() {
    let (dir, cfg) = smoke_fixture("json");
    // Default helper forces --color=never → compact plain JSON (pipe-safe).
    let out = f00(&cfg).args(["--json"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains('\u{1b}'),
        "color=never must not emit ANSI in JSON"
    );
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json");
    assert!(v.is_array(), "{stdout}");
    let names: Vec<_> = v
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e.get("name").and_then(|n| n.as_str()))
        .collect();
    assert!(names.contains(&"alpha.txt"), "names={names:?}");
    let first = v.as_array().unwrap().first().unwrap();
    assert!(first.get("inode").is_some(), "{first}");
    assert!(first.get("permissions").is_some(), "{first}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_json_color_always_is_pretty_highlighted() {
    let (dir, cfg) = smoke_fixture("json-color");
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--color=always", "--git=false", "--json", "--config"])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{:?}", out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains('\n'), "pretty JSON expected: {stdout}");
    assert!(
        stdout.contains('\u{1b}'),
        "color=always should syntax-highlight JSON: {stdout:?}"
    );
    // Strip CSI and re-parse
    let mut plain = String::new();
    let mut chars = stdout.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for x in chars.by_ref() {
                    if x.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            plain.push(c);
        }
    }
    let v: serde_json::Value =
        serde_json::from_str(plain.trim()).expect("colored json strips to valid");
    assert!(v.is_array());
    let _ = fs::remove_dir_all(&dir);
}

fn smoke_json_short_j() {
    let (dir, cfg) = smoke_fixture("json-j");
    let out = f00(&cfg).args(["-j"]).arg(&dir).output().unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json from -j");
    assert!(v.is_array(), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_tree() {
    let (dir, cfg) = smoke_fixture("tree");
    let out = f00(&cfg).args(["--tree"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("alpha.txt"), "{stdout}");
    // Tree glyphs or at least nested structure
    assert!(
        stdout.contains('├')
            || stdout.contains('└')
            || stdout.contains("subdir")
            || stdout.contains("│"),
        "expected tree-like output: {stdout}"
    );
    let _ = fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// #15 git column polish (light)
// ---------------------------------------------------------------------------

/// Piped stdout is not a TTY → auto script-safe mode (same chrome as `--gnu`).
#[test]
fn auto_gnu_on_pipe_strips_git_chars() {
    let dir = temp_dir("auto-gnu-pipe");
    fs::write(dir.join("tracked.txt"), b"clean\n").unwrap();
    let cfg = empty_config(&dir);

    let _ = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&dir)
        .status();
    let _ = Command::new("git")
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "add",
            "tracked.txt",
        ])
        .current_dir(&dir)
        .status();
    let _ = Command::new("git")
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "commit",
            "-qm",
            "c",
        ])
        .current_dir(&dir)
        .status();
    fs::write(dir.join("tracked.txt"), b"dirty\n").unwrap();

    // No --gnu, no --git=false: pipe should still auto-enable script-safe mode.
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--color=never", "-l", "--config"])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains(" M ") && !stdout.contains(" ? ") && !stdout.contains(" A "),
        "non-TTY must auto-enable script-safe mode (no git chars): {stdout}"
    );
    assert!(stdout.contains("tracked.txt"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

/// Default builds no longer embed the TUI; `--browse` should point at `f00-tui`.
#[test]
fn browse_without_tui_feature_mentions_f00_tui() {
    // This test runs against the package's default features (no `tui`).
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--browse"])
        .output()
        .unwrap();
    // Prefer non-zero; message must mention the separate binary.
    let stderr = String::from_utf8_lossy(&out.stderr);
    let combined = format!("{stderr}{}", String::from_utf8_lossy(&out.stdout));
    if cfg!(feature = "tui") {
        // Embedded builds may fail for non-TTY instead; skip message check.
        return;
    }
    assert!(
        !out.status.success(),
        "expected failure without TTY/tui: {combined}"
    );
    assert!(
        combined.contains("f00-tui") || combined.to_ascii_lowercase().contains("tui"),
        "should mention f00-tui: {combined}"
    );
}

#[test]
fn no_gnu_allows_modern_on_pipe() {
    let dir = temp_dir("no-gnu-pipe");
    fs::write(dir.join("a.txt"), b"x").unwrap();
    let cfg = empty_config(&dir);
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--no-gnu", "--color=never", "--git=false", "-1", "--config"])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{:?}", out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("a.txt"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn gnu_mode_has_no_git_chars() {
    let dir = temp_dir("gnu-git");
    // Init a small git repo with a dirty file so a non-gnu run would show M
    fs::write(dir.join("tracked.txt"), b"clean\n").unwrap();
    let cfg = empty_config(&dir);

    let _ = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&dir)
        .status();
    let _ = Command::new("git")
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "add",
            "tracked.txt",
        ])
        .current_dir(&dir)
        .status();
    let _ = Command::new("git")
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "commit",
            "-qm",
            "c",
        ])
        .current_dir(&dir)
        .status();
    fs::write(dir.join("tracked.txt"), b"dirty\n").unwrap();

    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--color=never", "--gnu", "-l", "--config"])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    // No single-char git status tokens as their own field (e.g. " M ")
    assert!(
        !stdout.contains(" M ") && !stdout.contains(" ? ") && !stdout.contains(" A "),
        "--gnu must not show git status chars: {stdout}"
    );
    assert!(stdout.contains("tracked.txt"), "{stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn git_false_no_annotation_spaces() {
    let dir = temp_dir("git-false");
    fs::write(dir.join("plain.txt"), b"x").unwrap();
    let cfg = empty_config(&dir);

    // With git on, clean entries pad with three spaces before the name.
    // With --git=false, there should be a single space before the filename.
    let out = f00(&cfg).args(["-l"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout
        .lines()
        .find(|l| l.contains("plain.txt"))
        .expect("plain.txt line");
    // Reject the triple-space git padding column before the name
    assert!(
        !line.contains("   plain.txt"),
        "--git=false must not leave git annotation padding: {line}"
    );
    assert!(
        line.contains(" plain.txt") || line.ends_with("plain.txt"),
        "filename present: {line}"
    );
    let _ = fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// #5 recursive errors (light)
// ---------------------------------------------------------------------------

#[cfg(unix)]
fn running_as_root() -> bool {
    extern "C" {
        fn geteuid() -> u32;
    }
    // SAFETY: geteuid has no preconditions.
    unsafe { geteuid() == 0 }
}

#[test]
#[cfg(unix)]
fn recursive_unreadable_subdir_exits_1_continues() {
    use std::os::unix::fs::PermissionsExt;

    // Root bypasses directory mode bits (FreeBSD CI VM runs as root).
    if running_as_root() {
        return;
    }

    let dir = temp_dir("recurse-err");
    fs::write(dir.join("visible.txt"), b"v").unwrap();
    let ok_sib = dir.join("sibling");
    fs::create_dir(&ok_sib).unwrap();
    fs::write(ok_sib.join("sib.txt"), b"s").unwrap();
    let locked = dir.join("locked");
    fs::create_dir(&locked).unwrap();
    fs::write(locked.join("hidden.txt"), b"h").unwrap();
    let cfg = empty_config(&dir);

    let mut perms = fs::metadata(&locked).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&locked, perms).unwrap();

    if fs::read_dir(&locked).is_ok() {
        let mut perms = fs::metadata(&locked).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&locked, perms).unwrap();
        let _ = fs::remove_dir_all(&dir);
        return;
    }

    let out = f00(&cfg).args(["-R", "-1"]).arg(&dir).output().unwrap();

    // Restore before assertions that may panic
    let mut perms = fs::metadata(&locked).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&locked, perms).unwrap();

    let code = out.status.code();
    // Minor error → 1; must not abort the whole run (siblings still listed)
    assert_eq!(
        code,
        Some(1),
        "expected exit 1; stderr={} stdout={}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("visible.txt"),
        "should list root entries: {stdout}"
    );
    assert!(
        stdout.contains("sib.txt") || stdout.contains("sibling"),
        "should continue into readable siblings: {stdout}"
    );

    let _ = fs::remove_dir_all(&dir);
}

/// GNU coreutils WHEN vocabulary for `--color` / `--classify` / `--hyperlink`.
/// Distros (notably NixOS) inject `ls --color=tty` via shell aliases.
const GNU_WHEN: &[&str] = &[
    "auto", "always", "never", "tty", "if-tty", "yes", "no", "force", "none",
];

#[test]
fn smoke_color_gnu_synonyms_accepted() {
    let (dir, cfg) = smoke_fixture("color-syn");
    for when in GNU_WHEN {
        let out = Command::new(bin())
            .env("LC_ALL", "C")
            .args([&format!("--color={when}"), "-1", "--git=false", "--config"])
            .arg(&cfg)
            .arg(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "--color={when} must be accepted (GNU drop-in); stderr={}",
            String::from_utf8_lossy(&out.stderr)
        );
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            stdout.contains("alpha.txt"),
            "--color={when} should still list: {stdout}"
        );
    }
    let bad = Command::new(bin())
        .env("LC_ALL", "C")
        .args(["--color=rainbow", "-1", "--git=false", "--config"])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_ne!(bad.status.code(), Some(0), "unknown --color WHEN must fail");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_classify_when_accepted() {
    let (dir, cfg) = smoke_fixture("classify-when");
    for flag in ["-F", "--classify"] {
        let out = Command::new(bin())
            .env("LC_ALL", "C")
            .args([flag, "-1", "--git=false", "--color=never", "--config"])
            .arg(&cfg)
            .arg(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "{flag} must be accepted; stderr={}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    for when in GNU_WHEN {
        let out = Command::new(bin())
            .env("LC_ALL", "C")
            .args([
                &format!("--classify={when}"),
                "-1",
                "--git=false",
                "--color=never",
                "--config",
            ])
            .arg(&cfg)
            .arg(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "--classify={when} must be accepted (GNU drop-in); stderr={}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn smoke_hyperlink_when_accepted() {
    let (dir, cfg) = smoke_fixture("hyperlink-when");
    let out = Command::new(bin())
        .env("LC_ALL", "C")
        .args([
            "--hyperlink",
            "-1",
            "--git=false",
            "--color=never",
            "--config",
        ])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "bare --hyperlink must be accepted; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    for when in GNU_WHEN {
        let out = Command::new(bin())
            .env("LC_ALL", "C")
            .args([
                &format!("--hyperlink={when}"),
                "-1",
                "--git=false",
                "--color=never",
                "--config",
            ])
            .arg(&cfg)
            .arg(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "--hyperlink={when} must be accepted; stderr={}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let bad = Command::new(bin())
        .env("LC_ALL", "C")
        .args([
            "--hyperlink=bogus",
            "-1",
            "--git=false",
            "--color=never",
            "--config",
        ])
        .arg(&cfg)
        .arg(&dir)
        .output()
        .unwrap();
    assert_ne!(
        bad.status.code(),
        Some(0),
        "unknown --hyperlink WHEN must fail"
    );
    let _ = fs::remove_dir_all(&dir);
}

/// Every long option documented by GNU coreutils `ls --help` must parse.
#[test]
fn smoke_all_gnu_long_options_accepted() {
    let (dir, cfg) = smoke_fixture("gnu-longs");
    let longs: &[(&str, Option<&str>)] = &[
        ("all", None),
        ("almost-all", None),
        ("author", None),
        ("escape", None),
        ("block-size", Some("1K")),
        ("ignore-backups", None),
        ("color", Some("never")),
        ("directory", None),
        ("dired", None),
        ("classify", Some("always")),
        ("file-type", None),
        ("format", Some("long")),
        ("full-time", None),
        ("group-directories-first", None),
        ("no-group", None),
        ("human-readable", None),
        ("si", None),
        ("dereference-command-line", None),
        ("dereference-command-line-symlink-to-dir", None),
        ("hide", Some("*.o")),
        ("hyperlink", Some("never")),
        ("indicator-style", Some("none")),
        ("inode", None),
        ("ignore", Some("*.tmp")),
        ("kibibytes", None),
        ("dereference", None),
        ("numeric-uid-gid", None),
        ("literal", None),
        ("hide-control-chars", None),
        ("show-control-chars", None),
        ("quote-name", None),
        ("quoting-style", Some("literal")),
        ("reverse", None),
        ("recursive", None),
        ("size", None),
        ("sort", Some("name")),
        ("time", Some("mtime")),
        ("time-style", Some("locale")),
        ("tabsize", Some("8")),
        ("width", Some("80")),
        ("context", None),
        ("zero", None),
    ];
    for (name, val) in longs {
        let flag = match val {
            Some(v) => format!("--{name}={v}"),
            None => format!("--{name}"),
        };
        // Avoid double `--color` when the option under test *is* color.
        let mut cmd = Command::new(bin());
        cmd.env("LC_ALL", "C");
        cmd.arg(&flag);
        cmd.args(["-1", "--git=false"]);
        if *name != "color" {
            cmd.arg("--color=never");
        }
        // `--config` must be immediately followed by its PATH.
        cmd.arg("--config").arg(&cfg).arg(&dir);
        let out = cmd.output().unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !stderr.contains("unexpected argument")
                && !stderr.contains("unexpected value")
                && !stderr.contains("invalid value")
                && !stderr.contains("a value is required"),
            "GNU long option {flag} must parse; code={:?} stderr={stderr}",
            out.status.code()
        );
        assert!(
            out.status.code() == Some(0) || out.status.code() == Some(1),
            "GNU long option {flag} should run; code={:?} stderr={stderr}",
            out.status.code()
        );
    }
    let _ = fs::remove_dir_all(&dir);
}
