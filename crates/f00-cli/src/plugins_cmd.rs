//! Plugin listing / load helpers for the CLI (feature `plugins`).

use anyhow::{Context, Result};

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

/// Load plugins if feature enabled; used for future decorate hooks.
#[allow(dead_code)]
pub fn load_plugins_quiet() -> Vec<f00_plugin::Plugin> {
    f00_plugin::load_all_plugins(false).unwrap_or_default()
}

/// Smoke-load a specific path (tests / diagnostics).
pub fn load_one(path: &std::path::Path) -> Result<String> {
    let p = f00_plugin::load_plugin(path).with_context(|| format!("load {}", path.display()))?;
    Ok(p.name().to_string())
}
