import { describe, expect, it, vi } from "vitest"
import { createUILayoutActions } from "../layout-actions"

describe("layout-actions", () => {
  it("should return expected action names", () => {
    const set = vi.fn()
    const actions = createUILayoutActions(set)

    expect(actions).toHaveProperty("setActiveView")
    expect(actions).toHaveProperty("toggleLeftSidebar")
    expect(actions).toHaveProperty("toggleRightDrawer")
    expect(actions).toHaveProperty("setRightDrawerOpen")
    expect(actions).toHaveProperty("setMobileMenuOpen")
    expect(actions).toHaveProperty("setMobileDrawerOpen")
    expect(actions).toHaveProperty("setMainlineExpanded")
    expect(actions).toHaveProperty("setEvidencePanelOpen")
    expect(actions).toHaveProperty("setKeyEvidencePanelOpen")
    expect(actions).toHaveProperty("setSwitchPanelOpen")
    expect(actions).toHaveProperty("setPersonalizedPanelOpen")
    expect(actions).toHaveProperty("setTimeBudgetPanelOpen")
  })

  it("should call set when an action is invoked", () => {
    const set = vi.fn()
    const actions = createUILayoutActions(set)

    actions.setActiveView("logs")
    expect(set).toHaveBeenCalledTimes(1)

    actions.toggleLeftSidebar()
    expect(set).toHaveBeenCalledTimes(2)
  })
})
