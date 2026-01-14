import { Page, expect } from "@playwright/test";

/**
 * Wait for the game canvas to be ready.
 */
export async function waitForCanvas(page: Page): Promise<void> {
  await page.waitForSelector("canvas");
  await page.waitForTimeout(1000);
}

/**
 * Click on the canvas at a specific position.
 */
export async function clickCanvas(
  page: Page,
  x: number,
  y: number
): Promise<void> {
  const canvas = page.locator("canvas");
  await canvas.click({ position: { x, y } });
  await page.waitForTimeout(300);
}

/**
 * Press a key while the canvas is focused.
 */
export async function pressKey(page: Page, key: string): Promise<void> {
  const canvas = page.locator("canvas");
  await canvas.focus();
  await page.keyboard.press(key);
  await page.waitForTimeout(300);
}

/**
 * Advance text by pressing Enter.
 */
export async function advanceText(page: Page): Promise<void> {
  await pressKey(page, "Enter");
}

/**
 * Take a screenshot with a descriptive name.
 */
export async function takeScreenshot(
  page: Page,
  name: string
): Promise<void> {
  await expect(page).toHaveScreenshot(`${name}.png`);
}

/**
 * Click on a choice button by index (0-based).
 * Assumes choices are vertically centered with 60px spacing.
 */
export async function selectChoice(
  page: Page,
  index: number
): Promise<void> {
  const centerX = 400;
  const startY = 250;
  const spacing = 60;
  const y = startY + index * spacing;

  await clickCanvas(page, centerX, y);
}

/**
 * Navigate to the game with a specific scenario.
 * This assumes the game supports a query parameter for scenario selection.
 */
export async function loadScenario(
  page: Page,
  scenarioPath: string
): Promise<void> {
  await page.goto(`/?scenario=${encodeURIComponent(scenarioPath)}`);
  await waitForCanvas(page);
}

/**
 * Start a new game from the title screen.
 * Clicks on the "New Game" button area.
 */
export async function startNewGame(page: Page): Promise<void> {
  // New Game button is typically centered around y=300
  await clickCanvas(page, 400, 300);
}

/**
 * Open settings screen by pressing Escape.
 */
export async function openSettings(page: Page): Promise<void> {
  await pressKey(page, "Escape");
}

/**
 * Toggle auto mode by pressing 'A'.
 */
export async function toggleAutoMode(page: Page): Promise<void> {
  await pressKey(page, "a");
}

/**
 * Toggle skip mode by pressing 'S'.
 */
export async function toggleSkipMode(page: Page): Promise<void> {
  await pressKey(page, "s");
}

/**
 * Open backlog by pressing 'L'.
 */
export async function openBacklog(page: Page): Promise<void> {
  await pressKey(page, "l");
}

/**
 * Quick save by pressing F5.
 */
export async function quickSave(page: Page): Promise<void> {
  await pressKey(page, "F5");
}

/**
 * Quick load by pressing F9.
 */
export async function quickLoad(page: Page): Promise<void> {
  await pressKey(page, "F9");
}

/**
 * Rollback by pressing arrow up.
 */
export async function rollback(page: Page): Promise<void> {
  await pressKey(page, "ArrowUp");
}

/**
 * Open debug console by pressing F12.
 */
export async function openDebug(page: Page): Promise<void> {
  await pressKey(page, "F12");
}
