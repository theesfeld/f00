//! JSON listing format — a **core** f00 surface.
//!
//! - Always valid JSON parseable by `jq` / `serde_json`
//! - **Compact** schema by default (`-j` / `--json`) for scripts
//! - **Full** schema via `--json-full` (every practical metadata field)
//! - Pretty + light structural emphasis when color mode is on
//! - Compact single-line when color is off (pipes, scripts, `--color=never`)

use std::time::{SystemTime, UNIX_EPOCH};

use f00_core::Entry;
use nu_ansi_term::Style;
use serde::Serialize;

use crate::perms::format_permissions;

/// Compact JSON object (default `-j`).
#[derive(Serialize)]
struct JsonEntryCompact<'a> {
    name: &'a str,
    path: String,
    kind: &'static str,
    size: u64,
    mode: String,
    mode_octal: String,
    permissions: String,
    readonly: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<String>,
    inode: u64,
    nlink: u64,
    blocks: u64,
    uid: u32,
    gid: u32,
    #[serde(skip_serializing_if = "str::is_empty")]
    owner: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    group: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<&'a str>,
    git_status: &'static str,
    depth: usize,
}

/// Full JSON object (`--json-full`) — leave nothing practical out.
#[derive(Serialize)]
struct JsonEntryFull<'a> {
    name: &'a str,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_path: Option<String>,
    kind: &'static str,
    /// Fine type: file, directory, symlink, fifo, socket, block_device, …
    type_detail: &'static str,
    size: u64,
    /// Full mode octal including type bits when known.
    mode: String,
    mode_octal: String,
    /// Permission nibble only (`mode & 0o7777`).
    mode_perms: String,
    mode_bits: u32,
    permissions: String,
    readonly: bool,
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
    is_executable: bool,
    is_hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified_unix: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessed_unix: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changed_unix: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_unix: Option<f64>,
    inode: u64,
    nlink: u64,
    blocks: u64,
    blksize: u64,
    dev: u64,
    dev_major: u32,
    dev_minor: u32,
    rdev: u64,
    rdev_major: u32,
    rdev_minor: u32,
    uid: u32,
    gid: u32,
    #[serde(skip_serializing_if = "str::is_empty")]
    owner: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    group: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    author: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink_target: Option<String>,
    #[serde(skip_serializing_if = "str::is_empty")]
    context: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<&'a str>,
    git_status: &'static str,
    depth: usize,
    /// Extended attribute names (Linux); empty when none / unavailable.
    xattrs: Vec<String>,
}

fn rfc3339(dt: Option<chrono::DateTime<chrono::Local>>) -> Option<String> {
    dt.map(|d| d.to_rfc3339())
}

fn unix_secs(t: Option<SystemTime>) -> Option<f64> {
    let t = t?;
    let d = t.duration_since(UNIX_EPOCH).ok()?;
    Some(d.as_secs_f64())
}

fn build_compact(entries: &[Entry]) -> Vec<JsonEntryCompact<'_>> {
    entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            let mode = e.mode_perms_octal();
            JsonEntryCompact {
                name: &e.name,
                path: e.path.display().to_string(),
                kind: e.kind.as_str(),
                size: e.size,
                mode: mode.clone(),
                mode_octal: mode,
                permissions: format_permissions(e),
                readonly: e.readonly,
                modified: rfc3339(e.modified_datetime()),
                accessed: rfc3339(e.accessed_datetime()),
                changed: rfc3339(e.changed_datetime()),
                created: rfc3339(e.created_datetime()),
                inode: e.inode,
                nlink: e.nlink,
                blocks: e.blocks,
                uid: e.uid,
                gid: e.gid,
                owner: e.owner.as_str(),
                group: e.group.as_str(),
                symlink_target: e.symlink_target.as_ref().map(|p| p.display().to_string()),
                extension: e.extension(),
                git_status: e.git_status.as_str(),
                depth: e.depth,
            }
        })
        .collect()
}

fn build_full(entries: &[Entry]) -> Vec<JsonEntryFull<'_>> {
    entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            let (dev_major, dev_minor) = e.dev_major_minor();
            let (rdev_major, rdev_minor) = e.rdev_major_minor();
            let absolute_path = std::fs::canonicalize(&e.path)
                .ok()
                .map(|p| p.display().to_string());
            JsonEntryFull {
                name: &e.name,
                path: e.path.display().to_string(),
                absolute_path,
                kind: e.kind.as_str(),
                type_detail: e.type_detail(),
                size: e.size,
                mode: e.mode_full_octal(),
                mode_octal: e.mode_full_octal(),
                mode_perms: e.mode_perms_octal(),
                mode_bits: e.mode,
                permissions: format_permissions(e),
                readonly: e.readonly,
                is_file: e.kind == f00_core::EntryKind::File,
                is_dir: e.is_dir(),
                is_symlink: e.kind == f00_core::EntryKind::Symlink,
                is_executable: e.is_executable(),
                is_hidden: e.is_hidden(),
                modified: rfc3339(e.modified_datetime()),
                accessed: rfc3339(e.accessed_datetime()),
                changed: rfc3339(e.changed_datetime()),
                created: rfc3339(e.created_datetime()),
                modified_unix: unix_secs(e.modified),
                accessed_unix: unix_secs(e.accessed),
                changed_unix: unix_secs(e.changed),
                created_unix: unix_secs(e.created),
                inode: e.inode,
                nlink: e.nlink,
                blocks: e.blocks,
                blksize: e.blksize,
                dev: e.dev,
                dev_major,
                dev_minor,
                rdev: e.rdev,
                rdev_major,
                rdev_minor,
                uid: e.uid,
                gid: e.gid,
                owner: e.owner.as_str(),
                group: e.group.as_str(),
                author: e.author.as_str(),
                symlink_target: e.symlink_target.as_ref().map(|p| p.display().to_string()),
                context: e.context.as_str(),
                extension: e.extension(),
                git_status: e.git_status.as_str(),
                depth: e.depth,
                xattrs: e.xattr_names(),
            }
        })
        .collect()
}

/// Serialize entries as JSON.
///
/// - `full == false`: compact schema (default `-j`)
/// - `full == true`: full metadata (`--json-full`)
/// - `color == true`: pretty + structural emphasis
/// - `color == false`: compact single-line, no ANSI
pub fn format_json(
    entries: &[Entry],
    color: bool,
    full: bool,
) -> Result<String, serde_json::Error> {
    let raw = if full {
        let items = build_full(entries);
        if color {
            serde_json::to_string_pretty(&items)?
        } else {
            serde_json::to_string(&items)?
        }
    } else {
        let items = build_compact(entries);
        if color {
            serde_json::to_string_pretty(&items)?
        } else {
            serde_json::to_string(&items)?
        }
    };
    if color {
        Ok(colorize_json(&raw))
    } else {
        Ok(raw)
    }
}

/// Back-compat: pretty compact JSON without color.
pub fn format_json_pretty(entries: &[Entry]) -> Result<String, serde_json::Error> {
    let items = build_compact(entries);
    serde_json::to_string_pretty(&items)
}

/// Pretty JSON emphasis using the **ANSI palette** (theme-following: Dracula, Monokai, …).
///
/// Keys = cyan, strings = green, numbers = yellow, literals = purple, punct = dark gray.
/// Machines should use `--color=never` (compact path).
pub fn colorize_json(src: &str) -> String {
    use nu_ansi_term::Color;
    let key = Style::new().fg(Color::Cyan).bold();
    let string = Style::new().fg(Color::Green);
    let number = Style::new().fg(Color::Yellow);
    let literal = Style::new().fg(Color::Purple).bold();
    let punct = Style::new().fg(Color::DarkGray);
    let mut out = String::with_capacity(src.len().saturating_mul(2));
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' => {
                let start = i;
                i += 1;
                while i < bytes.len() {
                    match bytes[i] {
                        b'\\' if i + 1 < bytes.len() => i += 2,
                        b'"' => {
                            i += 1;
                            break;
                        }
                        _ => i += 1,
                    }
                }
                let s = &src[start..i];
                let mut j = i;
                while j < bytes.len() && matches!(bytes[j], b' ' | b'\t') {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b':' {
                    out.push_str(&key.paint(s).to_string());
                } else {
                    out.push_str(&string.paint(s).to_string());
                }
            }
            b'-' | b'0'..=b'9' => {
                let start = i;
                if bytes[i] == b'-' {
                    i += 1;
                }
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'.' {
                    i += 1;
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                out.push_str(&number.paint(&src[start..i]).to_string());
            }
            b't' if src[i..].starts_with("true") => {
                out.push_str(&literal.paint("true").to_string());
                i += 4;
            }
            b'f' if src[i..].starts_with("false") => {
                out.push_str(&literal.paint("false").to_string());
                i += 5;
            }
            b'n' if src[i..].starts_with("null") => {
                out.push_str(&literal.paint("null").to_string());
                i += 4;
            }
            b'{' | b'}' | b'[' | b']' | b':' | b',' => {
                out.push_str(&punct.paint(&src[i..i + 1]).to_string());
                i += 1;
            }
            _ => {
                out.push(src[i..].chars().next().unwrap_or('\0'));
                i += src[i..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
            }
        }
    }
    out
}

/// Strip ANSI CSI sequences (for tests / consumers that want plain text).
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
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
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn sample() -> Entry {
        Entry {
            path: PathBuf::from("README.md"),
            name: "README.md".into(),
            kind: EntryKind::File,
            size: 100,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: 0o100644,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 1000,
            gid: 1000,
            inode: 42,
            blocks: 8,
            dev: 0x801,
            rdev: 0,
            blksize: 4096,
            owner: "u".into(),
            group: "g".into(),
            author: "u".into(),
            context: String::new(),
        }
    }

    #[test]
    fn compact_when_no_color_is_parseable() {
        let s = format_json(&[sample()], false, false).unwrap();
        assert!(!s.contains('\n') || s.starts_with('['));
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(v.is_array());
        assert_eq!(v[0]["name"], "README.md");
        assert!(v[0].get("xattrs").is_none());
        assert!(v[0].get("type_detail").is_none());
    }

    #[test]
    fn full_includes_extra_fields() {
        let s = format_json(&[sample()], false, true).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v[0]["type_detail"], "file");
        assert_eq!(v[0]["inode"], 42);
        assert!(v[0].get("xattrs").is_some());
        assert!(v[0].get("blksize").is_some());
        assert!(v[0].get("dev_major").is_some());
        assert_eq!(v[0]["is_executable"], false);
        assert_eq!(v[0]["is_hidden"], false);
    }

    #[test]
    fn color_mode_is_pretty_and_highlighted() {
        let s = format_json(&[sample()], true, false).unwrap();
        assert!(s.contains('\n'));
        assert!(s.contains("README.md"));
        let plain = strip_ansi(&s);
        let v: serde_json::Value = serde_json::from_str(&plain).unwrap();
        assert!(v.is_array());
    }

    #[test]
    fn colorize_preserves_structure() {
        let raw = "[\n  {\n    \"name\": \"a\",\n    \"size\": 1\n  }\n]";
        let colored = colorize_json(raw);
        let plain = strip_ansi(&colored);
        assert_eq!(plain, raw);
    }
}
