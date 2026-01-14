import { test, expect } from "@playwright/test";
import { waitForCanvas, startNewGame, advanceText } from "../helpers/game";

test.describe("Text Box", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("text box displays correctly", async ({ page }) => {
    await expect(page).toHaveScreenshot("text-box-initial.png");
  });

  test("text advances on click", async ({ page }) => {
    await advanceText(page);
    await expect(page).toHaveScreenshot("text-advanced.png");
  });

  test("long text wraps correctly", async ({ page }) => {
    // Advance to long text
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-wrapped.png");
  });
});

test.describe("Speaker Name", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("speaker name displays above text box", async ({ page }) => {
    // Advance to command with speaker
    for (let i = 0; i < 2; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("with-speaker-name.png");
  });

  test("different speakers have different names", async ({ page }) => {
    for (let i = 0; i < 4; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("different-speaker.png");
  });
});

test.describe("Colored Text", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("red colored text", async ({ page }) => {
    // Advance to colored text
    for (let i = 0; i < 5; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-red.png");
  });

  test("multiple colors in same line", async ({ page }) => {
    for (let i = 0; i < 6; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-multiple-colors.png");
  });

  test("nested colors", async ({ page }) => {
    for (let i = 0; i < 7; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-nested-colors.png");
  });
});

test.describe("Ruby Text", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("ruby (furigana) displays above text", async ({ page }) => {
    // Advance to ruby text
    for (let i = 0; i < 8; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-with-ruby.png");
  });

  test("multiple ruby annotations", async ({ page }) => {
    for (let i = 0; i < 9; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-multiple-ruby.png");
  });
});

test.describe("Variable Expansion", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("variable displays in text", async ({ page }) => {
    // Advance to variable expansion
    for (let i = 0; i < 10; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("text-with-variable.png");
  });
});

test.describe("Typewriter Effect", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("text appears character by character", async ({ page }) => {
    // Take screenshot during typewriter effect
    await page.waitForTimeout(200);
    await expect(page).toHaveScreenshot("typewriter-partial.png", {
      maxDiffPixels: 200, // Allow variance for animation
    });
  });

  test("typewriter completes on click", async ({ page }) => {
    const canvas = page.locator("canvas");
    await canvas.click({ position: { x: 400, y: 450 } });
    await page.waitForTimeout(100);
    await expect(page).toHaveScreenshot("typewriter-complete.png");
  });
});

test.describe("Continue Indicator", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("continue indicator blinks", async ({ page }) => {
    // Wait for typewriter to complete
    await page.waitForTimeout(1500);
    // Take screenshot - indicator should be visible
    await expect(page).toHaveScreenshot("continue-indicator.png", {
      maxDiffPixels: 100,
    });
  });
});
