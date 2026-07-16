//! List contents of zip / tar / tar.gz archives without full extraction.
//!
//! Safety:
//! - Caps the number of entries returned ([`MAX_ARCHIVE_ENTRIES`]).
//! - Reads only central-directory / tar headers for metadata (no payload unpack).
//! - Does not write archive members to disk.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::SystemTime;

use anyhow::{bail, Context, Result};
use f00_core::{Entry, EntryKind, GitStatus, Listing};
use flate2::read::GzDecoder;

/// Hard cap on listed members to bound memory and zip/tar bombs.
pub const MAX_ARCHIVE_ENTRIES: usize = 100_000;

/// A single member inside an archive (headers only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    /// Path as stored in the archive (forward slashes normalized).
    pub name: String,
    /// Uncompressed size when known (0 for directories / unknown).
    pub size: u64,
    /// Whether this member is a directory.
    pub is_dir: bool,
    /// Compressed size when known (zip central directory); `None` for tar.
    pub compressed_size: Option<u64>,
    /// Modification time when the format provides it.
    pub modified: Option<SystemTime>,
}

impl ArchiveEntry {
    /// Convert to a synthetic [`Entry`] for formatting pipelines.
    ///
    /// `archive_path` is the on-disk archive; the entry path is
    /// `{archive_path}/{member_name}` for display/tooling.
    pub fn to_entry(&self, archive_path: &Path, depth: usize) -> Entry {
        let display_name = self
            .name
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or(self.name.as_str())
            .to_string();

        let path = archive_path.join(self.name.trim_end_matches('/'));

        Entry {
            path,
            name: if self.name.contains('/') {
                self.name.trim_end_matches('/').to_string()
            } else {
                display_name
            },
            kind: if self.is_dir {
                EntryKind::Directory
            } else {
                EntryKind::File
            },
            size: self.size,
            modified: self.modified,
            created: None,
            accessed: None,
            changed: None,
            mode: if self.is_dir { 0o755 } else { 0o644 },
            readonly: false,
            symlink_target: None,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: self.size.div_ceil(512),
            owner: String::new(),
            group: String::new(),
            author: String::new(),
            context: String::new(),
        }
    }
}

/// Return true when `path` looks like a supported archive by extension.
pub fn is_archive(path: &Path) -> bool {
    archive_kind(path).is_some()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchiveKind {
    Zip,
    Tar,
    TarGz,
}

fn archive_kind(path: &Path) -> Option<ArchiveKind> {
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        return Some(ArchiveKind::TarGz);
    }
    if name.ends_with(".tar") {
        return Some(ArchiveKind::Tar);
    }
    if name.ends_with(".zip") {
        return Some(ArchiveKind::Zip);
    }
    None
}

/// List archive members (headers only). Errors if path is not a known archive type.
pub fn list_archive(path: &Path) -> Result<Vec<ArchiveEntry>> {
    match archive_kind(path) {
        Some(ArchiveKind::Zip) => list_zip(path),
        Some(ArchiveKind::Tar) => list_tar(path, false),
        Some(ArchiveKind::TarGz) => list_tar(path, true),
        None => bail!("not a supported archive: {}", path.display()),
    }
}

/// List an archive as a [`Listing`] of synthetic entries for the f00 format pipeline.
pub fn list_archive_as_listing(path: &Path) -> Result<Listing> {
    let members = list_archive(path)?;
    let entries: Vec<Entry> = members.iter().map(|m| m.to_entry(path, 0)).collect();
    Ok(Listing {
        root: path.to_path_buf(),
        root_is_dir: false,
        entries,
        minor_errors: 0,
    })
}

fn normalize_name(name: &str) -> String {
    name.replace('\\', "/")
}

fn list_zip(path: &Path) -> Result<Vec<ArchiveEntry>> {
    let file = File::open(path).with_context(|| format!("open zip {}", path.display()))?;
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .with_context(|| format!("read zip {}", path.display()))?;

    let total = archive.len();
    if total > MAX_ARCHIVE_ENTRIES {
        bail!(
            "archive has {} entries (cap is {}); refusing to list",
            total,
            MAX_ARCHIVE_ENTRIES
        );
    }

    let mut out = Vec::with_capacity(total.min(MAX_ARCHIVE_ENTRIES));
    for i in 0..total {
        if out.len() >= MAX_ARCHIVE_ENTRIES {
            break;
        }
        let entry = archive
            .by_index(i)
            .with_context(|| format!("zip entry {i} in {}", path.display()))?;
        // `by_index` yields central-directory metadata; we never call `read_to_end`
        // or extract, so compressed payloads stay untouched.
        let raw_name = entry.name().to_string();
        let name = normalize_name(&raw_name);
        let is_dir = entry.is_dir() || name.ends_with('/');
        // Convert MS-DOS datetime from the central directory when present.
        // Avoids enabling zip's optional `time` feature.
        let modified = entry.last_modified().and_then(|dt| {
            use chrono::{NaiveDate, TimeZone, Utc};
            NaiveDate::from_ymd_opt(dt.year() as i32, dt.month() as u32, dt.day() as u32)
                .and_then(|d| {
                    d.and_hms_opt(dt.hour() as u32, dt.minute() as u32, dt.second() as u32)
                })
                .map(|ndt| Utc.from_utc_datetime(&ndt).into())
        });
        out.push(ArchiveEntry {
            name,
            size: entry.size(),
            is_dir,
            compressed_size: Some(entry.compressed_size()),
            modified,
        });
    }
    Ok(out)
}

fn list_tar(path: &Path, gzip: bool) -> Result<Vec<ArchiveEntry>> {
    let file = File::open(path).with_context(|| format!("open tar {}", path.display()))?;
    let reader = BufReader::new(file);

    if gzip {
        let decoder = GzDecoder::new(reader);
        list_tar_from_reader(decoder)
    } else {
        list_tar_from_reader(reader)
    }
}

fn list_tar_from_reader<R: Read>(reader: R) -> Result<Vec<ArchiveEntry>> {
    let mut archive = tar::Archive::new(reader);
    // Prefer not following GNU sparse / pax fully; headers only.
    archive.set_overwrite(false);

    let mut out = Vec::new();
    for entry in archive.entries().context("read tar entries")? {
        if out.len() >= MAX_ARCHIVE_ENTRIES {
            bail!(
                "archive exceeds {} entries; refusing to list further",
                MAX_ARCHIVE_ENTRIES
            );
        }
        let entry = entry.context("tar entry")?;
        let header = entry.header();
        let path = entry.path().context("tar entry path")?;
        let name = normalize_name(&path.to_string_lossy());
        let is_dir = header.entry_type().is_dir() || name.ends_with('/');
        let size = header.size().unwrap_or(0);
        let modified = header
            .mtime()
            .ok()
            .map(|secs| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs));
        // Advancing to the next entry skips the payload without writing it out.
        out.push(ArchiveEntry {
            name,
            size,
            is_dir,
            compressed_size: None,
            modified,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!(
            "f00-archive-{}-{}-{}",
            std::process::id(),
            nanos,
            name
        ))
    }

    #[test]
    fn is_archive_by_extension() {
        assert!(is_archive(Path::new("foo.zip")));
        assert!(is_archive(Path::new("FOO.ZIP")));
        assert!(is_archive(Path::new("a.tar")));
        assert!(is_archive(Path::new("a.tar.gz")));
        assert!(is_archive(Path::new("a.tgz")));
        assert!(!is_archive(Path::new("a.txt")));
        assert!(!is_archive(Path::new("a.tar.bak")));
    }

    #[test]
    fn list_zip_temp() {
        let path = temp_path("sample.zip");
        {
            let file = File::create(&path).unwrap();
            let mut zip = zip::ZipWriter::new(file);
            let opts = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            zip.start_file("hello.txt", opts).unwrap();
            zip.write_all(b"hello world").unwrap();
            zip.start_file("dir/nested.rs", opts).unwrap();
            zip.write_all(b"fn main() {}").unwrap();
            // Directory marker
            zip.add_directory("dir/", opts).unwrap();
            zip.finish().unwrap();
        }

        let entries = list_archive(&path).unwrap();
        assert!(entries.len() >= 2);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"hello.txt"));
        assert!(names.iter().any(|n| n.contains("nested.rs")));

        let hello = entries.iter().find(|e| e.name == "hello.txt").unwrap();
        assert_eq!(hello.size, 11);
        assert!(!hello.is_dir);
        assert!(hello.compressed_size.is_some());

        let listing = list_archive_as_listing(&path).unwrap();
        assert_eq!(listing.entries.len(), entries.len());
        assert!(listing.entries.iter().any(|e| e.name == "hello.txt"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn list_tar_gz_temp() {
        let path = temp_path("sample.tar.gz");
        {
            let file = File::create(&path).unwrap();
            let enc = flate2::write::GzEncoder::new(file, flate2::Compression::fast());
            let mut builder = tar::Builder::new(enc);
            let mut header = tar::Header::new_gnu();
            let data = b"payload";
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder
                .append_data(&mut header, "payload.bin", &data[..])
                .unwrap();
            let enc = builder.into_inner().unwrap();
            enc.finish().unwrap();
        }

        let entries = list_archive(&path).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "payload.bin");
        assert_eq!(entries[0].size, 7);
        assert!(!entries[0].is_dir);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn list_plain_tar_temp() {
        let path = temp_path("sample.tar");
        {
            let file = File::create(&path).unwrap();
            let mut builder = tar::Builder::new(file);
            let mut header = tar::Header::new_gnu();
            let data = b"x";
            header.set_size(1);
            header.set_mode(0o644);
            header.set_cksum();
            builder
                .append_data(&mut header, "only.txt", &data[..])
                .unwrap();
            builder.finish().unwrap();
        }

        let entries = list_archive(&path).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "only.txt");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn unsupported_extension_errors() {
        let path = temp_path("nope.txt");
        std::fs::write(&path, b"x").unwrap();
        assert!(list_archive(&path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn to_entry_maps_fields() {
        let ae = ArchiveEntry {
            name: "sub/file.txt".into(),
            size: 42,
            is_dir: false,
            compressed_size: Some(10),
            modified: None,
        };
        let e = ae.to_entry(Path::new("/tmp/a.zip"), 0);
        assert_eq!(e.name, "sub/file.txt");
        assert_eq!(e.size, 42);
        assert_eq!(e.kind, EntryKind::File);
    }
}
