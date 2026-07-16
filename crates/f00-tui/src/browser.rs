//! Fullscreen directory browser powered by ratatui + f00-core.

use std::collections::HashSet;
use std::io::{self, stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use f00_core::{list_directory, Entry, EntryKind, ListOptions, SortBy};
use f00_format::{format_permissions, human_size, icon_prefix};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;

use crate::helpers::{
    clamp_index, filter_entries, format_selected_paths, join_child, move_selection, parent_dir,
};
use crate::BrowserOptions;

/// Minimum terminal width before long columns (perms / size / mtime) are shown.
const LONG_COLUMNS_MIN_WIDTH: u16 = 72;

/// Result of a browser session once the terminal is restored.
#[derive(Debug, Clone, Default)]
struct QuitAction {
    /// Paths to print to stdout after restore (one per line).
    print_paths: Vec<PathBuf>,
}

struct Browser {
    cwd: PathBuf,
    entries: Vec<Entry>,
    selected: usize,
    list_state: ListState,
    /// Absolute (or as-resolved) paths marked with Space.
    marked: HashSet<PathBuf>,
    show_hidden: bool,
    icons: bool,
    git: bool,
    filter: String,
    filtering: bool,
    show_help: bool,
    show_preview: bool,
    sort_by: SortBy,
    reverse: bool,
    status: String,
    error: Option<String>,
}

impl Browser {
    fn new(start: &Path, opts: &BrowserOptions) -> Result<Self> {
        let cwd = if start.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            start.to_path_buf()
        };
        let mut b = Self {
            cwd,
            entries: Vec::new(),
            selected: 0,
            list_state: ListState::default(),
            marked: HashSet::new(),
            show_hidden: opts.show_hidden,
            icons: opts.icons,
            git: opts.git,
            filter: String::new(),
            filtering: false,
            show_help: false,
            show_preview: true,
            sort_by: SortBy::Name,
            reverse: false,
            status: String::new(),
            error: None,
        };
        b.reload()?;
        Ok(b)
    }

    fn list_options(&self) -> ListOptions {
        ListOptions {
            almost_all: self.show_hidden,
            all: false,
            dirs_first: true,
            sort_by: self.sort_by,
            reverse: self.reverse,
            // TUI rarely needs owner names for display columns.
            resolve_owner_group: false,
            read_selinux: false,
            ..ListOptions::default()
        }
    }

    fn cycle_sort(&mut self) -> Result<()> {
        self.sort_by = match self.sort_by {
            SortBy::Name => SortBy::Size,
            SortBy::Size => SortBy::Time,
            SortBy::Time => SortBy::Extension,
            SortBy::Extension => SortBy::Name,
            other => other,
        };
        self.reload()?;
        self.status = format!(
            "sort: {}{}",
            self.sort_label(),
            if self.reverse { " rev" } else { "" }
        );
        Ok(())
    }

    fn sort_label(&self) -> &'static str {
        match self.sort_by {
            SortBy::Name => "name",
            SortBy::Size => "size",
            SortBy::Time => "mtime",
            SortBy::Extension => "ext",
            SortBy::Version => "version",
            SortBy::None => "none",
        }
    }

    fn open_external(&mut self, pager: bool) {
        let Some(entry) = self.current_entry().cloned() else {
            self.status = "nothing selected".into();
            return;
        };
        if entry.is_dir() {
            self.status = "use Enter to open directories".into();
            return;
        }
        let path = entry.path;
        // Leave raw mode while the external program runs.
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let status = if pager {
            let pager = std::env::var("PAGER").unwrap_or_else(|_| "less".into());
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "{pager} {}",
                    shell_quote(&path.display().to_string())
                ))
                .status()
        } else {
            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| "vi".into());
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "{editor} {}",
                    shell_quote(&path.display().to_string())
                ))
                .status()
        };
        let _ = execute!(stdout(), EnterAlternateScreen);
        let _ = enable_raw_mode();
        match status {
            Ok(s) if s.success() => {
                self.status = if pager {
                    "viewed".into()
                } else {
                    "edited".into()
                };
            }
            Ok(s) => self.status = format!("external exit {s}"),
            Err(e) => self.status = format!("open failed: {e}"),
        }
    }

    fn reload(&mut self) -> Result<()> {
        let opts = self.list_options();
        match list_directory(&self.cwd, &opts) {
            Ok(listing) => {
                self.entries = listing.entries;
                self.error = None;
                if self.git {
                    annotate_git(&mut self.entries, &self.cwd);
                }
            }
            Err(e) => {
                self.entries.clear();
                self.error = Some(e.to_string());
            }
        }
        self.selected = clamp_index(self.selected, self.visible().len());
        self.sync_list_state();
        Ok(())
    }

    fn visible(&self) -> Vec<&Entry> {
        filter_entries(&self.entries, &self.filter)
    }

    fn sync_list_state(&mut self) {
        let len = self.visible().len();
        if len == 0 {
            self.list_state.select(None);
        } else {
            self.selected = clamp_index(self.selected, len);
            self.list_state.select(Some(self.selected));
        }
    }

    fn current_entry(&self) -> Option<&Entry> {
        let vis = self.visible();
        vis.get(self.selected).copied()
    }

    fn enter_selected(&mut self) -> Result<Option<QuitAction>> {
        let Some(entry) = self.current_entry().cloned() else {
            return Ok(None);
        };
        if entry.is_dir() {
            let next = join_child(&self.cwd, &entry.name);
            self.cwd = next;
            self.filter.clear();
            self.filtering = false;
            self.selected = 0;
            self.reload()?;
            Ok(None)
        } else {
            // Enter on file: mark and quit with its path.
            Ok(Some(QuitAction {
                print_paths: vec![entry.path.clone()],
            }))
        }
    }

    fn go_parent(&mut self) -> Result<()> {
        let parent = parent_dir(&self.cwd);
        if parent != self.cwd {
            self.cwd = parent;
            self.filter.clear();
            self.filtering = false;
            self.selected = 0;
            self.reload()?;
        }
        Ok(())
    }

    fn toggle_mark(&mut self) {
        if let Some(entry) = self.current_entry() {
            let path = entry.path.clone();
            if !self.marked.remove(&path) {
                self.marked.insert(path);
            }
            // Advance after mark for multi-select ergonomics.
            let len = self.visible().len();
            self.selected = move_selection(self.selected, len, 1);
            self.sync_list_state();
        }
    }

    fn quit_with_marked(&self) -> QuitAction {
        let mut paths: Vec<PathBuf> = self.marked.iter().cloned().collect();
        paths.sort();
        // If nothing marked, use the current cursor entry when present.
        if paths.is_empty() {
            if let Some(e) = self.current_entry() {
                paths.push(e.path.clone());
            }
        }
        QuitAction { print_paths: paths }
    }
}

fn annotate_git(entries: &mut [Entry], cwd: &Path) {
    #[cfg(feature = "git")]
    {
        f00_git::annotate_entries(entries, cwd);
    }
    #[cfg(not(feature = "git"))]
    {
        let _ = (entries, cwd);
    }
}

/// Run the interactive browser.
///
/// Returns process exit code (`0` on normal quit). When the user confirms
/// selection (`y` or Enter on a file), selected paths are printed to stdout
/// **after** the terminal is restored (one path per line).
pub fn run(start: &Path, opts: BrowserOptions) -> Result<i32> {
    if !crate::helpers::is_interactive_tty() {
        anyhow::bail!("f00 browser requires an interactive TTY (stdin and stdout)");
    }

    let mut browser = Browser::new(start, &opts)?;
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal, &mut browser);
    restore_terminal(&mut terminal)?;

    match result {
        Ok(action) => {
            if !action.print_paths.is_empty() {
                let text = format_selected_paths(&action.print_paths);
                // Write to stdout after leaving the alternate screen.
                let mut out = io::stdout();
                writeln!(out, "{text}")?;
                out.flush()?;
            }
            Ok(0)
        }
        Err(e) => Err(e),
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode().context("enable raw mode")?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen).context("enter alternate screen")?;
    let backend = CrosstermBackend::new(out);
    let terminal = Terminal::new(backend).context("create terminal")?;

    // Best-effort restore if we panic while raw/alt-screen is active.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let mut out = stdout();
        let _ = execute!(out, LeaveAlternateScreen);
        original_hook(info);
    }));

    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode().context("disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).context("leave alternate screen")?;
    terminal.show_cursor().context("show cursor")?;
    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    browser: &mut Browser,
) -> Result<QuitAction> {
    loop {
        terminal.draw(|frame| draw_ui(frame, browser))?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        // Ignore key-release / repeat on terminals that emit them.
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if browser.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('H') | KeyCode::Char('?') => {
                    browser.show_help = false;
                }
                _ => {}
            }
            continue;
        }

        if browser.filtering {
            match key.code {
                KeyCode::Esc => {
                    browser.filtering = false;
                    browser.filter.clear();
                    browser.selected = 0;
                    browser.sync_list_state();
                }
                KeyCode::Enter => {
                    browser.filtering = false;
                }
                KeyCode::Backspace => {
                    browser.filter.pop();
                    browser.selected = 0;
                    browser.sync_list_state();
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    browser.filter.push(c);
                    browser.selected = 0;
                    browser.sync_list_state();
                }
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                return Ok(QuitAction::default());
            }
            KeyCode::Char('y') => {
                return Ok(browser.quit_with_marked());
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let len = browser.visible().len();
                browser.selected = move_selection(browser.selected, len, 1);
                browser.sync_list_state();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let len = browser.visible().len();
                browser.selected = move_selection(browser.selected, len, -1);
                browser.sync_list_state();
            }
            KeyCode::Char('g') => {
                browser.selected = 0;
                browser.sync_list_state();
            }
            KeyCode::Char('G') => {
                let len = browser.visible().len();
                browser.selected = if len == 0 { 0 } else { len - 1 };
                browser.sync_list_state();
            }
            KeyCode::Char('h') | KeyCode::Backspace | KeyCode::Left => {
                browser.go_parent()?;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if let Some(entry) = browser.current_entry() {
                    if entry.is_dir() {
                        if let Some(action) = browser.enter_selected()? {
                            return Ok(action);
                        }
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(action) = browser.enter_selected()? {
                    return Ok(action);
                }
            }
            KeyCode::Char(' ') => {
                browser.toggle_mark();
            }
            KeyCode::Char('r') => {
                browser.reload()?;
                browser.status = "refreshed".into();
            }
            KeyCode::Char('.') => {
                browser.show_hidden = !browser.show_hidden;
                browser.selected = 0;
                browser.reload()?;
                browser.status = if browser.show_hidden {
                    "showing hidden".into()
                } else {
                    "hiding hidden".into()
                };
            }
            KeyCode::Char('/') => {
                browser.filtering = true;
                browser.status.clear();
            }
            KeyCode::Char('H') | KeyCode::Char('?') => {
                browser.show_help = true;
            }
            KeyCode::Char('s') => {
                browser.cycle_sort()?;
            }
            KeyCode::Char('S') => {
                browser.reverse = !browser.reverse;
                browser.reload()?;
                browser.status = if browser.reverse {
                    "reverse on".into()
                } else {
                    "reverse off".into()
                };
            }
            KeyCode::Char('p') => {
                browser.show_preview = !browser.show_preview;
                browser.status = if browser.show_preview {
                    "preview on".into()
                } else {
                    "preview off".into()
                };
            }
            KeyCode::Char('e') => {
                browser.open_external(false);
            }
            KeyCode::Char('v') => {
                browser.open_external(true);
            }
            KeyCode::PageDown => {
                let len = browser.visible().len();
                browser.selected = move_selection(browser.selected, len, 10);
                browser.sync_list_state();
            }
            KeyCode::PageUp => {
                let len = browser.visible().len();
                browser.selected = move_selection(browser.selected, len, -10);
                browser.sync_list_state();
            }
            _ => {}
        }
    }
}

fn shell_quote(s: &str) -> String {
    // Minimal single-quote escaping for POSIX sh.
    format!("'{}'", s.replace('\'', "'\"'\"'"))
}

fn draw_ui(frame: &mut ratatui::Frame, browser: &mut Browser) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header / status
            Constraint::Min(3),    // body
            Constraint::Length(1), // footer
        ])
        .split(area);

    draw_header(frame, chunks[0], browser);

    if browser.show_preview && chunks[1].width >= 48 {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
            .split(chunks[1]);
        draw_list(frame, body[0], browser);
        draw_preview(frame, body[1], browser);
    } else {
        draw_list(frame, chunks[1], browser);
    }

    draw_footer(frame, chunks[2], browser);

    if browser.show_help {
        draw_help_overlay(frame, area);
    }
}

fn draw_header(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let vis = browser.visible().len();
    let total = browser.entries.len();
    let marked = browser.marked.len();
    let hidden = if browser.show_hidden {
        " · hidden"
    } else {
        ""
    };
    let rev = if browser.reverse { "↓" } else { "↑" };
    let filter = if browser.filter.is_empty() {
        String::new()
    } else {
        format!(" · /{}", browser.filter)
    };
    let err = browser
        .error
        .as_ref()
        .map(|e| format!(" · err: {e}"))
        .unwrap_or_default();
    let marks = if marked == 0 {
        String::new()
    } else {
        format!(" · {marked} marked")
    };

    let title = format!(
        " {}  · {vis}/{total}{hidden} · sort:{}{rev}{filter}{marks}{err} ",
        browser.cwd.display(),
        browser.sort_label(),
    );
    let header = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(header, area);
}

fn draw_preview(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let text = match browser.current_entry() {
        None => " (no selection) ".to_string(),
        Some(entry) => preview_text(entry),
    };
    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" preview ")
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(widget, area);
}

fn preview_text(entry: &Entry) -> String {
    let mut lines = Vec::new();
    lines.push(format!(" name: {}", entry.name));
    lines.push(format!(" path: {}", entry.path.display()));
    lines.push(format!(" kind: {}", entry.kind.as_str()));
    if !entry.is_dir() {
        lines.push(format!(
            " size: {} ({})",
            human_size(entry.size),
            entry.size
        ));
    }
    if let Some(t) = entry.modified {
        let dt: DateTime<Local> = t.into();
        lines.push(format!(" mtime: {}", dt.format("%Y-%m-%d %H:%M:%S")));
    }
    if let Some(ref target) = entry.symlink_target {
        lines.push(format!(" link → {}", target.display()));
    }
    lines.push(format!(" mode: {:o}", entry.mode));
    if entry.git_status != f00_core::GitStatus::Clean {
        lines.push(format!(" git: {}", entry.git_status.as_str()));
    }
    lines.push(String::new());

    if entry.is_dir() {
        match std::fs::read_dir(&entry.path) {
            Ok(rd) => {
                let mut names: Vec<String> = rd
                    .flatten()
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .take(40)
                    .collect();
                names.sort();
                lines.push(format!(" children (≤40): {}", names.len()));
                for n in names {
                    lines.push(format!("  · {n}"));
                }
            }
            Err(e) => lines.push(format!(" (unreadable: {e})")),
        }
    } else {
        match std::fs::File::open(&entry.path) {
            Ok(mut f) => {
                use std::io::Read;
                let mut buf = vec![0u8; 2048];
                match f.read(&mut buf) {
                    Ok(n) => {
                        buf.truncate(n);
                        let lossy = String::from_utf8_lossy(&buf);
                        // Only show text-ish previews.
                        let printable = lossy
                            .chars()
                            .filter(|c| *c == '\n' || *c == '\t' || !c.is_control())
                            .count();
                        if n > 0 && printable * 10 >= n * 8 {
                            lines.push(" ── head ──".into());
                            for line in lossy.lines().take(24) {
                                lines.push(format!(" {line}"));
                            }
                        } else {
                            lines.push(" (binary or non-text; no preview)".into());
                        }
                    }
                    Err(e) => lines.push(format!(" (read error: {e})")),
                }
            }
            Err(e) => lines.push(format!(" (open error: {e})")),
        }
    }
    lines.join("\n")
}

fn draw_list(frame: &mut ratatui::Frame, area: Rect, browser: &mut Browser) {
    let width = area.width;
    let show_long = width >= LONG_COLUMNS_MIN_WIDTH;
    let vis = browser.visible();

    let items: Vec<ListItem> = vis
        .iter()
        .map(|entry| {
            let marked = browser.marked.contains(&entry.path);
            let mark = if marked { "● " } else { "  " };
            let icon = icon_prefix(entry, browser.icons);
            let kind_tag = match entry.kind {
                EntryKind::Directory => "/",
                EntryKind::Symlink => "@",
                _ => "",
            };
            let git = if browser.git {
                entry
                    .git_status
                    .as_char()
                    .map(|c| format!(" [{c}]"))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            let name = format!("{mark}{icon}{}{kind_tag}{git}", entry.name);

            let line = if show_long {
                let perms = format_permissions(entry);
                let size = if entry.is_dir() {
                    "-".to_string()
                } else {
                    human_size(entry.size)
                };
                let mtime = entry
                    .modified
                    .map(|t| {
                        let dt: DateTime<Local> = t.into();
                        dt.format("%Y-%m-%d %H:%M").to_string()
                    })
                    .unwrap_or_else(|| "—".into());
                Line::from(vec![
                    Span::raw(format!("{perms}  ")),
                    Span::raw(format!("{size:>8}  ")),
                    Span::raw(format!("{mtime}  ")),
                    Span::styled(
                        name,
                        if entry.is_dir() {
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD)
                        } else if entry.kind == EntryKind::Symlink {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default()
                        },
                    ),
                ])
            } else {
                Line::from(Span::styled(
                    name,
                    if entry.is_dir() {
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD)
                    } else if entry.kind == EntryKind::Symlink {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default()
                    },
                ))
            };
            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(if browser.filtering {
            " filter (Esc clear · Enter apply) "
        } else {
            " entries "
        });

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("› ");

    frame.render_stateful_widget(list, area, &mut browser.list_state);
}

fn draw_footer(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let hints = if browser.filtering {
        format!(" filter: /{}  · Esc clear · Enter done ", browser.filter)
    } else {
        let status = if browser.status.is_empty() {
            String::new()
        } else {
            format!(" · {}", browser.status)
        };
        format!(
            " j/k · Enter · h parent · Space mark · y yield · s sort · p preview · e edit · v view · . hidden · / · H · q{status} "
        )
    };
    let footer = Paragraph::new(hints).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}

fn draw_help_overlay(frame: &mut ratatui::Frame, area: Rect) {
    let help = "\
f00 browser — keys

  j / ↓          move down
  k / ↑          move up
  g / G          top / bottom
  Enter          enter directory · open file (print path & quit)
  l / →          enter directory
  h / ← / BS     parent directory
  Space          toggle mark (multi-select)
  y              print marked paths & quit (cursor if none marked)
  /              filter by name (Esc clears)
  .              toggle hidden (almost_all)
  r              refresh listing
  s              cycle sort (name → size → mtime → ext)
  S              reverse sort
  p              toggle preview pane
  e              open file in $EDITOR (or $VISUAL / vi)
  v              open file in $PAGER (or less)
  H / ?          toggle this help
  q / Esc        quit (print nothing)

  Long columns (perms, size, mtime) appear when the terminal is wide enough.
  Preview shows metadata + text head or directory children.

  Press Esc / q / H to close help.
";

    let popup = centered_rect(70, 80, area);
    frame.render_widget(Clear, popup);
    let widget = Paragraph::new(help)
        .block(
            Block::default()
                .title(" help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(widget, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
