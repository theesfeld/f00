//! JSON listing format — a **core** f00 surface (rich metadata for scripts and humans).
//!
//! - Always valid JSON parseable by `jq` / `serde_json`
//! - **Pretty** + **syntax-colored** when color mode is on (TTY / `--color=always`)
//! - Compact single-line when color is off (pipes, scripts, `--color=never`)

use f00_core::Entry;
use nu_ansi_term::{Color, Style};
use serde::Serialize;

use crate::perms::format_permissions;

#[derive(Serialize)]
struct JsonEntry<'a> {
    name: &'a str,
    path: String,
    /// Absolute path when canonicalization succeeds; otherwise omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_path: Option<String>,
    kind: &'static str,
    size: u64,
    /// Permission bits as octal string (e.g. `"644"`), matching prior schema.
    mode: String,
    /// Same as `mode` — explicit name for machine consumers.
    mode_octal: String,
    /// `ls -l` style permission string (e.g. `"-rw-r--r--"`).
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
    /// Allocated blocks in 512-byte units (GNU `ls -s` style) when known.
    blocks: u64,
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
}

fn rfc3339(dt: Option<chrono::DateTime<chrono::Local>>) -> Option<String> {
    dt.map(|d| d.to_rfc3339())
}

fn build_items(entries: &[Entry]) -> Vec<JsonEntry<'_>> {
    entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            let mode = format!("{:o}", e.mode);
            let absolute_path = std::fs::canonicalize(&e.path)
                .ok()
                .map(|p| p.display().to_string());
            JsonEntry {
                name: &e.name,
                path: e.path.display().to_string(),
                absolute_path,
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
                author: e.author.as_str(),
                symlink_target: e.symlink_target.as_ref().map(|p| p.display().to_string()),
                context: e.context.as_str(),
                extension: e.extension(),
                git_status: e.git_status.as_str(),
                depth: e.depth,
            }
        })
        .collect()
}

/// Serialize entries (skipping directory headers) as JSON.
///
/// When `color` is true: indented (pretty) JSON with ANSI syntax highlighting.
/// When `color` is false: compact JSON with no ANSI (script / pipe friendly).
pub fn format_json(entries: &[Entry], color: bool) -> Result<String, serde_json::Error> {
    let items = build_items(entries);
    if color {
        let pretty = serde_json::to_string_pretty(&items)?;
        Ok(colorize_json(&pretty))
    } else {
        serde_json::to_string(&items)
    }
}

/// Back-compat helper: pretty JSON without color (tests / callers that ignore theme).
pub fn format_json_pretty(entries: &[Entry]) -> Result<String, serde_json::Error> {
    let items = build_items(entries);
    serde_json::to_string_pretty(&items)
}

/// Apply lightweight ANSI syntax highlighting to pretty-printed JSON text.
///
/// Does not re-parse as a tree — walks the pretty output so structure stays intact.
/// Output remains human-oriented; strip ANSI for machines (`--color=never` path avoids this).
pub fn colorize_json(src: &str) -> String {
    let key = Style::new().fg(Color::Cyan);
    let string = Style::new().fg(Color::Green);
    let number = Style::new().fg(Color::Yellow);
    let literal = Style::new().fg(Color::Purple).bold();
    let punct = Style::new().fg(Color::DarkGray);

    let mut out = String::with_capacity(src.len().saturating_mul(2));
    let bytes = src.as_bytes();
    let mut i = 0;
    // After a string that was a key (followed by ':'), don't recolor the same way —
    // we detect keys as: string immediately followed by optional space and ':'.
    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' => {
                // Scan string (with escapes).
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
                // Peek for key: whitespace then ':'
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
                if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
                    i += 1;
                    if i < bytes.len() && matches!(bytes[i], b'+' | b'-') {
                        i += 1;
                    }
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
                // whitespace and anything else
                out.push(src[i..].chars().next().unwrap_or('\0'));
                i += src[i..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
            }
        }
    }
    out
}

/// Strip ANSI CSI sequences (for tests / tooling).
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
    use f00_core::{Entry, EntryKind};
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn sample_entry(name: &str) -> Entry {
        Entry {
            name: name.into(),
            path: PathBuf::from(name),
            kind: EntryKind::File,
            size: 42,
            mode: 0o644,
            modified: Some(SystemTime::UNIX_EPOCH),
            accessed: None,
            changed: None,
            created: None,
            inode: 1,
            nlink: 1,
            blocks: 8,
            uid: 1000,
            gid: 1000,
            owner: "u".into(),
            group: "g".into(),
            author: String::new(),
            symlink_target: None,
            context: String::new(),
            git_status: f00_core::GitStatus::Clean,
            depth: 0,
            is_dir_header: false,
            readonly: false,
        }
    }

    #[test]
    fn compact_when_no_color_is_parseable() {
        let entries = vec![sample_entry("a.txt")];
        let s = format_json(&entries, false).unwrap();
        assert!(!s.contains('\n') || s.lines().count() == 1, "compact: {s}");
        assert!(!s.contains('\u{1b}'), "no ANSI: {s:?}");
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(v.is_array());
        assert_eq!(v[0]["name"], "a.txt");
        assert_eq!(v[0]["size"], 42);
    }

    #[test]
    fn color_mode_is_pretty_and_highlighted() {
        let entries = vec![sample_entry("b.rs")];
        let s = format_json(&entries, true).unwrap();
        assert!(s.contains('\n'), "pretty should multiline");
        assert!(s.contains('\u{1b}'), "should contain ANSI when colored");
        let plain = strip_ansi(&s);
        let v: serde_json::Value = serde_json::from_str(&plain).unwrap();
        assert_eq!(v[0]["name"], "b.rs");
    }

    #[test]
    fn colorize_preserves_structure() {
        let raw = "[\n  {\n    \"name\": \"x\",\n    \"size\": 1,\n    \"ok\": true\n  }\n]";
        let colored = colorize_json(raw);
        let plain = strip_ansi(&colored);
        assert_eq!(
            plain
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
            raw.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        );
    }
}
