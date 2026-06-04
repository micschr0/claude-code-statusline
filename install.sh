#!/usr/bin/env bash
# install.sh — one-command installer for claude-code-statusline
# Usage: curl -fsSL https://raw.githubusercontent.com/micschr0/claude-code-statusline/main/install.sh | bash
set -euo pipefail

REPO="https://raw.githubusercontent.com/micschr0/claude-code-statusline/main"
SCRIPT_DEST="$HOME/.claude/statusline-command.sh"
SETTINGS="$HOME/.claude/settings.json"

red()   { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
bold()  { printf '\033[1m%s\033[0m\n' "$*"; }

tmp_script=
tmp_cfg=
trap 'rm -f "$tmp_script" "$tmp_cfg"' EXIT

# ── Preflight ──────────────────────────────────────────────────────────────────
bold "Checking dependencies..."
echo "  Requires a Nerd Font set as your terminal font — https://www.nerdfonts.com"
echo ""
install_hint() {
  case "$1" in
    jq)  echo "  macOS:  brew install jq" ;;
    git) echo "  macOS:  brew install git" ;;
  esac
  echo "  Linux:  sudo apt install $1   # or: sudo dnf install $1"
}

ok=true
for cmd in jq git; do
  if command -v "$cmd" >/dev/null 2>&1; then
    printf '  %-6s %s\n' "$cmd" "$(command -v "$cmd")"
  else
    red "  $cmd    not found"
    install_hint "$cmd"
    ok=false
  fi
done
if [ "$ok" = false ]; then
  echo ""
  red "Install the missing tools above, then re-run this script."
  exit 1
fi
echo ""

mkdir -p "$HOME/.claude"

# ── Download script ────────────────────────────────────────────────────────────
printf 'Downloading statusline-command.sh ... '
tmp_script=$(mktemp)
curl -fsSL "$REPO/statusline-command.sh" -o "$tmp_script"
if [ ! -s "$tmp_script" ]; then
  red "Download failed — file is empty"
  exit 1
fi
mv "$tmp_script" "$SCRIPT_DEST"
chmod +x "$SCRIPT_DEST"
green "done"

# ── Patch settings.json ────────────────────────────────────────────────────────
STATUS_LINE_VALUE='{"type":"command","command":"bash ~/.claude/statusline-command.sh"}'

if [ -f "$SETTINGS" ]; then
  backup="${SETTINGS}.backup.$(date +%s)"
  cp "$SETTINGS" "$backup"
  printf 'Backed up settings.json to %s\n' "$backup"
  tmp_cfg=$(mktemp)
  if ! jq --argjson v "$STATUS_LINE_VALUE" '.statusLine = $v' "$SETTINGS" > "$tmp_cfg"; then
    red "$SETTINGS is not valid JSON — fix it manually before re-running."
    exit 1
  fi
  mv "$tmp_cfg" "$SETTINGS"
else
  jq -n --argjson v "$STATUS_LINE_VALUE" '{statusLine:$v}' > "$SETTINGS"
fi
green "settings.json updated"

# ── Done ───────────────────────────────────────────────────────────────────────
echo ""
bold "Installation complete."
echo "Restart Claude Code — the statusline appears on the next turn."
echo ""
echo "If glyphs show as boxes, install a Nerd Font and set it as your terminal font."
echo "Troubleshooting: https://github.com/micschr0/claude-code-statusline#troubleshooting"
