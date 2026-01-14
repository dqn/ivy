import { test, expect } from "@playwright/test";

test.describe("Visual Novel Engine", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    // Wait for canvas to be ready
    await page.waitForSelector("canvas");
    // Wait for initial render
    await page.waitForTimeout(1000);
  });

  test("title screen renders correctly", async ({ page }) => {
    // Take screenshot of title screen
    await expect(page).toHaveScreenshot("title-screen.png");
  });

  test("clicking advances text", async ({ page }) => {
    const canvas = page.locator("canvas");

    // Click to start game (assuming title screen has "New Game" button)
    await canvas.click({ position: { x: 400, y: 300 } });
    await page.waitForTimeout(500);

    // Take screenshot after first click
    await expect(page).toHaveScreenshot("after-first-click.png");
  });

  test("keyboard navigation works", async ({ page }) => {
    const canvas = page.locator("canvas");

    // Focus canvas
    await canvas.focus();

    // Press Enter to advance
    await page.keyboard.press("Enter");
    await page.waitForTimeout(500);

    // Take screenshot
    await expect(page).toHaveScreenshot("after-enter.png");
  });

  test("settings screen accessible", async ({ page }) => {
    const canvas = page.locator("canvas");

    // Focus canvas
    await canvas.focus();

    // Press Escape to open settings
    await page.keyboard.press("Escape");
    await page.waitForTimeout(500);

    // Take screenshot of settings
    await expect(page).toHaveScreenshot("settings-screen.png");
  });

  test("mouse hover on buttons", async ({ page }) => {
    const canvas = page.locator("canvas");

    // Hover over a button area
    await canvas.hover({ position: { x: 400, y: 300 } });
    await page.waitForTimeout(200);

    // Take screenshot with hover state
    await expect(page).toHaveScreenshot("button-hover.png");
  });
});

test.describe("Game Flow", () => {
  test("complete simple interaction", async ({ page }) => {
    await page.goto("/");
    await page.waitForSelector("canvas");
    await page.waitForTimeout(1000);

    const canvas = page.locator("canvas");

    // Start game
    await canvas.click({ position: { x: 400, y: 300 } });
    await page.waitForTimeout(500);

    // Advance through a few screens
    for (let i = 0; i < 3; i++) {
      await canvas.focus();
      await page.keyboard.press("Enter");
      await page.waitForTimeout(300);
    }

    // Final screenshot
    await expect(page).toHaveScreenshot("after-multiple-advances.png");
  });
});
