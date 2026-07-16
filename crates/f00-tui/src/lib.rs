//! Interactive TUI directory browser for **f00**.
//!
//! # CLI wiring (f00-cli)
//!
//! Gate the dependency behind a cargo feature (not in default features):
//!
//! ```toml
//! # crates/f00-cli/Cargo.toml
//! [dependencies]
//! f00-tui = { workspace = true, optional = true }
//!
//! [features]
//! default = ["git"]
//! git = ["dep:f00-git"]
//! tui = ["dep:f00-tui"]
//! # optional: enable git status inside the browser too
//! # tui = ["dep:f00-tui", "f00-tui/git"]
//! ```
//!
//! Add CLI flags such as `--browse` / `--tui`, then:
//!
//! ```rust,ignore
//! #[cfg(feature = "tui")]
//! {
//!     if args.browse || args.tui {
//!         let start = args
//!             .paths
//!             .first()
//!             .cloned()
//!             .unwrap_or_else(|| PathBuf::from("."));
//!         let code = f00_tui::run_browser(
//!             &start,
//!             f00_tui::BrowserOptions {
//!                 show_hidden: args.almost_all || args.all,
//!                 icons: config.icons,
//!                 git: cfg!(feature = "git") && config.show_git,
//!             },
//!         )?;
//!         std::process::exit(code);
//!     }
//! }
//! ```
//!
//! Without the `tui` feature, print a short message that the binary was built
//! without browser support.

mod browser;
mod helpers;

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
