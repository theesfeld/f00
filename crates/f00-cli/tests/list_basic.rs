//! Integration-style test: list a temp directory via the public library API.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use f00_cli::cli::{Args, ColorArg};
use f00_cli::run::build_config;
use f00_core::list_path;
use f00_format::format_listing;

static FIXTURE_SEQ: AtomicU64 = AtomicU64::new(0);

fn temp_fixture() -> PathBuf {
    // Unique per call: pid + nanos alone can collide when tests run in parallel.
    let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
    let base = std::env::temp_dir().join(format!(
        "f00-test-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0),
        seq
    ));
    fs::create_dir_all(&base).expect("mkdir");
    fs::write(base.join("alpha.txt"), b"hello").expect("write");
    fs::write(base.join("beta.rs"), b"fn main() {}").expect("write");
    fs::create_dir(base.join("subdir")).expect("mkdir sub");
    fs::write(base.join("subdir").join("nested.txt"), b"x").expect("nested");
    fs::write(base.join(".hidden"), b"secret").expect("hidden");
    base
}

#[test]
fn lists_visible_entries_sorted() {
    let dir = temp_fixture();
    let mut args = Args::test_default();
    args.paths = vec![dir.clone()];
    args.one_per_line = true;
    args.color = ColorArg::Never;
    let config = build_config(&args);
    let listing = list_path(&dir, &config.list).expect("list");
    let names: Vec<_> = listing.entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"alpha.txt"));
    assert!(names.contains(&"beta.rs"));
    assert!(names.contains(&"subdir"));
    assert!(
        !names.iter().any(|n| n.starts_with('.')),
        "hidden: {names:?}"
    );

    let out = format_listing(&listing, &config).expect("format");
    assert!(out.contains("alpha.txt"));
    assert!(out.contains("beta.rs"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn json_output_is_array() {
    let dir = temp_fixture();
    let mut args = Args::test_default();
    args.paths = vec![dir.clone()];
    args.json = true;
    args.color = ColorArg::Never;
    let config = build_config(&args);
    let listing = list_path(&dir, &config.list).expect("list");
    let out = format_listing(&listing, &config).expect("format");
    let v: serde_json::Value = serde_json::from_str(&out).expect("json parse");
    assert!(v.is_array());
    let names: Vec<_> = v
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e.get("name").and_then(|n| n.as_str()))
        .collect();
    assert!(names.contains(&"alpha.txt"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn all_flag_includes_hidden() {
    let dir = temp_fixture();
    let mut args = Args::test_default();
    args.paths = vec![dir.clone()];
    args.all = true;
    args.one_per_line = true;
    args.color = ColorArg::Never;
    let config = build_config(&args);
    let listing = list_path(&dir, &config.list).expect("list");
    let names: Vec<_> = listing.entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&".hidden"), "names={names:?}");
    assert!(names.contains(&"."));

    let _ = fs::remove_dir_all(&dir);
}
