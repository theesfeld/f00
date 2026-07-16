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

    /// Do not sort; list entries in directory order
    #[arg(short = 'U')]
    pub sort_none: bool,

    /// Sort by WORD (name, size, time, extension, none)
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

    /// Emit structured JSON
    #[arg(long = "json")]
    pub json: bool,

    /// Show entries as a tree
    #[arg(long = "tree", visible_alias = "T")]
    pub tree: bool,

    /// Stricter GNU ls-compatible behavior
    #[arg(long = "gnu")]
    pub gnu: bool,

    /// Show file icons
    #[arg(long = "icons")]
    pub icons: bool,

    /// Append indicator (one of */=@|) to entries
    #[arg(short = 'F', long = "classify")]
    pub classify: bool,

    /// Append / indicator to directories
    #[arg(short = 'p')]
    pub indicator_slash: bool,

    /// Like -F, except do not append '*'
    #[arg(long = "file-type")]
    pub file_type: bool,

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

    /// When showing file information for a symbolic link, show info for the file it references
    #[arg(short = 'L', long = "dereference")]
    pub dereference: bool,

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

    /// Like -l --time-style=full-iso
    #[arg(long = "full-time")]
    pub full_time: bool,

    /// Across format (row-major columns); treated as multi-column
    #[arg(short = 'x')]
    pub across: bool,

    /// Same as -a -U (and disable decorations in GNU mode)
    #[arg(short = 'f')]
    pub unsorted_all: bool,

    /// Across/commas/long/single-column/vertical
    #[arg(long = "format", value_name = "WORD")]
    pub format: Option<String>,

    /// Maximum recursion depth (with -R / --tree)
    #[arg(long = "max-depth", value_name = "N")]
    pub max_depth: Option<usize>,

    /// Annotate with git status (requires feature `git`)
    #[arg(long = "git", default_value_t = true, action = clap::ArgAction::Set)]
    pub git: bool,

    /// Path to TOML config file (overrides default search path)
    #[arg(long = "config", value_name = "PATH")]
    pub config: Option<PathBuf>,
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
            sort_none: false,
            sort: None,
            time: None,
            access_time: false,
            change_time: false,
            color: ColorArg::Auto,
            json: false,
            tree: false,
            gnu: false,
            icons: false,
            classify: false,
            indicator_slash: false,
            file_type: false,
            dirs_first: false,
            directory: false,
            ignore_backups: false,
            ignore: vec![],
            dereference: false,
            no_owner: false,
            no_group_long: false,
            no_group: false,
            numeric_uid_gid: false,
            inode: false,
            size_blocks: false,
            full_time: false,
            across: false,
            unsorted_all: false,
            format: None,
            max_depth: None,
            git: false,
            config: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ColorArg {
    #[default]
    Auto,
    Always,
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
