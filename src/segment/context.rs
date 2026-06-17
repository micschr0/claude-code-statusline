//! Context (tokens + usage bar) segment — STUB, to be implemented by the
//! context worker.
//!
//! Contract (matches the bash script's "session tokens + context bar"):
//! ```text
//! total = context_window.total_input_tokens + total_output_tokens
//!   (each via Coerce::or_default). If total == 0 -> emit nothing, return false.
//! Format the count with crate::sanitize::fmt_tokens.
//! If used_percentage present and in range 0..=999 (round to nearest int first):
//!   pick the bar color by threshold:
//!     pct > 100 -> bar_crit; pct >= th.crit -> bar_crit;
//!     pct >= th.warn -> bar_warn; else bar_ok.
//!   Emit: context icon (style.glyphs.context, dim) + bar (width th.bar_width,
//!   fill = chosen color) + " " + "<pct>%" in the chosen color + " " +
//!   token icon (style.glyphs.token) + count in theme.token.
//!   Bash: ${C_DIM}<ctx> %bar% ${cc}%d%%${R} ${C_TOK}<tok> %s${R}
//! If used_percentage absent/out-of-range: emit only the token part
//!   (token icon + count in theme.token).
//! Return true.
//! ```

use crate::render::SegmentWriter;
use crate::segment::{RenderCtx, Segment};

pub struct Context;

impl Segment for Context {
    fn render(&self, _ctx: &RenderCtx, _out: &mut SegmentWriter) -> bool {
        false
    }
}
