import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./specs",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",

  use: {
    baseURL: "http://localhost:8080",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],

  webServer: {
    command: "npx serve ../../dist -l 8080 -s",
    port: 8080,
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },

  expect: {
    toHaveScreenshot: {
      maxDiffPixels: 100,
      threshold: 0.2,
    },
  },
});
