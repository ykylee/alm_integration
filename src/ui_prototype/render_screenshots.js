const fs = require("fs/promises");
const path = require("path");
const { chromium } = require("playwright");

const RENDER_VERSION = process.env.UI_PROTOTYPE_RENDER_VERSION || "v2_role_based";
const OUTPUT_DIR = path.resolve(
  process.cwd(),
  "output/playwright/ui_prototype",
  RENDER_VERSION
);
const BASE_URL = "http://127.0.0.1:8000/src/ui_prototype/index.html";

const screens = [
  { key: "overview", file: "01-role-home.png" },
  { key: "tasks", file: "02-task-workspace.png" },
  { key: "delivery", file: "03-project-delivery.png" },
  { key: "organization", file: "04-organization-operations.png" },
  { key: "quality", file: "05-quality-workspace.png" },
  { key: "admin", file: "06-admin-console.png" },
];

async function waitForStableRender(page) {
  await page.waitForLoadState("networkidle");

  await page.evaluate(async () => {
    if (document.fonts && document.fonts.ready) {
      await document.fonts.ready;
    }

    await new Promise((resolve) => requestAnimationFrame(() => resolve()));
    await new Promise((resolve) => requestAnimationFrame(() => resolve()));
  });

  await page.waitForTimeout(400);
}

async function capture() {
  await fs.mkdir(OUTPUT_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1600, height: 1200 } });

  for (const screen of screens) {
    const screenUrl = `${BASE_URL}#${screen.key}`;
    await page.goto(screenUrl, { waitUntil: "domcontentloaded" });
    await waitForStableRender(page);

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
