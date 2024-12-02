import { test, expect } from "@playwright/test";

test("homepage has title and heading text and button works", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Welcome to Leptos");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");

  const caseInsensitiveButton = page.locator('button:has-text(/click me/i)'); // Case-insensitive

  await expect(caseInsensitiveButton).toContainText('0');

  await caseInsensitiveButton.click();

  await expect(caseInsensitiveButton).not.toContainText('0');
  await expect(caseInsensitiveButton).toContainText('1');

});
