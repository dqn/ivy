import { test, expect } from "@playwright/test";
import {
  waitForCanvas,
  startNewGame,
  advanceText,
  selectChoice,
  clickCanvas,
} from "../helpers/game";

test.describe("Choice Buttons", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
    // Advance to choices
    await advanceText(page);
  });

  test("choice buttons display correctly", async ({ page }) => {
    await expect(page).toHaveScreenshot("choice-buttons.png");
  });

  test("choice button hover state", async ({ page }) => {
    const canvas = page.locator("canvas");
    // Hover over first choice
    await canvas.hover({ position: { x: 400, y: 250 } });
    await page.waitForTimeout(200);
    await expect(page).toHaveScreenshot("choice-hover.png");
  });

  test("selecting first choice navigates correctly", async ({ page }) => {
    await selectChoice(page, 0);
    await expect(page).toHaveScreenshot("after-choice-0.png");
  });

  test("selecting second choice navigates correctly", async ({ page }) => {
    await selectChoice(page, 1);
    await expect(page).toHaveScreenshot("after-choice-1.png");
  });

  test("three choices display", async ({ page }) => {
    // Advance to three-choice command
    await selectChoice(page, 0);
    await advanceText(page);
    await expect(page).toHaveScreenshot("three-choices.png");
  });
});

test.describe("Timed Choices", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("timer bar displays", async ({ page }) => {
    // Advance to timed choice
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("timed-choice-timer.png");
  });

  test("timer bar decreases over time", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    // Wait for timer to decrease
    await page.waitForTimeout(2000);
    await expect(page).toHaveScreenshot("timed-choice-timer-decreased.png", {
      maxDiffPixels: 300, // Allow variance for timer animation
    });
  });

  test("default choice is highlighted", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("timed-choice-default-highlighted.png");
  });

  test("timeout selects default choice", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    // Wait for timeout (assuming 5 second timeout)
    await page.waitForTimeout(6000);
    await expect(page).toHaveScreenshot("after-timeout-default.png");
  });

  test("selecting before timeout works", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    // Select immediately
    await selectChoice(page, 0);
    await expect(page).toHaveScreenshot("timed-choice-selected-early.png");
  });
});

test.describe("Choice Flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("complete choice flow path A", async ({ page }) => {
    await advanceText(page);
    await selectChoice(page, 0);
    await advanceText(page);
    await expect(page).toHaveScreenshot("choice-flow-path-a.png");
  });

  test("complete choice flow path B", async ({ page }) => {
    await advanceText(page);
    await selectChoice(page, 1);
    await advanceText(page);
    await expect(page).toHaveScreenshot("choice-flow-path-b.png");
  });

  test("choices affect game state", async ({ page }) => {
    // Make choices that set variables
    await advanceText(page);
    await selectChoice(page, 0);
    // Continue to where variable is used
    for (let i = 0; i < 5; i++) {
      await advanceText(page);
    }
    await expect(page).toHaveScreenshot("choice-affects-state.png");
  });
});
