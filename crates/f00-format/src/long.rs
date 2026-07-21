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

    let gnu = config.list.gnu_mode;
    let size = format_size_field(entry, config);
    let mtime = format_entry_time(entry, config);
    let icon = icon_prefix(entry, config.icons);
    let suffix = classify_suffix(entry, config.indicator_style());

    let quoted = display_name(
        &entry.name,
        config.quoting_style,
        config.hide_control_chars(),
    );
    let name_core = format!("{icon}{quoted}{suffix}");
    let arrow_target = if let Some(target) = &entry.symlink_target {
        let tq = display_name(
            &target.display().to_string(),
            config.quoting_style,
            config.hide_control_chars(),
        );
        format!(" -> {tq}")
    } else {
        String::new()
    };
    let name_colored = if entry.symlink_target.is_some() {
        colorizer.paint_symlink_name(entry, &name_core, &arrow_target, gnu)
    } else {
        colorizer.paint_name(entry, &name_core)
    };
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

    // Build metadata columns; apply modern theme colors when not --gnu.
    let mut parts = Vec::new();
    if config.show_inode {
        let s = format!("{:>w$}", entry.inode, w = widths.inode);
        parts.push(colorizer.paint_meta(&s, gnu));
    }
    if config.show_blocks {
        let s = format!(
            "{:>w$}",
            block_display_with_unit(entry.blocks, config.blocks_unit()),
            w = widths.blocks
        );
        parts.push(colorizer.paint_meta(&s, gnu));
    }
    if config.show_context {
        let ctx = if entry.context.is_empty() {
            "?"
        } else {
            entry.context.as_str()
        };
        let s = format!("{:<w$}", ctx, w = widths.context);
        parts.push(colorizer.paint_meta(&s, gnu));
    }
    parts.push(colorizer.paint_perms(&perms, gnu));
    {
        let s = format!("{:>w$}", nlink, w = widths.nlink);
        parts.push(colorizer.paint_meta(&s, gnu));
    }
    if config.show_owner {
        let s = format!("{:<w$}", owner, w = widths.owner);
        parts.push(colorizer.paint_user(&s, gnu));
    }
    if config.show_group {
        let s = format!("{:<w$}", group, w = widths.group);
        parts.push(colorizer.paint_group(&s, gnu));
    }
    if config.show_author {
        let s = format!("{:<w$}", author, w = widths.author);
        parts.push(colorizer.paint_user(&s, gnu));
    }
    {
        let s = format!("{:>w$}", size, w = widths.size);
        parts.push(colorizer.paint_size(&s, entry.size, gnu));
    }
    parts.push(colorizer.paint_time(&mtime, gnu));
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
                let len = if config.numeric_uid_gid {
                    decimal_digits(u64::from(e.uid))
                } else {
                    e.owner.len().max(1)
                };
                w.owner = w.owner.max(len);
            }
            if config.show_group {
                let len = if config.numeric_uid_gid {
                    decimal_digits(u64::from(e.gid))
                } else {
                    e.group.len().max(1)
                };
                w.group = w.group.max(len);
            }
            if config.show_author {
                let len = if config.numeric_uid_gid {
                    decimal_digits(u64::from(e.uid))
                } else if !e.author.is_empty() {
                    e.author.len()
                } else {
                    e.owner.len().max(1)
                };
                w.author = w.author.max(len);
            }
            // Digit-count for plain bytes; human / non-1 block sizes need format.
            let size_len = if config.human_sizes
                || config.si_sizes
                || !matches!(config.block_size, f00_core::BlockSize::Bytes(1))
            {
                format_size_field(e, config).len()
            } else {
                decimal_digits(e.size)
            };
            w.size = w.size.max(size_len);
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

    let write_total = |out: &mut String, section: &[Entry]| {
        if config.zero || !config.emit_block_total {
            return;
        }
        // Sum allocated 512-byte blocks, then format like GNU `ls -s` / `-sh`.
        let total_512: u64 = section
            .iter()
            .filter(|e| !e.is_dir_header)
            .map(|e| e.blocks)
            .fold(0u64, u64::saturating_add);
        let total_str = if config.human_sizes || config.si_sizes {
            // GNU humanizes the *disk usage* (blocks * 512), not the rounded unit count.
            let bytes = total_512.saturating_mul(512);
            crate::human::format_size_bytes(bytes, config.block_size, true, config.si_sizes)
        } else {
            crate::human::block_display_with_unit(total_512, config.blocks_unit()).to_string()
        };
        out.push_str(&format!("total {total_str}"));
        out.push_str(ending);
    };

    let write_entry = |out: &mut String, dired_global: &mut Vec<(usize, usize)>, entry: &Entry| {
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
    };

    // Walk sections (optional dir headers for `-R`). GNU prints `total` *before*
    // any entry lines in each section.
    let mut i = 0;
    let has_headers = entries.iter().any(|e| e.is_dir_header);
    if !has_headers {
        write_total(&mut out, entries);
        for entry in entries {
            write_entry(&mut out, &mut dired_global, entry);
        }
    } else {
        while i < entries.len() {
            if entries[i].is_dir_header {
                if !out.is_empty() {
                    out.push_str(if config.zero { "\0" } else { "\n" });
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
            for entry in section {
                write_entry(&mut out, &mut dired_global, entry);
            }
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
