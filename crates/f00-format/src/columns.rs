use f00_core::{Config, Entry, IndicatorStyle};
use unicode_width::UnicodeWidthStr;

use crate::color::Colorizer;
use crate::human::block_display_with_unit;
use crate::hyperlink::hyperlink_name;
use crate::icons::icon_prefix;
use crate::perms::classify_suffix;
use crate::quoting::display_name;

struct Prepared {
    plain: String,
    painted: String,
    width: usize,
}

fn prepare_entries(entries: &[Entry], colorizer: &Colorizer, config: &Config) -> Vec<Prepared> {
    let icons = config.icons;
    let indicator = config.indicator_style();
    let hide_ctrl = config.hide_control_chars();
    let hyper = config.hyperlink_enabled();

    entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            let icon = icon_prefix(e, icons);
            let suffix = classify_suffix(e, indicator);
            let quoted = display_name(&e.name, config.quoting_style, hide_ctrl);
            let mut plain = format!("{icon}{quoted}{suffix}");
            if config.show_inode {
                plain = format!("{} {plain}", e.inode);
            }
            if config.show_blocks {
                let b = block_display_with_unit(e.blocks, config.blocks_unit());
                plain = format!("{b} {plain}");
            }
            if config.show_context {
                let ctx = if e.context.is_empty() {
                    "?"
                } else {
                    e.context.as_str()
                };
                plain = format!("{ctx} {plain}");
            }
            let width = UnicodeWidthStr::width(plain.as_str());
            let painted = colorizer.paint_name(e, &plain);
            let painted = hyperlink_name(&e.path, &painted, hyper);
            Prepared {
                plain,
                painted,
                width,
            }
        })
        .collect()
}

/// One entry per line.
pub fn format_one_per_line(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    indicator: IndicatorStyle,
) -> String {
    // Backward-compatible path without full Config.
    let mut config = Config {
        icons,
        indicator,
        classify: matches!(indicator, IndicatorStyle::Classify),
        ..Config::default()
    };
    if matches!(indicator, IndicatorStyle::Classify) {
        config.classify = true;
    }
    format_one_per_line_cfg(entries, colorizer, &config)
}

/// One entry per line with full config (quoting, zero, inode, etc.).
pub fn format_one_per_line_cfg(
    entries: &[Entry],
    colorizer: &Colorizer,
    config: &Config,
) -> String {
    let ending = config.line_ending();
    // Pre-size: ~48 bytes/line typical for short names.
    let mut out = String::with_capacity(entries.len().saturating_mul(48).saturating_add(64));
    let mut dired: Vec<(usize, usize)> = Vec::new();

    let write_total = |out: &mut String, section: &[Entry]| {
        // GNU `ls -s` prints `total N` for directory contents (even without `-l`).
        if !config.show_blocks || config.zero || !config.emit_block_total {
            return;
        }
        let total: u64 = section
            .iter()
            .filter(|e| !e.is_dir_header)
            .map(|e| crate::human::block_display_with_unit(e.blocks, config.blocks_unit()))
            .fold(0u64, u64::saturating_add);
        use std::fmt::Write as _;
        let _ = write!(out, "total {total}");
        out.push_str(ending);
    };

    let write_prepared = |out: &mut String, dired: &mut Vec<(usize, usize)>, p: &Prepared| {
        if config.dired {
            let start = out.len();
            out.push_str(&p.painted);
            let end = out.len();
            dired.push((start, end));
        } else {
            out.push_str(&p.painted);
        }
        out.push_str(ending);
    };

    let has_headers = entries.iter().any(|e| e.is_dir_header);
    if !has_headers {
        write_total(&mut out, entries);
        // Prepare once for the whole section (not per-entry).
        let prepared = prepare_entries(entries, colorizer, config);
        for p in &prepared {
            write_prepared(&mut out, &mut dired, p);
        }
    } else {
        let mut i = 0;
        while i < entries.len() {
            if entries[i].is_dir_header {
                if !out.is_empty() && !config.zero {
                    out.push('\n');
                }
                out.push_str(&format!("{}:", entries[i].path.display()));
                out.push_str(ending);
                i += 1;
            }
            let start = i;
            while i < entries.len() && !entries[i].is_dir_header {
                i += 1;
            }
            let section = &entries[start..i];
            write_total(&mut out, section);
            let prepared = prepare_entries(section, colorizer, config);
            for p in &prepared {
                write_prepared(&mut out, &mut dired, p);
            }
        }
    }

    if config.dired && !config.zero {
        out.push_str("//DIRED//");
        for (s, e) in &dired {
            out.push_str(&format!(" {s} {e}"));
        }
        out.push('\n');
    }
    out
}

/// Multi-column layout similar to `ls` default (column-major).
pub fn format_columns(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    indicator: IndicatorStyle,
    terminal_width: usize,
) -> String {
    let mut config = Config {
        icons,
        indicator,
        terminal_width,
        classify: matches!(indicator, IndicatorStyle::Classify),
        ..Config::default()
    };
    if matches!(indicator, IndicatorStyle::Classify) {
        config.classify = true;
    }
    format_columns_cfg(entries, colorizer, &config, false)
}

/// Multi-column with full config. `across` selects row-major (`-x`).
pub fn format_columns_cfg(
    entries: &[Entry],
    colorizer: &Colorizer,
    config: &Config,
    across: bool,
) -> String {
    let mut out = String::new();
    let mut section: Vec<&Entry> = Vec::new();
    let ending = config.line_ending();

    let flush = |section: &mut Vec<&Entry>, out: &mut String| {
        if section.is_empty() {
            return;
        }
        let owned: Vec<Entry> = section.iter().map(|e| (*e).clone()).collect();
        out.push_str(&format_columns_section(&owned, colorizer, config, across));
        section.clear();
    };

    for entry in entries {
        if entry.is_dir_header {
            flush(&mut section, &mut out);
            if !out.is_empty() && !config.zero {
                out.push('\n');
            }
            out.push_str(&format!("{}:", entry.path.display()));
            out.push_str(ending);
        } else {
            section.push(entry);
        }
    }
    flush(&mut section, &mut out);
    out
}

fn format_columns_section(
    entries: &[Entry],
    colorizer: &Colorizer,
    config: &Config,
    across: bool,
) -> String {
    let prepared = prepare_entries(entries, colorizer, config);
    if prepared.is_empty() {
        return String::new();
    }

    // Width 0 ⇒ unlimited (use a huge value).
    let term_w = if config.terminal_width == 0 {
        usize::MAX / 4
    } else {
        config.terminal_width.max(1)
    };
    let n = prepared.len();
    let max_w = prepared.iter().map(|p| p.width).max().unwrap_or(1);
    let col_gap = 2;
    let ending = config.line_ending();

    // With --zero, GNU falls back to one-per-line-ish; keep simple.
    if config.zero {
        let mut out = String::new();
        for p in &prepared {
            out.push_str(&p.painted);
            out.push_str(ending);
        }
        return out;
    }

    let max_cols = ((term_w + col_gap) / (1 + col_gap)).max(1).min(n);

    let mut best_cols = 1;
    let mut best_rows = n;
    let mut col_widths = vec![max_w];

    for cols in (1..=max_cols).rev() {
        let rows = n.div_ceil(cols);
        let mut widths = vec![0usize; cols];
        for (i, p) in prepared.iter().enumerate() {
            let col = if across { i % cols } else { i / rows };
            if col < cols {
                widths[col] = widths[col].max(p.width);
            }
        }
        let total: usize = widths.iter().sum::<usize>() + col_gap * cols.saturating_sub(1);
        if total <= term_w {
            best_cols = cols;
            best_rows = rows;
            col_widths = widths;
            break;
        }
    }

    let mut out = String::new();
    for row in 0..best_rows {
        for (col, col_width) in col_widths.iter().copied().enumerate().take(best_cols) {
            let idx = if across {
                row * best_cols + col
            } else {
                col * best_rows + row
            };
            if idx >= n {
                continue;
            }
            let p = &prepared[idx];
            out.push_str(&p.painted);
            if col + 1 < best_cols {
                let next_idx = if across {
                    row * best_cols + col + 1
                } else {
                    (col + 1) * best_rows + row
                };
                if next_idx < n {
                    let pad = col_width + col_gap - p.width;
                    out.push_str(&" ".repeat(pad));
                }
            }
        }
        out.push_str(ending);
    }
    let _ = &prepared.iter().map(|p| &p.plain);
    out
}
