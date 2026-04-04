const fs = require("fs/promises");
const path = require("path");
const { chromium } = require("playwright");

const OUTPUT_DIR = path.resolve(process.cwd(), "output/playwright/ui_prototype");
const BASE_URL = "http://127.0.0.1:8000/src/ui_prototype/index.html";

const screens = [
  { key: "overview", file: "01-overview.png" },
  { key: "registration", file: "02-registration.png" },
  { key: "task", file: "03-task-detail.png" },
  { key: "migration", file: "04-organization-migration.png" },
  { key: "calendar", file: "05-shared-calendar.png" },
];

async function capture() {
  await fs.mkdir(OUTPUT_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1600, height: 1200 } });

  await page.goto(BASE_URL, { waitUntil: "networkidle" });

  for (const screen of screens) {
    if (screen.key !== "overview") {
      await page.click(`button[data-screen="${screen.key}"]`);
      await page.waitForTimeout(250);
    }

    await page.screenshot({
      path: path.join(OUTPUT_DIR, screen.file),
      fullPage: true,
    });
  }

  await browser.close();
}

capture().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
