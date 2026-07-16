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
    /// Natural / version sort (`-v`, strverscmp-like).
    Version,
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

/// When to show file-type icons (`--icons=WHEN`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconsWhen {
    #[default]
    Auto,
    Always,
    Never,
}

impl IconsWhen {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "always" | "yes" | "force" | "true" | "on" => Some(Self::Always),
            "never" | "no" | "none" | "false" | "off" => Some(Self::Never),
            _ => None,
        }
    }

    /// Icons on for `Always`, off for `Never`, and for `Auto` only when `is_tty`.
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
    /// Force multi-column column-major (`-C`).
    Columns,
    /// Multi-column row-major (`-x` / `--format=across`).
    Across,
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
    /// CSV rows.
    Csv,
    /// TSV rows.
    Tsv,
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

impl IndicatorStyle {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "none" => Some(Self::None),
            "slash" => Some(Self::Slash),
            "file-type" | "filetype" => Some(Self::FileType),
            "classify" => Some(Self::Classify),
            _ => None,
        }
    }
}

/// Filename quoting style (GNU `--quoting-style`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuotingStyle {
    /// No quoting (`-N` / `literal`).
    #[default]
    Literal,
    /// Locale-aware (treated like shell-escape for nongraphic).
    Locale,
    /// Quote when needed for the shell.
    Shell,
    /// Always single-quote.
    ShellAlways,
    /// Shell quoting with `$''` escapes for nongraphic.
    ShellEscape,
    /// Always use shell-escape style.
    ShellEscapeAlways,
    /// Double quotes with C escapes (`-Q` / `c`).
    C,
    /// C escapes without surrounding quotes (`-b` / `escape`).
    Escape,
}

impl QuotingStyle {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "literal" => Some(Self::Literal),
            "locale" => Some(Self::Locale),
            "shell" => Some(Self::Shell),
            "shell-always" => Some(Self::ShellAlways),
            "shell-escape" => Some(Self::ShellEscape),
            "shell-escape-always" => Some(Self::ShellEscapeAlways),
            "c" => Some(Self::C),
            "escape" => Some(Self::Escape),
            _ => None,
        }
    }

    /// From `QUOTING_STYLE` env var.
    pub fn from_env() -> Option<Self> {
        std::env::var("QUOTING_STYLE")
            .ok()
            .as_deref()
            .and_then(Self::parse)
    }
}

/// Whether to hide nongraphic characters as `?` (`-q` / `--show-control-chars`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlChars {
    /// Default: hide on TTY, show otherwise (GNU-ish).
    #[default]
    Auto,
    /// Replace nongraphic with `?` (`-q`).
    Hide,
    /// Show control chars as-is (`--show-control-chars`).
    Show,
}

/// Long-listing time style (`--time-style`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeStyle {
    /// Locale default (`Mon DD HH:MM` / `Mon DD  YYYY`).
    #[default]
    Locale,
    /// `%Y-%m-%d %H:%M:%S.%N %z` (full-iso / `--full-time`).
    FullIso,
    /// `%Y-%m-%d %H:%M`.
    LongIso,
    /// Recent: `%m-%d %H:%M`; older: `%Y-%m-%d`.
    Iso,
    /// Custom strftime format (`+FORMAT`).
    Format(String),
}

impl TimeStyle {
    pub fn parse(s: &str) -> Option<Self> {
        if let Some(fmt) = s.strip_prefix('+') {
            return Some(Self::Format(fmt.to_string()));
        }
        match s.to_ascii_lowercase().as_str() {
            "full-iso" | "full_iso" => Some(Self::FullIso),
            "long-iso" | "long_iso" => Some(Self::LongIso),
            "iso" => Some(Self::Iso),
            "locale" => Some(Self::Locale),
            _ => None,
        }
    }

    pub fn from_env() -> Option<Self> {
        std::env::var("TIME_STYLE")
            .ok()
            .as_deref()
            .and_then(Self::parse)
    }
}

/// Hyperlink (OSC 8) emission (`--hyperlink`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HyperlinkWhen {
    #[default]
    Never,
    Auto,
    Always,
}

impl HyperlinkWhen {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "always" | "yes" | "force" | "true" | "on" => Some(Self::Always),
            "never" | "no" | "none" | "false" | "off" => Some(Self::Never),
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

/// How to scale sizes / block counts (`--block-size`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockSize {
    /// Human binary (`-h`): powers of 1024 with unit suffix.
    HumanBinary,
    /// Human SI (`--si`): powers of 1000.
    HumanSi,
    /// Fixed divisor in bytes (e.g. 1024 for 1K, 1000 for KB).
    Bytes(u64),
}

impl Default for BlockSize {
    fn default() -> Self {
        // GNU default for long size field: 1-byte units.
        Self::Bytes(1)
    }
}

impl BlockSize {
    /// Parse GNU-style SIZE: `K`, `M`, `G`, `T`, `KB`, `MB`, `1K`, `1024`, etc.
    ///
    /// - Binary suffixes without `B` (`K`/`M`/`G`/`T`/`P`/`E`) are powers of 1024.
    /// - SI suffixes (`KB`/`MB`/…) are powers of 1000.
    /// - Bare number is bytes.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "human-readable" | "human" => return Some(Self::HumanBinary),
            "si" => return Some(Self::HumanSi),
            _ => {}
        }

        // Optional leading number, then optional unit.
        let (num_part, unit_part) = split_size_parts(s)?;
        let mult = if unit_part.is_empty() {
            1u64
        } else {
            parse_size_unit(unit_part)?
        };
        let n: u64 = if num_part.is_empty() {
            1
        } else {
            num_part.parse().ok()?
        };
        n.checked_mul(mult).map(Self::Bytes)
    }

    /// Divisor used when displaying allocated blocks (`-s`).
    /// Human modes fall back to 1024 (kibibytes).
    pub fn block_divisor(self) -> u64 {
        match self {
            Self::HumanBinary | Self::HumanSi => 1024,
            Self::Bytes(n) => n.max(1),
        }
    }
}

fn split_size_parts(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Allow a leading number or a bare unit.
    if i == 0 && !bytes[0].is_ascii_alphabetic() {
        return None;
    }
    Some((&s[..i], &s[i..]))
}

fn parse_size_unit(unit: &str) -> Option<u64> {
    let u = unit.to_ascii_lowercase();
    // Strip optional trailing `ib` / `b` for forms like KiB, MiB, KB.
    let (base, power_of_1000) = match u.as_str() {
        "k" | "ki" | "kib" => (1024u64, false),
        "m" | "mi" | "mib" => (1024u64.pow(2), false),
        "g" | "gi" | "gib" => (1024u64.pow(3), false),
        "t" | "ti" | "tib" => (1024u64.pow(4), false),
        "p" | "pi" | "pib" => (1024u64.pow(5), false),
        "e" | "ei" | "eib" => (1024u64.pow(6), false),
        "kb" => (1000u64, true),
        "mb" => (1000u64.pow(2), true),
        "gb" => (1000u64.pow(3), true),
        "tb" => (1000u64.pow(4), true),
        "pb" => (1000u64.pow(5), true),
        "eb" => (1000u64.pow(6), true),
        "b" => (1u64, true),
        _ => return None,
    };
    let _ = power_of_1000;
    Some(base)
}

/// How to follow symlinks for command-line arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CliSymlinkMode {
    /// Never follow CLI symlinks specially (use `follow_links` for all).
    #[default]
    Never,
    /// Follow all CLI path symlinks (`-H`).
    Always,
    /// Follow CLI symlinks only when they resolve to a directory.
    DirOnly,
}

/// Minimum number of directory children before parallel metadata is used.
pub const PARALLEL_STAT_THRESHOLD: usize = 32;

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
    /// Shell-style ignore patterns (`-I` / `--ignore`); always hidden.
    pub ignore_patterns: Vec<String>,
    /// Shell-style hide patterns (`--hide`); hidden unless `-a`/`-A`.
    pub hide_patterns: Vec<String>,
    /// When true, honor `.gitignore` / `.f00ignore` in listed directories.
    pub use_ignore_files: bool,
    /// When true, list inside zip/tar archives when a path is an archive file.
    /// (Applied by callers that integrate `f00-archive`; core itself does not open archives.)
    pub list_archives: bool,
    /// Which timestamp to sort/display by.
    pub time_field: TimeField,
    /// How CLI path arguments that are symlinks are handled.
    pub cli_symlink: CliSymlinkMode,
    /// Parallelize metadata (`stat`) for large directories (rayon).
    /// Forced off when [`Self::threads`] is `1`.
    pub parallel: bool,
    /// Rayon worker count: `0` = auto, `1` = force serial, `N>1` = fixed pool size.
    pub threads: usize,
    /// When true, fill [`crate::Listing::timing`] with phase durations.
    pub collect_timing: bool,
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
            hide_patterns: Vec::new(),
            use_ignore_files: false,
            list_archives: true,
            time_field: TimeField::Modified,
            cli_symlink: CliSymlinkMode::Never,
            parallel: true,
            threads: 0,
            collect_timing: false,
        }
    }
}

impl ListOptions {
    /// Whether metadata should use rayon for this listing size.
    pub fn use_parallel_stat(&self, entry_count: usize) -> bool {
        self.parallel && self.threads != 1 && entry_count > PARALLEL_STAT_THRESHOLD
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
    /// Terminal width used for multi-column layout (`-w`); `0` means unlimited.
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
    /// Filename quoting style.
    pub quoting_style: QuotingStyle,
    /// Control-character handling for names.
    pub control_chars: ControlChars,
    /// Show author column in long format (`--author`); usually same as owner.
    pub show_author: bool,
    /// Block size for size/`-s` display.
    pub block_size: BlockSize,
    /// Force 1024-byte blocks for `-s` (`-k` / `--kibibytes`).
    pub kibibytes: bool,
    /// Tab stop size (`-T` / `--tabsize`); stored for future column layout.
    pub tabsize: usize,
    /// OSC 8 hyperlinks for file names.
    pub hyperlink: HyperlinkWhen,
    /// Show SELinux security context (`-Z`).
    pub show_context: bool,
    /// Use NUL as line terminator (`--zero`).
    pub zero: bool,
    /// Emacs dired mode (`-D` / `--dired`).
    pub dired: bool,
    /// Long-listing time style.
    pub time_style: TimeStyle,
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
            quoting_style: QuotingStyle::Literal,
            control_chars: ControlChars::Auto,
            show_author: false,
            block_size: BlockSize::default(),
            kibibytes: false,
            tabsize: 8,
            hyperlink: HyperlinkWhen::Never,
            show_context: false,
            zero: false,
            dired: false,
            time_style: TimeStyle::Locale,
        }
    }
}

impl Config {
    pub fn color_enabled(&self) -> bool {
        self.color.enabled(self.is_stdout_tty)
    }

    pub fn hyperlink_enabled(&self) -> bool {
        self.hyperlink.enabled(self.is_stdout_tty)
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

    /// Effective block size for allocated-size display (`-s`).
    pub fn blocks_unit(&self) -> u64 {
        if self.kibibytes {
            return 1024;
        }
        self.block_size.block_divisor()
    }

    /// Line terminator (newline or NUL).
    pub fn line_ending(&self) -> &'static str {
        if self.zero {
            "\0"
        } else {
            "\n"
        }
    }

    /// Whether control characters should be replaced with `?`.
    pub fn hide_control_chars(&self) -> bool {
        match self.control_chars {
            ControlChars::Hide => true,
            ControlChars::Show => false,
            ControlChars::Auto => {
                self.is_stdout_tty
                    && matches!(
                        self.quoting_style,
                        QuotingStyle::Literal | QuotingStyle::Locale
                    )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_block_size_binary_and_si() {
        assert_eq!(BlockSize::parse("K"), Some(BlockSize::Bytes(1024)));
        assert_eq!(BlockSize::parse("1K"), Some(BlockSize::Bytes(1024)));
        assert_eq!(BlockSize::parse("M"), Some(BlockSize::Bytes(1024 * 1024)));
        assert_eq!(BlockSize::parse("KB"), Some(BlockSize::Bytes(1000)));
        assert_eq!(BlockSize::parse("MB"), Some(BlockSize::Bytes(1_000_000)));
        assert_eq!(BlockSize::parse("512"), Some(BlockSize::Bytes(512)));
        assert_eq!(
            BlockSize::parse("G"),
            Some(BlockSize::Bytes(1024u64.pow(3)))
        );
        assert_eq!(
            BlockSize::parse("T"),
            Some(BlockSize::Bytes(1024u64.pow(4)))
        );
        assert_eq!(
            BlockSize::parse("human-readable"),
            Some(BlockSize::HumanBinary)
        );
        assert_eq!(BlockSize::parse("si"), Some(BlockSize::HumanSi));
        assert_eq!(BlockSize::parse("bogus"), None);
    }

    #[test]
    fn parse_quoting_styles() {
        assert_eq!(QuotingStyle::parse("escape"), Some(QuotingStyle::Escape));
        assert_eq!(QuotingStyle::parse("c"), Some(QuotingStyle::C));
        assert_eq!(
            QuotingStyle::parse("shell-always"),
            Some(QuotingStyle::ShellAlways)
        );
        assert_eq!(QuotingStyle::parse("literal"), Some(QuotingStyle::Literal));
        assert_eq!(QuotingStyle::parse("nope"), None);
    }

    #[test]
    fn parse_time_styles() {
        assert_eq!(TimeStyle::parse("long-iso"), Some(TimeStyle::LongIso));
        assert_eq!(TimeStyle::parse("full-iso"), Some(TimeStyle::FullIso));
        assert_eq!(TimeStyle::parse("iso"), Some(TimeStyle::Iso));
        assert_eq!(
            TimeStyle::parse("+%Y"),
            Some(TimeStyle::Format("%Y".into()))
        );
    }

    #[test]
    fn parse_indicator_style() {
        assert_eq!(
            IndicatorStyle::parse("classify"),
            Some(IndicatorStyle::Classify)
        );
        assert_eq!(IndicatorStyle::parse("slash"), Some(IndicatorStyle::Slash));
        assert_eq!(
            IndicatorStyle::parse("file-type"),
            Some(IndicatorStyle::FileType)
        );
        assert_eq!(IndicatorStyle::parse("none"), Some(IndicatorStyle::None));
    }
}
