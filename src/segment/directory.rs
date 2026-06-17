//! Directory segment — STUB, to be implemented by the directory worker.
//!
//! Contract (matches the bash script's directory segment):
//! - If `ctx.input.cwd` is empty/None → emit nothing, return `false`.
//! - Abbreviate with [`crate::sanitize::abbreviate_path`] (fish style, `~` home,
//!   passing `ctx.home`).
//! - Emit a single leading space, then the abbreviated path in `theme.dir`:
//!   bash does `printf "${C_DIR} %s${R}"`. There is **no icon** on this segment.
//! - The path is host-provided; `abbreviate_path` already strips control bytes.
//! - Return `true` when a path was emitted.

use crate::render::SegmentWriter;
use crate::segment::{RenderCtx, Segment};

pub struct Directory;

impl Segment for Directory {
    fn render(&self, _ctx: &RenderCtx, _out: &mut SegmentWriter) -> bool {
        false
    }
}
