//! Git segment — STUB, to be implemented by the git worker.
//!
//! Contract (matches the bash script's git segment):
//! - Only run when `ctx.input.cwd` is a non-empty **absolute** path (starts with
//!   `/`). Otherwise emit nothing, return `false`.
//! - Run exactly one git command:
//!   `git -C <cwd> -c gc.auto=0 status --branch --porcelain` (suppress stderr).
//!   If it fails or output is empty → return `false`.
//! - Parse the `## ` branch line:
//!   - `## No commits yet on <branch>` → branch = `<branch>`.
//!   - `## HEAD (no branch)` → no branch (detached) → return `false`.
//!   - `## <branch>...<upstream> [ahead N, behind M]` → branch before `...`,
//!     ahead/behind from the bracketed counts (each optional).
//! - Count the remaining porcelain lines: `?? ` prefixed → untracked (`n_new`);
//!   any other non-`## ` non-empty line → modified (`n_mod`).
//! - Strip control bytes from the branch name (host-provided) via
//!   [`crate::sanitize::strip_control`].
//! - Emit (using the writer + style glyphs): branch icon + branch in
//!   `theme.git_branch`; then ` ↑N` (theme.ahead) if ahead>0; ` ↓M`
//!   (theme.behind) if behind>0; ` MN` (theme.modified) if n_mod>0; ` ?N`
//!   (theme.untracked) if n_new>0. Glyphs come from `style.glyphs`
//!   (branch/ahead/behind/modified/untracked).
//! - Return `true` once a branch was emitted.
//!
//! Use `std::process::Command`. Keep it to the single git invocation.

use crate::render::SegmentWriter;
use crate::segment::{RenderCtx, Segment};

pub struct Git;

impl Segment for Git {
    fn render(&self, _ctx: &RenderCtx, _out: &mut SegmentWriter) -> bool {
        false
    }
}
