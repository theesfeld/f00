use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};

/// f00 — a modern, friendly directory lister
#[derive(Debug, Parser)]
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
    #[arg(short = 'l', long = "long")]
    pub long: bool,

    /// List one file per line
    #[arg(short = '1')]
    pub one_per_line: bool,

    /// With -l, print human-readable sizes
    #[arg(short = 'h', long = "human-readable")]
    pub human_readable: bool,

    /// List subdirectories recursively
    #[arg(short = 'R', long = "recursive")]
    pub recursive: bool,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// Sort by time, newest first
    #[arg(short = 't', long = "time")]
    pub sort_time: bool,

    /// Sort by size
    #[arg(short = 'S', long = "size-sort")]
    pub sort_size: bool,

    /// Sort by extension
    #[arg(long = "sort-extension")]
    pub sort_extension: bool,

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
    #[arg(long = "tree")]
    pub tree: bool,

    /// Stricter GNU ls-compatible behavior (partial)
    #[arg(long = "gnu")]
    pub gnu: bool,

    /// Show file icons
    #[arg(long = "icons")]
    pub icons: bool,

    /// Append indicator (one of */=@|) to entries
    #[arg(short = 'F', long = "classify")]
    pub classify: bool,

    /// List directories before files
    #[arg(long = "dirs-first")]
    pub dirs_first: bool,

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
