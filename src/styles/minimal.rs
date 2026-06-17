//! minimal style — STUB. The styles worker replaces this with the real definition.
//! Until then it clones Powerline so the crate builds.
//!
//! Intended character (see plan): plain = " | " pipe separators, full glyphs;
//! rounded = rounded powerline caps; minimal = no icons (icons:false), spaces;
//! ascii = ASCII-only glyphs (^/v/M/?), icons:false, '#'/'-' bars, ASCII
//! separator — a safe fallback for fonts without Nerd glyphs.

use crate::model::Style;

pub fn style() -> Style {
    super::powerline::style()
}
