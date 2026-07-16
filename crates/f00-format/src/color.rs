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
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            ls: LsColors::from_env().unwrap_or_default(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Colorize a display name for an entry.
    pub fn paint_name(&self, entry: &Entry, text: &str) -> String {
        if !self.enabled {
            return text.to_string();
        }

        // Prefer LS_COLORS style for the path.
        if let Some(style) = self.ls.style_for_path(&entry.path) {
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
