//! Color engine: terminal theme inheritance.
//!
//! **Policy (v0.12):**
//! - **Names** use only `LS_COLORS` (dircolors / `lscolors`).
//! - **Long-format metadata** (perms, size, user, date, git) use **ANSI palette
//!   indexes** so themes like Dracula/Monokai apply, plus optional
//!   **`F00_COLORS` / `EZA_COLORS` / `EXA_COLORS`** maps (eza-compatible keys).
//! - Never hardcode truecolor RGB for listing chrome.
//! - When color is off, all paint helpers return plain text.

use f00_core::{Entry, EntryKind};
use lscolors::{Indicator, LsColors, Style};
use nu_ansi_term::Color as AnsiColor;
use nu_ansi_term::Style as AnsiStyle;
use std::collections::HashMap;
use std::env;
use std::path::Path;

/// Color engine wrapping `LS_COLORS` + optional eza-style metadata maps.
#[derive(Clone)]
pub struct Colorizer {
    enabled: bool,
    ls: LsColors,
    meta: MetaTheme,
}

/// Long-listing / git roles. Values are ANSI SGR sequences (eza style) or defaults.
#[derive(Clone, Default)]
struct MetaTheme {
    /// Parsed `key=ansi` overrides (from F00_COLORS / EZA_COLORS / EXA_COLORS).
    map: HashMap<String, Style>,
}

impl MetaTheme {
    fn from_env() -> Self {
        let raw = env::var("F00_COLORS")
            .or_else(|_| env::var("EZA_COLORS"))
            .or_else(|_| env::var("EXA_COLORS"))
            .unwrap_or_default();
        Self::from_string(&raw)
    }

    fn from_string(input: &str) -> Self {
        let mut map = HashMap::new();
        for part in input.split(':') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let Some((key, code)) = part.split_once('=') else {
                continue;
            };
            if let Some(style) = Style::from_ansi_sequence(code) {
                map.insert(key.to_string(), style);
            }
        }
        Self { map }
    }

    fn style(&self, key: &str) -> Option<&Style> {
        self.map.get(key)
    }

    fn paint_key(&self, key: &str, text: &str, fallback: AnsiStyle) -> String {
        if let Some(style) = self.style(key) {
            return style.to_nu_ansi_term_style().paint(text).to_string();
        }
        fallback.paint(text).to_string()
    }
}

/// Default ANSI roles (palette indexes 0–15) — follow the terminal theme.
fn def_blue_bold() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Blue).bold()
}
fn def_cyan() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Cyan)
}
fn def_cyan_bold() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Cyan).bold()
}
fn def_yellow() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Yellow)
}
fn def_yellow_bold() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Yellow).bold()
}
fn def_green() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Green)
}
fn def_green_bold() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Green).bold()
}
fn def_red() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Red)
}
fn def_red_bold() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Red).bold()
}
fn def_purple() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::Purple)
}
fn def_dark_gray() -> AnsiStyle {
    AnsiStyle::new().fg(AnsiColor::DarkGray)
}

impl Colorizer {
    /// Build from environment when enabled.
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            ls: LsColors::from_env().unwrap_or_default(),
            meta: MetaTheme::from_env(),
        }
    }

    /// Explicit `LS_COLORS` (tests); metadata uses empty override map + ANSI defaults.
    pub fn from_ls_colors(enabled: bool, ls_colors: &str) -> Self {
        Self {
            enabled,
            ls: LsColors::from_string(ls_colors),
            meta: MetaTheme::default(),
        }
    }

    /// Test helper: LS_COLORS + metadata color map string.
    pub fn from_ls_and_meta(enabled: bool, ls_colors: &str, meta: &str) -> Self {
        Self {
            enabled,
            ls: LsColors::from_string(ls_colors),
            meta: MetaTheme::from_string(meta),
        }
    }

    pub fn ls_colors(&self) -> &LsColors {
        &self.ls
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Filenames: **only** `LS_COLORS`.
    pub fn paint_name(&self, entry: &Entry, text: &str) -> String {
        if !self.enabled {
            return text.to_string();
        }
        if let Some(style) = self.style_for_entry(entry) {
            return paint_with_ls_style(text, style);
        }
        text.to_string()
    }

    pub fn style_for_entry<'a>(&'a self, entry: &'a Entry) -> Option<&'a Style> {
        if let Some(style) = self.ls.style_for_path(&entry.path) {
            return Some(style);
        }
        if let Some(style) = self.ls.style_for_path(Path::new(&entry.name)) {
            return Some(style);
        }
        let indicator = match entry.kind {
            EntryKind::Directory => Some(Indicator::Directory),
            EntryKind::Symlink => Some(Indicator::SymbolicLink),
            EntryKind::File if entry_is_exec(entry) => Some(Indicator::ExecutableFile),
            EntryKind::File => Some(Indicator::RegularFile),
            EntryKind::Other => {
                #[cfg(unix)]
                {
                    match entry.mode & 0o170000 {
                        0o010000 => Some(Indicator::FIFO),
                        0o140000 => Some(Indicator::Socket),
                        0o060000 => Some(Indicator::BlockDevice),
                        0o020000 => Some(Indicator::CharacterDevice),
                        _ => None,
                    }
                }
                #[cfg(not(unix))]
                {
                    None
                }
            }
        };
        indicator.and_then(|i| self.ls.style_for_indicator(i))
    }

    /// Git status: eza keys `gm`/`ga`/`gd`/`gv`/`gt` or ANSI defaults.
    pub fn paint_git_char(&self, ch: char) -> String {
        if !self.enabled {
            return ch.to_string();
        }
        let s = ch.to_string();
        let (key, fb) = match ch {
            'M' => ("gm", def_yellow_bold()),
            'A' => ("ga", def_green_bold()),
            'D' | 'U' => ("gd", def_red_bold()),
            '?' => ("gv", def_purple()),
            '!' => ("gt", def_dark_gray()),
            'R' => ("gm", def_cyan_bold()),
            _ => return s,
        };
        self.meta.paint_key(key, &s, fb)
    }

    pub fn modern_long_theme(&self, gnu_mode: bool) -> bool {
        self.enabled && !gnu_mode
    }

    /// Permissions: eza-style bit keys (`ur`/`uw`/`ux`/…) or ANSI defaults.
    pub fn paint_perms(&self, perms: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) || perms.is_empty() {
            return perms.to_string();
        }
        let mut out = String::with_capacity(perms.len() * 12);
        for (i, ch) in perms.chars().enumerate() {
            let painted = if i == 0 {
                match ch {
                    'd' => self.meta.paint_key("di", "d", def_blue_bold()),
                    'l' => self.meta.paint_key("ln", "l", def_cyan_bold()),
                    'c' | 'b' => self
                        .meta
                        .paint_key("bd", &ch.to_string(), def_yellow_bold()),
                    'p' => self.meta.paint_key("pi", "p", def_purple()),
                    's' => self.meta.paint_key("so", "s", def_purple()),
                    '-' => self.meta.paint_key("fi", "-", def_dark_gray()),
                    _ => self.meta.paint_key("fi", &ch.to_string(), def_dark_gray()),
                }
            } else {
                // owner/group/other triplets: positions 1-3, 4-6, 7-9
                let key = match (i, ch) {
                    (1, 'r') | (4, 'r') | (7, 'r') => Some(if i == 1 {
                        "ur"
                    } else if i == 4 {
                        "gr"
                    } else {
                        "tr"
                    }),
                    (2, 'w') | (5, 'w') | (8, 'w') => Some(if i == 2 {
                        "uw"
                    } else if i == 5 {
                        "gw"
                    } else {
                        "tw"
                    }),
                    (3, 'x' | 's' | 'S' | 't' | 'T')
                    | (6, 'x' | 's' | 'S' | 't' | 'T')
                    | (9, 'x' | 's' | 'S' | 't' | 'T') => Some(if i == 3 {
                        "ux"
                    } else if i == 6 {
                        "gx"
                    } else {
                        "tx"
                    }),
                    (_, '-') => None,
                    _ => None,
                };
                match (key, ch) {
                    (Some(k), _) => {
                        let fb = match ch {
                            'r' => def_yellow(),
                            'w' => def_red(),
                            'x' | 's' | 't' | 'S' | 'T' => def_green_bold(),
                            _ => AnsiStyle::new(),
                        };
                        self.meta.paint_key(k, &ch.to_string(), fb)
                    }
                    (None, '-') => self.meta.paint_key("xx", "-", def_dark_gray()),
                    _ => ch.to_string(),
                }
            };
            out.push_str(&painted);
        }
        out
    }

    /// Dim metadata (nlink, inode, blocks, context) — key `mp` (meta punct) or gray.
    pub fn paint_meta(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        self.meta.paint_key("mp", text, def_dark_gray())
    }

    /// Owner — `uu`; group uses same path via paint_user (callers share).
    pub fn paint_user(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        self.meta.paint_key("uu", text, def_yellow())
    }

    /// Group column — `gu`.
    pub fn paint_group(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        self.meta.paint_key("gu", text, def_yellow())
    }

    /// Size — `sn` (number); magnitude still picks weight via defaults if no override.
    pub fn paint_size(&self, text: &str, bytes: u64, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        if self.meta.style("sn").is_some() {
            return self.meta.paint_key("sn", text, def_green());
        }
        let fb = if bytes >= 1_073_741_824 {
            def_red_bold()
        } else if bytes >= 10_485_760 {
            def_yellow_bold()
        } else if bytes >= 1_048_576 {
            def_green_bold()
        } else if bytes > 0 {
            def_green()
        } else {
            def_dark_gray()
        };
        fb.paint(text).to_string()
    }

    /// Timestamp — `da`.
    pub fn paint_time(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        self.meta
            .paint_key("da", text, AnsiStyle::new().fg(AnsiColor::Blue))
    }

    /// Symlink name via LS_COLORS; arrow dim; target via path styles.
    pub fn paint_symlink_name(
        &self,
        entry: &Entry,
        icon_and_name: &str,
        arrow_and_target: &str,
        gnu_mode: bool,
    ) -> String {
        if !self.enabled {
            return format!("{icon_and_name}{arrow_and_target}");
        }

        let name = self.paint_name(entry, icon_and_name);
        if arrow_and_target.is_empty() {
            return name;
        }

        if let Some(rest) = arrow_and_target.strip_prefix(" -> ") {
            let target_painted = if let Some(style) = self.ls.style_for_path(Path::new(rest)) {
                paint_with_ls_style(rest, style)
            } else if self.modern_long_theme(gnu_mode) {
                self.meta.paint_key("lp", rest, def_cyan())
            } else {
                rest.to_string()
            };
            if self.modern_long_theme(gnu_mode) {
                let arrow = self.meta.paint_key("cc", "→", def_dark_gray());
                format!("{name} {arrow} {target_painted}")
            } else {
                format!("{name} -> {target_painted}")
            }
        } else if self.modern_long_theme(gnu_mode) {
            format!(
                "{name}{}",
                self.meta.paint_key("cc", arrow_and_target, def_dark_gray())
            )
        } else {
            format!("{name}{arrow_and_target}")
        }
    }
}

fn paint_with_ls_style(text: &str, style: &Style) -> String {
    style.to_nu_ansi_term_style().paint(text).to_string()
}

fn entry_is_exec(entry: &Entry) -> bool {
    #[cfg(unix)]
    {
        entry.mode & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        let _ = entry;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn file(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.into(),
            kind: EntryKind::File,
            size: 0,
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
            dev: 0,
            rdev: 0,
            blksize: 0,
            owner: "u".into(),
            group: "g".into(),
            author: "u".into(),
            context: String::new(),
        }
    }

    #[test]
    fn disabled_no_ansi() {
        let c = Colorizer::from_ls_colors(false, "*.rs=01;31");
        let e = file("main.rs");
        assert_eq!(c.paint_name(&e, "main.rs"), "main.rs");
    }

    #[test]
    fn ls_colors_extension() {
        let c = Colorizer::from_ls_colors(true, "*.rs=01;31:");
        let e = file("main.rs");
        let painted = c.paint_name(&e, "main.rs");
        assert_ne!(painted, "main.rs");
    }

    #[test]
    fn meta_eza_override_date() {
        let c = Colorizer::from_ls_and_meta(true, "", "da=31");
        let t = c.paint_time("Jul 20", false);
        assert!(t.contains("Jul 20"));
        assert_ne!(t, "Jul 20");
        assert!(t.contains('\u{1b}'));
    }

    #[test]
    fn modern_theme_off_under_gnu() {
        let c = Colorizer::from_ls_colors(true, "");
        assert!(!c.modern_long_theme(true));
        assert!(c.modern_long_theme(false));
        assert_eq!(c.paint_perms("-rwxr-xr-x", true), "-rwxr-xr-x");
        let modern = c.paint_perms("-rwxr-xr-x", false);
        assert_ne!(modern, "-rwxr-xr-x");
        assert!(modern.contains('\u{1b}'));
    }

    #[test]
    fn names_only_ls_colors_no_forced_dot_grey() {
        let c = Colorizer::from_ls_colors(true, "*.rs=01;31:");
        let hidden = file(".gitignore");
        assert_eq!(c.paint_name(&hidden, ".gitignore"), ".gitignore");
    }

    #[test]
    fn size_and_user_use_palette() {
        let c = Colorizer::from_ls_colors(true, "");
        let u = c.paint_user("alice", false);
        let s = c.paint_size("1.0K", 1024, false);
        assert!(u.contains("alice") && u.contains('\u{1b}'));
        assert!(s.contains("1.0K") && s.contains('\u{1b}'));
    }

    #[test]
    fn git_char_styled() {
        let c = Colorizer::from_ls_colors(true, "");
        assert_ne!(c.paint_git_char('M'), "M");
    }
}
