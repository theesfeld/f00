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
pub use human::human_size;
pub use icons::{icon_for, icon_prefix};
pub use json::format_json;
pub use long::{format_long, format_long_line};
pub use perms::{classify_suffix, format_permissions};
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
    let multi = listings.len() > 1
        || listings.iter().any(|l| {
            l.root_is_dir
                && listings.len() > 1
        });

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
        // Print path header when multiple args or when useful for dirs.
        if multi && listing.root_is_dir {
            out.push_str(&format!("{}:\n", listing.root.display()));
        } else if listings.len() > 1 && listing.root_is_dir {
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
    let classify = config.classify;
    match config.effective_output() {
        OutputMode::Long => Ok(format_long(
            entries,
            colorizer,
            config.human_sizes,
            icons,
            classify,
        )),
        OutputMode::OnePerLine => Ok(format_one_per_line(entries, colorizer, icons, classify)),
        OutputMode::Json => format_json(entries).map_err(|e| e.to_string()),
        OutputMode::Tree => Ok(format_tree(entries, colorizer, icons, classify)),
        OutputMode::Default => Ok(format_columns(
            entries,
            colorizer,
            icons,
            classify,
            config.terminal_width,
        )),
    }
}
