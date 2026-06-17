//! Rate-limits segment — STUB, to be implemented by the rate-limits worker.
//!
//! Contract (matches the bash script's "rate limits" block). Two windows; this
//! one segment renders both, separated by a single space (the segment is a unit;
//! the composer separator only appears around the whole segment).
//! ```text
//! 5-hour window (rate_limits.five_hour): shown whenever the window is present
//!   (a percentage OR a future resets_at).
//!   - If used_percentage present and in 0..=999 (rounded): color by
//!       pct >= th.crit -> bar_crit; pct >= th.warn -> bar_warn; else bar_ok.
//!       Emit clock icon (style.glyphs.clock, dim) + bar + " " + "<pct>%" colored.
//!   - Then the reset countdown via crate::sanitize::fmt_reset(resets_at, now):
//!       if Some, emit " " + reset icon (style.glyphs.reset, dim) + " " +
//!       "<countdown>" in theme.reset.
//!   Bash: ${C_DIM}<clock> %bar% ${rlc}%d%%${R} ${C_DIM}<reset>${R} ${C_RST}%s${R}
//!
//! Weekly window (rate_limits.seven_day): only surfaced once
//!   used_percentage >= th.weekly_show_at (and <= 999). Color: pct >= th.crit
//!   -> bar_crit, else bar_warn. Emit weekly icon (style.glyphs.weekly, dim) +
//!   bar + " " + "<pct>%" colored, then its own reset countdown like the 5h
//!   window. If shown after the 5h window, separate them with a single space.
//!
//! Emit nothing / return false when neither window has anything to show.
//! ```

use crate::render::SegmentWriter;
use crate::segment::{RenderCtx, Segment};

pub struct RateLimits;

impl Segment for RateLimits {
    fn render(&self, _ctx: &RenderCtx, _out: &mut SegmentWriter) -> bool {
        false
    }
}
