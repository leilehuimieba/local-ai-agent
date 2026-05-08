import { test, expect } from "@playwright/test"

test.describe("time budget flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/")
    await page.waitForLoadState("networkidle")
  })

  test("can open mainline shell and see time budget action", async ({ page, isMobile }) => {
    test.skip(isMobile, "MainlineShell is hidden on mobile")
    const shellBtn = page.locator("button").filter({ hasText: "当前主目标" }).first()
    await shellBtn.click()

    await expect(page.getByRole("button", { name: "时间预算接管", exact: true })).toBeVisible()
    await expect(page.getByRole("button", { name: "补关键证据", exact: true })).toBeVisible()
    await expect(page.getByRole("button", { name: "晚间证据", exact: true })).toBeVisible()
  })
})
