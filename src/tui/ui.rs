//! Drawing: lay out the three top panels and the preview strip, plus the key
//! hint line. No state mutation happens here.

use crate::tui::app::{App, Focus};
use crate::tui::preview;
use crate::{styles, themes};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

/// Top-level draw entrypoint.
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),    // panels
            Constraint::Length(3), // preview
            Constraint::Length(1), // status
            Constraint::Length(1), // key hint
        ])
        .split(f.area());

    draw_panels(f, app, chunks[0]);
    draw_preview(f, app, chunks[1]);
    draw_status(f, app, chunks[2]);
    draw_hint(f, chunks[3]);
}

fn draw_panels(f: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area);

    draw_segments(f, app, cols[0]);
    draw_theme(f, app, cols[1]);
    draw_style(f, app, cols[2]);
}

fn panel_block(title: &str, focused: bool) -> Block<'static> {
    let style = if focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    Block::default()
        .borders(Borders::ALL)
        .border_style(style)
        .title(format!(" {title} "))
}

fn draw_segments(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .seg_order()
        .iter()
        .map(|seg| {
            let enabled = app.config.segments.contains(seg);
            let order = app
                .config
                .segments
                .iter()
                .position(|s| s == seg)
                .map(|i| format!("{}.", i + 1))
                .unwrap_or_else(|| "  ".to_string());
            let check = if enabled { "[x]" } else { "[ ]" };
            let style = if enabled {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{order:<3} "), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{check} "), style),
                Span::raw(seg.label()),
            ]))
        })
        .collect();

    let focused = app.focus == Focus::Segments;
    let mut state = ListState::default();
    if focused {
        state.select(Some(app.seg_cursor));
    }
    let list = List::new(items)
        .block(panel_block("Segments", focused))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_theme(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::Theme;
    draw_picker(
        f,
        area,
        "Theme",
        themes::NAMES,
        app.theme_cursor,
        &app.config.theme,
        focused,
    );
}

fn draw_style(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::Style;
    draw_picker(
        f,
        area,
        "Style",
        styles::NAMES,
        app.style_cursor,
        &app.config.style,
        focused,
    );
}

fn draw_picker(
    f: &mut Frame,
    area: Rect,
    title: &str,
    names: &[&str],
    cursor: usize,
    selected: &str,
    focused: bool,
) {
    let items: Vec<ListItem> = names
        .iter()
        .map(|n| {
            let marker = if *n == selected { "● " } else { "  " };
            let style = if *n == selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(vec![Span::styled(marker, style), Span::raw(*n)]))
        })
        .collect();

    let mut state = ListState::default();
    if focused {
        state.select(Some(cursor));
    }
    let list = List::new(items)
        .block(panel_block(title, focused))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let sample = app.current_sample();
    let text = preview::render(&app.config, sample);
    let title = format!(" Preview — {} ", sample.name);
    let para = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(title),
    );
    f.render_widget(para, area);
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let text = app.status.clone().unwrap_or_default();
    let para = Paragraph::new(Line::from(Span::styled(
        text,
        Style::default().fg(Color::Yellow),
    )));
    f.render_widget(para, area);
}

fn draw_hint(f: &mut Frame, area: Rect) {
    let hint = "Tab/S-Tab focus  ↑/↓ move  Space toggle  J/K reorder  \
                p sample  s save  r reset  q quit";
    let para = Paragraph::new(Line::from(Span::styled(
        hint,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(para, area);
}
