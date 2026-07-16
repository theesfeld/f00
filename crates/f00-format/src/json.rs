use f00_core::Entry;
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

/// Serialize entries (skipping directory headers) as pretty JSON.
pub fn format_json(entries: &[Entry]) -> Result<String, serde_json::Error> {
    let items: Vec<JsonEntry<'_>> = entries
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
        .collect();

    serde_json::to_string_pretty(&items)
}
