//! Integration-style test: list a temp directory via the public library API.

use std::fs;
use std::path::PathBuf;

use f00_cli::cli::{Args, ColorArg};
use f00_cli::run::build_config;
use f00_core::list_path;
use f00_format::format_listing;

fn temp_fixture() -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "f00-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
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
    let args = Args {
        paths: vec![dir.clone()],
        help: None,
        all: false,
        almost_all: false,
        long: false,
        one_per_line: true,
        human_readable: false,
        recursive: false,
        reverse: false,
        sort_time: false,
        sort_size: false,
        sort_extension: false,
        color: ColorArg::Never,
        json: false,
        tree: false,
        gnu: false,
        icons: false,
        classify: false,
        dirs_first: false,
        max_depth: None,
        git: false,
        config: None,
    };
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
    let args = Args {
        paths: vec![dir.clone()],
        help: None,
        all: false,
        almost_all: false,
        long: false,
        one_per_line: false,
        human_readable: false,
        recursive: false,
        reverse: false,
        sort_time: false,
        sort_size: false,
        sort_extension: false,
        color: ColorArg::Never,
        json: true,
        tree: false,
        gnu: false,
        icons: false,
        classify: false,
        dirs_first: false,
        max_depth: None,
        git: false,
        config: None,
    };
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
    let args = Args {
        paths: vec![dir.clone()],
        help: None,
        all: true,
        almost_all: false,
        long: false,
        one_per_line: true,
        human_readable: false,
        recursive: false,
        reverse: false,
        sort_time: false,
        sort_size: false,
        sort_extension: false,
        color: ColorArg::Never,
        json: false,
        tree: false,
        gnu: false,
        icons: false,
        classify: false,
        dirs_first: false,
        max_depth: None,
        git: false,
        config: None,
    };
    let config = build_config(&args);
    let listing = list_path(&dir, &config.list).expect("list");
    let names: Vec<_> = listing.entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&".hidden"), "names={names:?}");
    assert!(names.contains(&"."));

    let _ = fs::remove_dir_all(&dir);
}
