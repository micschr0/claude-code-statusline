#!/usr/bin/env python3
"""Generate screenshot.svg: macOS terminal window with statusline from real script output."""
import subprocess, re, time

SCRIPT = "/home/claude/projects/claude-code-statusline/statusline-command.sh"
OUT    = "/home/claude/projects/claude-code-statusline/screenshot.svg"
DEMO_CWD = "/tmp/demo-app"

# Embedded WOFF2 subset: U+E0B3 (Powerline ), U+F035B (󰍛), U+F051F (󰔟), U+F00ED (󰃭)
NF_WOFF2_B64 = (
    "d09GMgABAAAAAAJUAAsAAAAABGAAAAIJAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAABmAAgQQRCAqDIIJuATYCJAMMCwwABCAFZAcgG2MDYB6HsWP6hTPTJXcImsQDn87yjzTyk7UHQBVRR4CWjxCStEAV9wDUordLmfYuzdYm+x/v2/P/3RQ9PLxr0Aeg1ni+lkWHvSjkwHo+FkgvDajAAwl7N5Wa5bLVt/EOMRfuychGZS8Seo0bp38ok3YksioA7RABC8tpW8OHf75HpXyGShleQgBAHwERKQYgYqpUwACpAlNFfRBFgStVXd17vgBagImAuK0qELT2HyTROl0DSbXOej+RiPCcBGSgQAucxAeACAhSUazELjEOj7HIsuzk8aUnTi47fqJ6coc2oVDc+NXxc781nbz5Rzh5/veiCMdaxdYn21Xb10IRMnGhOFkqNKFUUTZpCiBVmsFwEXYV9go7on3cWovtsnp5vNyatD1RT9skK1paunzMSLN/eaMVizuYPceW2/X2t6s22sJk5PffD/++UVt+9Ohy2ROS0u2QqX1fWVoPTmx5e9CRN/j060ajefXly6ubG43Gn43uvfU/Af3771s1XZ/4vv70BXbraz4xrb+A1baRXQ/YbV+9sHnff0dXAwGBli9vfbyh9dj/82oOft3y4xlonjEo55lcpUQuAYEP1SUHefsBCDlTk1Kd8IKAzFgRBD6UGigVYlUwmEBMtDL4zGgwljhut8322uWgPuarOWCrPqbba49D+pgnX9xrsZq6w3bZ5IAUeB6UjwA="
)

# Layout
W         = 820
LH        = 21       # line height for content
PAD_X     = 20
PAD_Y     = 10       # tighter top gap (was 18)
TITLEBAR_H = 38      # slightly slimmer (was 40)
STATUS_H  = 34
BOTTOM_PAD = 8

# Colors — Tokyo Night
BG, BAR_BG, WIN_BORDER = "#1a1b2e", "#1f2035", "#2a2b3d"
TITLE_SEP = "#16172a"
SL_SEP    = "#33344a"   # brighter separator (was #2a2b3d)
SL_BG     = "#13141f"
FG        = "#c0caf5"
C = {
    "dim":    "#565f89",
    "muted":  "#8a8a8a",
    "green":  "#9ece6a",
    "purple": "#bb9af7",
    "yellow": "#e0af68",
}

# Font stacks
FONT_CONTENT = ("font-family=\"'SF Mono','Menlo','JetBrains Mono','Courier New',monospace\""
                " font-size=\"13\"")
FONT_SL      = ("font-family=\"NF,'SF Mono','Menlo','JetBrains Mono','Courier New',monospace\""
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

# --- 256-color ANSI parser ---

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

def run_statusline(cwd, ctx_pct, tok_in, tok_out, rl_5h_pct, rl_5h_reset_in, model):
    now = int(time.time())
    j = (f'{{"cwd":"{cwd}",'
         f'"context_window":{{"total_input_tokens":{tok_in},"total_output_tokens":{tok_out},"used_percentage":{ctx_pct}}},'
         f'"rate_limits":{{"five_hour":{{"used_percentage":{rl_5h_pct},"resets_at":{now + rl_5h_reset_in}}}}},'
         f'"model":{{"display_name":"{model}"}}}}')
    return subprocess.run(["bash", SCRIPT], input=j, capture_output=True, text=True).stdout.rstrip("\n")

# --- Simulated Claude Code session content ---
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

# --- Build SVG ---
out = []
out.append(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{TOTAL_H}" viewBox="0 0 {W} {TOTAL_H}">')
out.append(f'  <defs>')
out.append(f'    <style>')
out.append(f'      @font-face {{ font-family: "NF"; src: url("data:font/woff2;base64,{NF_WOFF2_B64}") format("woff2"); }}')
out.append(f'    </style>')
out.append(f'    <clipPath id="win"><rect width="{W}" height="{TOTAL_H}" rx="10"/></clipPath>')
out.append(f'  </defs>')

# Window border (visible on both light and dark GitHub themes)
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

# Content lines
for i, parts in enumerate(lines):
    if not parts:
        continue
    y = TITLEBAR_H + PAD_Y + (i + 1) * LH
    out.append(f'    {text_line(PAD_X, y, parts)}')

# Statusline area
sep_y = TITLEBAR_H + PAD_Y + N_LINES * LH + 4
out.append(f'    <rect x="0" y="{sep_y}" width="{W}" height="1" fill="{SL_SEP}"/>')
out.append(f'    <rect x="0" y="{sep_y + 1}" width="{W}" height="{STATUS_H}" fill="{SL_BG}"/>')

# Real statusline from script
ansi_out = run_statusline(
    cwd=DEMO_CWD,
    ctx_pct=67.0, tok_in=55000, tok_out=9200,
    rl_5h_pct=45.0, rl_5h_reset_in=8400,
    model="Claude Sonnet 4.6"
)
sl_spans = parse_ansi(ansi_out)
sl_y = sep_y + STATUS_H - 8
sl_tspans = "".join(f'<tspan fill="{c}">{esc(t)}</tspan>' for c, t in sl_spans)
out.append(f'    <text x="{PAD_X}" y="{sl_y}" {FONT_SL} xml:space="preserve">{sl_tspans}</text>')

out.append(f'  </g>')
out.append(f'</svg>')

svg = "\n".join(out)
with open(OUT, "w") as f:
    f.write(svg)
print(f"Written {len(svg)} bytes → {OUT}")
print(f"Window: {W}×{TOTAL_H}px")
print(f"Statusline: {re.sub(chr(27) + r'[^m]*m', '', ansi_out)}")
