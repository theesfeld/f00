//! Smoke: list a directory of many files with sequential and parallel options.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use f00_core::{list_directory, ListOptions};

fn temp_many(n: usize) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!(
        "f00-speed-smoke-{}-{}-{}",
        std::process::id(),
        nanos,
        n
    ));
    fs::create_dir_all(&dir).unwrap();
    for i in 0..n {
        fs::write(dir.join(format!("f{i:04}")), b"x").unwrap();
    }
    dir
}

#[test]
fn list_200_files_serial_and_parallel() {
    let dir = temp_many(200);
    let serial = ListOptions {
        parallel: false,
        threads: 1,
        ..Default::default()
    };
    let a = list_directory(&dir, &serial).unwrap();
    assert_eq!(a.entries.len(), 200);

    // FreeBSD CI (vmactions/qemu) has hit rayon SIGSEGV under parallel metadata;
    // serial path still validates listing correctness there.
    if cfg!(target_os = "freebsd") {
        let _ = fs::remove_dir_all(&dir);
        return;
    }

    let parallel = ListOptions {
        parallel: true,
        threads: 0,
        ..Default::default()
    };
    let b = list_directory(&dir, &parallel).unwrap();
    assert_eq!(b.entries.len(), 200);

    let names_a: Vec<_> = a.entries.iter().map(|e| e.name.as_str()).collect();
    let names_b: Vec<_> = b.entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names_a, names_b);

    let _ = fs::remove_dir_all(&dir);
}
