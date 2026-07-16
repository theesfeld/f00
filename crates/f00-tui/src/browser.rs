//! Fullscreen directory browser powered by ratatui + f00-core.
//!
//! Supports single-pane and dual-pane (file-manager) layouts with copy / move /
//! delete between panes.

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

/// Default dual-pane on when the terminal is at least this wide.
const DUAL_DEFAULT_MIN_WIDTH: u16 = 80;

/// Result of a browser session once the terminal is restored.
#[derive(Debug, Clone, Default)]
struct QuitAction {
    /// Paths to print to stdout after restore (one per line).
    print_paths: Vec<PathBuf>,
}

/// Pending file operation awaiting `y` / `n` confirmation.
#[derive(Debug, Clone)]
enum ConfirmOp {
    Copy {
        sources: Vec<PathBuf>,
        dest_dir: PathBuf,
    },
    Move {
        sources: Vec<PathBuf>,
        dest_dir: PathBuf,
    },
    Delete {
        paths: Vec<PathBuf>,
    },
}

impl ConfirmOp {
    fn label(&self) -> &'static str {
        match self {
            ConfirmOp::Copy { .. } => "COPY",
            ConfirmOp::Move { .. } => "MOVE",
            ConfirmOp::Delete { .. } => "DELETE",
        }
    }

    fn paths(&self) -> &[PathBuf] {
        match self {
            ConfirmOp::Copy { sources, .. } | ConfirmOp::Move { sources, .. } => sources,
            ConfirmOp::Delete { paths } => paths,
        }
    }

    fn dest_hint(&self) -> Option<&Path> {
        match self {
            ConfirmOp::Copy { dest_dir, .. } | ConfirmOp::Move { dest_dir, .. } => Some(dest_dir),
            ConfirmOp::Delete { .. } => None,
        }
    }
}

/// One side of the dual-pane browser (also used alone in single-pane mode).
struct Pane {
    cwd: PathBuf,
    entries: Vec<Entry>,
    selected: usize,
    list_state: ListState,
    /// Absolute (or as-resolved) paths marked with Space.
    marked: HashSet<PathBuf>,
    filter: String,
    filtering: bool,
    /// Last listing error for this pane (if any).
    error: Option<String>,
}

impl Pane {
    fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            entries: Vec::new(),
            selected: 0,
            list_state: ListState::default(),
            marked: HashSet::new(),
            filter: String::new(),
            filtering: false,
            error: None,
        }
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

    /// Paths targeted by copy/move/delete: marks if any, else cursor entry.
    fn op_paths(&self) -> Vec<PathBuf> {
        if !self.marked.is_empty() {
            let mut paths: Vec<PathBuf> = self.marked.iter().cloned().collect();
            paths.sort();
            return paths;
        }
        if let Some(e) = self.current_entry() {
            return vec![e.path.clone()];
        }
        Vec::new()
    }
}

struct Browser {
    panes: [Pane; 2],
    active: usize,
    dual: bool,
    show_help: bool,
    show_preview: bool,
    sort_by: SortBy,
    reverse: bool,
    icons: bool,
    git: bool,
    show_hidden: bool,
    status: String,
    error: Option<String>,
    confirm: Option<ConfirmOp>,
}

impl Browser {
    fn new(start: &Path, opts: &BrowserOptions, dual_default: bool) -> Result<Self> {
        let cwd = if start.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            start.to_path_buf()
        };
        let mut b = Self {
            panes: [Pane::new(cwd.clone()), Pane::new(cwd)],
            active: 0,
            dual: dual_default,
            show_help: false,
            show_preview: true,
            sort_by: SortBy::Name,
            reverse: false,
            icons: opts.icons,
            git: opts.git,
            show_hidden: opts.show_hidden,
            status: if dual_default {
                "dual pane on".into()
            } else {
                String::new()
            },
            error: None,
            confirm: None,
        };
        b.reload_all()?;
        Ok(b)
    }

    fn active_pane(&self) -> &Pane {
        &self.panes[self.active]
    }

    fn active_pane_mut(&mut self) -> &mut Pane {
        &mut self.panes[self.active]
    }

    fn other_index(&self) -> usize {
        1 - self.active
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
        self.reload_all()?;
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
        let Some(entry) = self.active_pane().current_entry().cloned() else {
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

    fn reload_pane(&mut self, idx: usize) -> Result<()> {
        let opts = self.list_options();
        let git = self.git;
        let pane = &mut self.panes[idx];
        match list_directory(&pane.cwd, &opts) {
            Ok(listing) => {
                pane.entries = listing.entries;
                pane.error = None;
                if git {
                    annotate_git(&mut pane.entries, &pane.cwd);
                }
            }
            Err(e) => {
                pane.entries.clear();
                pane.error = Some(e.to_string());
            }
        }
        pane.selected = clamp_index(pane.selected, pane.visible().len());
        pane.sync_list_state();
        Ok(())
    }

    fn reload_active(&mut self) -> Result<()> {
        self.reload_pane(self.active)
    }

    fn reload_all(&mut self) -> Result<()> {
        self.reload_pane(0)?;
        self.reload_pane(1)?;
        Ok(())
    }

    fn switch_pane(&mut self) {
        if !self.dual {
            // In single-pane mode, Tab still flips the active index so the
            // other cwd is ready when dual is re-enabled.
            self.active = self.other_index();
            self.status = format!("pane {}", self.active + 1);
            return;
        }
        // Leave filter mode on the pane we're leaving.
        self.active_pane_mut().filtering = false;
        self.active = self.other_index();
        self.status = format!("pane {}", self.active + 1);
    }

    fn toggle_dual(&mut self) {
        self.dual = !self.dual;
        if self.dual {
            self.status = "dual pane on · preview off in dual".into();
        } else {
            self.status = "dual pane off".into();
        }
    }

    fn enter_selected(&mut self) -> Result<Option<QuitAction>> {
        let Some(entry) = self.active_pane().current_entry().cloned() else {
            return Ok(None);
        };
        if entry.is_dir() {
            let next = join_child(&self.active_pane().cwd, &entry.name);
            let pane = self.active_pane_mut();
            pane.cwd = next;
            pane.filter.clear();
            pane.filtering = false;
            pane.selected = 0;
            self.reload_active()?;
            Ok(None)
        } else {
            // Enter on file: mark and quit with its path.
            Ok(Some(QuitAction {
                print_paths: vec![entry.path.clone()],
            }))
        }
    }

    fn go_parent(&mut self) -> Result<()> {
        let parent = parent_dir(&self.active_pane().cwd);
        if parent != self.active_pane().cwd {
            let pane = self.active_pane_mut();
            pane.cwd = parent;
            pane.filter.clear();
            pane.filtering = false;
            pane.selected = 0;
            self.reload_active()?;
        }
        Ok(())
    }

    fn toggle_mark(&mut self) {
        let pane = self.active_pane_mut();
        if let Some(entry) = pane.current_entry() {
            let path = entry.path.clone();
            if !pane.marked.remove(&path) {
                pane.marked.insert(path);
            }
            // Advance after mark for multi-select ergonomics.
            let len = pane.visible().len();
            pane.selected = move_selection(pane.selected, len, 1);
            pane.sync_list_state();
        }
    }

    fn quit_with_marked(&self) -> QuitAction {
        let pane = self.active_pane();
        let mut paths: Vec<PathBuf> = pane.marked.iter().cloned().collect();
        paths.sort();
        // If nothing marked, use the current cursor entry when present.
        if paths.is_empty() {
            if let Some(e) = pane.current_entry() {
                paths.push(e.path.clone());
            }
        }
        QuitAction { print_paths: paths }
    }

    /// Begin copy of marked/cursor items into the other pane's cwd.
    fn begin_copy(&mut self) {
        let sources = self.active_pane().op_paths();
        if sources.is_empty() {
            self.status = "nothing to copy".into();
            return;
        }
        let dest_dir = self.panes[self.other_index()].cwd.clone();
        if sources.len() == 1 {
            self.execute_copy(&sources, &dest_dir);
        } else {
            self.confirm = Some(ConfirmOp::Copy { sources, dest_dir });
            self.status.clear();
        }
    }

    /// Begin move of marked/cursor items into the other pane's cwd.
    fn begin_move(&mut self) {
        let sources = self.active_pane().op_paths();
        if sources.is_empty() {
            self.status = "nothing to move".into();
            return;
        }
        let dest_dir = self.panes[self.other_index()].cwd.clone();
        if sources.len() == 1 {
            self.execute_move(&sources, &dest_dir);
        } else {
            self.confirm = Some(ConfirmOp::Move { sources, dest_dir });
            self.status.clear();
        }
    }

    /// Begin delete of marked/cursor items (always confirms).
    fn begin_delete(&mut self) {
        let paths = self.active_pane().op_paths();
        if paths.is_empty() {
            self.status = "nothing to delete".into();
            return;
        }
        self.confirm = Some(ConfirmOp::Delete { paths });
        self.status.clear();
    }

    fn cancel_confirm(&mut self) {
        self.confirm = None;
        self.status = "cancelled".into();
    }

    fn accept_confirm(&mut self) {
        let Some(op) = self.confirm.take() else {
            return;
        };
        match op {
            ConfirmOp::Copy { sources, dest_dir } => self.execute_copy(&sources, &dest_dir),
            ConfirmOp::Move { sources, dest_dir } => self.execute_move(&sources, &dest_dir),
            ConfirmOp::Delete { paths } => self.execute_delete(&paths),
        }
    }

    fn execute_copy(&mut self, sources: &[PathBuf], dest_dir: &Path) {
        let mut ok = 0usize;
        let mut err_msg = None;
        for src in sources {
            let Some(name) = src.file_name() else {
                err_msg = Some(format!("bad path: {}", src.display()));
                break;
            };
            let dest = dest_dir.join(name);
            match copy_path(src, &dest) {
                Ok(()) => ok += 1,
                Err(e) => {
                    err_msg = Some(format!("copy {}: {e}", src.display()));
                    break;
                }
            }
        }
        if let Some(msg) = err_msg {
            self.status = msg.clone();
            self.error = Some(msg);
        } else {
            let pane = self.active_pane_mut();
            for src in sources {
                pane.marked.remove(src);
            }
            self.status = format!("copied {ok} → {}", dest_dir.display());
            self.error = None;
        }
        let _ = self.reload_all();
    }

    fn execute_move(&mut self, sources: &[PathBuf], dest_dir: &Path) {
        let mut ok = 0usize;
        let mut err_msg = None;
        for src in sources {
            let Some(name) = src.file_name() else {
                err_msg = Some(format!("bad path: {}", src.display()));
                break;
            };
            let dest = dest_dir.join(name);
            match move_path(src, &dest) {
                Ok(()) => ok += 1,
                Err(e) => {
                    err_msg = Some(format!("move {}: {e}", src.display()));
                    break;
                }
            }
        }
        if let Some(msg) = err_msg {
            self.status = msg.clone();
            self.error = Some(msg);
        } else {
            let pane = self.active_pane_mut();
            for src in sources {
                pane.marked.remove(src);
            }
            self.status = format!("moved {ok} → {}", dest_dir.display());
            self.error = None;
        }
        let _ = self.reload_all();
    }

    fn execute_delete(&mut self, paths: &[PathBuf]) {
        let mut ok = 0usize;
        let mut err_msg = None;
        for path in paths {
            match delete_path(path) {
                Ok(()) => ok += 1,
                Err(e) => {
                    err_msg = Some(format!("delete {}: {e}", path.display()));
                    break;
                }
            }
        }
        if let Some(msg) = err_msg {
            self.status = msg.clone();
            self.error = Some(msg);
        } else {
            let pane = self.active_pane_mut();
            for path in paths {
                pane.marked.remove(path);
            }
            self.status = format!("deleted {ok}");
            self.error = None;
        }
        let _ = self.reload_all();
    }
}

/// Recursive copy (files + directories) without shelling out.
fn copy_path(src: &Path, dest: &Path) -> io::Result<()> {
    let meta = std::fs::symlink_metadata(src)?;
    if meta.file_type().is_dir() {
        std::fs::create_dir_all(dest)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let dest_child = dest.join(entry.file_name());
            copy_path(&entry.path(), &dest_child)?;
        }
        Ok(())
    } else {
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src, dest)?;
        Ok(())
    }
}

/// Rename when possible; otherwise copy + delete.
fn move_path(src: &Path, dest: &Path) -> io::Result<()> {
    match std::fs::rename(src, dest) {
        Ok(()) => Ok(()),
        Err(_) => {
            copy_path(src, dest)?;
            delete_path(src)
        }
    }
}

fn delete_path(path: &Path) -> io::Result<()> {
    let meta = std::fs::symlink_metadata(path)?;
    if meta.file_type().is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
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

    let dual_default = crossterm::terminal::size()
        .map(|(w, _)| w >= DUAL_DEFAULT_MIN_WIDTH)
        .unwrap_or(false);

    let mut browser = Browser::new(start, &opts, dual_default)?;
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

        // Confirmation overlay steals all keys.
        if browser.confirm.is_some() {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => browser.accept_confirm(),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    browser.cancel_confirm();
                }
                _ => {}
            }
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

        if browser.active_pane().filtering {
            match key.code {
                KeyCode::Esc => {
                    let pane = browser.active_pane_mut();
                    pane.filtering = false;
                    pane.filter.clear();
                    pane.selected = 0;
                    pane.sync_list_state();
                }
                KeyCode::Enter => {
                    browser.active_pane_mut().filtering = false;
                }
                KeyCode::Backspace => {
                    let pane = browser.active_pane_mut();
                    pane.filter.pop();
                    pane.selected = 0;
                    pane.sync_list_state();
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let pane = browser.active_pane_mut();
                    pane.filter.push(c);
                    pane.selected = 0;
                    pane.sync_list_state();
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
            KeyCode::Tab => {
                browser.switch_pane();
            }
            KeyCode::Char('\\') | KeyCode::Char('|') => {
                browser.toggle_dual();
            }
            KeyCode::Char('c') => {
                browser.begin_copy();
            }
            KeyCode::Char('m') => {
                browser.begin_move();
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                browser.begin_delete();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let pane = browser.active_pane_mut();
                let len = pane.visible().len();
                pane.selected = move_selection(pane.selected, len, 1);
                pane.sync_list_state();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let pane = browser.active_pane_mut();
                let len = pane.visible().len();
                pane.selected = move_selection(pane.selected, len, -1);
                pane.sync_list_state();
            }
            KeyCode::Char('g') => {
                let pane = browser.active_pane_mut();
                pane.selected = 0;
                pane.sync_list_state();
            }
            KeyCode::Char('G') => {
                let pane = browser.active_pane_mut();
                let len = pane.visible().len();
                pane.selected = if len == 0 { 0 } else { len - 1 };
                pane.sync_list_state();
            }
            KeyCode::Char('h') | KeyCode::Backspace | KeyCode::Left => {
                browser.go_parent()?;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if let Some(entry) = browser.active_pane().current_entry() {
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
                browser.reload_all()?;
                browser.status = "refreshed".into();
            }
            KeyCode::Char('.') => {
                browser.show_hidden = !browser.show_hidden;
                for pane in &mut browser.panes {
                    pane.selected = 0;
                }
                browser.reload_all()?;
                browser.status = if browser.show_hidden {
                    "showing hidden".into()
                } else {
                    "hiding hidden".into()
                };
            }
            KeyCode::Char('/') => {
                browser.active_pane_mut().filtering = true;
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
                browser.reload_all()?;
                browser.status = if browser.reverse {
                    "reverse on".into()
                } else {
                    "reverse off".into()
                };
            }
            KeyCode::Char('p') => {
                if browser.dual {
                    browser.status = "preview off in dual".into();
                } else {
                    browser.show_preview = !browser.show_preview;
                    browser.status = if browser.show_preview {
                        "preview on".into()
                    } else {
                        "preview off".into()
                    };
                }
            }
            KeyCode::Char('e') => {
                browser.open_external(false);
            }
            KeyCode::Char('v') => {
                browser.open_external(true);
            }
            KeyCode::PageDown => {
                let pane = browser.active_pane_mut();
                let len = pane.visible().len();
                pane.selected = move_selection(pane.selected, len, 10);
                pane.sync_list_state();
            }
            KeyCode::PageUp => {
                let pane = browser.active_pane_mut();
                let len = pane.visible().len();
                pane.selected = move_selection(pane.selected, len, -10);
                pane.sync_list_state();
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

    if browser.dual {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        draw_list(frame, body[0], browser, 0);
        draw_list(frame, body[1], browser, 1);
    } else if browser.show_preview && chunks[1].width >= 48 {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
            .split(chunks[1]);
        draw_list(frame, body[0], browser, browser.active);
        draw_preview(frame, body[1], browser);
    } else {
        draw_list(frame, chunks[1], browser, browser.active);
    }

    draw_footer(frame, chunks[2], browser);

    if browser.show_help {
        draw_help_overlay(frame, area);
    }

    if browser.confirm.is_some() {
        draw_confirm_overlay(frame, area, browser);
    }
}

fn draw_header(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let pane = browser.active_pane();
    let vis = pane.visible().len();
    let total = pane.entries.len();
    let marked = pane.marked.len();
    let hidden = if browser.show_hidden {
        " · hidden"
    } else {
        ""
    };
    let rev = if browser.reverse { "↓" } else { "↑" };
    let filter = if pane.filter.is_empty() {
        String::new()
    } else {
        format!(" · /{}", pane.filter)
    };
    let err = browser
        .error
        .as_ref()
        .or(pane.error.as_ref())
        .map(|e| format!(" · err: {e}"))
        .unwrap_or_default();
    let marks = if marked == 0 {
        String::new()
    } else {
        format!(" · {marked} marked")
    };
    let dual = if browser.dual { " · dual" } else { "" };

    let title = format!(
        " {}  · pane {} · {vis}/{total}{hidden} · sort:{}{rev}{filter}{marks}{dual}{err} ",
        pane.cwd.display(),
        browser.active + 1,
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
    let text = match browser.active_pane().current_entry() {
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

fn draw_list(frame: &mut ratatui::Frame, area: Rect, browser: &mut Browser, pane_idx: usize) {
    let width = area.width;
    let show_long = width >= LONG_COLUMNS_MIN_WIDTH;
    let is_active = pane_idx == browser.active;
    let icons = browser.icons;
    let git = browser.git;

    // Snapshot pane fields we need so we can release the immutable borrow
    // before mutably borrowing list_state for render.
    let (items, title, border_color) = {
        let pane = &browser.panes[pane_idx];
        let vis = pane.visible();
        let marked = &pane.marked;
        let filtering = pane.filtering;
        let cwd_display = pane.cwd.display().to_string();
        let vis_len = vis.len();
        let total = pane.entries.len();
        let pane_err = pane.error.as_deref().unwrap_or("");

        let items: Vec<ListItem> = vis
            .iter()
            .map(|entry| {
                let is_marked = marked.contains(&entry.path);
                let mark = if is_marked { "● " } else { "  " };
                let icon = icon_prefix(entry, icons);
                let kind_tag = match entry.kind {
                    EntryKind::Directory => "/",
                    EntryKind::Symlink => "@",
                    _ => "",
                };
                let git_tag = if git {
                    entry
                        .git_status
                        .as_char()
                        .map(|c| format!(" [{c}]"))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                let name = format!("{mark}{icon}{}{kind_tag}{git_tag}", entry.name);

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

        let border_color = if is_active {
            Color::Cyan
        } else {
            Color::DarkGray
        };

        let active_tag = if is_active { " · ACTIVE" } else { "" };
        let err_tag = if pane_err.is_empty() {
            String::new()
        } else {
            format!(" · err:{pane_err}")
        };
        let title = if filtering {
            format!(" filter (Esc clear · Enter apply){active_tag} ")
        } else {
            format!(" {cwd_display} · {vis_len}/{total}{active_tag}{err_tag} ")
        };

        (items, title, border_color)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(border_color));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("› ");

    frame.render_stateful_widget(list, area, &mut browser.panes[pane_idx].list_state);
}

fn draw_footer(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let hints = if browser.confirm.is_some() {
        " y confirm · n/Esc cancel ".to_string()
    } else if browser.active_pane().filtering {
        format!(
            " filter: /{}  · Esc clear · Enter done ",
            browser.active_pane().filter
        )
    } else {
        let status = if browser.status.is_empty() {
            String::new()
        } else {
            format!(" · {}", browser.status)
        };
        format!(
            " j/k · Tab pane · \\ dual · c copy · m move · d del · Space mark · y yield · s sort · p preview · e/v · H · q{status} "
        )
    };
    let footer = Paragraph::new(hints).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}

fn draw_confirm_overlay(frame: &mut ratatui::Frame, area: Rect, browser: &Browser) {
    let Some(op) = browser.confirm.as_ref() else {
        return;
    };
    let paths = op.paths();
    let mut lines = vec![
        format!(" {}  · {} item(s)", op.label(), paths.len()),
        String::new(),
    ];
    if let Some(dest) = op.dest_hint() {
        lines.push(format!(" → {}", dest.display()));
        lines.push(String::new());
    }
    for p in paths.iter().take(12) {
        lines.push(format!("  · {}", p.display()));
    }
    if paths.len() > 12 {
        lines.push(format!("  … and {} more", paths.len() - 12));
    }
    lines.push(String::new());
    lines.push(" y confirm · n / Esc cancel ".into());

    let text = lines.join("\n");
    let popup = centered_rect(70, 50, area);
    frame.render_widget(Clear, popup);
    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .title(format!(" confirm {} ", op.label()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(widget, popup);
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
  p              toggle preview pane (single-pane only)
  e              open file in $EDITOR (or $VISUAL / vi)
  v              open file in $PAGER (or less)
  H / ?          toggle this help
  q / Esc        quit (print nothing)

  Dual-pane file manager
  Tab            switch active pane
  \\ / |          toggle dual-pane (default on when width ≥ 80)
  c              copy marked/cursor → other pane cwd (confirm if multi)
  m              move marked/cursor → other pane cwd (confirm if multi)
  d / Delete     delete marked/cursor (confirm y/n)

  Long columns (perms, size, mtime) appear when the pane is wide enough.
  Preview shows metadata + text head or directory children (hidden in dual).

  Press Esc / q / H to close help.
";

    let popup = centered_rect(72, 86, area);
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
