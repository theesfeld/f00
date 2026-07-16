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
}

impl GnuProfile {
    pub fn enabled() -> Self {
        Self {
            force_one_per_line_when_not_tty: true,
            disable_dirs_first: true,
            strict_name_sort: true,
        }
    }
}

/// Apply GNU-mode tweaks onto listing options.
pub fn apply_gnu_list_options(opts: &mut ListOptions, gnu: bool) {
    if !gnu {
        return;
    }
    opts.gnu_mode = true;
    // GNU ls does not default to directories-first.
    opts.dirs_first = false;
    // Keep sort as-is unless caller left it default.
    if opts.sort_by == SortBy::Name {
        // already name
    }
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

/// Whether a flag combination is "strict GNU" enough to emit a notice in verbose mode.
pub fn gnu_mode_active(gnu_flag: bool) -> bool {
    gnu_flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_gnu_disables_dirs_first() {
        let mut opts = ListOptions {
            dirs_first: true,
            ..Default::default()
        };
        apply_gnu_list_options(&mut opts, true);
        assert!(!opts.dirs_first);
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
}
