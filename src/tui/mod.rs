//! TUI configurator: three panels (segments toggle+reorder, theme picker, style
//! picker) plus a full-width live-preview strip that reuses [`crate::render`] so
//! the preview is byte-identical to the hook's output. Save writes the edited
//! [`crate::model::Config`] back to the config path as TOML.

mod app;
mod preview;
mod sample;
mod ui;

use app::{App, Dir, Focus};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;

/// RAII guard: the terminal is restored when this drops, including on panic or
/// any early return from the event loop.
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    fn enter() -> Result<TerminalGuard, String> {
        enable_raw_mode().map_err(|e| format!("enable raw mode: {e}"))?;
        let mut out = io::stdout();
        out.execute(EnterAlternateScreen)
            .map_err(|e| format!("enter alternate screen: {e}"))?;
        let backend = CrosstermBackend::new(out);
        let terminal = Terminal::new(backend).map_err(|e| format!("init terminal: {e}"))?;
        Ok(TerminalGuard { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

/// Run the interactive configurator, persisting to `config_path` on save.
pub fn run(config_path: Option<PathBuf>) -> Result<(), String> {
    let config = crate::model::Config::load_or_default(config_path.as_deref());
    let save_path = config_path.or_else(crate::model::Config::default_path);
    let mut app = App::new(config, save_path);

    let mut guard = TerminalGuard::enter()?;
    let res = event_loop(&mut guard.terminal, &mut app);
    drop(guard);
    res
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), String> {
    loop {
        terminal
            .draw(|f| ui::draw(f, app))
            .map_err(|e| format!("draw: {e}"))?;

        // Poll so the loop stays responsive; only key presses drive state.
        if event::poll(Duration::from_millis(200)).map_err(|e| format!("poll: {e}"))? {
            if let Event::Key(key) = event::read().map_err(|e| format!("read: {e}"))? {
                if key.kind == KeyEventKind::Press {
                    handle_key(app, key);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    // Any keypress clears the transient status line.
    app.status = None;
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Tab => app.focus = app.focus.next(),
        KeyCode::BackTab => app.focus = app.focus.prev(),
        KeyCode::Char('s') => app.save(),
        KeyCode::Char('r') => app.reset(),
        KeyCode::Char('p') => app.cycle_sample(),
        // Shift+Up/Down reorders in the Segments panel; elsewhere it is inert
        // (so it can't silently change the live theme/style selection).
        KeyCode::Up if shift => {
            if app.focus == Focus::Segments {
                app.reorder_cursor(Dir::Up);
            }
        }
        KeyCode::Down if shift => {
            if app.focus == Focus::Segments {
                app.reorder_cursor(Dir::Down);
            }
        }
        KeyCode::Up => app.move_cursor(Dir::Up),
        KeyCode::Down => app.move_cursor(Dir::Down),
        KeyCode::Char('k') | KeyCode::Char('K') if app.focus == Focus::Segments => {
            app.reorder_cursor(Dir::Up)
        }
        KeyCode::Char('j') | KeyCode::Char('J') if app.focus == Focus::Segments => {
            app.reorder_cursor(Dir::Down)
        }
        KeyCode::Char(' ') if app.focus == Focus::Segments => app.toggle_cursor(),
        _ => {}
    }
}
