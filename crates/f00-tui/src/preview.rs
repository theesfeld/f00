//! File preview helpers: metadata + optional syntax highlighting.

use std::io::Read;
use std::path::Path;
use std::sync::OnceLock;

use f00_core::{Entry, EntryKind};
use f00_format::human_size;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

const PREVIEW_BYTES: usize = 12 * 1024;
const PREVIEW_LINES: usize = 80;

fn syntax_set() -> &'static SyntaxSet {
    static SS: OnceLock<SyntaxSet> = OnceLock::new();
    SS.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme() -> &'static Theme {
    static TS: OnceLock<Theme> = OnceLock::new();
    TS.get_or_init(|| {
        let set = ThemeSet::load_defaults();
        set.themes
            .get("base16-ocean.dark")
            .or_else(|| set.themes.get("InspiredGitHub"))
            .cloned()
            .unwrap_or_else(|| {
                set.themes.values().next().cloned().unwrap_or_else(|| {
                    ThemeSet::load_defaults()
                        .themes
                        .into_values()
                        .next()
                        .unwrap()
                })
            })
    })
}

fn syn_color_to_ratatui(c: syntect::highlighting::Color) -> Color {
    if c.a == 0 {
        return Color::Reset;
    }
    Color::Rgb(c.r, c.g, c.b)
}

/// Build ratatui lines for the preview pane.
pub fn preview_lines(entry: &Entry) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    lines.push(meta_line("name", &entry.name));
    lines.push(meta_line("path", &entry.path.display().to_string()));
    lines.push(meta_line("kind", entry.kind.as_str()));
    if entry.kind != EntryKind::Directory {
        lines.push(meta_line(
            "size",
            &format!("{} ({})", human_size(entry.size), entry.size),
        ));
    }
    if let Some(t) = entry.modified {
        let dt: chrono::DateTime<chrono::Local> = t.into();
        lines.push(meta_line(
            "mtime",
            &dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        ));
    }
    if let Some(ref target) = entry.symlink_target {
        lines.push(meta_line("link", &target.display().to_string()));
    }
    lines.push(meta_line("mode", &format!("{:o}", entry.mode)));
    if entry.git_status != f00_core::GitStatus::Clean {
        lines.push(meta_line("git", entry.git_status.as_str()));
    }
    lines.push(Line::from(""));

    if entry.is_dir() {
        lines.extend(dir_children(&entry.path));
    } else {
        lines.extend(file_preview(&entry.path, &entry.name));
    }
    lines
}

fn meta_line(key: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!(" {key}: "), Style::default().fg(Color::DarkGray)),
        Span::styled(value.to_string(), Style::default().fg(Color::Gray)),
    ])
}

fn dir_children(path: &Path) -> Vec<Line<'static>> {
    let mut out = Vec::new();
    match std::fs::read_dir(path) {
        Ok(rd) => {
            let mut names: Vec<String> = rd
                .flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .take(40)
                .collect();
            names.sort();
            out.push(Line::from(Span::styled(
                format!(" children (≤40): {}", names.len()),
                Style::default().fg(Color::DarkGray),
            )));
            for n in names {
                out.push(Line::from(vec![
                    Span::raw("  · "),
                    Span::styled(n, Style::default().fg(Color::Cyan)),
                ]));
            }
        }
        Err(e) => out.push(Line::from(format!(" (unreadable: {e})"))),
    }
    out
}

fn file_preview(path: &Path, name: &str) -> Vec<Line<'static>> {
    let mut out = Vec::new();
    let mut f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return vec![Line::from(format!(" (open error: {e})"))],
    };
    let mut buf = vec![0u8; PREVIEW_BYTES];
    let n = match f.read(&mut buf) {
        Ok(n) => n,
        Err(e) => return vec![Line::from(format!(" (read error: {e})"))],
    };
    buf.truncate(n);
    if n == 0 {
        out.push(Line::from(Span::styled(
            " (empty file)",
            Style::default().fg(Color::DarkGray),
        )));
        return out;
    }

    let lossy = String::from_utf8_lossy(&buf);
    let printable = lossy
        .chars()
        .filter(|c| *c == '\n' || *c == '\t' || !c.is_control())
        .count();
    if printable * 10 < n * 8 {
        out.push(Line::from(Span::styled(
            " (binary or non-text; no preview)",
            Style::default().fg(Color::DarkGray),
        )));
        return out;
    }

    out.push(Line::from(Span::styled(
        " ── head ──",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    )));

    let ss = syntax_set();
    let theme = theme();
    let syntax = ss
        .find_syntax_by_extension(
            Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or(""),
        )
        .or_else(|| ss.find_syntax_by_name("Plain Text"))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);
    for (i, line) in LinesWithEndings::from(&lossy).enumerate() {
        if i >= PREVIEW_LINES {
            out.push(Line::from(Span::styled(
                " …",
                Style::default().fg(Color::DarkGray),
            )));
            break;
        }
        match highlighter.highlight_line(line, ss) {
            Ok(ranges) => {
                let mut spans = Vec::with_capacity(ranges.len() + 1);
                spans.push(Span::raw(" "));
                for (style, text) in ranges {
                    let mut s = Style::default().fg(syn_color_to_ratatui(style.foreground));
                    if style
                        .font_style
                        .contains(syntect::highlighting::FontStyle::BOLD)
                    {
                        s = s.add_modifier(Modifier::BOLD);
                    }
                    if style
                        .font_style
                        .contains(syntect::highlighting::FontStyle::ITALIC)
                    {
                        s = s.add_modifier(Modifier::ITALIC);
                    }
                    // Strip trailing newline for ratatui Line
                    let t = text.trim_end_matches(['\n', '\r']);
                    spans.push(Span::styled(t.to_string(), s));
                }
                out.push(Line::from(spans));
            }
            Err(_) => {
                out.push(Line::from(format!(" {}", line.trim_end())));
            }
        }
    }
    out
}
