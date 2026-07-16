use f00_core::Entry;
use unicode_width::UnicodeWidthStr;

use crate::color::Colorizer;
use crate::icons::icon_prefix;
use crate::perms::classify_suffix;

struct Prepared {
    plain: String,
    painted: String,
    width: usize,
}

fn prepare_entries(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    classify: bool,
) -> Vec<Prepared> {
    entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            let icon = icon_prefix(e, icons);
            let suffix = classify_suffix(e, classify);
            let plain = format!("{icon}{}{suffix}", e.name);
            let width = UnicodeWidthStr::width(plain.as_str());
            let painted = colorizer.paint_name(e, &plain);
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
    classify: bool,
) -> String {
    let mut out = String::new();
    for entry in entries {
        if entry.is_dir_header {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("{}:\n", entry.path.display()));
            continue;
        }
        let icon = icon_prefix(entry, icons);
        let suffix = classify_suffix(entry, classify);
        let plain = format!("{icon}{}{suffix}", entry.name);
        out.push_str(&colorizer.paint_name(entry, &plain));
        out.push('\n');
    }
    out
}

/// Multi-column layout similar to `ls` default.
pub fn format_columns(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    classify: bool,
    terminal_width: usize,
) -> String {
    // Split into sections on dir headers for recursive mode.
    let mut out = String::new();
    let mut section: Vec<&Entry> = Vec::new();

    let flush = |section: &mut Vec<&Entry>, out: &mut String| {
        if section.is_empty() {
            return;
        }
        let owned: Vec<Entry> = section.iter().map(|e| (*e).clone()).collect();
        out.push_str(&format_columns_section(
            &owned,
            colorizer,
            icons,
            classify,
            terminal_width,
        ));
        section.clear();
    };

    for entry in entries {
        if entry.is_dir_header {
            flush(&mut section, &mut out);
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("{}:\n", entry.path.display()));
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
    icons: bool,
    classify: bool,
    terminal_width: usize,
) -> String {
    let prepared = prepare_entries(entries, colorizer, icons, classify);
    if prepared.is_empty() {
        return String::new();
    }

    let term_w = terminal_width.max(1);
    let n = prepared.len();
    let max_w = prepared.iter().map(|p| p.width).max().unwrap_or(1);
    let col_gap = 2;

    // Try from max columns down to 1; column-major like classic ls.
    let max_cols = ((term_w + col_gap) / (1 + col_gap)).max(1).min(n);

    let mut best_cols = 1;
    let mut best_rows = n;
    let mut col_widths = vec![max_w];

    for cols in (1..=max_cols).rev() {
        let rows = n.div_ceil(cols);
        let mut widths = vec![0usize; cols];
        for (i, p) in prepared.iter().enumerate() {
            let col = i / rows;
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
            let idx = col * best_rows + row;
            if idx >= n {
                continue;
            }
            let p = &prepared[idx];
            out.push_str(&p.painted);
            if col + 1 < best_cols {
                let next_idx = (col + 1) * best_rows + row;
                if next_idx < n {
                    let pad = col_width + col_gap - p.width;
                    out.push_str(&" ".repeat(pad));
                }
            }
        }
        out.push('\n');
    }
    // silence unused plain field warning by referencing in debug assert
    debug_assert!(prepared.iter().all(|p| !p.plain.is_empty() || true));
    out
}
