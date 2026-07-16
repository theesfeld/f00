use chrono::{DateTime, Local};

use f00_core::{Config, Entry, IndicatorStyle};

use crate::color::Colorizer;
use crate::human::{block_display, human_size, human_size_si};
use crate::icons::icon_prefix;
use crate::perms::{classify_suffix, format_permissions};

/// Format a single long-listing line (no trailing newline).
pub fn format_long_line(
    entry: &Entry,
    colorizer: &Colorizer,
    config: &Config,
    widths: &LongWidths,
) -> String {
    let perms = format_permissions(entry);
    let nlink = entry.nlink.to_string();
    let owner = if config.show_owner {
        entry.owner_display(config.numeric_uid_gid)
    } else {
        String::new()
    };
    let group = if config.show_group {
        entry.group_display(config.numeric_uid_gid)
    } else {
        String::new()
    };

    let size = format_size_field(entry, config);
    let mtime = format_mtime(entry, config.full_time);
    let icon = icon_prefix(entry, config.icons);
    let suffix = classify_suffix(entry, config.indicator_style());

    let mut name = format!("{icon}{}{suffix}", entry.name);
    if let Some(target) = &entry.symlink_target {
        name = format!("{name} -> {}", target.display());
    }
    let name = colorizer.paint_name(entry, &name);

    let git = if config.show_git {
        entry
            .git_status
            .as_char()
            .map(|c| format!(" {} ", colorizer.paint_git_char(c)))
            .unwrap_or_else(|| "   ".to_string())
    } else {
        String::new()
    };

    let mut parts = Vec::new();
    if config.show_inode {
        parts.push(format!("{:>w$}", entry.inode, w = widths.inode));
    }
    if config.show_blocks {
        parts.push(format!(
            "{:>w$}",
            block_display(entry.blocks),
            w = widths.blocks
        ));
    }
    parts.push(perms);
    parts.push(format!("{:>w$}", nlink, w = widths.nlink));
    if config.show_owner {
        parts.push(format!("{:<w$}", owner, w = widths.owner));
    }
    if config.show_group {
        parts.push(format!("{:<w$}", group, w = widths.group));
    }
    parts.push(format!("{:>w$}", size, w = widths.size));
    parts.push(mtime);
    let prefix = parts.join(" ");
    format!("{prefix}{git} {name}")
}

fn format_size_field(entry: &Entry, config: &Config) -> String {
    if config.human_sizes {
        if config.si_sizes {
            human_size_si(entry.size)
        } else {
            human_size(entry.size)
        }
    } else if config.si_sizes {
        human_size_si(entry.size)
    } else {
        entry.size.to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LongWidths {
    pub inode: usize,
    pub blocks: usize,
    pub nlink: usize,
    pub owner: usize,
    pub group: usize,
    pub size: usize,
}

impl LongWidths {
    pub fn compute(entries: &[Entry], config: &Config) -> Self {
        let mut w = Self {
            inode: 1,
            blocks: 1,
            nlink: 1,
            owner: 1,
            group: 1,
            size: 1,
        };
        for e in entries.iter().filter(|e| !e.is_dir_header) {
            w.inode = w.inode.max(e.inode.to_string().len());
            w.blocks = w.blocks.max(block_display(e.blocks).to_string().len());
            w.nlink = w.nlink.max(e.nlink.to_string().len());
            if config.show_owner {
                w.owner = w.owner.max(e.owner_display(config.numeric_uid_gid).len());
            }
            if config.show_group {
                w.group = w.group.max(e.group_display(config.numeric_uid_gid).len());
            }
            w.size = w.size.max(format_size_field(e, config).len());
        }
        w
    }
}

/// Format many entries in long mode with aligned columns.
pub fn format_long(entries: &[Entry], colorizer: &Colorizer, config: &Config) -> String {
    let widths = LongWidths::compute(entries, config);
    let mut out = String::new();
    for entry in entries {
        if entry.is_dir_header {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("{}:\n", entry.path.display()));
            continue;
        }
        out.push_str(&format_long_line(entry, colorizer, config, &widths));
        out.push('\n');
    }
    out
}

/// Legacy signature used by older tests/callers.
pub fn format_long_simple(
    entries: &[Entry],
    colorizer: &Colorizer,
    human: bool,
    icons: bool,
    classify: bool,
) -> String {
    let mut config = Config {
        human_sizes: human,
        icons,
        show_git: false,
        show_owner: true,
        show_group: true,
        ..Config::default()
    };
    if classify {
        config.classify = true;
        config.indicator = IndicatorStyle::Classify;
    }
    format_long(entries, colorizer, &config)
}

/// Re-export-friendly wrapper matching prior API.
pub fn format_long_line_simple(
    entry: &Entry,
    colorizer: &Colorizer,
    human: bool,
    icons: bool,
    classify: bool,
    size_width: usize,
) -> String {
    let mut config = Config {
        human_sizes: human,
        icons,
        show_git: false,
        ..Config::default()
    };
    if classify {
        config.classify = true;
    }
    let widths = LongWidths {
        size: size_width,
        nlink: 1,
        owner: 1,
        group: 1,
        inode: 1,
        blocks: 1,
    };
    format_long_line(entry, colorizer, &config, &widths)
}

fn format_mtime(entry: &Entry, full_time: bool) -> String {
    match entry.modified_datetime() {
        Some(dt) if full_time => dt.format("%Y-%m-%d %H:%M:%S.%f %z").to_string(),
        Some(dt) => format_ls_time(dt),
        None => "            ".to_string(),
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
