//! Display formatting for **f00**: columns, long listing, tree, JSON, icons, color, quoting.

mod color;
mod columns;
mod csv;
mod human;
mod hyperlink;
mod icons;
mod json;
mod long;
mod perms;
mod quoting;
mod tree;

pub use color::Colorizer;
pub use columns::{
    format_columns, format_columns_cfg, format_one_per_line, format_one_per_line_cfg,
};
pub use csv::{format_csv, format_tsv};
pub use human::{
    block_display, block_display_with_unit, format_size_bytes, human_size, human_size_si,
};
pub use hyperlink::hyperlink_name;
pub use icons::{icon_for, icon_prefix};
pub use json::{colorize_json, format_json, format_json_pretty, strip_ansi};
pub use long::{
    format_long, format_long_line, format_long_line_simple, format_long_simple, format_time_style,
};
pub use perms::{classify_suffix, classify_suffix_bool, format_permissions};
pub use quoting::{display_name, quote_name};
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

    // JSON is a core surface: combine all entries into one array.
    // Pretty + syntax color when color mode is on; compact plain for pipes/scripts.
    if matches!(config.effective_output(), OutputMode::Json) {
        let mut all = Vec::new();
        for l in listings {
            all.extend(l.entries.iter().cloned());
        }
        return format_json(&all, config.color_enabled(), config.json_full)
            .map_err(|e| e.to_string());
    }

    // CSV/TSV: combine entries.
    if matches!(config.effective_output(), OutputMode::Csv | OutputMode::Tsv) {
        let mut all = Vec::new();
        for l in listings {
            all.extend(l.entries.iter().cloned());
        }
        return Ok(match config.effective_output() {
            OutputMode::Csv => format_csv(&all),
            OutputMode::Tsv => format_tsv(&all),
            _ => unreachable!(),
        });
    }

    let mut out = String::new();
    let ending = config.line_ending();
    for (i, listing) in listings.iter().enumerate() {
        if i > 0 {
            if !config.zero {
                out.push('\n');
            } else {
                // still separate with NUL if zero? GNU uses newline between dirs usually;
                // with --zero, entries are NUL-separated; headers still appear.
            }
        }
        // Print path header when multiple args list directory contents.
        // Skip for `-d` (single entry that is the directory itself).
        let is_dir_self = listing.root_is_dir
            && listing.entries.len() == 1
            && listing.entries[0].path == listing.root;
        if (multi || listings.len() > 1) && listing.root_is_dir && !is_dir_self {
            out.push_str(&format!("{}:", listing.root.display()));
            out.push_str(ending);
        }

        // GNU prints `total N` only for directory *contents*, not file operands / `-d`.
        let mut cfg = config.clone();
        cfg.emit_block_total = listing.root_is_dir && !is_dir_self;
        out.push_str(&format_entries(&listing.entries, &cfg, &colorizer)?);
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
        OutputMode::OnePerLine => Ok(format_one_per_line_cfg(entries, colorizer, config)),
        OutputMode::Commas => Ok(format_commas(entries, colorizer, config)),
        OutputMode::Json => format_json(entries, config.color_enabled(), config.json_full)
            .map_err(|e| e.to_string()),
        OutputMode::Tree => Ok(format_tree(entries, colorizer, icons, indicator)),
        OutputMode::Csv => Ok(format_csv(entries)),
        OutputMode::Tsv => Ok(format_tsv(entries)),
        OutputMode::Across => Ok(format_columns_cfg(entries, colorizer, config, true)),
        OutputMode::Default | OutputMode::Columns => {
            Ok(format_columns_cfg(entries, colorizer, config, false))
        }
    }
}

fn format_commas(entries: &[Entry], colorizer: &Colorizer, config: &Config) -> String {
    let hide_ctrl = config.hide_control_chars();
    let icons = config.icons;
    let indicator = config.indicator_style();
    let hyper = config.hyperlink_enabled();
    let mut parts = Vec::new();
    for entry in entries.iter().filter(|e| !e.is_dir_header) {
        let icon = icon_prefix(entry, icons);
        let suffix = classify_suffix(entry, indicator);
        let quoted = display_name(&entry.name, config.quoting_style, hide_ctrl);
        let plain = format!("{icon}{quoted}{suffix}");
        let painted = colorizer.paint_name(entry, &plain);
        parts.push(hyperlink_name(&entry.path, &painted, hyper));
    }
    let mut out = parts.join(", ");
    if !out.is_empty() {
        out.push_str(config.line_ending());
    }
    out
}
