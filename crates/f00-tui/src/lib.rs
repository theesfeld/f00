//! Interactive TUI directory browser for **f00**.
//!
//! # Binary
//!
//! This crate ships the **`f00-tui`** binary (dual-pane FM). Prefer that over
//! embedding the browser into the main `f00` CLI.
//!
//! # Optional embed in f00-cli
//!
//! Gate the dependency behind cargo feature `tui` (not a default feature):
//!
//! ```toml
//! # crates/f00-cli/Cargo.toml
//! [dependencies]
//! f00-tui = { workspace = true, optional = true }
//!
//! [features]
//! default = ["git", "io-uring"]
//! tui = ["dep:f00-tui", "f00-tui/git"]
//! ```
//!
//! Then `f00 --browse` can call [`run_browser`]. Without the feature, the CLI
//! points users at the `f00-tui` binary.

mod browser;
mod helpers;
mod preview;

use std::path::Path;

/// Options for [`run_browser`].
#[derive(Debug, Clone, Default)]
pub struct BrowserOptions {
    /// Show hidden entries (`almost_all` / `-A` semantics via f00-core).
    pub show_hidden: bool,
    /// Prefix names with emoji icons from f00-format.
    pub icons: bool,
    /// Annotate entries with git status when the `git` feature is enabled.
    pub git: bool,
}

/// Run the fullscreen directory browser starting at `start`.
///
/// # Behavior
///
/// - Lists the current directory with j/k navigation, Enter to open, etc.
/// - On quit with selection (`y`, or Enter on a file), prints paths to stdout
///   (one per line) **after** restoring the terminal.
/// - On quit with `q` / Esc, prints nothing.
/// - Returns exit code `0` on success.
///
/// # Errors
///
/// Returns an error if stdin/stdout are not TTYs, or if terminal setup fails.
/// A panic hook is installed so raw mode / alternate screen are left on panic.
pub fn run_browser(start: &Path, opts: BrowserOptions) -> anyhow::Result<i32> {
    browser::run(start, opts)
}

// Re-export pure helpers for tests / advanced callers.
pub use helpers::{
    clamp_index, filter_entries, format_selected_paths, is_interactive_tty, join_child,
    move_selection, parent_dir,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::IsTerminal;

    #[test]
    fn run_browser_skips_without_tty() {
        // In CI / non-interactive environments this must not hang or panic.
        if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
            // Actual fullscreen TUI is not exercised in automated tests.
            return;
        }
        let err = run_browser(Path::new("."), BrowserOptions::default());
        assert!(err.is_err());
        let msg = format!("{:#}", err.unwrap_err());
        assert!(
            msg.contains("TTY") || msg.contains("tty") || msg.contains("interactive"),
            "unexpected error: {msg}"
        );
    }
}
