//! catppuccin theme — STUB. The theme worker replaces this with a real palette,
//! filling every slot of [`crate::model::Theme`] with 256-color indices that
//! evoke the catppuccin palette. Until then it clones Tokyo Night so the crate builds.
//!
//! Keep the slot *semantics* (see `themes/tokyo_night.rs` and
//! `model/palette.rs`): bar_ok/warn/crit must read as green/yellow/red-ish for
//! the threshold colors to make sense; separator & bar_track should be a muted
//! background tone; ahead green-ish, behind red-ish, etc.

use crate::model::Theme;

pub fn theme() -> Theme {
    super::tokyo_night::theme()
}
