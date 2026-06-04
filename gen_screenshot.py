#!/usr/bin/env python3
"""Generate animated screenshot SVG cycling through statusline states."""
import subprocess, re, time

SCRIPT   = "/home/claude/projects/claude-code-statusline/statusline-command.sh"
OUT      = "/home/claude/projects/claude-code-statusline/screenshot.svg"
DEMO_CWD = "/tmp/demo-app"


# Layout
W          = 820
LH         = 21
PAD_X      = 20
PAD_Y      = 10
TITLEBAR_H = 38
STATUS_H   = 34
BOTTOM_PAD = 8

# Colors
BG, BAR_BG, WIN_BORDER = "#1a1b2e", "#1f2035", "#2a2b3d"
TITLE_SEP = "#16172a"
SL_SEP    = "#33344a"
SL_BG     = "#13141f"
FG        = "#c0caf5"
C = {
    "dim":    "#565f89",
    "muted":  "#8a8a8a",
    "green":  "#9ece6a",
    "purple": "#bb9af7",
    "yellow": "#e0af68",
}

FONT_CONTENT = ("font-family=\"'SF Mono','Menlo','JetBrains Mono','Courier New',monospace\""
                " font-size=\"13\"")
FONT_SL      = ("font-family=\"'Noto Sans Mono','SF Mono','Menlo','JetBrains Mono','Courier New',monospace\""
                " font-size=\"13\"")
FONT_TITLE   = ("font-family=\"-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif\""
                " font-size=\"11.5\"")

def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")

def span(text, color=None):
    fill = f'fill="{color}"' if color else f'fill="{FG}"'
    return f'<tspan {fill}>{esc(text)}</tspan>'

def text_line(x, y, parts):
    inner = "".join(span(t, c) for t, c in parts)
    return f'<text x="{x}" y="{y}" {FONT_CONTENT}>{inner}</text>'

def ansi256(n):
    if n < 16:
        t = [(0,0,0),(128,0,0),(0,128,0),(128,128,0),(0,0,128),(128,0,128),
             (0,128,128),(192,192,192),(128,128,128),(255,0,0),(0,255,0),
             (255,255,0),(0,0,255),(255,0,255),(0,255,255),(255,255,255)]
        r, g, b = t[n]
    elif n < 232:
        n -= 16; v = [0, 95, 135, 175, 215, 255]
        r, g, b = v[n // 36], v[(n % 36) // 6], v[n % 6]
    else:
        c = 8 + (n - 232) * 10; r = g = b = c
    return f"#{r:02x}{g:02x}{b:02x}"

def parse_ansi(text):
    spans, color, pos = [], "#a9b1d6", 0
    for m in re.finditer(r'\x1b\[([0-9;]*)m', text):
        if m.start() > pos:
            spans.append((color, text[pos:m.start()]))
        parts = m.group(1).split(";")
        if parts[0] in ("0", ""):
            color = "#a9b1d6"
        elif len(parts) >= 3 and parts[0] == "38" and parts[1] == "5":
            color = ansi256(int(parts[2]))
        pos = m.end()
    if pos < len(text):
        spans.append((color, text[pos:]))
    return [(c, t) for c, t in spans if t]

def run_statusline(ctx_pct, tok_in, tok_out, rl_pct, reset_in):
    now = int(time.time())
    j = (f'{{"cwd":"{DEMO_CWD}",'
         f'"context_window":{{"total_input_tokens":{tok_in},"total_output_tokens":{tok_out},"used_percentage":{ctx_pct}}},'
         f'"rate_limits":{{"five_hour":{{"used_percentage":{rl_pct},"resets_at":{now + reset_in}}}}},'
         f'"model":{{"display_name":"Claude Sonnet 4.6"}}}}')
    return subprocess.run(["bash", SCRIPT], input=j, capture_output=True, text=True).stdout.rstrip("\n")

# --- Animation states ---
# (label, ctx_pct, tok_in, tok_out, rl_pct, reset_in_seconds)
STATES = [
    ("normal",    67.0,  55000,  9200,  38.0, 12000),
    ("warning",   72.0,  90000, 18000,  62.0,  6300),
    ("critical",  88.0, 140000, 26000,  80.0,  2700),
    ("overlimit", 101.0, 160000, 8000,  93.0,   900),
]

# Labels shown above statusline during animation
STATE_LABELS = {
    "normal":    "Normal — all good",
    "warning":   "Warning — context filling up",
    "critical":  "Critical — rate limit approaching",
    "overlimit": "Over limit — context bar hidden",
}

# Animation timing
N       = len(STATES)
CYCLE   = 16.0   # seconds per full loop
HOLD    = CYCLE / N          # 4s per state
FADE    = 0.4                # fade in/out duration
# Keyframe percentages for one state's visibility window
# (0..HOLD/CYCLE of one state's slice)
pct = lambda s: f"{s:.3f}%"

def keyframes_for_state(i):
    """CSS keyframe: visible during [i*HOLD .. (i+1)*HOLD] of CYCLE."""
    start  = i * HOLD / CYCLE * 100
    end    = (i + 1) * HOLD / CYCLE * 100
    fin    = FADE / CYCLE * 100
    # Clamp to avoid going past 100%
    e_fade = min(end, 100.0)
    return (
        f"0%         {{ opacity: 0 }}\n"
        f"        {pct(max(start - fin, 0))} {{ opacity: 0 }}\n"
        f"        {pct(start)}         {{ opacity: 1 }}\n"
        f"        {pct(e_fade - fin)}  {{ opacity: 1 }}\n"
        f"        {pct(e_fade)}        {{ opacity: 0 }}\n"
        f"        100%        {{ opacity: 0 }}"
    )

# Simulated Claude Code content
lines = [
    [("❯ ", C["purple"]), ("# refactor auth middleware to use JWT validation", C["dim"])],
    [],
    [("⏺ ", C["green"]), ("Read", C["muted"]), ("(src/auth.rs)", C["dim"])],
    [("⏺ ", C["green"]), ("Read", C["muted"]), ("(src/config/jwt.rs)", C["dim"])],
    [],
    [("Replacing DB-backed session validation with stateless JWT verification.", FG)],
    [("DB lookup is kept only for token revocation checks.", FG)],
    [],
    [("⏺ ", C["green"]), ("Edit", C["muted"]), ("(src/auth.rs) ", C["dim"]), ("+47 -23", C["muted"])],
    [("⏺ ", C["green"]), ("Edit", C["muted"]), ("(src/config/jwt.rs) ", C["dim"]), ("+12 -4", C["muted"])],
    [("⏺ ", C["green"]), ("Bash", C["muted"]), ("(cargo test middleware -- --nocapture)", C["dim"])],
    [],
    [("All 14 tests pass. Set ", FG), ("JWT_SECRET", C["yellow"]), (" env var before deploying.", FG)],
]

N_LINES = len(lines)
TOTAL_H = TITLEBAR_H + PAD_Y + N_LINES * LH + STATUS_H + BOTTOM_PAD
sep_y   = TITLEBAR_H + PAD_Y + N_LINES * LH + 4
sl_y    = sep_y + STATUS_H - 8
lbl_y   = sep_y + 13  # label baseline inside statusline bar

# --- Generate real ANSI output for each state ---
state_spans = []
for label, ctx_pct, tok_in, tok_out, rl_pct, reset_in in STATES:
    ansi_out = run_statusline(ctx_pct, tok_in, tok_out, rl_pct, reset_in)
    spans    = parse_ansi(ansi_out)
    plain    = re.sub(r'\x1b\[[^m]*m', '', ansi_out)
    state_spans.append((label, spans))
    print(f"  [{label:10s}] {plain}")

# --- Build SVG ---
out = []
out.append(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{TOTAL_H}" viewBox="0 0 {W} {TOTAL_H}">')
out.append(f'  <defs>')

# CSS: font-face + keyframe animations per state
css_lines = []
for i, (label, _) in enumerate(state_spans):
    kf = keyframes_for_state(i)
    css_lines.append(f'@keyframes show-{label} {{\n        {kf}\n      }}')
    css_lines.append(
        f'.sl-{label} {{ opacity: 0; animation: show-{label} {CYCLE:.1f}s ease-in-out infinite; }}'
    )

out.append('    <style>\n      ' + '\n      '.join(css_lines) + '\n    </style>')
out.append(f'    <clipPath id="win"><rect width="{W}" height="{TOTAL_H}" rx="10"/></clipPath>')
out.append(f'  </defs>')

# Window border
out.append(f'  <rect width="{W}" height="{TOTAL_H}" rx="10" fill="{WIN_BORDER}"/>')
out.append(f'  <g clip-path="url(#win)">')
out.append(f'    <rect width="{W}" height="{TOTAL_H}" fill="{BG}"/>')

# Title bar
out.append(f'    <rect width="{W}" height="{TITLEBAR_H}" fill="{BAR_BG}"/>')
out.append(f'    <rect y="{TITLEBAR_H - 1}" width="{W}" height="1" fill="{TITLE_SEP}"/>')
for i, col in enumerate(["#ff5f57", "#febc2e", "#28c840"]):
    out.append(f'    <circle cx="{20 + i * 20}" cy="{TITLEBAR_H // 2}" r="6" fill="{col}"/>')
cy = TITLEBAR_H // 2 + 4
out.append(f'    <text x="{W // 2}" y="{cy}" text-anchor="middle" {FONT_TITLE} fill="{C["muted"]}">claude — /tmp/demo-app</text>')

# Static content lines
for i, parts in enumerate(lines):
    if not parts:
        continue
    y = TITLEBAR_H + PAD_Y + (i + 1) * LH
    out.append(f'    {text_line(PAD_X, y, parts)}')

# Statusline area (static bg + separator)
out.append(f'    <rect x="0" y="{sep_y}" width="{W}" height="1" fill="{SL_SEP}"/>')
out.append(f'    <rect x="0" y="{sep_y + 1}" width="{W}" height="{STATUS_H}" fill="{SL_BG}"/>')

# Animated state label (top-right corner of statusline bar)
for i, (label, spans) in enumerate(state_spans):
    lbl_text = STATE_LABELS[label]
    out.append(
        f'    <text x="{W - PAD_X}" y="{lbl_y}" text-anchor="end" '
        f'{FONT_TITLE} fill="{C["dim"]}" class="sl-{label}">'
        f'{esc(lbl_text)}</text>'
    )

# Animated statusline text layers
for i, (label, spans) in enumerate(state_spans):
    tspans = "".join(f'<tspan fill="{c}">{esc(t)}</tspan>' for c, t in spans)
    out.append(
        f'    <text x="{PAD_X}" y="{sl_y}" {FONT_SL} xml:space="preserve" '
        f'class="sl-{label}">{tspans}</text>'
    )

out.append(f'  </g>')
out.append(f'</svg>')

svg = "\n".join(out)
with open(OUT, "w") as f:
    f.write(svg)
print(f"\nWritten {len(svg)} bytes → {OUT}")
print(f"Window: {W}×{TOTAL_H}px | Cycle: {CYCLE}s | {N} states × {HOLD}s each")
