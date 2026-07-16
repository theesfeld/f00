//! Criterion benchmarks: sequential vs parallel directory listing.
//!
//! Run from the workspace root:
//! ```bash
//! cargo bench -p f00-core --bench list_bench
//! ```

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use f00_core::{list_directory, ListOptions};

const FILE_COUNT: usize = 1000;

fn bench_dir() -> &'static PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir =
            std::env::temp_dir().join(format!("f00-list-bench-{}-{}", std::process::id(), nanos));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("mkdir bench dir");
        for i in 0..FILE_COUNT {
            fs::write(dir.join(format!("file_{i:04}.txt")), b"x").expect("write");
        }
        dir
    })
}

fn list_sequential(c: &mut Criterion) {
    let dir = bench_dir();
    let opts = ListOptions {
        parallel: false,
        threads: 1,
        ..Default::default()
    };
    c.bench_function("list_directory_1000_sequential", |b| {
        b.iter(|| {
            let listing = list_directory(black_box(dir.as_path()), black_box(&opts))
                .expect("list sequential");
            black_box(listing.entries.len())
        })
    });
}

fn list_parallel(c: &mut Criterion) {
    let dir = bench_dir();
    let opts = ListOptions {
        parallel: true,
        threads: 0,
        ..Default::default()
    };
    c.bench_function("list_directory_1000_parallel", |b| {
        b.iter(|| {
            let listing =
                list_directory(black_box(dir.as_path()), black_box(&opts)).expect("list parallel");
            black_box(listing.entries.len())
        })
    });
}

fn list_parallel_fixed_threads(c: &mut Criterion) {
    let dir = bench_dir();
    let opts = ListOptions {
        parallel: true,
        threads: 4,
        ..Default::default()
    };
    c.bench_function("list_directory_1000_parallel_4t", |b| {
        b.iter(|| {
            let listing = list_directory(black_box(dir.as_path()), black_box(&opts))
                .expect("list parallel 4t");
            black_box(listing.entries.len())
        })
    });
}

criterion_group!(
    benches,
    list_sequential,
    list_parallel,
    list_parallel_fixed_threads
);
criterion_main!(benches);
