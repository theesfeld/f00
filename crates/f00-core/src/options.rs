/// How entries should be ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortBy {
    #[default]
    Name,
    Size,
    Time,
    Extension,
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
    /// Force one entry per line.
    OnePerLine,
    /// Long listing (`-l`).
    Long,
    /// JSON array of entries.
    Json,
    /// Tree view.
    Tree,
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
    /// Stricter GNU-compatible behavior (partial).
    pub gnu_mode: bool,
    pub follow_links: bool,
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
    pub icons: bool,
    pub classify: bool,
    /// Terminal width used for multi-column layout.
    pub terminal_width: usize,
    pub is_stdout_tty: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            list: ListOptions::default(),
            output: OutputMode::Default,
            color: ColorWhen::Auto,
            human_sizes: false,
            icons: false,
            classify: false,
            terminal_width: 80,
            is_stdout_tty: false,
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
            other => other,
        }
    }
}
