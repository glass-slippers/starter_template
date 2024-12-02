import { test, expect } from "@playwright/test";

test("homepage has title and heading text and button works", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Welcome to Leptos");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");

  const btn = page.locator('button:has-text("Click")'); // Case-insensitive

  await expect(btn).toContainText('0');


  await btn.click({ force: true });

  await expect(btn).not.toContainText('0');
  await expect(btn).toContainText('1');

});
