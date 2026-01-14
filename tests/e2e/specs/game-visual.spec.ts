import { test, expect } from "@playwright/test";
import {
  waitForCanvas,
  startNewGame,
  advanceText,
  clickCanvas,
} from "../helpers/game";

test.describe("Background Display", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("background image displays correctly", async ({ page }) => {
    // Advance to a command with background
    await advanceText(page);
    await expect(page).toHaveScreenshot("with-background.png");
  });

  test("background persists across commands", async ({ page }) => {
    await advanceText(page);
    await advanceText(page);
    // Background should still be visible
    await expect(page).toHaveScreenshot("background-persisted.png");
  });

  test("background clears when set to empty", async ({ page }) => {
    // Advance to background clear
    for (let i = 0; i < 5; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("background-cleared.png");
  });
});

test.describe("Character Display", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("character displays at center position", async ({ page }) => {
    // Advance to character display
    await advanceText(page);
    await advanceText(page);
    await expect(page).toHaveScreenshot("character-center.png");
  });

  test("character at left position", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("character-left.png");
  });

  test("character at right position", async ({ page }) => {
    for (let i = 0; i < 4; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("character-right.png");
  });

  test("character persists without explicit field", async ({ page }) => {
    await advanceText(page);
    await advanceText(page);
    await advanceText(page);
    // Character should still be visible
    await expect(page).toHaveScreenshot("character-persisted.png");
  });

  test("character clears when set to empty", async ({ page }) => {
    for (let i = 0; i < 6; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("character-cleared.png");
  });
});

test.describe("Multiple Characters", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("two characters display simultaneously", async ({ page }) => {
    // Advance to multiple characters command
    for (let i = 0; i < 5; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("two-characters.png");
  });

  test("three characters display simultaneously", async ({ page }) => {
    for (let i = 0; i < 6; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("three-characters.png");
  });
});

test.describe("Character Animations", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("character fade in animation", async ({ page }) => {
    // Advance to fade animation
    for (let i = 0; i < 7; i++) {
      await advanceText(page);
    }
    // Take screenshot during animation
    await page.waitForTimeout(150);
    await expect(page).toHaveScreenshot("character-fade-in.png", {
      maxDiffPixels: 500, // Allow more variance for animation
    });
  });

  test("character slide animation", async ({ page }) => {
    for (let i = 0; i < 8; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(150);
    await expect(page).toHaveScreenshot("character-slide.png", {
      maxDiffPixels: 500,
    });
  });
});
