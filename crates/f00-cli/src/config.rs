//! User TOML configuration for f00.
//!
//! Search order (first found wins):
//! 1. `--config PATH` if provided
//! 2. `$F00_CONFIG` if set
//! 3. Platform config dir:
//!    - Unix: `$XDG_CONFIG_HOME/f00/config.toml` or `~/.config/f00/config.toml`
//!      (macOS via `directories`: `~/Library/Application Support/f00/config.toml`)
//!    - Windows: `%APPDATA%\f00\config.toml`

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::cli::{Args, ColorArg};

/// Optional defaults from a TOML config file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ConfigDefaults {
    pub all: Option<bool>,
    pub almost_all: Option<bool>,
    pub long: Option<bool>,
    #[serde(alias = "human")]
    pub human_readable: Option<bool>,
    pub color: Option<String>,
    pub icons: Option<bool>,
    pub dirs_first: Option<bool>,
    pub git: Option<bool>,
    pub classify: Option<bool>,
}

/// Root of `config.toml`. Fields may live under `[defaults]` or at the root.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct FileConfig {
    #[serde(default)]
    pub defaults: ConfigDefaults,
    // Root-level keys (take precedence over nested `[defaults]` when present).
    pub all: Option<bool>,
    pub almost_all: Option<bool>,
    pub long: Option<bool>,
    #[serde(alias = "human")]
    pub human_readable: Option<bool>,
    pub color: Option<String>,
    pub icons: Option<bool>,
    pub dirs_first: Option<bool>,
    pub git: Option<bool>,
    pub classify: Option<bool>,
}

impl FileConfig {
    /// Flatten root-level keys on top of `[defaults]`.
    pub fn resolved_defaults(&self) -> ConfigDefaults {
        ConfigDefaults {
            all: self.all.or(self.defaults.all),
            almost_all: self.almost_all.or(self.defaults.almost_all),
            long: self.long.or(self.defaults.long),
            human_readable: self.human_readable.or(self.defaults.human_readable),
            color: self.color.clone().or_else(|| self.defaults.color.clone()),
            icons: self.icons.or(self.defaults.icons),
            dirs_first: self.dirs_first.or(self.defaults.dirs_first),
            git: self.git.or(self.defaults.git),
            classify: self.classify.or(self.defaults.classify),
        }
    }
}

/// Parse TOML text into a [`FileConfig`].
pub fn parse_config_str(s: &str) -> Result<FileConfig, toml::de::Error> {
    toml::from_str(s)
}

/// Load config from an explicit path.
pub fn load_config_from_path(path: &Path) -> anyhow::Result<FileConfig> {
    let text = fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("failed to read config {}: {e}", path.display()))?;
    parse_config_str(&text)
        .map_err(|e| anyhow::anyhow!("failed to parse config {}: {e}", path.display()))
}

/// Resolve the platform user config path (`…/f00/config.toml`), if available.
pub fn platform_config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "f00").map(|d| d.config_dir().join("config.toml"))
}

/// Paths to try when no `--config` override is given.
pub fn config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(p) = std::env::var("F00_CONFIG") {
        if !p.is_empty() {
            paths.push(PathBuf::from(p));
        }
    }
    if let Some(p) = platform_config_path() {
        paths.push(p);
    }
    paths
}

/// Load config: explicit path, else first existing search path, else `None`.
pub fn load_user_config(explicit: Option<&Path>) -> anyhow::Result<Option<FileConfig>> {
    if let Some(path) = explicit {
        return Ok(Some(load_config_from_path(path)?));
    }
    for path in config_search_paths() {
        if path.is_file() {
            return Ok(Some(load_config_from_path(&path)?));
        }
    }
    Ok(None)
}

fn parse_color_arg(s: &str) -> Option<ColorArg> {
    match s.to_ascii_lowercase().as_str() {
        "auto" => Some(ColorArg::Auto),
        "always" | "yes" | "force" | "true" | "on" => Some(ColorArg::Always),
        "never" | "no" | "none" | "false" | "off" => Some(ColorArg::Never),
        _ => None,
    }
}

/// Whether `flag` (e.g. `--icons` or `--git`) appears in process args.
pub fn cli_has_long(flag: &str) -> bool {
    let eq = format!("{flag}=");
    std::env::args().any(|a| a == flag || a.starts_with(&eq))
}

/// Merge file defaults into CLI args.
///
/// Precedence: built-in defaults < config < CLI flags (when set / non-default).
/// For false-default bools, config `true` enables.
/// For `git` (CLI default true), config applies unless `--git` was on the command line.
/// For `color`, config applies when no explicit `--color` on the command line.
pub fn merge_config_into_args(args: &mut Args, file: &FileConfig) {
    let d = file.resolved_defaults();

    if let Some(true) = d.all {
        args.all = true;
    }
    if let Some(true) = d.almost_all {
        args.almost_all = true;
    }
    if let Some(true) = d.long {
        args.long = true;
    }
    if let Some(true) = d.human_readable {
        args.human_readable = true;
    }
    if let Some(true) = d.classify {
        args.classify = true;
    }

    if let Some(v) = d.icons {
        if v {
            args.icons = true;
        } else if !cli_has_long("--icons") {
            args.icons = false;
        }
    }
    if let Some(v) = d.dirs_first {
        if v {
            args.dirs_first = true;
        } else if !cli_has_long("--dirs-first") {
            args.dirs_first = false;
        }
    }

    if let Some(v) = d.git {
        if !cli_has_long("--git") {
            args.git = v;
        }
    }

    if let Some(ref c) = d.color {
        if !cli_has_long("--color") {
            if let Some(parsed) = parse_color_arg(c) {
                args.color = parsed;
            }
        }
    }
}

/// Apply env overrides (`F00_GNU`).
pub fn apply_env_overrides(args: &mut Args) {
    if args.gnu {
        return;
    }
    if let Ok(v) = std::env::var("F00_GNU") {
        let v = v.to_ascii_lowercase();
        if matches!(v.as_str(), "1" | "true" | "yes" | "on") {
            args.gnu = true;
        }
    }
}

/// Detect whether the binary was invoked as `ls` / `ls.exe`.
pub fn invoked_as_ls() -> bool {
    invoked_as_ls_from(std::env::args_os().next())
}

/// Testable argv0 check.
pub fn invoked_as_ls_from(argv0: Option<std::ffi::OsString>) -> bool {
    let Some(argv0) = argv0 else {
        return false;
    };
    // Prefer Path (correct separators for the host OS).
    if let Some(stem) = Path::new(&argv0).file_stem().and_then(|s| s.to_str()) {
        if stem.eq_ignore_ascii_case("ls") {
            return true;
        }
    }
    // Also handle Windows-style paths when parsing on Unix (and vice versa).
    if let Some(s) = argv0.to_str() {
        let name = s.rsplit(['/', '\\']).next().unwrap_or(s);
        let stem = name
            .strip_suffix(".exe")
            .or_else(|| name.strip_suffix(".EXE"))
            .unwrap_or(name);
        return stem.eq_ignore_ascii_case("ls");
    }
    false
}

/// Resolve effective args after clap parse.
///
/// Order: argv0 soft defaults → config file → env (`F00_GNU`).
/// CLI flags already present in `args` win over config where merge logic allows.
pub fn resolve_args(args: &mut Args, file: Option<&FileConfig>, as_ls: bool) {
    if as_ls {
        // Icons / dirs_first off unless the user passed the flags (config may re-enable).
        f00_compat::prefer_ls_defaults(
            &mut args.icons,
            &mut args.dirs_first,
            cli_has_long("--icons"),
            cli_has_long("--dirs-first"),
        );
    }
    if let Some(file) = file {
        merge_config_into_args(args, file);
    }
    apply_env_overrides(args);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::ColorArg;

    fn empty_args() -> Args {
        Args {
            paths: vec![],
            help: None,
            all: false,
            almost_all: false,
            long: false,
            one_per_line: false,
            human_readable: false,
            recursive: false,
            reverse: false,
            sort_time: false,
            sort_size: false,
            sort_extension: false,
            color: ColorArg::Auto,
            json: false,
            tree: false,
            gnu: false,
            icons: false,
            classify: false,
            dirs_first: false,
            max_depth: None,
            git: true,
            config: None,
        }
    }

    #[test]
    fn parse_defaults_section() {
        let cfg = parse_config_str(
            r#"
            [defaults]
            all = true
            icons = true
            color = "never"
            dirs_first = true
            git = false
            "#,
        )
        .unwrap();
        let d = cfg.resolved_defaults();
        assert_eq!(d.all, Some(true));
        assert_eq!(d.icons, Some(true));
        assert_eq!(d.color.as_deref(), Some("never"));
        assert_eq!(d.dirs_first, Some(true));
        assert_eq!(d.git, Some(false));
    }

    #[test]
    fn parse_root_level_and_human_alias() {
        let cfg = parse_config_str(
            r#"
            long = true
            human = true
            classify = true
            "#,
        )
        .unwrap();
        let d = cfg.resolved_defaults();
        assert_eq!(d.long, Some(true));
        assert_eq!(d.human_readable, Some(true));
        assert_eq!(d.classify, Some(true));
    }

    #[test]
    fn merge_enables_flags_from_config() {
        let cfg = parse_config_str(
            r#"
            [defaults]
            all = true
            long = true
            icons = true
            human_readable = true
            "#,
        )
        .unwrap();
        let mut args = empty_args();
        merge_config_into_args(&mut args, &cfg);
        assert!(args.all);
        assert!(args.long);
        assert!(args.icons);
        assert!(args.human_readable);
    }

    #[test]
    fn merge_color_from_config() {
        let cfg = parse_config_str(r#"color = "never""#).unwrap();
        let mut args = empty_args();
        merge_config_into_args(&mut args, &cfg);
        assert!(matches!(args.color, ColorArg::Never));
    }

    #[test]
    fn merge_git_false_from_config() {
        let cfg = parse_config_str(r#"git = false"#).unwrap();
        let mut args = empty_args();
        assert!(args.git);
        merge_config_into_args(&mut args, &cfg);
        // Without --git on the real CLI of this test process, config applies.
        if !cli_has_long("--git") {
            assert!(!args.git);
        }
    }

    #[test]
    fn argv0_ls_detection() {
        assert!(invoked_as_ls_from(Some("ls".into())));
        assert!(invoked_as_ls_from(Some("/usr/bin/ls".into())));
        assert!(invoked_as_ls_from(Some(r"C:\bin\ls.exe".into())));
        assert!(invoked_as_ls_from(Some("LS".into())));
        assert!(!invoked_as_ls_from(Some("f00".into())));
        assert!(!invoked_as_ls_from(Some("/usr/local/bin/f00".into())));
        assert!(!invoked_as_ls_from(None));
    }

    #[test]
    fn platform_path_ends_with_config_toml() {
        if let Some(p) = platform_config_path() {
            assert_eq!(p.file_name().and_then(|s| s.to_str()), Some("config.toml"));
            assert!(p
                .components()
                .any(|c| c.as_os_str() == "f00" || c.as_os_str() == ".config"));
        }
    }

    #[test]
    fn resolve_ls_then_config_can_enable_icons() {
        let cfg = parse_config_str(r#"icons = true"#).unwrap();
        let mut args = empty_args();
        resolve_args(&mut args, Some(&cfg), true);
        assert!(args.icons, "config should re-enable icons under argv0 ls");
    }
}
