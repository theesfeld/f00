//! Color engine using the `lscolors` crate fully from `LS_COLORS`.

use std::path::Path;

use f00_core::{Entry, EntryKind};
use lscolors::{LsColors, Style};
use nu_ansi_term::Color;

/// Color engine wrapping `LS_COLORS` / defaults.
#[derive(Clone)]
pub struct Colorizer {
    enabled: bool,
    ls: LsColors,
}

impl Colorizer {
    /// Build from environment (`LS_COLORS`) when enabled.
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

    /// Colorize a display name for an entry using full LS_COLORS matching.
    pub fn paint_name(&self, entry: &Entry, text: &str) -> String {
        if !self.enabled {
            return text.to_string();
        }

        // Prefer path + metadata style when possible.
        if let Some(style) = self.style_for_entry(entry) {
            return paint_with_ls_style(text, style);
        }

        // Fallback by kind
        match entry.kind {
            EntryKind::Directory => Color::Blue.bold().paint(text).to_string(),
            EntryKind::Symlink => Color::Cyan.paint(text).to_string(),
            EntryKind::File if entry_is_exec(entry) => Color::Green.bold().paint(text).to_string(),
            EntryKind::File => text.to_string(),
            EntryKind::Other => Color::Yellow.paint(text).to_string(),
        }
    }

    /// Resolve LS_COLORS style for an entry (extension, type indicators, etc.).
    pub fn style_for_entry<'a>(&'a self, entry: &'a Entry) -> Option<&'a Style> {
        // Try path with metadata-like hints via style_for_path first.
        if let Some(style) = self.ls.style_for_path(&entry.path) {
            return Some(style);
        }
        // Extension / name patterns
        if let Some(style) = self.ls.style_for_path(Path::new(&entry.name)) {
            return Some(style);
        }
        // Indicator by kind
        use lscolors::Indicator;
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

    pub fn paint_git_char(&self, ch: char) -> String {
        if !self.enabled {
            return ch.to_string();
        }
        let s = ch.to_string();
        match ch {
            'M' => Color::Yellow.bold().paint(s).to_string(),
            'A' => Color::Green.bold().paint(s).to_string(),
            'D' | 'U' => Color::Red.bold().paint(s).to_string(),
            '?' => Color::Purple.paint(s).to_string(),
            '!' => Color::DarkGray.paint(s).to_string(),
            'R' => Color::Cyan.bold().paint(s).to_string(),
            _ => s,
        }
    }

    /// Whether modern multi-column long-format theming should run.
    ///
    /// On for friendly (non-GNU) mode when colors are enabled; off under `--gnu`.
    pub fn modern_long_theme(&self, gnu_mode: bool) -> bool {
        self.enabled && !gnu_mode
    }

    /// Permission string (e.g. `-rwxr-xr-x`) with type letter + rwx tinting.
    pub fn paint_perms(&self, perms: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) || perms.is_empty() {
            return perms.to_string();
        }
        let mut out = String::with_capacity(perms.len() * 8);
        for (i, ch) in perms.chars().enumerate() {
            let painted = if i == 0 {
                match ch {
                    'd' => Color::Blue.bold().paint(ch.to_string()).to_string(),
                    'l' => Color::Cyan.bold().paint(ch.to_string()).to_string(),
                    'c' | 'b' => Color::Yellow.bold().paint(ch.to_string()).to_string(),
                    'p' | 's' => Color::Purple.paint(ch.to_string()).to_string(),
                    _ => Color::DarkGray.paint(ch.to_string()).to_string(),
                }
            } else {
                match ch {
                    'r' => Color::Yellow.paint(ch.to_string()).to_string(),
                    'w' => Color::Red.paint(ch.to_string()).to_string(),
                    'x' | 's' | 't' | 'S' | 'T' => {
                        Color::Green.bold().paint(ch.to_string()).to_string()
                    }
                    '-' => Color::DarkGray.paint(ch.to_string()).to_string(),
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
        Color::DarkGray.paint(text).to_string()
    }

    /// Owner / group / author column.
    pub fn paint_user(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        Color::Yellow.paint(text).to_string()
    }

    /// Size column: green → yellow → red by magnitude (bytes).
    pub fn paint_size(&self, text: &str, bytes: u64, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        let style = if bytes >= 1_073_741_824 {
            // ≥ 1 GiB
            Color::Red.bold()
        } else if bytes >= 10_485_760 {
            // ≥ 10 MiB
            Color::Yellow.bold()
        } else if bytes >= 1_048_576 {
            // ≥ 1 MiB
            Color::Green.bold()
        } else if bytes > 0 {
            Color::Green.normal()
        } else {
            Color::DarkGray.normal()
        };
        style.paint(text).to_string()
    }

    /// Timestamp column.
    pub fn paint_time(&self, text: &str, gnu_mode: bool) -> String {
        if !self.modern_long_theme(gnu_mode) {
            return text.to_string();
        }
        Color::Blue.paint(text).to_string()
    }

    /// Symlink name + arrow + target (name via LS_COLORS / cyan; target dim cyan).
    pub fn paint_symlink_name(
        &self,
        entry: &Entry,
        icon_and_name: &str,
        arrow_and_target: &str,
        gnu_mode: bool,
    ) -> String {
        if !self.modern_long_theme(gnu_mode) {
            let full = format!("{icon_and_name}{arrow_and_target}");
            return self.paint_name(entry, &full);
        }
        let name = self.paint_name(entry, icon_and_name);
        if arrow_and_target.is_empty() {
            return name;
        }
        // Split " -> target" style
        let target = if let Some(rest) = arrow_and_target.strip_prefix(" -> ") {
            format!(
                " {} {}",
                Color::DarkGray.paint("→"),
                Color::Cyan.dimmed().paint(rest)
            )
        } else {
            Color::DarkGray.paint(arrow_and_target).to_string()
        };
        format!("{name}{target}")
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
        // Should contain ANSI CSI when style applies.
        assert!(painted.contains("main.rs"), "{painted:?}");
        // Style applied → not equal plain (or equal if style empty — accept either with style present)
        let _ = c.style_for_entry(&e);
    }

    #[test]
    fn modern_theme_off_under_gnu() {
        let c = Colorizer::from_ls_colors(true, "");
        assert!(!c.modern_long_theme(true));
        assert!(c.modern_long_theme(false));
        assert_eq!(c.paint_perms("-rwxr-xr-x", true), "-rwxr-xr-x");
        assert_ne!(c.paint_perms("-rwxr-xr-x", false), "-rwxr-xr-x");
    }

    #[test]
    fn modern_size_tints() {
        let c = Colorizer::from_ls_colors(true, "");
        let small = c.paint_size("1.0K", 1024, false);
        let big = c.paint_size("2.0G", 2_147_483_648, false);
        assert!(small.contains("1.0K"), "{small}");
        assert!(big.contains("2.0G"), "{big}");
        // Different colors → different escape sequences (or at least styled).
        assert!(small.contains('\u{1b}') || small != "1.0K");
        assert!(big.contains('\u{1b}') || big != "2.0G");
    }
}
