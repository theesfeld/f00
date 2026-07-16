use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use f00_compat::{apply_gnu_list_options, apply_gnu_output, parse_format_word, parse_sort_word};
use f00_core::{
    list_paths_with_errors, Config, IndicatorStyle, ListOptions, OutputMode, SortBy, TimeField,
};
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

fn resolve_sort(args: &Args) -> SortBy {
    if let Some(ref word) = args.sort {
        if let Some(s) = parse_sort_word(word) {
            return s;
        }
    }
    if args.sort_none || args.unsorted_all {
        SortBy::None
    } else if args.sort_time || args.access_time || args.change_time {
        SortBy::Time
    } else if args.sort_size {
        SortBy::Size
    } else if args.sort_extension {
        SortBy::Extension
    } else {
        SortBy::Name
    }
}

fn resolve_time_field(args: &Args) -> TimeField {
    if let Some(ref word) = args.time {
        return match word.to_ascii_lowercase().as_str() {
            "atime" | "access" | "use" => TimeField::Accessed,
            "ctime" | "status" | "change" => TimeField::Changed,
            "birth" | "creation" => TimeField::Birth,
            _ => TimeField::Modified,
        };
    }
    if args.access_time {
        TimeField::Accessed
    } else if args.change_time {
        TimeField::Changed
    } else {
        TimeField::Modified
    }
}

fn resolve_output(args: &Args) -> OutputMode {
    if let Some(ref word) = args.format {
        if let Some(m) = parse_format_word(word) {
            return m;
        }
    }
    if args.json {
        OutputMode::Json
    } else if args.tree {
        OutputMode::Tree
    } else if args.long || args.no_owner || args.no_group_long || args.numeric_uid_gid {
        // -g/-o/-n imply long format in GNU ls
        OutputMode::Long
    } else if args.one_per_line {
        OutputMode::OnePerLine
    } else if args.commas {
        OutputMode::Commas
    } else if args.columns || args.across {
        OutputMode::Columns
    } else {
        OutputMode::Default
    }
}

fn resolve_indicator(args: &Args) -> IndicatorStyle {
    if args.classify {
        IndicatorStyle::Classify
    } else if args.file_type {
        IndicatorStyle::FileType
    } else if args.indicator_slash {
        IndicatorStyle::Slash
    } else {
        IndicatorStyle::None
    }
}

pub fn build_config(args: &Args) -> Config {
    let is_stdout_tty = io::stdout().is_terminal();
    let sort_by = resolve_sort(args);
    let mut output = resolve_output(args);
    output = apply_gnu_output(output, is_stdout_tty, args.gnu);

    let mut all = args.all || args.unsorted_all;
    let mut almost_all = args.almost_all;

    let mut list = ListOptions {
        all,
        almost_all: almost_all || all,
        sort_by,
        reverse: args.reverse,
        dirs_first: args.dirs_first && !args.gnu,
        recursive: (args.recursive || args.tree) && !args.directory,
        max_depth: args.max_depth,
        gnu_mode: args.gnu,
        follow_links: args.dereference,
        directory: args.directory,
        ignore_backups: args.ignore_backups,
        ignore_patterns: args.ignore.clone(),
        time_field: resolve_time_field(args),
    };

    apply_gnu_list_options(&mut list, args.gnu);

    // Classic ls: `-a` includes `.`/`..`; almost_all alone does not.
    if all {
        list.all = true;
        list.almost_all = false;
    } else if almost_all {
        list.almost_all = true;
        list.all = false;
    }

    // Strict GNU / -f: no icons, no git decorations.
    let icons = if args.gnu || args.unsorted_all {
        false
    } else {
        args.icons
    };
    let show_git = args.git && !args.gnu && !args.unsorted_all;

    // -g implies long without owner; -o implies long without group.
    let show_owner = !args.no_owner;
    let show_group = !(args.no_group || args.no_group_long);

    // -l is implied by several flags; ensure long mode when only -i/-s with -l-like need?
    // GNU: -i and -s work with any format; we support them primarily in long mode but
    // also prefix one-per-line when set.
    if (args.inode || args.size_blocks || args.full_time) && matches!(output, OutputMode::Default) {
        // keep default; fields appear when long
    }
    if args.full_time && !matches!(output, OutputMode::Long) {
        output = OutputMode::Long;
    }

    let _ = (&mut all, &mut almost_all);

    Config {
        list,
        output,
        color: if args.unsorted_all && !args.gnu {
            // GNU -f disables color; honor that unless user forced --color=
            args.color.into()
        } else {
            args.color.into()
        },
        human_sizes: args.human_readable || args.si,
        si_sizes: args.si,
        icons,
        classify: args.classify,
        indicator: resolve_indicator(args),
        terminal_width: terminal_width(),
        is_stdout_tty,
        show_owner,
        show_group,
        numeric_uid_gid: args.numeric_uid_gid,
        show_inode: args.inode,
        show_blocks: args.size_blocks,
        full_time: args.full_time,
        show_git,
    }
}

/// Prepare args: load config, apply argv0 / env merges. Returns owned Args.
pub fn prepare_args(mut args: Args, as_ls: bool) -> Result<Args> {
    let file = load_user_config(args.config.as_deref())?;
    resolve_args(&mut args, file.as_ref(), as_ls);
    // Strict GNU disables git unless user re-enables after — config may set git=true;
    // apply_gnu at build_config handles decorations. Also force git=false when --gnu.
    if args.gnu {
        args.git = false;
        args.icons = false;
        args.dirs_first = false;
    }
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
    if config.show_git && args.git {
        #[cfg(feature = "git")]
        {
            f00_git::annotate_listings(&mut listings);
        }
        #[cfg(not(feature = "git"))]
        {}
    }

    if !listings.is_empty() {
        let rendered = format_listings(&listings, &config).map_err(|e| anyhow::anyhow!(e))?;

        let mut stdout = io::stdout().lock();
        stdout
            .write_all(rendered.as_bytes())
            .context("writing output")?;
    } else if exit_code == 0 {
        eprintln!("f00: no listings produced");
        return Ok(2);
    }

    Ok(exit_code)
}
