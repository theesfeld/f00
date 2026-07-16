use chrono::{DateTime, Local};

use f00_core::Entry;

use crate::color::Colorizer;
use crate::human::human_size;
use crate::icons::icon_prefix;
use crate::perms::{classify_suffix, format_permissions};

/// Format a single long-listing line (no trailing newline).
pub fn format_long_line(
    entry: &Entry,
    colorizer: &Colorizer,
    human: bool,
    icons: bool,
    classify: bool,
    size_width: usize,
) -> String {
    let perms = format_permissions(entry);
    let size = if human {
        human_size(entry.size)
    } else {
        entry.size.to_string()
    };
    let mtime = format_mtime(entry);
    let icon = icon_prefix(entry, icons);
    let suffix = classify_suffix(entry, classify);

    let mut name = format!("{icon}{}{suffix}", entry.name);
    if let Some(target) = &entry.symlink_target {
        name = format!("{name} -> {}", target.display());
    }
    let name = colorizer.paint_name(entry, &name);

    let git = entry
        .git_status
        .as_char()
        .map(|c| format!(" {} ", colorizer.paint_git_char(c)))
        .unwrap_or_else(|| "   ".to_string());

    format!("{perms}{git}{size:>width$} {mtime} {name}", width = size_width)
}

/// Format many entries in long mode with aligned size column.
pub fn format_long(
    entries: &[Entry],
    colorizer: &Colorizer,
    human: bool,
    icons: bool,
    classify: bool,
) -> String {
    let size_width = entries
        .iter()
        .filter(|e| !e.is_dir_header)
        .map(|e| {
            if human {
                human_size(e.size).len()
            } else {
                e.size.to_string().len()
            }
        })
        .max()
        .unwrap_or(1)
        .max(1);

    let mut out = String::new();
    for entry in entries {
        if entry.is_dir_header {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("{}:\n", entry.path.display()));
            continue;
        }
        out.push_str(&format_long_line(
            entry, colorizer, human, icons, classify, size_width,
        ));
        out.push('\n');
    }
    out
}

fn format_mtime(entry: &Entry) -> String {
    match entry.modified_datetime() {
        Some(dt) => format_ls_time(dt),
        None => "            ".to_string(), // 12 chars
    }
}

/// Approximate GNU ls time format: `Mon DD HH:MM` or `Mon DD  YYYY` if old.
fn format_ls_time(dt: DateTime<Local>) -> String {
    let now = Local::now();
    let six_months = chrono::Duration::days(365 / 2);
    if (now - dt).abs() > six_months {
        dt.format("%b %e  %Y").to_string()
    } else {
        dt.format("%b %e %H:%M").to_string()
    }
}
