//! Core filesystem listing, filtering, and sorting for **f00**.

mod entry;
mod error;
mod filter;
mod list;
mod options;
mod sort;

pub use entry::{Entry, EntryKind, GitStatus, TimeField};
pub use error::{Error, Result};
pub use filter::{filter_entries, glob_match, should_show};
pub use list::{
    list_directory, list_path, list_paths, list_paths_with_errors, list_recursive, ListOutcome,
    Listing,
};
pub use options::{ColorWhen, Config, IndicatorStyle, ListOptions, OutputMode, SortBy};
pub use sort::{cmp_name, cmp_name_with_mode, sort_entries};
