use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use f00_compat::{apply_gnu_list_options, apply_gnu_output};
use f00_core::{list_paths_with_errors, Config, ListOptions, OutputMode, SortBy};
use f00_format::format_listings;

use crate::cli::Args;
use crate::config::{load_user_config, resolve_args};

/// Detect terminal width; fall back to 80.
fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n: &usize| n > 0)
        .or_else(|| terminal_size::terminal_size().map(|(terminal_size::Width(w), _)| w as usize))
        .unwrap_or(80)
}

pub fn build_config(args: &Args) -> Config {
    let is_stdout_tty = io::stdout().is_terminal();

    let sort_by = if args.sort_time {
        SortBy::Time
    } else if args.sort_size {
        SortBy::Size
    } else if args.sort_extension {
        SortBy::Extension
    } else {
        SortBy::Name
    };

    let mut output = if args.json {
        OutputMode::Json
    } else if args.tree {
        OutputMode::Tree
    } else if args.long {
        OutputMode::Long
    } else if args.one_per_line {
        OutputMode::OnePerLine
    } else {
        OutputMode::Default
    };

    output = apply_gnu_output(output, is_stdout_tty, args.gnu);

    let mut list = ListOptions {
        all: args.all,
        almost_all: args.almost_all || args.all,
        sort_by,
        reverse: args.reverse,
        dirs_first: args.dirs_first,
        recursive: args.recursive || args.tree,
        max_depth: args.max_depth,
        gnu_mode: args.gnu,
        follow_links: false,
    };

    // Tree implies a practical depth if not set (still unlimited by default).
    if args.tree && args.max_depth.is_none() {
        // unlimited
    }

    apply_gnu_list_options(&mut list, args.gnu);

    // Classic ls: `-a` includes `.`/`..`; almost_all alone does not.
    // Our filter treats `all` as show everything including . and ..
    // and `almost_all` as show hidden but not . / ..
    if args.all {
        list.all = true;
        list.almost_all = false;
    } else if args.almost_all {
        list.almost_all = true;
        list.all = false;
    }

    // Strict GNU mode: no icons (and other non-GNU decorations).
    let icons = if args.gnu { false } else { args.icons };

    Config {
        list,
        output,
        color: args.color.into(),
        human_sizes: args.human_readable,
        icons,
        classify: args.classify,
        terminal_width: terminal_width(),
        is_stdout_tty,
    }
}

/// Prepare args: load config, apply argv0 / env merges. Returns owned Args.
pub fn prepare_args(mut args: Args, as_ls: bool) -> Result<Args> {
    let file = load_user_config(args.config.as_deref())?;
    resolve_args(&mut args, file.as_ref(), as_ls);
    Ok(args)
}

/// Run the lister. Returns a GNU-aligned exit code: 0 / 1 / 2.
pub fn run(args: Args) -> Result<i32> {
    run_with_argv0(args, false)
}

/// Run with explicit argv0-as-ls mode (for tests / main).
pub fn run_with_argv0(args: Args, as_ls: bool) -> Result<i32> {
    let args = prepare_args(args, as_ls)?;
    let config = build_config(&args);
    let paths: Vec<PathBuf> = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let outcome = list_paths_with_errors(&paths, &config.list);

    for err in &outcome.path_errors {
        eprintln!("f00: {err}");
    }

    let exit_code = outcome.exit_code();
    let mut listings = outcome.listings;

    // Optional git annotation
    if args.git {
        #[cfg(feature = "git")]
        {
            f00_git::annotate_listings(&mut listings);
        }
        #[cfg(not(feature = "git"))]
        {
            // Feature disabled: ignore silently.
        }
    }

    if !listings.is_empty() {
        let rendered = format_listings(&listings, &config).map_err(|e| anyhow::anyhow!(e))?;

        let mut stdout = io::stdout().lock();
        stdout
            .write_all(rendered.as_bytes())
            .context("writing output")?;
    } else if exit_code == 0 {
        // Should be rare (empty path set always lists `.`).
        eprintln!("f00: no listings produced");
        return Ok(2);
    }

    Ok(exit_code)
}
