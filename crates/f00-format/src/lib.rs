//! Display formatting for **f00**: columns, long listing, tree, JSON, icons, color.

mod color;
mod columns;
mod human;
mod icons;
mod json;
mod long;
mod perms;
mod tree;

pub use color::Colorizer;
pub use columns::{format_columns, format_one_per_line};
pub use human::{block_display, human_size, human_size_si};
pub use icons::{icon_for, icon_prefix};
pub use json::format_json;
pub use long::{format_long, format_long_line, format_long_line_simple, format_long_simple};
pub use perms::{classify_suffix, classify_suffix_bool, format_permissions};
pub use tree::format_tree;

use f00_core::{Config, Entry, Listing, OutputMode};

/// Format a single listing according to config.
pub fn format_listing(listing: &Listing, config: &Config) -> std::result::Result<String, String> {
    let colorizer = Colorizer::new(config.color_enabled());
    format_entries(&listing.entries, config, &colorizer)
}

/// Format multiple listings (multiple path arguments).
pub fn format_listings(
    listings: &[Listing],
    config: &Config,
) -> std::result::Result<String, String> {
    let colorizer = Colorizer::new(config.color_enabled());
    let multi = listings.len() > 1 || listings.iter().any(|l| l.root_is_dir && listings.len() > 1);

    // JSON: combine all entries into one array.
    if matches!(config.effective_output(), OutputMode::Json) {
        let mut all = Vec::new();
        for l in listings {
            all.extend(l.entries.iter().cloned());
        }
        return format_json(&all).map_err(|e| e.to_string());
    }

    let mut out = String::new();
    for (i, listing) in listings.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        // Print path header when multiple args list directory contents.
        // Skip for `-d` (single entry that is the directory itself).
        let is_dir_self = listing.root_is_dir
            && listing.entries.len() == 1
            && listing.entries[0].path == listing.root;
        if (multi || listings.len() > 1) && listing.root_is_dir && !is_dir_self {
            out.push_str(&format!("{}:\n", listing.root.display()));
        }

        out.push_str(&format_entries(&listing.entries, config, &colorizer)?);
    }
    Ok(out)
}

fn format_entries(
    entries: &[Entry],
    config: &Config,
    colorizer: &Colorizer,
) -> std::result::Result<String, String> {
    let icons = config.icons;
    let indicator = config.indicator_style();
    match config.effective_output() {
        OutputMode::Long => Ok(format_long(entries, colorizer, config)),
        OutputMode::OnePerLine => Ok(format_one_per_line(entries, colorizer, icons, indicator)),
        OutputMode::Commas => Ok(format_commas(entries, colorizer, icons, indicator)),
        OutputMode::Json => format_json(entries).map_err(|e| e.to_string()),
        OutputMode::Tree => Ok(format_tree(entries, colorizer, icons, indicator)),
        OutputMode::Default | OutputMode::Columns => Ok(format_columns(
            entries,
            colorizer,
            icons,
            indicator,
            config.terminal_width,
        )),
    }
}

fn format_commas(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    indicator: f00_core::IndicatorStyle,
) -> String {
    let mut parts = Vec::new();
    for entry in entries.iter().filter(|e| !e.is_dir_header) {
        let icon = icon_prefix(entry, icons);
        let suffix = classify_suffix(entry, indicator);
        let plain = format!("{icon}{}{suffix}", entry.name);
        parts.push(colorizer.paint_name(entry, &plain));
    }
    let mut out = parts.join(", ");
    if !out.is_empty() {
        out.push('\n');
    }
    out
}
