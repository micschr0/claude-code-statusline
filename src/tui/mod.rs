//! TUI configurator — STUB. The TUI worker owns this whole subtree
//! (`tui/{mod,app,ui,preview,sample}.rs`) and replaces this with a ratatui app.
//!
//! Design (from the plan): three panels (segments with toggle+reorder, theme
//! picker, style picker) plus a full-width live-preview strip that calls
//! [`crate::render::render_with`] / [`crate::render_line`] on a sample
//! `InputData` so the preview is byte-identical to the hook's output. Save
//! writes the edited [`crate::model::Config`] to `config_path` as TOML.

use std::path::PathBuf;

/// Run the interactive configurator, persisting to `config_path` on save.
pub fn run(_config_path: Option<PathBuf>) -> Result<(), String> {
    Err("the TUI configurator is not built in this binary yet".into())
}
