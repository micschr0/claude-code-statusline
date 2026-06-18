//! Configurator state and the pure logic that mutates it. Drawing lives in
//! `ui.rs`; this module is kept free of ratatui draw calls so its helpers can be
//! unit-tested directly.

use crate::model::{Config, SegmentKind};
use crate::tui::sample::{self, Sample};
use std::path::PathBuf;

/// Which of the three top panels currently has keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Segments,
    Theme,
    Style,
}

impl Focus {
    /// Next panel in `Tab` order.
    pub fn next(self) -> Focus {
        match self {
            Focus::Segments => Focus::Theme,
            Focus::Theme => Focus::Style,
            Focus::Style => Focus::Segments,
        }
    }

    /// Previous panel in `Shift-Tab` order.
    pub fn prev(self) -> Focus {
        match self {
            Focus::Segments => Focus::Style,
            Focus::Theme => Focus::Segments,
            Focus::Style => Focus::Theme,
        }
    }
}

/// Direction a reorder moves the focused segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
}

/// Full configurator state.
pub struct App {
    pub config: Config,
    pub save_path: Option<PathBuf>,
    pub focus: Focus,
    /// Cursor into [`SegmentKind::ALL`] for the Segments panel.
    pub seg_cursor: usize,
    /// Cursor into [`themes::NAMES`].
    pub theme_cursor: usize,
    /// Cursor into [`styles::NAMES`].
    pub style_cursor: usize,
    pub samples: Vec<Sample>,
    pub sample_idx: usize,
    /// Transient one-line status (e.g. "saved").
    pub status: Option<String>,
    pub should_quit: bool,
}

impl App {
    /// Build state from a loaded config and its resolved save path.
    pub fn new(config: Config, save_path: Option<PathBuf>) -> App {
        let theme_cursor = crate::themes::NAMES
            .iter()
            .position(|n| *n == config.theme)
            .unwrap_or(0);
        let style_cursor = crate::styles::NAMES
            .iter()
            .position(|n| *n == config.style)
            .unwrap_or(0);
        App {
            config,
            save_path,
            focus: Focus::Segments,
            seg_cursor: 0,
            theme_cursor,
            style_cursor,
            samples: sample::all(),
            sample_idx: 0,
            status: None,
            should_quit: false,
        }
    }

    /// Display order of the Segments panel: enabled segments in their
    /// `config.segments` order first, then any disabled ones. Reordering an
    /// enabled segment moves it within this list, so the cursor (an index into
    /// this order) keeps tracking the same segment.
    pub fn seg_order(&self) -> Vec<SegmentKind> {
        let mut order = self.config.segments.clone();
        for seg in SegmentKind::ALL {
            if !order.contains(&seg) {
                order.push(seg);
            }
        }
        order
    }

    /// The segment the Segments cursor points at.
    pub fn cursor_segment(&self) -> SegmentKind {
        self.seg_order()[self.seg_cursor]
    }

    /// The currently displayed preview sample.
    pub fn current_sample(&self) -> &Sample {
        &self.samples[self.sample_idx]
    }

    /// Move the cursor of the focused list by one, clamped to its bounds.
    pub fn move_cursor(&mut self, dir: Dir) {
        match self.focus {
            Focus::Segments => {
                self.seg_cursor = step(self.seg_cursor, SegmentKind::ALL.len(), dir);
            }
            Focus::Theme => {
                self.theme_cursor = step(self.theme_cursor, crate::themes::NAMES.len(), dir);
                self.config.theme = crate::themes::NAMES[self.theme_cursor].to_string();
            }
            Focus::Style => {
                self.style_cursor = step(self.style_cursor, crate::styles::NAMES.len(), dir);
                self.config.style = crate::styles::NAMES[self.style_cursor].to_string();
            }
        }
    }

    /// Toggle the cursor segment's enabled state (Segments panel only).
    pub fn toggle_cursor(&mut self) {
        let seg = self.cursor_segment();
        toggle_segment(&mut self.config.segments, seg);
    }

    /// Reorder the cursor segment within `config.segments` (Segments panel only).
    /// No-op if the segment is disabled (not in `config.segments`).
    pub fn reorder_cursor(&mut self, dir: Dir) {
        let seg = self.cursor_segment();
        if let Some(idx) = self.config.segments.iter().position(|s| *s == seg) {
            move_segment(&mut self.config.segments, idx, dir);
        }
    }

    /// Advance the preview sample.
    pub fn cycle_sample(&mut self) {
        self.sample_idx = (self.sample_idx + 1) % self.samples.len();
    }

    /// Reset config to default; resync the theme/style cursors.
    pub fn reset(&mut self) {
        self.config = Config::default();
        self.theme_cursor = crate::themes::NAMES
            .iter()
            .position(|n| *n == self.config.theme)
            .unwrap_or(0);
        self.style_cursor = crate::styles::NAMES
            .iter()
            .position(|n| *n == self.config.style)
            .unwrap_or(0);
        self.status = Some("reset to defaults".into());
    }

    /// Persist the config to `save_path`, recording the outcome in `status`.
    pub fn save(&mut self) {
        match &self.save_path {
            Some(path) => match self.config.save(path) {
                Ok(()) => self.status = Some(format!("saved to {}", path.display())),
                Err(e) => self.status = Some(format!("save failed: {e}")),
            },
            None => self.status = Some("no save path (set $HOME or --config)".into()),
        }
    }
}

/// Step `cursor` by `dir` within `[0, len)`, clamping at the ends.
fn step(cursor: usize, len: usize, dir: Dir) -> usize {
    match dir {
        Dir::Up => cursor.saturating_sub(1),
        Dir::Down => {
            if len == 0 {
                0
            } else {
                (cursor + 1).min(len - 1)
            }
        }
    }
}

/// Enable `seg` (append) if absent, else disable it (remove).
pub fn toggle_segment(segments: &mut Vec<SegmentKind>, seg: SegmentKind) {
    if let Some(idx) = segments.iter().position(|s| *s == seg) {
        segments.remove(idx);
    } else {
        segments.push(seg);
    }
}

/// Swap the element at `idx` with its neighbor in `dir`. Out-of-range or
/// boundary moves are no-ops.
pub fn move_segment(segments: &mut [SegmentKind], idx: usize, dir: Dir) {
    if idx >= segments.len() {
        return;
    }
    match dir {
        Dir::Up if idx > 0 => segments.swap(idx, idx - 1),
        Dir::Down if idx + 1 < segments.len() => segments.swap(idx, idx + 1),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_enables_absent_segment() {
        let mut segs = vec![SegmentKind::Directory];
        toggle_segment(&mut segs, SegmentKind::Git);
        assert_eq!(segs, vec![SegmentKind::Directory, SegmentKind::Git]);
    }

    #[test]
    fn toggle_disables_present_segment() {
        let mut segs = vec![SegmentKind::Directory, SegmentKind::Git];
        toggle_segment(&mut segs, SegmentKind::Directory);
        assert_eq!(segs, vec![SegmentKind::Git]);
    }

    #[test]
    fn toggle_is_involutive() {
        let mut segs = vec![SegmentKind::Context];
        toggle_segment(&mut segs, SegmentKind::Git);
        toggle_segment(&mut segs, SegmentKind::Git);
        assert_eq!(segs, vec![SegmentKind::Context]);
    }

    #[test]
    fn move_up_swaps_with_predecessor() {
        let mut segs = vec![SegmentKind::Directory, SegmentKind::Git, SegmentKind::Model];
        move_segment(&mut segs, 1, Dir::Up);
        assert_eq!(
            segs,
            vec![SegmentKind::Git, SegmentKind::Directory, SegmentKind::Model]
        );
    }

    #[test]
    fn move_down_swaps_with_successor() {
        let mut segs = vec![SegmentKind::Directory, SegmentKind::Git, SegmentKind::Model];
        move_segment(&mut segs, 1, Dir::Down);
        assert_eq!(
            segs,
            vec![SegmentKind::Directory, SegmentKind::Model, SegmentKind::Git]
        );
    }

    #[test]
    fn move_at_boundary_is_noop() {
        let mut segs = vec![SegmentKind::Directory, SegmentKind::Git];
        move_segment(&mut segs, 0, Dir::Up);
        move_segment(&mut segs, 1, Dir::Down);
        assert_eq!(segs, vec![SegmentKind::Directory, SegmentKind::Git]);
    }

    #[test]
    fn move_out_of_range_is_noop() {
        let mut segs = vec![SegmentKind::Directory];
        move_segment(&mut segs, 5, Dir::Up);
        assert_eq!(segs, vec![SegmentKind::Directory]);
    }

    #[test]
    fn focus_cycles_both_ways() {
        assert_eq!(Focus::Segments.next(), Focus::Theme);
        assert_eq!(Focus::Theme.next(), Focus::Style);
        assert_eq!(Focus::Style.next(), Focus::Segments);
        assert_eq!(Focus::Segments.prev(), Focus::Style);
    }

    #[test]
    fn step_clamps_at_ends() {
        assert_eq!(step(0, 3, Dir::Up), 0);
        assert_eq!(step(2, 3, Dir::Down), 2);
        assert_eq!(step(1, 3, Dir::Down), 2);
        assert_eq!(step(1, 3, Dir::Up), 0);
        assert_eq!(step(0, 0, Dir::Down), 0);
    }

    #[test]
    fn new_syncs_theme_style_cursors() {
        let cfg = Config {
            theme: "nord".into(),
            style: "ascii".into(),
            ..Config::default()
        };
        let app = App::new(cfg, None);
        assert_eq!(crate::themes::NAMES[app.theme_cursor], "nord");
        assert_eq!(crate::styles::NAMES[app.style_cursor], "ascii");
    }

    #[test]
    fn moving_theme_cursor_updates_config() {
        let mut app = App::new(Config::default(), None);
        app.focus = Focus::Theme;
        let before = app.config.theme.clone();
        app.move_cursor(Dir::Down);
        assert_ne!(app.config.theme, before);
        assert_eq!(app.config.theme, crate::themes::NAMES[app.theme_cursor]);
    }

    #[test]
    fn reset_restores_defaults_and_cursors() {
        let mut app = App::new(Config::default(), None);
        app.config.theme = "dracula".into();
        app.config.segments.clear();
        app.reset();
        assert_eq!(app.config, Config::default());
        assert_eq!(crate::themes::NAMES[app.theme_cursor], app.config.theme);
    }

    #[test]
    fn seg_order_lists_disabled_after_enabled() {
        let cfg = Config {
            segments: vec![SegmentKind::Model, SegmentKind::Git],
            ..Config::default()
        };
        let app = App::new(cfg, None);
        let order = app.seg_order();
        // Enabled (in config order) come first, then the remaining disabled ones.
        assert_eq!(&order[..2], &[SegmentKind::Model, SegmentKind::Git]);
        assert_eq!(order.len(), SegmentKind::ALL.len());
        for seg in SegmentKind::ALL {
            assert!(order.contains(&seg));
        }
    }

    #[test]
    fn reorder_follows_cursor_in_display_order() {
        // Cursor on the second enabled row; reordering it up swaps it with the
        // first, and the cursor (a display-order index) now points at it.
        let mut app = App::new(Config::default(), None);
        app.seg_cursor = 1;
        let moved = app.cursor_segment();
        app.reorder_cursor(Dir::Up);
        assert_eq!(app.config.segments[0], moved);
        // Same display index now shows the segment that was bumped down.
        assert_eq!(app.seg_order()[1], SegmentKind::ALL[0]);
    }

    #[test]
    fn cycle_sample_wraps() {
        let mut app = App::new(Config::default(), None);
        let n = app.samples.len();
        for _ in 0..n {
            app.cycle_sample();
        }
        assert_eq!(app.sample_idx, 0);
    }
}
