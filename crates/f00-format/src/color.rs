//! Color engine driven by the user's `LS_COLORS` theme.
//!
//! **Policy (v0.12):**
//! - Filenames and symlink targets use **only** `LS_COLORS` (via `lscolors`).
//! - Long-format metadata, git status, and similar chrome never force named hues
//!   (Blue / Yellow / Red / Cyan / …). At most relative **dim** / **bold** so the
//!   terminal palette and theme stay in control.
//! - When color is off, all paint helpers return plain text.

use std::path::Path;

use f00_core::{Entry, EntryKind};
use lscolors::{Indicator, LsColors, Style};
use nu_ansi_term::Style as AnsiStyle;

/// Color engine wrapping `LS_COLORS` / dircolors defaults.
#[derive(Clone)]
pub struct Colorizer {
    enabled: bool,
    ls: LsColors,
}

impl Colorizer {
    /// Build from environment (`LS_COLORS`) when enabled.
    ///
    /// `LsColors::from_env()` falls back to the standard dircolors defaults when
    /// the variable is unset, so a color-capable terminal still gets the user's
    /// (or distro's) type colors without hard-coding a private palette.
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            ls: LsColors::from_env().unwrap_or_default(),
        }
    }

    /// Build with an explicit LS_COLORS string (tests / overrides).
    pub fn from_ls_colors(enabled: bool, ls_colors: &str) -> Self {
        Self {
            enabled,
            ls: LsColors::from_string(ls_colors),
        }
    }

    /// Access the underlying `LsColors` map.
    pub fn ls_colors(&self) -> &LsColors {
        &self.ls
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Colorize a display name using **only** `LS_COLORS` matching.
    ///
    /// No private fallback palette and no forced grey for dotfiles — hidden
    /// names follow `mh=` / extension / type rules from the user's theme.
    pub fn paint_name(&self, entry: &Entry, text: &str) -> String {
        if !self.enabled {
            return text.to_string();
        }
        if let Some(style) = self.style_for_entry(entry) {
            return paint_with_ls_style(text, style);
        }
        text.to_string()
    }

    /// Resolve LS_COLORS style for an entry (extension, type indicators, etc.).
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

    /// Git status character: bold when dirty, dim when ignored — no fixed hues.
    pub fn paint_git_char(&self, ch: char) -> String {
        if !self.enabled {
            return ch.to_string();
        }
        let s = ch.to_string();
        match ch {
            '!' => dim_paint(&s),
            ' ' => s,
            _ => bold_paint(&s),
        }
    }

    /// Whether modern long-format chrome (dim/bold accents) should run.
    ///
    /// On for friendly (non-GNU) mode when colors are enabled; off under `--gnu`
    /// so GNU listings stay plain aside from name `LS_COLORS` when forced on.
    pub fn modern_long_theme(&self, gnu_mode: bool) -> bool {
        self.enabled && !gnu_mode
    }

    /// Permission string: type letter bold, dashes dim — no hue roles.
    pub fn paint_perms(&self, perms: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) || perms.is_empty() {
            return perms.to_string();
        }
        let mut out = String::with_capacity(perms.len() * 8);
        for (i, ch) in perms.chars().enumerate() {
            let painted = if i == 0 {
                match ch {
                    '-' => dim_paint(&ch.to_string()),
                    _ => bold_paint(&ch.to_string()),
                }
            } else {
                match ch {
                    '-' => dim_paint(&ch.to_string()),
                    'x' | 's' | 't' | 'S' | 'T' => bold_paint(&ch.to_string()),
                    _ => ch.to_string(),
                }
            };
            out.push_str(&painted);
        }
        out
    }

    /// Dim metadata (nlink, inode, blocks, context).
    pub fn paint_meta(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        dim_paint(text)
    }

    /// Owner / group / author column (plain — theme stays neutral).
    pub fn paint_user(&self, text: &str, gnu_mode: bool) -> String {
        let _ = gnu_mode;
        text.to_string()
    }

    /// Size column: bold for large files, dim for empty — no size→hue map.
    pub fn paint_size(&self, text: &str, bytes: u64, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        if bytes == 0 {
            dim_paint(text)
        } else if bytes >= 1_073_741_824 {
            bold_paint(text)
        } else {
            text.to_string()
        }
    }

    /// Timestamp column (plain under theme inheritance).
    pub fn paint_time(&self, text: &str, gnu_mode: bool) -> String {
        let _ = gnu_mode;
        text.to_string()
    }

    /// Symlink name via LS_COLORS; arrow dim; target via LS_COLORS when possible.
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
            let arrow = if self.modern_long_theme(gnu_mode) {
                dim_paint("→")
            } else {
                "->".to_string()
            };
            let target_painted = if let Some(style) = self.ls.style_for_path(Path::new(rest)) {
                paint_with_ls_style(rest, style)
            } else if let Some(style) = self.ls.style_for_indicator(Indicator::SymbolicLink) {
                // Orphan / unknown target: still allow ln= if path match failed.
                let _ = style;
                if self.modern_long_theme(gnu_mode) {
                    dim_paint(rest)
                } else {
                    rest.to_string()
                }
            } else if self.modern_long_theme(gnu_mode) {
                dim_paint(rest)
            } else {
                rest.to_string()
            };
            let sep = if self.modern_long_theme(gnu_mode) {
                format!(" {arrow} ")
            } else {
                format!(" {arrow} ")
            };
            // gnu uses " -> "; modern uses dim arrow
            if self.modern_long_theme(gnu_mode) {
                format!("{name} {arrow} {target_painted}")
            } else {
                let _ = sep;
                format!("{name} -> {target_painted}")
            }
        } else if self.modern_long_theme(gnu_mode) {
            format!("{name}{}", dim_paint(arrow_and_target))
        } else {
            format!("{name}{arrow_and_target}")
        }
    }
}

fn paint_with_ls_style(text: &str, style: &Style) -> String {
    style.to_nu_ansi_term_style().paint(text).to_string()
}

/// Relative dim (SGR 2) — inherits the terminal foreground, no fixed color index.
fn dim_paint(text: &str) -> String {
    AnsiStyle::new().dimmed().paint(text).to_string()
}

/// Relative bold (SGR 1).
fn bold_paint(text: &str) -> String {
    AnsiStyle::new().bold().paint(text).to_string()
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
        assert!(painted.contains("main.rs"), "{painted:?}");
        assert_ne!(painted, "main.rs", "LS_COLORS extension should paint");
    }

    #[test]
    fn modern_theme_off_under_gnu() {
        let c = Colorizer::from_ls_colors(true, "");
        assert!(!c.modern_long_theme(true));
        assert!(c.modern_long_theme(false));
        assert_eq!(c.paint_perms("-rwxr-xr-x", true), "-rwxr-xr-x");
        // modern dims dashes / bolds type+exec bits (relative SGR, no hues)
        let modern = c.paint_perms("-rwxr-xr-x", false);
        assert_ne!(modern, "-rwxr-xr-x");
        assert!(modern.contains('\u{1b}'), "expected ANSI SGR in modern perms");
        assert!(modern.contains('r') && modern.contains('w') && modern.contains('x'));
    }

    #[test]
    fn names_follow_ls_colors_only_no_forced_dot_grey() {
        // Without mh=/dot rules, a bare name is plain (not forced DarkGray).
        let c = Colorizer::from_ls_colors(true, "*.rs=01;31:");
        let hidden = file(".gitignore");
        let painted_h = c.paint_name(&hidden, ".gitignore");
        // No extension match → plain (LS_COLORS only).
        assert_eq!(painted_h, ".gitignore");

        let with_mh = Colorizer::from_ls_colors(true, "mh=01;90:*.rs=01;31:");
        let mh_paint = with_mh.paint_name(&hidden, ".gitignore");
        // mh= may or may not apply depending on lscolors path rules; either way
        // we must not inject a private palette when LS_COLORS has no match.
        assert!(mh_paint.contains(".gitignore"));

        let off = Colorizer::from_ls_colors(false, "");
        assert_eq!(off.paint_name(&hidden, ".gitignore"), ".gitignore");
    }

    #[test]
    fn size_uses_weight_not_hue() {
        let c = Colorizer::from_ls_colors(true, "");
        let small = c.paint_size("1.0K", 1024, false);
        let big = c.paint_size("2.0G", 2_147_483_648, false);
        let empty = c.paint_size("0", 0, false);
        assert!(small.contains("1.0K"), "{small}");
        assert!(big.contains("2.0G"), "{big}");
        assert!(empty.contains('0'), "{empty}");
        // Empty is dimmed (SGR), large may be bold — both relative attributes.
        assert_ne!(empty, "0");
    }

    #[test]
    fn git_char_no_named_hues() {
        let c = Colorizer::from_ls_colors(true, "");
        let m = c.paint_git_char('M');
        let ign = c.paint_git_char('!');
        assert!(m.contains('M'));
        assert!(ign.contains('!'));
        // Bold / dim only — both should differ from plain when enabled.
        assert_ne!(m, "M");
        assert_ne!(ign, "!");
    }

    #[test]
    fn no_hardcoded_blue_yellow_red_in_meta() {
        let c = Colorizer::from_ls_colors(true, "");
        let user = c.paint_user("alice", false);
        let time = c.paint_time("Jul 20 12:00", false);
        assert_eq!(user, "alice");
        assert_eq!(time, "Jul 20 12:00");
    }
}
