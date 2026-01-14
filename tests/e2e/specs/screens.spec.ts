import { test, expect } from "@playwright/test";
import {
  waitForCanvas,
  startNewGame,
  openSettings,
  pressKey,
  advanceText,
} from "../helpers/game";

test.describe("Title Screen", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
  });

  test("renders title screen correctly", async ({ page }) => {
    await expect(page).toHaveScreenshot("title-screen.png");
  });

  test("menu button hover state", async ({ page }) => {
    const canvas = page.locator("canvas");
    // Hover over New Game button area
    await canvas.hover({ position: { x: 400, y: 300 } });
    await page.waitForTimeout(200);
    await expect(page).toHaveScreenshot("title-menu-hover.png");
  });

  test("new game starts game", async ({ page }) => {
    await startNewGame(page);
    await expect(page).toHaveScreenshot("game-started.png");
  });
});

test.describe("Settings Screen", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("settings screen opens with Escape", async ({ page }) => {
    await openSettings(page);
    await expect(page).toHaveScreenshot("settings-screen.png");
  });

  test("settings screen closes with Escape", async ({ page }) => {
    await openSettings(page);
    await page.waitForTimeout(300);
    await openSettings(page);
    await expect(page).toHaveScreenshot("settings-closed.png");
  });
});

test.describe("Game Navigation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("advance text with Enter", async ({ page }) => {
    await advanceText(page);
    await expect(page).toHaveScreenshot("after-advance.png");
  });

  test("advance text with click", async ({ page }) => {
    const canvas = page.locator("canvas");
    await canvas.click({ position: { x: 400, y: 450 } });
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot("after-click-advance.png");
  });

  test("rollback with arrow up", async ({ page }) => {
    // Advance a few times
    await advanceText(page);
    await advanceText(page);

    // Rollback
    await pressKey(page, "ArrowUp");
    await expect(page).toHaveScreenshot("after-rollback.png");
  });
});

test.describe("Special Modes", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("auto mode toggle", async ({ page }) => {
    await pressKey(page, "a");
    await page.waitForTimeout(200);
    // Auto mode indicator should be visible
    await expect(page).toHaveScreenshot("auto-mode-on.png");
  });

  test("skip mode toggle", async ({ page }) => {
    await pressKey(page, "s");
    await page.waitForTimeout(200);
    // Skip mode indicator should be visible
    await expect(page).toHaveScreenshot("skip-mode-on.png");
  });

  test("backlog view", async ({ page }) => {
    // Advance a few times to build history
    await advanceText(page);
    await advanceText(page);
    await advanceText(page);

    // Open backlog
    await pressKey(page, "l");
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot("backlog-view.png");
  });
});

test.describe("Save/Load", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("quick save", async ({ page }) => {
    await advanceText(page);
    await pressKey(page, "F5");
    await page.waitForTimeout(500);
    // Save confirmation should appear briefly
    await expect(page).toHaveScreenshot("after-quick-save.png");
  });

  test("quick load restores state", async ({ page }) => {
    // Advance and save
    await advanceText(page);
    await pressKey(page, "F5");
    await page.waitForTimeout(500);

    // Advance more
    await advanceText(page);
    await advanceText(page);

    // Load
    await pressKey(page, "F9");
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("after-quick-load.png");
  });
});
