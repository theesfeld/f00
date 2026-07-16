use f00_core::Entry;
use serde::Serialize;

#[derive(Serialize)]
struct JsonEntry<'a> {
    name: &'a str,
    path: String,
    kind: &'static str,
    size: u64,
    modified: Option<String>,
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink_target: Option<String>,
    git_status: &'static str,
    depth: usize,
}

/// Serialize entries (skipping directory headers) as pretty JSON.
pub fn format_json(entries: &[Entry]) -> Result<String, serde_json::Error> {
    let items: Vec<JsonEntry<'_>> = entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| JsonEntry {
            name: &e.name,
            path: e.path.display().to_string(),
            kind: e.kind.as_str(),
            size: e.size,
            modified: e.modified_datetime().map(|dt| dt.to_rfc3339()),
            mode: format!("{:o}", e.mode),
            symlink_target: e.symlink_target.as_ref().map(|p| p.display().to_string()),
            git_status: e.git_status.as_str(),
            depth: e.depth,
        })
        .collect();

    serde_json::to_string_pretty(&items)
}
