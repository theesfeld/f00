use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};

/// f00 — a modern, friendly directory lister
#[derive(Debug, Clone, Parser)]
#[command(
    name = "f00",
    version,
    about = "List directory contents with a friendly default UI",
    long_about = None,
    // Free `-h` for human-readable (GNU ls style); keep `--help`.
    disable_help_flag = true
)]
pub struct Args {
    /// Paths to list (default: current directory)
    pub paths: Vec<PathBuf>,

    /// Print help
    #[arg(long = "help", action = ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Do not ignore entries starting with `.`
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Do not list implied `.` and `..` (show other hidden files)
    #[arg(short = 'A', long = "almost-all")]
    pub almost_all: bool,

    /// Use a long listing format
    #[arg(short = 'l')]
    pub long: bool,

    /// List one file per line
    #[arg(short = '1')]
    pub one_per_line: bool,

    /// List entries by columns
    #[arg(short = 'C')]
    pub columns: bool,

    /// Fill width with a comma separated list of entries
    #[arg(short = 'm')]
    pub commas: bool,

    /// With -l and -s, print sizes like 1K 234M 2G etc.
    #[arg(short = 'h', long = "human-readable")]
    pub human_readable: bool,

    /// Likewise, but use powers of 1000 not 1024
    #[arg(long = "si")]
    pub si: bool,

    /// List subdirectories recursively
    #[arg(short = 'R', long = "recursive")]
    pub recursive: bool,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// Sort by time, newest first
    #[arg(short = 't')]
    pub sort_time: bool,

    /// Sort by file size, largest first
    #[arg(short = 'S')]
    pub sort_size: bool,

    /// Sort alphabetically by entry extension
    #[arg(short = 'X')]
    pub sort_extension: bool,

    /// Natural sort of (version) numbers within text (`strverscmp`)
    #[arg(short = 'v')]
    pub sort_version: bool,

    /// Do not sort; list entries in directory order
    #[arg(short = 'U')]
    pub sort_none: bool,

    /// Sort by WORD (name, size, time, extension, version, none)
    #[arg(long = "sort", value_name = "WORD")]
    pub sort: Option<String>,

    /// Use time WORD for display/sort: mtime, atime, ctime, birth
    #[arg(long = "time", value_name = "WORD")]
    pub time: Option<String>,

    /// Sort by, and show, access time
    #[arg(short = 'u')]
    pub access_time: bool,

    /// Sort by, and show, ctime (status change) when possible
    #[arg(short = 'c')]
    pub change_time: bool,

    /// Colorize output
    #[arg(
        long = "color",
        value_name = "WHEN",
        default_value = "auto",
        num_args = 0..=1,
        default_missing_value = "always",
        require_equals = true
    )]
    pub color: ColorArg,

    /// Emit structured JSON (`-j` is free: GNU ls has no short `-j`)
    #[arg(short = 'j', long = "json")]
    pub json: bool,

    /// Emit CSV
    #[arg(long = "csv")]
    pub csv: bool,

    /// Emit TSV
    #[arg(long = "tsv")]
    pub tsv: bool,

    /// Show entries as a tree
    #[arg(long = "tree")]
    pub tree: bool,

    /// Stricter GNU ls-compatible behavior
    #[arg(long = "gnu")]
    pub gnu: bool,

    /// Show file icons (auto/always/never; default: auto — TTY only, off under --gnu)
    #[arg(
        long = "icons",
        value_name = "WHEN",
        default_value = "auto",
        num_args = 0..=1,
        default_missing_value = "always",
        require_equals = true
    )]
    pub icons: IconsArg,

    /// Append indicator (one of */=@|) to entries
    #[arg(short = 'F', long = "classify")]
    pub classify: bool,

    /// Append / indicator to directories
    #[arg(short = 'p')]
    pub indicator_slash: bool,

    /// Like -F, except do not append '*'
    #[arg(long = "file-type")]
    pub file_type: bool,

    /// Append indicator with style WORD: none, slash, file-type, classify
    #[arg(long = "indicator-style", value_name = "WORD")]
    pub indicator_style: Option<String>,

    /// Group directories before files
    #[arg(long = "group-directories-first", alias = "dirs-first")]
    pub dirs_first: bool,

    /// List directories themselves, not their contents
    #[arg(short = 'd', long = "directory")]
    pub directory: bool,

    /// Do not list implied entries ending with ~
    #[arg(short = 'B', long = "ignore-backups")]
    pub ignore_backups: bool,

    /// Do not list implied entries matching shell PATTERN (repeatable)
    #[arg(short = 'I', long = "ignore", value_name = "PATTERN", action = ArgAction::Append)]
    pub ignore: Vec<String>,

    /// Do not list implied entries matching shell PATTERN (unless -a/-A)
    #[arg(long = "hide", value_name = "PATTERN", action = ArgAction::Append)]
    pub hide: Vec<String>,

    /// When showing file information for a symbolic link, show info for the file it references
    #[arg(short = 'L', long = "dereference")]
    pub dereference: bool,

    /// Follow symbolic links listed on the command line
    #[arg(short = 'H', long = "dereference-command-line")]
    pub dereference_command_line: bool,

    /// Follow each command line symbolic link that points to a directory
    #[arg(long = "dereference-command-line-symlink-to-dir")]
    pub dereference_command_line_symlink_to_dir: bool,

    /// Like -l, but do not list owner
    #[arg(short = 'g')]
    pub no_owner: bool,

    /// Like -l, but do not list group information
    #[arg(short = 'o')]
    pub no_group_long: bool,

    /// In a long listing, don't print group names
    #[arg(short = 'G', long = "no-group")]
    pub no_group: bool,

    /// Like -l, but list numeric user and group IDs
    #[arg(short = 'n', long = "numeric-uid-gid")]
    pub numeric_uid_gid: bool,

    /// Print the index number of each file
    #[arg(short = 'i', long = "inode")]
    pub inode: bool,

    /// Print the allocated size of each file, in blocks
    #[arg(short = 's', long = "size")]
    pub size_blocks: bool,

    /// Default to 1024-byte blocks for filesystem disk usage (`-s`)
    #[arg(short = 'k', long = "kibibytes")]
    pub kibibytes: bool,

    /// Scale sizes by SIZE before printing (e.g. 1K, 1M, KB, MB)
    #[arg(long = "block-size", value_name = "SIZE")]
    pub block_size: Option<String>,

    /// Like -l --time-style=full-iso
    #[arg(long = "full-time")]
    pub full_time: bool,

    /// Time/date format with -l; also TIME_STYLE env
    #[arg(long = "time-style", value_name = "TIME_STYLE")]
    pub time_style: Option<String>,

    /// Print ? instead of nongraphic characters
    #[arg(short = 'q', long = "hide-control-chars")]
    pub hide_control_chars: bool,

    /// Show nongraphic characters as-is (the default unless -q)
    #[arg(long = "show-control-chars")]
    pub show_control_chars: bool,

    /// Print C-style escapes for nongraphic characters
    #[arg(short = 'b', long = "escape")]
    pub escape: bool,

    /// Enclose entry names in double quotes
    #[arg(short = 'Q', long = "quote-name")]
    pub quote_name: bool,

    /// Print entry names without quoting
    #[arg(short = 'N', long = "literal")]
    pub literal: bool,

    /// Use quoting style WORD for entry names
    #[arg(long = "quoting-style", value_name = "WORD")]
    pub quoting_style: Option<String>,

    /// List entries by lines instead of by columns (row-major)
    #[arg(short = 'x')]
    pub across: bool,

    /// Same as -a -U (and disable decorations in GNU mode)
    #[arg(short = 'f')]
    pub unsorted_all: bool,

    /// Across/commas/long/single-column/vertical
    #[arg(long = "format", value_name = "WORD")]
    pub format: Option<String>,

    /// Assume tab stops at each COLS instead of 8 (stored for layout)
    #[arg(short = 'T', long = "tabsize", value_name = "COLS")]
    pub tabsize: Option<usize>,

    /// Set output width to COLS; 0 means no limit
    #[arg(short = 'w', long = "width", value_name = "COLS")]
    pub width: Option<usize>,

    /// Print any security context of each file (SELinux)
    #[arg(short = 'Z', long = "context")]
    pub context: bool,

    /// End each output line with NUL, not newline
    #[arg(long = "zero")]
    pub zero: bool,

    /// Generate output designed for Emacs' dired mode
    #[arg(short = 'D', long = "dired")]
    pub dired: bool,

    /// With -l, print the author of each file
    #[arg(long = "author")]
    pub author: bool,

    /// Hyperlink file names (auto/always/never)
    #[arg(
        long = "hyperlink",
        value_name = "WHEN",
        num_args = 0..=1,
        default_missing_value = "always",
        require_equals = true
    )]
    pub hyperlink: Option<String>,

    /// Maximum recursion depth (with -R / --tree)
    #[arg(long = "max-depth", value_name = "N")]
    pub max_depth: Option<usize>,

    /// Annotate with git status (requires feature `git`)
    #[arg(long = "git", default_value_t = true, action = clap::ArgAction::Set)]
    pub git: bool,

    /// Path to TOML config file (overrides default search path)
    #[arg(long = "config", value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Interactive TUI directory browser (requires feature `tui`)
    #[arg(long = "browse", visible_alias = "tui")]
    pub browse: bool,

    /// Honor `.gitignore` / `.f00ignore` when listing
    #[arg(long = "ignore-files")]
    pub ignore_files: bool,

    /// Do not honor ignore files (overrides --ignore-files)
    #[arg(long = "no-ignore")]
    pub no_ignore: bool,

    /// Auto-list zip/tar archives as directories (default: true; off under --gnu)
    #[arg(long = "archive", default_value_t = true, action = clap::ArgAction::Set)]
    pub archive: bool,

    /// Parallel metadata threads: `0` = auto (rayon default), `1` = serial, `N>1` = fixed pool
    #[arg(long = "threads", value_name = "N", default_value_t = 0)]
    pub threads: usize,

    /// Print phase timing breakdown to stderr (readdir/stat/sort/format/total)
    #[arg(long = "profile")]
    pub profile: bool,

    /// Use io_uring batch metadata when built with `--features io-uring` (Linux)
    #[arg(long = "io-uring", action = ArgAction::Set, default_value_t = true)]
    pub io_uring: bool,

    /// Generate shell completions to stdout and exit (`bash`, `zsh`, `fish`, `powershell`, `elvish`)
    #[arg(long = "generate-completions", value_name = "SHELL", hide = true)]
    pub generate_completions: Option<clap_complete::Shell>,

    /// Generate a man page to stdout and exit
    #[arg(long = "generate-man", hide = true, action = ArgAction::SetTrue)]
    pub generate_man: bool,

    /// Update f00 from the latest GitHub Release (checksum verified)
    #[arg(long = "update", action = ArgAction::SetTrue)]
    pub update: bool,

    /// Check whether a newer release is available (no mutation); exit 1 if behind
    #[arg(long = "check-update", action = ArgAction::SetTrue)]
    pub check_update: bool,

    /// List loaded plugins (requires feature `plugins`)
    #[arg(long = "list-plugins", action = ArgAction::SetTrue)]
    pub list_plugins: bool,
}

impl Args {
    /// Minimal args for tests (all flags default/off).
    pub fn test_default() -> Self {
        Self {
            paths: vec![],
            help: None,
            all: false,
            almost_all: false,
            long: false,
            one_per_line: false,
            columns: false,
            commas: false,
            human_readable: false,
            si: false,
            recursive: false,
            reverse: false,
            sort_time: false,
            sort_size: false,
            sort_extension: false,
            sort_version: false,
            sort_none: false,
            sort: None,
            time: None,
            access_time: false,
            change_time: false,
            color: ColorArg::Auto,
            json: false,
            csv: false,
            tsv: false,
            tree: false,
            gnu: false,
            icons: IconsArg::Auto,
            classify: false,
            indicator_slash: false,
            file_type: false,
            indicator_style: None,
            dirs_first: false,
            directory: false,
            ignore_backups: false,
            ignore: vec![],
            hide: vec![],
            dereference: false,
            dereference_command_line: false,
            dereference_command_line_symlink_to_dir: false,
            no_owner: false,
            no_group_long: false,
            no_group: false,
            numeric_uid_gid: false,
            inode: false,
            size_blocks: false,
            kibibytes: false,
            block_size: None,
            full_time: false,
            time_style: None,
            hide_control_chars: false,
            show_control_chars: false,
            escape: false,
            quote_name: false,
            literal: false,
            quoting_style: None,
            across: false,
            unsorted_all: false,
            format: None,
            tabsize: None,
            width: None,
            context: false,
            zero: false,
            dired: false,
            author: false,
            hyperlink: None,
            max_depth: None,
            git: false,
            config: None,
            browse: false,
            ignore_files: false,
            no_ignore: false,
            archive: true,
            threads: 0,
            profile: false,
            io_uring: true,
            generate_completions: None,
            generate_man: false,
            update: false,
            check_update: false,
            list_plugins: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ColorArg {
    /// Color when stdout is a TTY (`tty` is the GNU ls synonym used by e.g. NixOS).
    #[default]
    #[value(alias = "tty")]
    Auto,
    #[value(alias = "yes", alias = "force", alias = "on", alias = "true")]
    Always,
    #[value(alias = "no", alias = "none", alias = "off", alias = "false")]
    Never,
}

impl From<ColorArg> for f00_core::ColorWhen {
    fn from(value: ColorArg) -> Self {
        match value {
            ColorArg::Auto => Self::Auto,
            ColorArg::Always => Self::Always,
            ColorArg::Never => Self::Never,
        }
    }
}

/// When to show file-type icons (mirrors [`f00_core::IconsWhen`]).
#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq, Eq)]
pub enum IconsArg {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<IconsArg> for f00_core::IconsWhen {
    fn from(value: IconsArg) -> Self {
        match value {
            IconsArg::Auto => Self::Auto,
            IconsArg::Always => Self::Always,
            IconsArg::Never => Self::Never,
        }
    }
}

impl From<f00_core::IconsWhen> for IconsArg {
    fn from(value: f00_core::IconsWhen) -> Self {
        match value {
            f00_core::IconsWhen::Auto => Self::Auto,
            f00_core::IconsWhen::Always => Self::Always,
            f00_core::IconsWhen::Never => Self::Never,
        }
    }
}
