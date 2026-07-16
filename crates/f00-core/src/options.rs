use crate::entry::TimeField;

/// How entries should be ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortBy {
    #[default]
    Name,
    /// Largest first (GNU `ls -S`).
    Size,
    /// Newest first (GNU `ls -t`).
    Time,
    Extension,
    /// Directory order / no sort (`-U`).
    None,
}

/// Color when to emit ANSI sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorWhen {
    #[default]
    Auto,
    Always,
    Never,
}

impl ColorWhen {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "always" | "yes" | "force" => Some(Self::Always),
            "never" | "no" | "none" => Some(Self::Never),
            _ => None,
        }
    }

    pub fn enabled(self, is_tty: bool) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => is_tty,
        }
    }
}

/// How to present the listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputMode {
    /// Multi-column when TTY, otherwise one-per-line.
    #[default]
    Default,
    /// Force multi-column (`-C`).
    Columns,
    /// Force one entry per line (`-1`).
    OnePerLine,
    /// Long listing (`-l`).
    Long,
    /// Comma-separated (`-m`).
    Commas,
    /// JSON array of entries.
    Json,
    /// Tree view.
    Tree,
}

/// Indicator style (`ls -F` / `-p` / `--indicator-style`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndicatorStyle {
    #[default]
    None,
    /// Append `/` to directories only (`-p`).
    Slash,
    /// Classify (`-F`): `*/=@|`.
    Classify,
    /// Like classify without `*` (`--file-type`).
    FileType,
}

/// Options controlling which entries appear and how they are ordered.
#[derive(Debug, Clone)]
pub struct ListOptions {
    pub all: bool,
    pub almost_all: bool,
    pub sort_by: SortBy,
    pub reverse: bool,
    pub dirs_first: bool,
    pub recursive: bool,
    pub max_depth: Option<usize>,
    /// Stricter GNU-compatible behavior.
    pub gnu_mode: bool,
    /// Follow symlinks when stating (`-L`).
    pub follow_links: bool,
    /// List directories themselves, not contents (`-d`).
    pub directory: bool,
    /// Hide `*~` backup names (`-B`).
    pub ignore_backups: bool,
    /// Shell-style ignore patterns (`-I` / `--ignore`); multiple allowed.
    pub ignore_patterns: Vec<String>,
    /// Which timestamp to sort/display by.
    pub time_field: TimeField,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            all: false,
            almost_all: false,
            sort_by: SortBy::Name,
            reverse: false,
            dirs_first: false,
            recursive: false,
            max_depth: None,
            gnu_mode: false,
            follow_links: false,
            directory: false,
            ignore_backups: false,
            ignore_patterns: Vec::new(),
            time_field: TimeField::Modified,
        }
    }
}

/// Full runtime configuration combining listing and presentation.
#[derive(Debug, Clone)]
pub struct Config {
    pub list: ListOptions,
    pub output: OutputMode,
    pub color: ColorWhen,
    pub human_sizes: bool,
    /// SI units (powers of 1000) instead of 1024 (`--si`).
    pub si_sizes: bool,
    pub icons: bool,
    pub classify: bool,
    pub indicator: IndicatorStyle,
    /// Terminal width used for multi-column layout.
    pub terminal_width: usize,
    pub is_stdout_tty: bool,
    pub show_owner: bool,
    pub show_group: bool,
    pub numeric_uid_gid: bool,
    pub show_inode: bool,
    pub show_blocks: bool,
    pub full_time: bool,
    /// When true, suppress git decorations in format (also controlled by CLI).
    pub show_git: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list: ListOptions::default(),
            output: OutputMode::Default,
            color: ColorWhen::Auto,
            human_sizes: false,
            si_sizes: false,
            icons: false,
            classify: false,
            indicator: IndicatorStyle::None,
            terminal_width: 80,
            is_stdout_tty: false,
            show_owner: true,
            show_group: true,
            numeric_uid_gid: false,
            show_inode: false,
            show_blocks: false,
            full_time: false,
            show_git: false,
        }
    }
}

impl Config {
    pub fn color_enabled(&self) -> bool {
        self.color.enabled(self.is_stdout_tty)
    }

    /// Resolve default output mode: multi-column on TTY, one-per-line otherwise.
    pub fn effective_output(&self) -> OutputMode {
        match self.output {
            OutputMode::Default if !self.is_stdout_tty => OutputMode::OnePerLine,
            OutputMode::Default if self.is_stdout_tty => OutputMode::Columns,
            other => other,
        }
    }

    pub fn indicator_style(&self) -> IndicatorStyle {
        if self.classify {
            IndicatorStyle::Classify
        } else {
            self.indicator
        }
    }
}
