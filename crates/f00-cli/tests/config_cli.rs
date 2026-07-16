//! Config file integration via `--config`.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_f00"))
}

fn temp_dir(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-cfg-{}-{}-{}",
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

#[test]
fn config_all_shows_hidden() {
    let dir = temp_dir("all");
    fs::write(dir.join("visible.txt"), b"v").unwrap();
    fs::write(dir.join(".secret"), b"s").unwrap();

    let cfg_path = dir.join("cfg.toml");
    fs::write(
        &cfg_path,
        r#"
        [defaults]
        all = true
        "#,
    )
    .unwrap();

    let output = Command::new(bin())
        .args(["--color=never", "--git=false", "-1", "--config"])
        .arg(&cfg_path)
        .arg(&dir)
        .output()
        .expect("spawn");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(".secret"),
        "config all=true should show hidden; stdout={stdout}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn cli_flag_overrides_absence_of_config_long() {
    let dir = temp_dir("long");
    fs::write(dir.join("a.txt"), b"aaaaaaaaaa").unwrap();

    let cfg_path = dir.join("cfg.toml");
    fs::write(&cfg_path, "long = false\n").unwrap();

    let output = Command::new(bin())
        .args(["--color=never", "--git=false", "-l", "--config"])
        .arg(&cfg_path)
        .arg(&dir)
        .output()
        .expect("spawn");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Long format typically includes permissions or size column
    assert!(
        stdout.contains("a.txt")
            && (stdout.contains('-')
                || stdout.contains('r')
                || stdout.split_whitespace().count() > 2),
        "expected long listing; stdout={stdout}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn bad_config_path_exits_2() {
    let missing = std::env::temp_dir().join(format!(
        "f00-no-config-{}-{}.toml",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    let output = Command::new(bin())
        .args(["--config"])
        .arg(&missing)
        .arg(".")
        .output()
        .expect("spawn");

    assert_eq!(output.status.code(), Some(2));
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("config") || err.contains("f00:"),
        "stderr={err}"
    );
}
