import { test, expect } from "@playwright/test";
import { waitForCanvas, startNewGame, advanceText } from "../helpers/game";

// Effects tests have higher tolerance due to animation timing
const effectsConfig = {
  maxDiffPixels: 500,
  threshold: 0.3,
};

test.describe("Transition Effects", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("fade transition mid-animation", async ({ page }) => {
    // Advance to fade transition
    for (let i = 0; i < 2; i++) {
      await advanceText(page);
    }
    // Capture during fade
    await page.waitForTimeout(250);
    await expect(page).toHaveScreenshot("fade-transition-mid.png", effectsConfig);
  });

  test("fade transition complete", async ({ page }) => {
    for (let i = 0; i < 2; i++) {
      await advanceText(page);
    }
    // Wait for fade to complete
    await page.waitForTimeout(600);
    await expect(page).toHaveScreenshot("fade-transition-complete.png");
  });

  test("white fade transition", async ({ page }) => {
    for (let i = 0; i < 3; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(150);
    await expect(page).toHaveScreenshot("white-fade-mid.png", effectsConfig);
  });

  test("dissolve transition", async ({ page }) => {
    for (let i = 0; i < 4; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(350);
    await expect(page).toHaveScreenshot("dissolve-mid.png", effectsConfig);
  });
});

test.describe("Shake Effects", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("horizontal shake", async ({ page }) => {
    for (let i = 0; i < 5; i++) {
      await advanceText(page);
    }
    // Capture during shake
    await page.waitForTimeout(100);
    await expect(page).toHaveScreenshot("shake-horizontal.png", effectsConfig);
  });

  test("vertical shake", async ({ page }) => {
    for (let i = 0; i < 6; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(100);
    await expect(page).toHaveScreenshot("shake-vertical.png", effectsConfig);
  });

  test("both shake", async ({ page }) => {
    for (let i = 0; i < 7; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(100);
    await expect(page).toHaveScreenshot("shake-both.png", effectsConfig);
  });
});

test.describe("Particle Effects", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("snow particles", async ({ page }) => {
    for (let i = 0; i < 8; i++) {
      await advanceText(page);
    }
    // Wait for particles to spawn
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot("particles-snow.png", {
      maxDiffPixels: 2000, // Particles are random
      threshold: 0.4,
    });
  });

  test("rain particles", async ({ page }) => {
    for (let i = 0; i < 9; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot("particles-rain.png", {
      maxDiffPixels: 2000,
      threshold: 0.4,
    });
  });

  test("sakura particles", async ({ page }) => {
    for (let i = 0; i < 10; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot("particles-sakura.png", {
      maxDiffPixels: 2000,
      threshold: 0.4,
    });
  });

  test("sparkle particles", async ({ page }) => {
    for (let i = 0; i < 11; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot("particles-sparkle.png", {
      maxDiffPixels: 2000,
      threshold: 0.4,
    });
  });

  test("particles stop", async ({ page }) => {
    // Start particles then stop
    for (let i = 0; i < 12; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("particles-stopped.png");
  });
});

test.describe("Cinematic Mode", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("cinematic bars appear", async ({ page }) => {
    for (let i = 0; i < 13; i++) {
      await advanceText(page);
    }
    // Wait for bars to animate in
    await page.waitForTimeout(600);
    await expect(page).toHaveScreenshot("cinematic-on.png");
  });

  test("cinematic bars animation mid", async ({ page }) => {
    for (let i = 0; i < 13; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(250);
    await expect(page).toHaveScreenshot("cinematic-animating.png", effectsConfig);
  });

  test("cinematic bars disappear", async ({ page }) => {
    // Turn on then off
    for (let i = 0; i < 14; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(600);
    await expect(page).toHaveScreenshot("cinematic-off.png");
  });
});

test.describe("Combined Effects", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForCanvas(page);
    await startNewGame(page);
  });

  test("multiple effects simultaneously", async ({ page }) => {
    // Advance to combined effects
    for (let i = 0; i < 15; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(500);
    await expect(page).toHaveScreenshot("combined-effects.png", {
      maxDiffPixels: 3000,
      threshold: 0.5,
    });
  });

  test("transition with background change", async ({ page }) => {
    for (let i = 0; i < 2; i++) {
      await advanceText(page);
    }
    await page.waitForTimeout(600);
    await expect(page).toHaveScreenshot("transition-with-background.png");
  });
});
