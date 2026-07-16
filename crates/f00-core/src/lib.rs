//! Core filesystem listing, filtering, and sorting for **f00**.

mod entry;
mod error;
mod filter;
mod ignore;
#[cfg(target_os = "linux")]
mod linux_statx;
mod list;
mod options;
mod sort;

pub use entry::{Entry, EntryKind, GitStatus, MetaFill, TimeField};
pub use error::{Error, Result};
pub use filter::{filter_entries, glob_match, should_show};
pub use ignore::{apply_ignore_set, load_ignore_set, IgnoreSet, IGNORE_FILE_NAMES};
pub use list::{
    list_directory, list_path, list_paths, list_paths_with_errors, list_recursive, ListOutcome,
    ListTiming, Listing,
};
pub use options::{
    BlockSize, CliSymlinkMode, ColorWhen, Config, ControlChars, HyperlinkWhen, IconsWhen,
    IndicatorStyle, ListOptions, OutputMode, QuotingStyle, SortBy, TimeStyle,
    PARALLEL_STAT_THRESHOLD,
};
pub use sort::{cmp_name, cmp_name_with_mode, cmp_version, sort_entries};
