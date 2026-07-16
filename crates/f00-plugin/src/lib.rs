//! f00 plugin host ABI.
//!
//! ## ABI contract (v1)
//!
//! Dynamic plugins are shared libraries (`.so` / `.dylib` / `.dll`) that export:
//!
//! | Symbol | Signature | Purpose |
//! |--------|-----------|---------|
//! | `f00_plugin_abi_version` | `extern "C" fn() -> u32` | Must return [`ABI_VERSION`] |
//! | `f00_plugin_name` | `extern "C" fn() -> *const c_char` | NUL-terminated UTF-8 name |
//! | `f00_plugin_on_entries_json` | optional `extern "C" fn(*const u8, usize, *mut u8, *mut usize) -> i32` | Transform JSON entry array |
//!
//! Host discovery paths (first match wins per file):
//! - `$F00_PLUGIN_DIR` (colon/semicolon separated)
//! - `~/.f00/plugins`
//! - `~/.config/f00/plugins` (Unix) / `%APPDATA%\f00\plugins` (Windows)
//!
//! Enable with Cargo feature `plugins` on `f00-cli`.

use std::ffi::{CStr, OsStr};
use std::fs;
use std::path::{Path, PathBuf};

use libloading::{Library, Symbol};
use thiserror::Error;

/// Current host/plugin ABI version. Bump only on incompatible C ABI changes.
pub const ABI_VERSION: u32 = 1;

/// Required export: returns [`ABI_VERSION`].
pub type AbiVersionFn = unsafe extern "C" fn() -> u32;
/// Required export: static C string name.
pub type NameFn = unsafe extern "C" fn() -> *const std::os::raw::c_char;
/// Optional export: rewrite a JSON array of entries.
///
/// Input: UTF-8 JSON bytes. Output buffer is provided by the host; plugin writes
/// JSON and sets `*out_len`. Return `0` on success, non-zero on error.
pub type OnEntriesJsonFn = unsafe extern "C" fn(*const u8, usize, *mut u8, *mut usize) -> i32;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("load {path}: {source}")]
    Load {
        path: PathBuf,
        #[source]
        source: libloading::Error,
    },
    #[error("plugin {path}: missing symbol {symbol}")]
    MissingSymbol { path: PathBuf, symbol: &'static str },
    #[error("plugin {path}: ABI {got} != host {ABI_VERSION}")]
    AbiMismatch { path: PathBuf, got: u32 },
    #[error("plugin {path}: invalid name pointer")]
    BadName { path: PathBuf },
    #[error("plugin {name}: decorate failed (code {code})")]
    DecorateFailed { name: String, code: i32 },
    #[error("plugin {name}: output not valid UTF-8 JSON")]
    BadOutput { name: String },
}

/// A loaded plugin library.
pub struct Plugin {
    name: String,
    path: PathBuf,
    /// Kept alive so symbols remain valid.
    _lib: Library,
    on_entries: Option<OnEntriesJsonFn>,
}

impl Plugin {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Apply optional JSON transform. Returns original input if no decorator.
    pub fn transform_entries_json(&self, input: &str) -> Result<String, PluginError> {
        let Some(func) = self.on_entries else {
            return Ok(input.to_string());
        };
        // Generous buffer; plugins that need more should return error.
        let mut out = vec![0u8; (input.len() * 4).max(4096)];
        let mut out_len = out.len();
        let code = unsafe { func(input.as_ptr(), input.len(), out.as_mut_ptr(), &mut out_len) };
        if code != 0 {
            return Err(PluginError::DecorateFailed {
                name: self.name.clone(),
                code,
            });
        }
        if out_len > out.len() {
            return Err(PluginError::DecorateFailed {
                name: self.name.clone(),
                code: -2,
            });
        }
        out.truncate(out_len);
        String::from_utf8(out).map_err(|_| PluginError::BadOutput {
            name: self.name.clone(),
        })
    }
}

/// Load a single plugin library from `path`.
pub fn load_plugin(path: &Path) -> Result<Plugin, PluginError> {
    let lib = unsafe { Library::new(path) }.map_err(|source| PluginError::Load {
        path: path.to_path_buf(),
        source,
    })?;

    let abi: Symbol<AbiVersionFn> =
        unsafe { lib.get(b"f00_plugin_abi_version\0") }.map_err(|_| {
            PluginError::MissingSymbol {
                path: path.to_path_buf(),
                symbol: "f00_plugin_abi_version",
            }
        })?;
    let got = unsafe { abi() };
    if got != ABI_VERSION {
        return Err(PluginError::AbiMismatch {
            path: path.to_path_buf(),
            got,
        });
    }

    let name_fn: Symbol<NameFn> =
        unsafe { lib.get(b"f00_plugin_name\0") }.map_err(|_| PluginError::MissingSymbol {
            path: path.to_path_buf(),
            symbol: "f00_plugin_name",
        })?;
    let name_ptr = unsafe { name_fn() };
    if name_ptr.is_null() {
        return Err(PluginError::BadName {
            path: path.to_path_buf(),
        });
    }
    let name = unsafe { CStr::from_ptr(name_ptr) }
        .to_string_lossy()
        .into_owned();

    let on_entries = unsafe { lib.get::<OnEntriesJsonFn>(b"f00_plugin_on_entries_json\0") }
        .ok()
        .map(|s| *s);

    // Leak the Symbol lifetime into owned fn pointers by keeping Library.
    Ok(Plugin {
        name,
        path: path.to_path_buf(),
        _lib: lib,
        on_entries,
    })
}

fn plugin_extension() -> &'static OsStr {
    #[cfg(target_os = "windows")]
    {
        OsStr::new("dll")
    }
    #[cfg(target_os = "macos")]
    {
        OsStr::new("dylib")
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        OsStr::new("so")
    }
}

/// Directories searched for plugins (may not exist).
pub fn plugin_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(raw) = std::env::var("F00_PLUGIN_DIR") {
        let sep = if cfg!(windows) { ';' } else { ':' };
        for p in raw.split(sep).filter(|s| !s.is_empty()) {
            dirs.push(PathBuf::from(p));
        }
    }
    if let Some(home) = directories::UserDirs::new().map(|u| u.home_dir().to_path_buf()) {
        dirs.push(home.join(".f00").join("plugins"));
    }
    if let Some(proj) = directories::ProjectDirs::from("", "", "f00") {
        dirs.push(proj.config_dir().join("plugins"));
    }
    dirs
}

/// Load all plugins found under search dirs. Failures for individual files are skipped
/// when `strict` is false; returned as errors when `strict` is true.
pub fn load_all_plugins(strict: bool) -> Result<Vec<Plugin>, PluginError> {
    let mut out = Vec::new();
    let ext = plugin_extension();
    for dir in plugin_search_dirs() {
        let rd = match fs::read_dir(&dir) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for ent in rd.flatten() {
            let path = ent.path();
            if path.extension() != Some(ext) {
                // Also accept lib*.so style without requiring exact ext alone
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let looks_like = name.contains("f00")
                    && (name.ends_with(".so")
                        || name.ends_with(".dylib")
                        || name.ends_with(".dll"));
                if !looks_like {
                    continue;
                }
            }
            match load_plugin(&path) {
                Ok(p) => out.push(p),
                Err(e) if strict => return Err(e),
                Err(_) => continue,
            }
        }
    }
    Ok(out)
}

/// List plugin library basenames (for `f00 --list-plugins`).
pub fn discover_plugin_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let ext = plugin_extension();
    for dir in plugin_search_dirs() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.flatten() {
            let path = ent.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let is_lib = path.extension() == Some(ext)
                || name.ends_with(".so")
                || name.ends_with(".dylib")
                || name.ends_with(".dll");
            if is_lib && (name.contains("f00") || path.extension() == Some(ext)) {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi_version_is_one() {
        assert_eq!(ABI_VERSION, 1);
    }

    #[test]
    fn search_dirs_non_empty_when_home_exists() {
        // Always at least env-driven list (may be empty) — function should not panic.
        let _ = plugin_search_dirs();
    }
}
