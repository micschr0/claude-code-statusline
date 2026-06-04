const { chromium } = require("/node_modules/playwright-core");
(async () => {
  const browser = await chromium.launch({
    executablePath: "/ms-playwright/chromium-1148/chrome-linux/chrome",
    args: ["--no-sandbox", "--disable-setuid-sandbox"]
  });

  const shots = [
    { src: "file:///work/screenshot_src.html",          out: "/work/screenshot.png" },
    { src: "file:///work/screenshot_critical_src.html", out: "/work/screenshot_critical.png" },
    { src: "file:///work/screenshot_overlimit_src.html",out: "/work/screenshot_overlimit.png" },
  ];

  for (const { src, out } of shots) {
    const page = await browser.newPage();
    await page.setViewportSize({ width: 1200, height: 800 });
    await page.goto(src);
    await page.waitForTimeout(1500);
    const height = await page.evaluate(() => document.body.scrollHeight);
    await page.screenshot({ path: out, clip: { x: 0, y: 0, width: 1200, height } });
    await page.close();
    console.log("Saved:", out, `(${height}px)`);
  }

  await browser.close();
})().catch(e => { console.error("ERROR:", e.message); process.exit(1); });
