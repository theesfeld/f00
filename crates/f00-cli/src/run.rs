use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use f00_compat::{apply_gnu_list_options, apply_gnu_output};
use f00_core::{list_paths, Config, ListOptions, OutputMode, SortBy};
use f00_format::format_listings;

use crate::cli::Args;

/// Detect terminal width; fall back to 80.
fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n: &usize| n > 0)
        .or_else(|| {
            terminal_size::terminal_size().map(|(terminal_size::Width(w), _)| w as usize)
        })
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

    Config {
        list,
        output,
        color: args.color.into(),
        human_sizes: args.human_readable,
        icons: args.icons,
        classify: args.classify,
        terminal_width: terminal_width(),
        is_stdout_tty,
    }
}

pub fn run(args: Args) -> Result<()> {
    let config = build_config(&args);
    let paths: Vec<PathBuf> = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let mut listings = list_paths(&paths, &config.list).context("listing paths")?;

    // Optional git annotation
    if args.git {
        #[cfg(feature = "git")]
        {
            f00_git::annotate_listings(&mut listings);
        }
        #[cfg(not(feature = "git"))]
        {
            // Feature disabled: ignore silently unless user forced --git=true is default;
            // only warn when they might care — default true with no feature would be odd
            // but default feature enables git.
        }
    }

    let rendered = format_listings(&listings, &config).map_err(|e| anyhow::anyhow!(e))?;

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(rendered.as_bytes())
        .context("writing output")?;

    // Propagate not-found as error exit for bad paths — list_paths already errors.
    if listings.is_empty() {
        bail!("no listings produced");
    }

    Ok(())
}
