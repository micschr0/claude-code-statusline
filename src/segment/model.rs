//! Model + effort segment — STUB, to be implemented by the model worker.
//!
//! Contract (matches the bash script's "model + effort" block):
//! ```text
//! Strip control bytes from display_name and effort.level (host-provided)
//!   via crate::sanitize::strip_control.
//! If both are empty/None -> emit nothing, return false.
//! If a model name is present: emit model icon (style.glyphs.model, dim) +
//!   name in theme.model. Bash: ${C_MOD}<model> %s${R}
//! If an effort level is present: when a model name was also emitted, emit a
//!   separating space first; then effort icon (style.glyphs.effort, dim) + the
//!   level string colored by level:
//!     low|medium -> theme.dim; high -> bar_ok; xhigh -> bar_warn;
//!     max -> theme.effort_max; anything else -> theme.dim.
//!   Note: effort is ABSENT for models without the param — gate on presence,
//!   never on a specific value.
//! Return true if anything was emitted.
//! ```

use crate::render::SegmentWriter;
use crate::segment::{RenderCtx, Segment};

pub struct Model;

impl Segment for Model {
    fn render(&self, _ctx: &RenderCtx, _out: &mut SegmentWriter) -> bool {
        false
    }
}
