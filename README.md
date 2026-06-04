# claude-code-statusline

A Powerline-style status bar for the Claude Code CLI. Claude Code pipes session
JSON to the script on every turn; the script renders ANSI-colored segments to
the terminal's status line area.

## What it looks like

![claude-code-statusline screenshot](screenshot.svg)

Segments from left to right:

| Segment | Example | Description |
|---------|---------|-------------|
| Directory | `~/p/my-project` | Fish-style abbreviated path |
| Git | ` main ↑1 ↓0 M2 ?1` | Branch, ahead/behind, modified, untracked |
| Context window | `󰍛 ━━━━╌╌ 67%  ⬡ 42.3k` | Usage bar + token count |
| Rate limit (5h) | `󰔟 ━━━╌╌╌ 48%  ↺ 2h13m` | 5-hour window bar + live reset countdown |
| Rate limit (weekly) | `󰃭 ━━━━╌╌ 63%  ↺ 3d4h` | 7-day window — appears only when ≥ 50 % used |
| Model | `◈ Claude Sonnet 4.6` | Active model name |

The context and rate-limit bars change color automatically:

- Green — below 50 % used
- Yellow — 50–79 % used
- Red — 80 %+ used

The 5-hour reset countdown is always shown. The weekly (7-day) segment stays
hidden until it passes 50 % used, so it surfaces only when it actually
constrains you. Both rate-limit windows appear only for Claude.ai Pro/Max
subscribers (API-key sessions omit them).

## Installation

**Easy way:** Copy the [installation prompt](INSTALL.prompt) and give it to Claude Code. Claude will handle the download, setup, and configuration.

**Manual way:** 

```bash
chmod +x statusline-command.sh
cp statusline-command.sh ~/.claude/statusline-command.sh

# Then add this to ~/.claude/settings.json:
{
  "statusLine": {
    "type": "command",
    "command": "bash ~/.claude/statusline-command.sh"
  }
}
```

## Requirements

- [Claude Code](https://docs.anthropic.com/claude-code) CLI, installed and authenticated
- `bash` 3.2+ and standard utilities: `jq`, `git`, `awk`, `date`
- A [Nerd Font](https://www.nerdfonts.com/) or Powerline-patched font (optional — glyphs fall back to ASCII)

## Customization

All colors are defined as named constants at the top of the script — change any
of them without touching the rendering logic:

```bash
C_DIR="${ESC}[38;5;33m"   # blue — directory segment
C_GIT="${ESC}[38;5;141m"  # lavender — git branch
C_OK="${ESC}[38;5;114m"   # green — healthy resource bars
C_WARN="${ESC}[38;5;221m" # yellow — approaching limit
C_CRIT="${ESC}[38;5;203m" # red — critical / over limit
# ... see script for full list
```

Colors use 256-color ANSI codes (`38;5;<n>`). The defaults follow the
[Tokyo Night](https://github.com/folke/tokyonight.nvim) palette and are tuned
for dark backgrounds.

To find a color number you like:

```bash
for i in $(seq 0 255); do printf "\e[38;5;${i}m %3d \e[0m" $i; done; echo
```

## How it works

Claude Code invokes the script on every turn, writing a JSON object to stdin:

```json
{
  "cwd": "/home/user/my-project",
  "context_window": {
    "total_input_tokens": 35000,
    "total_output_tokens": 7300,
    "used_percentage": 42.5
  },
  "rate_limits": {
    "five_hour": {
      "used_percentage": 12.0,
      "resets_at": 1748900400
    }
  },
  "model": { "display_name": "Claude Sonnet 4.6" }
}
```

The script parses the entire payload in a single `jq` call (avoiding repeated
forks), runs lightweight `git` commands against `cwd`, builds the bar
characters with pure Bash arithmetic, and writes the colored output to stdout.

## License

MIT — see [LICENSE](LICENSE)
