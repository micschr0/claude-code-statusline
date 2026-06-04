#!/usr/bin/env bash
# Nimmt einen echten PNG-Screenshot der Statusline via Docker + Playwright
# Aufruf: bash take_screenshot.sh
set -euo pipefail

REPO="$(cd "$(dirname "$0")" && pwd)"
HTML_FILE="$REPO/screenshot_src.html"
OUT_PNG="$REPO/screenshot.png"

echo "Baue Screenshot via Docker..."

docker run --rm \
  -v "$REPO:/work" \
  --ipc=host \
  mcr.microsoft.com/playwright:v1.49.0-noble \
  bash -c '
    cd /work
    node - << "JSEOF"
const { chromium } = require("playwright");
(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.setViewportSize({ width: 964, height: 800 });
  await page.goto("file:///work/screenshot_src.html");
  // Warte auf Font-Load
  await page.waitForTimeout(1500);
  await page.screenshot({
    path: "/work/screenshot.png",
    clip: { x: 0, y: 0, width: 964, height: (await page.evaluate(() => document.body.scrollHeight)) },
    omitBackground: false,
  });
  await browser.close();
  console.log("Screenshot gespeichert.");
})();
JSEOF
  '

echo "Fertig: $OUT_PNG"
ls -lh "$OUT_PNG"
