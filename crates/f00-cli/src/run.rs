use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use f00_compat::{apply_gnu_list_options, apply_gnu_output, parse_format_word, parse_sort_word};
#[cfg(feature = "archives")]
use f00_core::list_path;
use f00_core::{
    list_paths_with_errors, BlockSize, CliSymlinkMode, Config, ControlChars, HyperlinkWhen,
    IndicatorStyle, ListOptions, ListOutcome, ListTiming, OutputMode, QuotingStyle, SortBy,
    TimeField, TimeStyle,
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
    } else if args.sort_version {
        SortBy::Version
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
    if args.csv {
        OutputMode::Csv
    } else if args.tsv {
        OutputMode::Tsv
    } else if args.json {
        OutputMode::Json
    } else if args.tree {
        OutputMode::Tree
    } else if args.long
        || args.no_owner
        || args.no_group_long
        || args.numeric_uid_gid
        || args.author
    {
        // -g/-o/-n/--author imply long format in GNU ls
        OutputMode::Long
    } else if args.one_per_line || args.zero {
        // --zero implies -1 in GNU ls
        OutputMode::OnePerLine
    } else if args.commas {
        OutputMode::Commas
    } else if args.across {
        OutputMode::Across
    } else if args.columns {
        OutputMode::Columns
    } else {
        OutputMode::Default
    }
}

fn resolve_indicator(args: &Args, is_tty: bool) -> IndicatorStyle {
    if let Some(ref word) = args.indicator_style {
        if let Some(s) = IndicatorStyle::parse(word) {
            return s;
        }
    }
    // GNU: `-F` / `--classify[=WHEN]` — WHEN uses the same vocabulary as `--color`.
    if let Some(when) = args.classify {
        let when = f00_core::ColorWhen::from(when);
        if when.enabled(is_tty) {
            return IndicatorStyle::Classify;
        }
        // `--classify=never` or auto on non-TTY: do not classify (still honor -p / --file-type).
    }
    if args.file_type {
        IndicatorStyle::FileType
    } else if args.indicator_slash {
        IndicatorStyle::Slash
    } else {
        IndicatorStyle::None
    }
}

fn resolve_quoting(args: &Args) -> QuotingStyle {
    if args.literal {
        return QuotingStyle::Literal;
    }
    if args.escape {
        return QuotingStyle::Escape;
    }
    if args.quote_name {
        return QuotingStyle::C;
    }
    if let Some(ref word) = args.quoting_style {
        if let Some(s) = QuotingStyle::parse(word) {
            return s;
        }
    }
    QuotingStyle::from_env().unwrap_or(QuotingStyle::Literal)
}

fn resolve_control_chars(args: &Args) -> ControlChars {
    if args.hide_control_chars {
        ControlChars::Hide
    } else if args.show_control_chars {
        ControlChars::Show
    } else {
        ControlChars::Auto
    }
}

fn resolve_time_style(args: &Args) -> TimeStyle {
    if args.full_time {
        return TimeStyle::FullIso;
    }
    if let Some(ref s) = args.time_style {
        if let Some(ts) = TimeStyle::parse(s) {
            return ts;
        }
    }
    TimeStyle::from_env().unwrap_or(TimeStyle::Locale)
}

fn resolve_block_size(args: &Args) -> BlockSize {
    if let Some(ref s) = args.block_size {
        if let Some(bs) = BlockSize::parse(s) {
            return bs;
        }
    }
    if args.human_readable {
        BlockSize::HumanBinary
    } else if args.si {
        BlockSize::HumanSi
    } else {
        BlockSize::Bytes(1)
    }
}

fn resolve_hyperlink(args: &Args) -> HyperlinkWhen {
    match args.hyperlink {
        None => HyperlinkWhen::Never,
        Some(crate::cli::ColorArg::Auto) => HyperlinkWhen::Auto,
        Some(crate::cli::ColorArg::Always) => HyperlinkWhen::Always,
        Some(crate::cli::ColorArg::Never) => HyperlinkWhen::Never,
    }
}

fn resolve_cli_symlink(args: &Args) -> CliSymlinkMode {
    if args.dereference {
        // -L supersedes -H
        return CliSymlinkMode::Never; // handled via follow_links
    }
    if args.dereference_command_line {
        CliSymlinkMode::Always
    } else if args.dereference_command_line_symlink_to_dir {
        CliSymlinkMode::DirOnly
    } else {
        CliSymlinkMode::Never
    }
}

fn resolve_width(args: &Args) -> usize {
    if let Some(w) = args.width {
        return w;
    }
    terminal_width()
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
        // Honor explicit `--group-directories-first` even under `--gnu` (GNU does).
        dirs_first: args.dirs_first,
        recursive: (args.recursive || args.tree) && !args.directory,
        max_depth: args.max_depth,
        gnu_mode: args.gnu,
        follow_links: args.dereference,
        directory: args.directory,
        ignore_backups: args.ignore_backups,
        ignore_patterns: args.ignore.clone(),
        hide_patterns: args.hide.clone(),
        use_ignore_files: args.ignore_files && !args.no_ignore && !args.gnu,
        list_archives: args.archive && !args.gnu && !args.directory,
        time_field: resolve_time_field(args),
        cli_symlink: resolve_cli_symlink(args),
        // `--threads 1` forces serial; `0` = auto rayon; `N>1` = fixed pool.
        parallel: args.threads != 1,
        threads: args.threads,
        collect_timing: args.profile,
        // Filled after we know output mode (long / -Z need expensive fields).
        resolve_owner_group: false,
        read_selinux: false,
        linux_statx: true,
        io_uring: cfg!(feature = "io-uring"),
        // Adjusted below for `--tree` (headers not needed).
        emit_dir_headers: true,
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
        f00_core::IconsWhen::from(args.icons).enabled(is_stdout_tty)
    };
    let show_git = args.git && !args.gnu && !args.unsorted_all;

    // -g implies long without owner; -o implies long without group.
    let show_owner = !args.no_owner;
    let show_group = !(args.no_group || args.no_group_long);

    if args.full_time && !matches!(output, OutputMode::Long) {
        output = OutputMode::Long;
    }
    // -Z alone often implies long in some ls builds; we show context in all modes.
    if args.dired && !matches!(output, OutputMode::Long | OutputMode::OnePerLine) {
        // dired is most useful with long; keep chosen mode otherwise.
    }

    // Expensive metadata only when the presentation needs it.
    // JSON/CSV/TSV are machine dumps: resolve owner/group names unless `-n`.
    let machine = matches!(output, OutputMode::Json | OutputMode::Csv | OutputMode::Tsv);
    let needs_names = (matches!(output, OutputMode::Long)
        && ((show_owner && !args.numeric_uid_gid) || (show_group && !args.numeric_uid_gid)))
        || (machine && !args.numeric_uid_gid);
    list.resolve_owner_group = needs_names || args.author;
    list.read_selinux = args.context;
    list.linux_statx = true;
    list.io_uring = args.io_uring && cfg!(feature = "io-uring");
    // Tree does not use section headers; skip them for less work and cleaner depths.
    list.emit_dir_headers = !matches!(output, OutputMode::Tree);

    let _ = (&mut all, &mut almost_all);

    // --zero disables color and hyperlinks in GNU ls.
    let color = if args.zero {
        f00_core::ColorWhen::Never
    } else {
        args.color.into()
    };

    let mut hyperlink = resolve_hyperlink(args);
    if args.zero {
        hyperlink = HyperlinkWhen::Never;
    }

    let indicator = resolve_indicator(args, is_stdout_tty);

    Config {
        list,
        output,
        color,
        human_sizes: args.human_readable || args.si,
        si_sizes: args.si,
        icons,
        classify: matches!(indicator, IndicatorStyle::Classify),
        indicator,
        terminal_width: resolve_width(args),
        is_stdout_tty,
        show_owner,
        show_group,
        numeric_uid_gid: args.numeric_uid_gid,
        show_inode: args.inode,
        show_blocks: args.size_blocks,
        full_time: args.full_time,
        show_git,
        quoting_style: resolve_quoting(args),
        control_chars: resolve_control_chars(args),
        show_author: args.author,
        block_size: resolve_block_size(args),
        kibibytes: args.kibibytes,
        tabsize: args.tabsize.unwrap_or(8),
        hyperlink,
        show_context: args.context,
        zero: args.zero,
        dired: args.dired,
        time_style: resolve_time_style(args),
        // Per-listing override in format_listings for file operands.
        emit_block_total: true,
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
        args.icons = crate::cli::IconsArg::Never;
        // Do **not** clear `dirs_first` here: `--group-directories-first` must
        // still work in GNU mode (coreutils accepts it). Default stays off.
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

    // Interactive browser (feature-gated).
    if args.browse {
        #[cfg(feature = "tui")]
        {
            let start = args
                .paths
                .first()
                .map(PathBuf::as_path)
                .unwrap_or_else(|| std::path::Path::new("."));
            let is_tty = io::stdout().is_terminal();
            let icons = !args.gnu && f00_core::IconsWhen::from(args.icons).enabled(is_tty);
            let code = f00_tui::run_browser(
                start,
                f00_tui::BrowserOptions {
                    show_hidden: args.almost_all || args.all,
                    icons,
                    git: args.git && !args.gnu,
                },
            )?;
            return Ok(code);
        }
        #[cfg(not(feature = "tui"))]
        {
            anyhow::bail!("TUI browser requires building with --features tui");
        }
    }

    let config = build_config(&args);
    let paths: Vec<PathBuf> = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let t_total = Instant::now();
    let outcome = list_paths_with_archives(&paths, &config.list);
    let core_timing = outcome.total_timing();

    for err in &outcome.path_errors {
        eprintln!("f00: {err}");
    }

    let exit_code = outcome.exit_code();
    #[cfg(feature = "git")]
    let listings = {
        let mut listings = outcome.listings;
        if config.show_git && args.git {
            f00_git::annotate_listings(&mut listings);
        }
        #[cfg(feature = "plugins")]
        {
            crate::plugins_cmd::decorate_listings(listings)
        }
        #[cfg(not(feature = "plugins"))]
        {
            listings
        }
    };
    #[cfg(all(not(feature = "git"), feature = "plugins"))]
    let listings = crate::plugins_cmd::decorate_listings(outcome.listings);
    #[cfg(all(not(feature = "git"), not(feature = "plugins")))]
    let listings = outcome.listings;

    let mut format_ms = 0u128;
    if !listings.is_empty() {
        let t_fmt = Instant::now();
        let rendered = format_listings(&listings, &config).map_err(|e| anyhow::anyhow!(e))?;
        format_ms = t_fmt.elapsed().as_millis();

        let mut stdout = io::stdout().lock();
        stdout
            .write_all(rendered.as_bytes())
            .context("writing output")?;
    } else if exit_code == 0 {
        eprintln!("f00: no listings produced");
        return Ok(2);
    }

    if args.profile {
        print_profile(&core_timing, format_ms, t_total.elapsed().as_millis());
    }

    Ok(exit_code)
}

fn print_profile(core: &ListTiming, format_ms: u128, total_ms: u128) {
    eprintln!(
        "f00 profile: readdir_ms={} stat_ms={} sort_ms={} format_ms={} total_ms={}",
        core.readdir_ms, core.stat_ms, core.sort_ms, format_ms, total_ms
    );
}

/// Like [`list_paths_with_errors`], but expands zip/tar when `list_archives` is set.
fn list_paths_with_archives(paths: &[PathBuf], opts: &ListOptions) -> ListOutcome {
    #[cfg(feature = "archives")]
    {
        if opts.list_archives {
            let mut listings = Vec::new();
            let mut path_errors = Vec::new();
            let mut minor = 0usize;
            for p in paths {
                if f00_archive::is_archive(p) {
                    match f00_archive::list_archive_as_listing(p) {
                        Ok(l) => {
                            minor += l.minor_errors;
                            listings.push(l);
                        }
                        Err(e) => path_errors.push(f00_core::Error::Io(std::io::Error::other(
                            format!("{}: {e}", p.display()),
                        ))),
                    }
                } else {
                    match list_path(p, opts) {
                        Ok(l) => {
                            minor += l.minor_errors;
                            listings.push(l);
                        }
                        Err(e) => path_errors.push(e),
                    }
                }
            }
            return ListOutcome {
                listings,
                path_errors,
                minor_errors: minor,
            };
        }
    }
    let _ = opts.list_archives;
    list_paths_with_errors(paths, opts)
}
