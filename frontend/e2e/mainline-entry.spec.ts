import { test, expect } from "@playwright/test"

test.describe("mainline entry", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/")
    await page.waitForLoadState("networkidle")
  })

  test("brand strip and entry identity are visible", async ({ page }) => {
    await expect(page.getByText("主线总控 Agent：围绕当前主目标运行，先看证据，再改判断。")).toBeVisible()
    await expect(page.getByText("核心链路").first()).toBeVisible()
    await expect(page.getByText("辅助入口").first()).toBeVisible()
  })

  test("mainline shell can expand and collapse", async ({ page, isMobile }) => {
    test.skip(isMobile, "MainlineShell is hidden on mobile")
    const button = page.locator("button").filter({ hasText: "当前主目标" }).first()
    await expect(button).toBeVisible()
    await button.click()
    await expect(page.getByText("请尽快补充关键证据")).toBeVisible()
    await expect(page.getByRole("button", { name: "补关键证据", exact: true })).toBeVisible()
    await expect(page.getByRole("button", { name: "晚间证据", exact: true })).toBeVisible()
    await button.click()
    await expect(page.getByText("请尽快补充关键证据")).toBeHidden()
  })

  test("task entry card shows product positioning", async ({ page }) => {
    await expect(page.getByText("当前产品口径")).toBeVisible()
    await expect(page.getByText(/证据驱动的主线总控入口/)).toBeVisible()
  })
})
