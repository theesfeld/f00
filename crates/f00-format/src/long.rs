use chrono::{DateTime, Local};

use f00_core::{Config, Entry, IndicatorStyle, TimeStyle};

use crate::color::Colorizer;
use crate::human::{block_display_with_unit, format_size_bytes};
use crate::hyperlink::hyperlink_name;
use crate::icons::icon_prefix;
use crate::perms::{classify_suffix, format_permissions};
use crate::quoting::display_name;

/// Format a single long-listing line (no trailing newline).
///
/// When `dired` is enabled, `name_offsets` receives `(start, end)` byte offsets
/// of the raw (pre-color) name within the returned line string.
pub fn format_long_line(
    entry: &Entry,
    colorizer: &Colorizer,
    config: &Config,
    widths: &LongWidths,
) -> String {
    format_long_line_dired(entry, colorizer, config, widths, None)
}

/// Like [`format_long_line`] but records dired name offsets into the full output.
pub fn format_long_line_dired(
    entry: &Entry,
    colorizer: &Colorizer,
    config: &Config,
    widths: &LongWidths,
    dired_offsets: Option<&mut Vec<(usize, usize)>>,
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
    let author = if config.show_author {
        entry.author_display(config.numeric_uid_gid)
    } else {
        String::new()
    };

    let size = format_size_field(entry, config);
    let mtime = format_entry_time(entry, config);
    let icon = icon_prefix(entry, config.icons);
    let suffix = classify_suffix(entry, config.indicator_style());

    let quoted = display_name(
        &entry.name,
        config.quoting_style,
        config.hide_control_chars(),
    );
    let mut name_plain = format!("{icon}{quoted}{suffix}");
    if let Some(target) = &entry.symlink_target {
        let tq = display_name(
            &target.display().to_string(),
            config.quoting_style,
            config.hide_control_chars(),
        );
        name_plain = format!("{name_plain} -> {tq}");
    }
    let name_colored = colorizer.paint_name(entry, &name_plain);
    let name = hyperlink_name(&entry.path, &name_colored, config.hyperlink_enabled());

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
            block_display_with_unit(entry.blocks, config.blocks_unit()),
            w = widths.blocks
        ));
    }
    if config.show_context {
        let ctx = if entry.context.is_empty() {
            "?"
        } else {
            entry.context.as_str()
        };
        parts.push(format!("{:<w$}", ctx, w = widths.context));
    }
    parts.push(perms);
    parts.push(format!("{:>w$}", nlink, w = widths.nlink));
    if config.show_owner {
        parts.push(format!("{:<w$}", owner, w = widths.owner));
    }
    if config.show_group {
        parts.push(format!("{:<w$}", group, w = widths.group));
    }
    if config.show_author {
        parts.push(format!("{:<w$}", author, w = widths.author));
    }
    parts.push(format!("{:>w$}", size, w = widths.size));
    parts.push(mtime);
    let prefix = parts.join(" ");
    // prefix + git + " " + name
    let line_without_name = format!("{prefix}{git} ");
    if let Some(offsets) = dired_offsets {
        let start = line_without_name.len();
        // Record offsets of the plain quoted name (without color/hyperlink) for dired.
        // GNU uses the positions of the displayed filename in the raw output stream.
        // We use the painted/hyperlinked name as written.
        let end = start + name.len();
        offsets.push((start, end));
    }
    format!("{line_without_name}{name}")
}

fn format_size_field(entry: &Entry, config: &Config) -> String {
    format_size_bytes(
        entry.size,
        config.block_size,
        config.human_sizes,
        config.si_sizes,
    )
}

#[derive(Debug, Clone, Default)]
pub struct LongWidths {
    pub inode: usize,
    pub blocks: usize,
    pub context: usize,
    pub nlink: usize,
    pub owner: usize,
    pub group: usize,
    pub author: usize,
    pub size: usize,
}

impl LongWidths {
    pub fn compute(entries: &[Entry], config: &Config) -> Self {
        let mut w = Self {
            inode: 1,
            blocks: 1,
            context: 1,
            nlink: 1,
            owner: 1,
            group: 1,
            author: 1,
            size: 1,
        };
        for e in entries.iter().filter(|e| !e.is_dir_header) {
            w.inode = w.inode.max(decimal_digits(e.inode));
            let blocks_n = block_display_with_unit(e.blocks, config.blocks_unit());
            w.blocks = w.blocks.max(decimal_digits(blocks_n));
            if config.show_context {
                let ctx = if e.context.is_empty() {
                    "?"
                } else {
                    e.context.as_str()
                };
                w.context = w.context.max(ctx.len());
            }
            w.nlink = w.nlink.max(decimal_digits(e.nlink));
            if config.show_owner {
                w.owner = w.owner.max(e.owner_display(config.numeric_uid_gid).len());
            }
            if config.show_group {
                w.group = w.group.max(e.group_display(config.numeric_uid_gid).len());
            }
            if config.show_author {
                w.author = w.author.max(e.author_display(config.numeric_uid_gid).len());
            }
            w.size = w.size.max(format_size_field(e, config).len());
        }
        w
    }
}

/// Decimal digit count for u64 without allocating.
fn decimal_digits(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

/// Format many entries in long mode with aligned columns.
pub fn format_long(entries: &[Entry], colorizer: &Colorizer, config: &Config) -> String {
    let widths = LongWidths::compute(entries, config);
    // Rough capacity: ~80 bytes/line for typical long rows.
    let mut out = String::with_capacity(entries.len().saturating_mul(96));
    let mut dired_global: Vec<(usize, usize)> = Vec::new();
    let ending = config.line_ending();

    for entry in entries {
        if entry.is_dir_header {
            if !out.is_empty() {
                out.push_str(if config.zero { "\0" } else { "\n" });
            }
            out.push_str(&format!("{}:", entry.path.display()));
            out.push_str(ending);
            continue;
        }
        if config.dired {
            let base = out.len();
            let mut local = Vec::new();
            let line = format_long_line_dired(entry, colorizer, config, &widths, Some(&mut local));
            for (s, e) in local {
                dired_global.push((base + s, base + e));
            }
            out.push_str(&line);
            out.push_str(ending);
        } else {
            out.push_str(&format_long_line(entry, colorizer, config, &widths));
            out.push_str(ending);
        }
    }

    if config.dired && !config.zero {
        // GNU: //DIRED// start end start end ...
        out.push_str("//DIRED//");
        for (s, e) in &dired_global {
            out.push_str(&format!(" {s} {e}"));
        }
        out.push('\n');
        out.push_str(&format!(
            "//DIRED-OPTIONS// --quoting-style={}\n",
            quoting_style_word(config.quoting_style)
        ));
    }
    out
}

fn quoting_style_word(style: f00_core::QuotingStyle) -> &'static str {
    use f00_core::QuotingStyle::*;
    match style {
        Literal => "literal",
        Locale => "locale",
        Shell => "shell",
        ShellAlways => "shell-always",
        ShellEscape => "shell-escape",
        ShellEscapeAlways => "shell-escape-always",
        C => "c",
        Escape => "escape",
    }
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
        author: 1,
        inode: 1,
        blocks: 1,
        context: 1,
    };
    format_long_line(entry, colorizer, &config, &widths)
}

fn format_entry_time(entry: &Entry, config: &Config) -> String {
    let style = if config.full_time {
        TimeStyle::FullIso
    } else {
        config.time_style.clone()
    };
    let field = config.list.time_field;
    match entry.datetime_for(field) {
        Some(dt) => format_time_style(dt, &style),
        None => "            ".to_string(),
    }
}

pub fn format_time_style(dt: DateTime<Local>, style: &TimeStyle) -> String {
    match style {
        TimeStyle::FullIso => dt.format("%Y-%m-%d %H:%M:%S.%f %z").to_string(),
        TimeStyle::LongIso => dt.format("%Y-%m-%d %H:%M").to_string(),
        TimeStyle::Iso => {
            let now = Local::now();
            let six_months = chrono::Duration::days(365 / 2);
            if (now - dt).abs() > six_months {
                dt.format("%Y-%m-%d").to_string()
            } else {
                dt.format("%m-%d %H:%M").to_string()
            }
        }
        TimeStyle::Locale => format_ls_time(dt),
        TimeStyle::Format(fmt) => dt.format(fmt).to_string(),
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
