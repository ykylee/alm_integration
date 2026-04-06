const fs = require("fs/promises");
const { existsSync } = require("fs");
const path = require("path");
const { chromium } = require("playwright");

const RENDER_VERSION =
  process.env.UI_PROTOTYPE_RENDER_VERSION || "v3_detail_pages";
const OUTPUT_DIR = path.resolve(
  process.cwd(),
  "output/playwright/ui_prototype",
  RENDER_VERSION
);
const BASE_PATH = "http://127.0.0.1:8000/src/ui_prototype";

const screens = [
  { path: "index.html", file: "01-role-home.png" },
  { path: "tasks.html", file: "02-task-workspace.png" },
  { path: "delivery.html", file: "03-project-delivery.png" },
  { path: "organization.html", file: "04-organization-operations.png" },
  { path: "quality.html", file: "05-quality-workspace.png" },
  { path: "admin.html", file: "06-admin-console.png" },
];

const browserExecutableCandidates = [
  process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,
  "/usr/bin/google-chrome",
  "/usr/bin/google-chrome-stable",
  "/usr/bin/chromium",
  "/usr/bin/chromium-browser",
].filter(Boolean);

function resolveExecutablePath() {
  return browserExecutableCandidates.find((candidate) => existsSync(candidate));
}

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

  const executablePath = resolveExecutablePath();
  const browser = await chromium.launch({
    headless: true,
    ...(executablePath ? { executablePath } : {}),
  });
  const page = await browser.newPage({ viewport: { width: 1600, height: 1200 } });

  for (const screen of screens) {
    const screenUrl = `${BASE_PATH}/${screen.path}`;
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
