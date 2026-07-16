//! Plugin listing and decorate hooks for the CLI (feature `plugins`).

use std::path::Path;

use anyhow::{Context, Result};
use f00_core::{Entry, EntryKind, Listing};
use serde::{Deserialize, Serialize};

/// Print discovered plugins and exit.
pub fn list_plugins() -> Result<()> {
    let paths = f00_plugin::discover_plugin_paths();
    if paths.is_empty() {
        println!("f00: no plugins found");
        println!("Search paths:");
        for d in f00_plugin::plugin_search_dirs() {
            println!("  {}", d.display());
        }
        println!("ABI version: {}", f00_plugin::ABI_VERSION);
        return Ok(());
    }
    for path in paths {
        match f00_plugin::load_plugin(&path) {
            Ok(p) => println!("{}\t{}", p.name(), path.display()),
            Err(e) => eprintln!("f00: skip {}: {e}", path.display()),
        }
    }
    Ok(())
}

/// Load plugins quietly (missing dirs / bad libs skipped).
pub fn load_plugins_quiet() -> Vec<f00_plugin::Plugin> {
    f00_plugin::load_all_plugins(false).unwrap_or_default()
}

/// Smoke-load a specific path (tests / diagnostics).
pub fn load_one(path: &Path) -> Result<String> {
    let p = f00_plugin::load_plugin(path).with_context(|| format!("load {}", path.display()))?;
    Ok(p.name().to_string())
}

/// Wire format for plugin decorate ABI (stable, versioned by host ABI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntryDto {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub size: u64,
    pub depth: usize,
    pub is_dir_header: bool,
    /// Optional display override; when set, host applies it to [`Entry::name`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

fn entry_to_dto(e: &Entry) -> PluginEntryDto {
    PluginEntryDto {
        name: e.name.clone(),
        path: e.path.display().to_string(),
        kind: e.kind.as_str().to_string(),
        size: e.size,
        depth: e.depth,
        is_dir_header: e.is_dir_header,
        display_name: None,
    }
}

fn entries_to_json(entries: &[Entry]) -> Result<String> {
    let dtos: Vec<PluginEntryDto> = entries.iter().map(entry_to_dto).collect();
    Ok(serde_json::to_string(&dtos)?)
}

/// Apply plugin JSON back onto entries (matched by `path`, then by index).
fn apply_json_to_entries(entries: &mut [Entry], json: &str) -> Result<()> {
    let dtos: Vec<PluginEntryDto> = serde_json::from_str(json).context("plugin JSON parse")?;
    for (i, dto) in dtos.into_iter().enumerate() {
        let idx = entries
            .iter()
            .position(|e| e.path.display().to_string() == dto.path)
            .unwrap_or(i);
        let Some(entry) = entries.get_mut(idx) else {
            continue;
        };
        // Prefer explicit display_name; otherwise accept renamed `name`.
        if let Some(dn) = dto.display_name {
            entry.name = dn;
        } else if !dto.name.is_empty() && dto.name != entry.name {
            entry.name = dto.name;
        }
        // Allow kind string updates only when valid.
        if let Some(k) = parse_kind(&dto.kind) {
            entry.kind = k;
        }
        entry.size = dto.size;
        entry.depth = dto.depth;
        entry.is_dir_header = dto.is_dir_header;
    }
    Ok(())
}

fn parse_kind(s: &str) -> Option<EntryKind> {
    match s {
        "file" => Some(EntryKind::File),
        "directory" => Some(EntryKind::Directory),
        "symlink" => Some(EntryKind::Symlink),
        "other" => Some(EntryKind::Other),
        _ => None,
    }
}

/// Run all loaded plugins' entry transforms on each listing.
///
/// Failures from individual plugins are reported on stderr and skipped so a bad
/// plugin cannot brick listing.
pub fn decorate_listings(mut listings: Vec<Listing>) -> Vec<Listing> {
    let plugins = load_plugins_quiet();
    if plugins.is_empty() {
        return listings;
    }
    for listing in &mut listings {
        let Ok(mut json) = entries_to_json(&listing.entries) else {
            continue;
        };
        for plugin in &plugins {
            match plugin.transform_entries_json(&json) {
                Ok(next) => json = next,
                Err(e) => {
                    eprintln!("f00: plugin {}: {e}", plugin.name());
                }
            }
        }
        if let Err(e) = apply_json_to_entries(&mut listing.entries, &json) {
            eprintln!("f00: plugin apply: {e:#}");
        }
    }
    listings
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::GitStatus;
    use std::path::PathBuf;

    fn sample_entry(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(format!("/tmp/{name}")),
            name: name.to_string(),
            kind: EntryKind::File,
            size: 1,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: 0o644,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: 0,
            owner: String::new(),
            group: String::new(),
            author: String::new(),
            context: String::new(),
        }
    }

    #[test]
    fn apply_json_renames_by_path() {
        let mut entries = vec![sample_entry("a.txt"), sample_entry("b.txt")];
        let json = r#"[
            {"name":"a.txt","path":"/tmp/a.txt","kind":"file","size":1,"depth":0,"is_dir_header":false,"display_name":"★ a.txt"},
            {"name":"b.txt","path":"/tmp/b.txt","kind":"file","size":9,"depth":0,"is_dir_header":false}
        ]"#;
        apply_json_to_entries(&mut entries, json).unwrap();
        assert_eq!(entries[0].name, "★ a.txt");
        assert_eq!(entries[1].size, 9);
    }

    #[test]
    fn roundtrip_json_preserves_names() {
        let entries = vec![sample_entry("x")];
        let json = entries_to_json(&entries).unwrap();
        let mut again = entries.clone();
        apply_json_to_entries(&mut again, &json).unwrap();
        assert_eq!(again[0].name, "x");
    }
}
