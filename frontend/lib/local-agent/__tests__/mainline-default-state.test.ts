import { describe, expect, it } from "vitest"
import { createUIMainlineState, createDefaultMainlineShell } from "../mainline-default-state"

describe("mainline-default-state", () => {
  describe("createUIMainlineState", () => {
    it("should return UI state with all panels closed and default shell", () => {
      const state = createUIMainlineState()

      expect(state.mainlineExpanded).toBe(false)
      expect(state.evidencePanelOpen).toBe(false)
      expect(state.keyEvidencePanelOpen).toBe(false)
      expect(state.switchPanelOpen).toBe(false)
      expect(state.personalizedPanelOpen).toBe(false)
      expect(state.timeBudgetPanelOpen).toBe(false)
      expect(state.mainlineShell).toBeDefined()
    })
  })

  describe("createDefaultMainlineShell", () => {
    it("should return a complete MainlineShellState with all sub-states as empty/default", () => {
      const shell = createDefaultMainlineShell()

      expect(shell.currentGoalLabel).toBe("四级冲刺")
      expect(shell.probabilityValue).toBeNull()
      expect(shell.probabilityState).toBe("unknown")
      expect(shell.riskLevel).toBe("yellow")
      expect(shell.evidenceExpired).toBe(true)
      expect(shell.evidenceSubmitted).toBe(false)
      expect(shell.lastCriticalEvidenceAt).toBeNull()
      expect(shell.keyEvidenceFeedback).toBeNull()
      expect(shell.switchFeedback).toBeNull()
      expect(shell.keyEvidenceHistory).toEqual([])
      expect(shell.personalizedHistory).toEqual([])
      expect(shell.timeBlocks).toEqual([])
      expect(shell.recentPlanHistory).toEqual([])
      expect(shell.temporaryMainline.isActive).toBe(false)
      expect(shell.temporaryMainline.history).toEqual([])
      expect(shell.nextDayPlan.ready).toBe(false)
      expect(shell.execution.status).toBe("idle")
      expect(shell.personalizedInsight.basedOnCount).toBe(0)
      expect(shell.timeBudgetInsight.totalAvailableMinutes).toBe(0)
    })

    it("should have default currentGoalLabel", () => {
      const shell = createDefaultMainlineShell()
      expect(shell.currentGoalLabel).toBe("四级冲刺")
    })
  })
})
