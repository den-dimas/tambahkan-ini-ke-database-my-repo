import { test, expect } from '@playwright/test';

test.describe('Responsive Layout', () => {
  test('app container expands on ultra-wide screens (2K)', async ({ page }) => {
    // Set a large viewport (2K resolution)
    await page.setViewportSize({ width: 2560, height: 1440 });

    await page.goto('/');

    // Wait for the main element to be visible
    const main = page.locator('main');
    await expect(main).toBeVisible();

    // Get the width of the main element
    const width = await main.evaluate((el) => el.clientWidth);

    // On a 2560px screen with px-4 or px-8 padding, the width should be significantly larger than 1280px
    // md:px-8 means 32px of padding on each side (total 64px)
    // 2560 - 64 = 2496px
    expect(width).toBeGreaterThan(2000);
    console.log(`Main container clientWidth on 2560px screen: ${width}px`);
  });

  test('app container is responsive on mobile', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/');

    const main = page.locator('main');
    await expect(main).toBeVisible();

    const width = await main.evaluate((el) => el.clientWidth);

    // Width should be close to viewport width minus padding
    // px-4 = 16px on each side (total 32px)
    // 375 - 32 = 343px
    expect(width).toBeLessThanOrEqual(375);
    expect(width).toBeGreaterThan(300);
    console.log(`Main container clientWidth on 375px screen: ${width}px`);
  });
});
