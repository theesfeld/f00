//! GNU / POSIX `ls` compatibility helpers for **f00**.

use f00_core::{ListOptions, OutputMode, SortBy};

/// Compatibility profile applied when `--gnu` is set.
#[derive(Debug, Clone, Default)]
pub struct GnuProfile {
    /// Prefer one-per-line when not a TTY (GNU ls default).
    pub force_one_per_line_when_not_tty: bool,
    /// Sort directories mixed with files (GNU default); disable dirs_first.
    pub disable_dirs_first: bool,
    /// Use strict name sort without stripping dots for collate key.
    pub strict_name_sort: bool,
    /// Disable icons and git decorations.
    pub disable_decorations: bool,
}

impl GnuProfile {
    pub fn enabled() -> Self {
        Self {
            force_one_per_line_when_not_tty: true,
            disable_dirs_first: true,
            strict_name_sort: true,
            disable_decorations: true,
        }
    }
}

/// Apply GNU-mode tweaks onto listing options.
pub fn apply_gnu_list_options(opts: &mut ListOptions, gnu: bool) {
    if !gnu {
        return;
    }
    opts.gnu_mode = true;
    // GNU ls does not *default* to directories-first, but `--group-directories-first`
    // is honored. Never clear `dirs_first` here — the CLI already sets the flag.
}

/// Adjust output mode for GNU-ish behavior.
pub fn apply_gnu_output(mode: OutputMode, is_tty: bool, gnu: bool) -> OutputMode {
    if !gnu {
        return mode;
    }
    match mode {
        OutputMode::Default if !is_tty => OutputMode::OnePerLine,
        other => other,
    }
}

/// Whether a flag combination is "strict GNU" enough.
pub fn gnu_mode_active(gnu_flag: bool) -> bool {
    gnu_flag
}

/// Soft defaults when the binary is invoked as `ls` / `ls.exe`.
///
/// Interactive drop-in keeps **full f00 chrome** (icons, git, modern colors) —
/// `icons=auto` already stays off when stdout is not a TTY. Only dirs-first is
/// defaulted off (GNU `ls` does not group dirs first) unless the user passed
/// `--group-directories-first` / config.
///
/// Full strict coreutils behavior remains opt-in via `--gnu` / `F00_GNU`.
pub fn prefer_ls_defaults(dirs_first: &mut bool, dirs_first_from_cli: bool) {
    if !dirs_first_from_cli {
        *dirs_first = false;
    }
}

/// Parse GNU `--sort=WORD`.
pub fn parse_sort_word(word: &str) -> Option<SortBy> {
    match word.to_ascii_lowercase().as_str() {
        "name" => Some(SortBy::Name),
        "size" => Some(SortBy::Size),
        "time" => Some(SortBy::Time),
        "extension" | "ext" => Some(SortBy::Extension),
        "version" | "v" => Some(SortBy::Version),
        "width" => Some(SortBy::Width),
        "none" => Some(SortBy::None),
        _ => None,
    }
}

/// Parse GNU `--format=WORD`.
pub fn parse_format_word(word: &str) -> Option<OutputMode> {
    match word.to_ascii_lowercase().as_str() {
        "across" | "horizontal" | "x" => Some(OutputMode::Across),
        "commas" | "m" => Some(OutputMode::Commas),
        "long" | "verbose" | "l" => Some(OutputMode::Long),
        "single-column" | "single" | "1" => Some(OutputMode::OnePerLine),
        "vertical" | "c" => Some(OutputMode::Columns),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_gnu_sets_mode_preserves_dirs_first_flag() {
        let mut opts = ListOptions {
            dirs_first: true,
            ..Default::default()
        };
        apply_gnu_list_options(&mut opts, true);
        // Explicit --group-directories-first must survive --gnu.
        assert!(opts.dirs_first);
        assert!(opts.gnu_mode);
    }

    #[test]
    fn without_flag_leaves_options() {
        let mut opts = ListOptions {
            dirs_first: true,
            ..Default::default()
        };
        apply_gnu_list_options(&mut opts, false);
        assert!(opts.dirs_first);
        assert!(!opts.gnu_mode);
    }

    #[test]
    fn prefer_ls_defaults_dirs_first_off() {
        let mut dirs_first = true;
        prefer_ls_defaults(&mut dirs_first, false);
        assert!(!dirs_first);
    }

    #[test]
    fn prefer_ls_keeps_explicit_dirs_first() {
        let mut dirs_first = true;
        prefer_ls_defaults(&mut dirs_first, true);
        assert!(dirs_first);
    }

    #[test]
    fn parse_sort_words() {
        assert_eq!(parse_sort_word("size"), Some(SortBy::Size));
        assert_eq!(parse_sort_word("none"), Some(SortBy::None));
        assert_eq!(parse_sort_word("version"), Some(SortBy::Version));
        assert_eq!(parse_sort_word("bogus"), None);
    }

    #[test]
    fn parse_format_words() {
        assert_eq!(parse_format_word("long"), Some(OutputMode::Long));
        assert_eq!(parse_format_word("commas"), Some(OutputMode::Commas));
        assert_eq!(parse_format_word("across"), Some(OutputMode::Across));
    }
}
