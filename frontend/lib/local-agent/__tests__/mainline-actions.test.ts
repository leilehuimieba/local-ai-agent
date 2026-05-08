import { describe, expect, it, vi } from "vitest"
import { createMainlineActions } from "../mainline-actions"

describe("mainline-actions", () => {
  it("should return expected action names", () => {
    const set = vi.fn()
    const actions = createMainlineActions(set)

    expect(actions).toHaveProperty("setMainlineGoal")
    expect(actions).toHaveProperty("setMainlineProbability")
    expect(actions).toHaveProperty("refreshMainlineEvidenceState")
    expect(actions).toHaveProperty("updateEvidencePacket")
    expect(actions).toHaveProperty("submitEvidencePacket")
    expect(actions).toHaveProperty("updateKeyEvidenceEntry")
    expect(actions).toHaveProperty("submitKeyEvidenceEntry")
    expect(actions).toHaveProperty("updateSwitchForm")
    expect(actions).toHaveProperty("submitTemporaryMainlineSwitch")
    expect(actions).toHaveProperty("restoreOriginalMainline")
    expect(actions).toHaveProperty("updatePersonalizedEntry")
    expect(actions).toHaveProperty("submitPersonalizedEntry")
    expect(actions).toHaveProperty("updateTimeBudgetEntry")
    expect(actions).toHaveProperty("updateTimeBlockDraft")
    expect(actions).toHaveProperty("submitTimeBudgetEntry")
    expect(actions).toHaveProperty("markTodayCoreTaskCompleted")
    expect(actions).toHaveProperty("closeToday")
    expect(actions).toHaveProperty("generateNextDayPlan")
  })

  it("should call set when an action is invoked", () => {
    const set = vi.fn()
    const actions = createMainlineActions(set)

    actions.setMainlineGoal("新目标")
    expect(set).toHaveBeenCalledTimes(1)

    actions.markTodayCoreTaskCompleted()
    expect(set).toHaveBeenCalledTimes(2)
  })
})
