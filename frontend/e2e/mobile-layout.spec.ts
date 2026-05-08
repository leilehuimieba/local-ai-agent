import { test, expect } from "@playwright/test"

test.describe("mobile layout", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/")
    await page.waitForLoadState("networkidle")
  })

  test("bottom navigation is visible", async ({ page, isMobile }) => {
    test.skip(!isMobile, "Only for mobile")
    const nav = page.locator("nav.fixed.bottom-0")
    await expect(nav).toBeVisible()
    await expect(nav).toContainText("任务")
    await expect(nav).toContainText("历史")
    await expect(nav).toContainText("知识")
    await expect(nav).toContainText("设置")
    await expect(nav).toContainText("新任务")
  })

  test("left sidebar is hidden", async ({ page, isMobile }) => {
    test.skip(!isMobile, "Only for mobile")
    const sidebar = page.locator("aside")
    await expect(sidebar).toBeHidden()
  })

  test("sheet panel can be opened and closed", async ({ page, isMobile }) => {
    test.skip(!isMobile, "Only for mobile")
    // Click the drawer toggle in top bar (last button in header controls)
    await page.locator("header >> div.flex.items-center.gap-3 >> button").last().click()

    // Sheet dialog should appear
    const sheet = page.locator("[data-state='open']").first()
    await expect(sheet).toBeVisible()

    // Close by pressing Escape
    await page.keyboard.press("Escape")

    // Dialog should be removed
    await expect(page.locator("[role='dialog']")).toHaveCount(0)
  })

  test("composer is present", async ({ page }) => {
    const textarea = page.locator("textarea").first()
    await expect(textarea).toBeVisible()

    const sendBtn = page.locator("button").filter({ has: page.locator("svg") }).last()
    await expect(sendBtn).toBeVisible()
  })
})
