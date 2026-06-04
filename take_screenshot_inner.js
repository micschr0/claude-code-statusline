const { chromium } = require("/node_modules/playwright-core");
(async () => {
  console.log("Launching browser...");
  const browser = await chromium.launch({
    executablePath: "/ms-playwright/chromium-1148/chrome-linux/chrome",
    args: ["--no-sandbox", "--disable-setuid-sandbox"]
  });
  const page = await browser.newPage();
  await page.setViewportSize({ width: 1200, height: 800 });
  console.log("Loading HTML...");
  await page.goto("file:///work/screenshot_src.html");
  await page.waitForTimeout(2000);
  const height = await page.evaluate(() => document.body.scrollHeight);
  console.log("Page height:", height);
  await page.screenshot({
    path: "/work/screenshot.png",
    clip: { x: 0, y: 0, width: 1200, height },
  });
  await browser.close();
  console.log("Screenshot saved.");
})().catch(e => { console.error("ERROR:", e.message); process.exit(1); });
