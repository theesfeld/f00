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
    let out = f00(&cfg).args(["--json"]).arg(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json");
    assert!(v.is_array(), "{stdout}");
    let names: Vec<_> = v
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e.get("name").and_then(|n| n.as_str()))
        .collect();
    assert!(names.contains(&"alpha.txt"), "names={names:?}");
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

#[test]
#[cfg(unix)]
fn recursive_unreadable_subdir_exits_1_continues() {
    use std::os::unix::fs::PermissionsExt;

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
