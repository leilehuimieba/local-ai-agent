import { describe, expect, it, vi } from "vitest"
import {
  buildEvidenceSubmittedState,
  buildTimeBudgetSubmittedState,
  buildCoreTaskCompletedState,
  buildTodayClosedPlanState,
  buildNextDayPlanGeneratedState,
  buildTimeBudgetPanelState,
} from "../plan-sync-flow-rules"
import type { MainlineShellState } from "../types"

function makeShell(patch: Partial<MainlineShellState> = {}): MainlineShellState {
  const base: MainlineShellState = {
    currentGoalLabel: "四级冲刺",
    probabilityValue: null,
    probabilityState: "unknown",
    riskLevel: "yellow",
    evidenceExpired: true,
    evidenceSubmitted: false,
    lastCriticalEvidenceAt: null,
    latestAdjustment: "test",
    keyEvidenceFeedback: null,
    switchFeedback: null,
    evidencePacket: { didWhat: "", resultSummary: "", blockers: "", nextAdjustment: "" },
    keyEvidenceEntry: {
      sourceType: "mock_exam",
      completedDate: "",
      durationMinutes: "",
      underExamCondition: false,
      totalScore: "",
      listeningScore: "",
      readingScore: "",
      writingTranslationScore: "",
      notes: "",
    },
    keyEvidenceHistory: [],
    switchForm: {
      temporaryGoalLabel: "",
      reason: "",
      dueDate: "",
      urgent: true,
      important: true,
      insistAfterReject: false,
    },
    temporaryMainline: {
      isActive: false,
      reviewStatus: "idle",
      reviewFeedback: null,
      currentTemporaryGoal: null,
      originalSnapshot: null,
      currentSwitchStartedAt: null,
      activeRecordId: null,
      history: [],
    },
    personalizedFeedback: null,
    personalizedEntry: {
      taskType: "vocabulary",
      isCoreTask: true,
      completed: null,
      hasShortTermGain: null,
      resultNote: "",
    },
    personalizedHistory: [],
    personalizedInsight: {
      updatedAt: null,
      basedOnCount: 0,
      preferredTaskType: null,
      boostTaskType: null,
      riskTaskType: null,
      recommendation: "test",
    },
    timeBudgetFeedback: null,
    timeBudgetEntry: {
      coreTaskLabel: "阅读训练",
      budgetChangeNote: "",
      timeBlockDraft: { startTime: "", endTime: "", label: "" },
    },
    timeBlocks: [],
    timeBudgetInsight: {
      updatedAt: null,
      totalAvailableMinutes: 0,
      level: "steady",
      recommendation: "test",
    },
    eveningReview: {
      scheduledLabel: "晚上固定查看",
      status: "pending",
      statusText: "今晚待查看",
      guidance: "test",
      deadlineLabel: null,
      branchDecision: "none",
      branchText: "test",
      lastReviewedAt: null,
      lastSubmittedForDate: null,
    },
    followthrough: {
      lastActionLabel: null,
      nextActionLabel: "test",
      nextActionHelper: "test",
      planUsesLatestEvidence: null,
    },
    execution: {
      status: "idle",
      currentTaskLabel: null,
      todayCoreTaskCompleted: false,
      todayClosed: false,
      statusText: "test",
      helperText: "test",
      activeDate: null,
      needsReopen: false,
      staleFromDate: null,
    },
    nextDayPlan: {
      generatedAt: null,
      ready: false,
      coreTaskLabel: "test",
      supportTaskLabel: "test",
      rationale: "test",
      restoreNote: null,
      targetDate: null,
      basedOnEvidenceDate: null,
      needsRefresh: false,
      statusText: "test",
      refreshReason: null,
      todayTaskLabel: null,
      takeoverStatus: "idle",
      takeoverHint: null,
    },
    todayPlanStatusText: "test",
    todayPlanReconcileText: null,
    recentPlanHistory: [],
  }
  return { ...base, ...patch }
}

describe("plan-sync-flow-rules", () => {
  describe("buildEvidenceSubmittedState", () => {
    it("should close evidence panel and update followthrough", () => {
      const shell = makeShell({ evidenceSubmitted: true })
      const result = buildEvidenceSubmittedState(shell)
      expect(result.evidencePanelOpen).toBe(false)
      expect(result.mainlineShell.followthrough.lastActionLabel).toBe("已提交晚间证据")
    })
  })

  describe("buildTimeBudgetSubmittedState", () => {
    it("should close time budget panel", () => {
      const shell = makeShell()
      const result = buildTimeBudgetSubmittedState(shell)
      expect(result.timeBudgetPanelOpen).toBe(false)
    })
  })

  describe("buildCoreTaskCompletedState", () => {
    it("should update followthrough after core task done", () => {
      const shell = makeShell()
      const result = buildCoreTaskCompletedState(shell)
      expect(result.followthrough.lastActionLabel).toBe("已完成今天核心任务")
      expect(result.followthrough.nextActionLabel).toBe("进入今天收尾")
    })
  })

  describe("buildTodayClosedPlanState", () => {
    it("should update followthrough after close", () => {
      const shell = makeShell()
      const result = buildTodayClosedPlanState(shell)
      expect(result.followthrough.lastActionLabel).toBe("已进入今天收尾")
      expect(result.followthrough.nextActionLabel).toBe("等待新的关键证据")
    })
  })

  describe("buildNextDayPlanGeneratedState", () => {
    it("should set nextDayPlan to ready and update followthrough", () => {
      const shell = makeShell()
      const result = buildNextDayPlanGeneratedState(shell)
      expect(result.nextDayPlan.ready).toBe(true)
      expect(result.nextDayPlan.generatedAt).not.toBeNull()
      expect(result.followthrough.lastActionLabel).toBe("已生成明日计划")
    })
  })

  describe("buildTimeBudgetPanelState", () => {
    beforeEach(() => {
      vi.useFakeTimers()
      vi.setSystemTime(new Date("2024-06-15T10:00:00.000Z"))
    })

    afterEach(() => {
      vi.useRealTimers()
    })

    it("should only return panel flag when closing", () => {
      const shell = makeShell()
      const result = buildTimeBudgetPanelState(shell, false)
      expect(result).toEqual({ timeBudgetPanelOpen: false })
    })

    it("should draft core task label from today's plan when opening", () => {
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          ready: true,
          targetDate: "2024-06-15",
          coreTaskLabel: "今日阅读",
        },
      })
      const result = buildTimeBudgetPanelState(shell, true)
      expect(result.timeBudgetPanelOpen).toBe(true)
      expect(result.mainlineShell.timeBudgetEntry.coreTaskLabel).toBe("今日阅读")
    })

    it("should not change mainlineShell when plan is not for today", () => {
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          ready: true,
          targetDate: "2024-06-16",
          coreTaskLabel: "明日听力",
        },
      })
      const result = buildTimeBudgetPanelState(shell, true)
      expect(result.timeBudgetPanelOpen).toBe(true)
      expect(result.mainlineShell).toEqual(shell)
    })

    it("should not change mainlineShell when plan is not ready", () => {
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          ready: false,
          targetDate: "2024-06-15",
          coreTaskLabel: "未准备",
        },
      })
      const result = buildTimeBudgetPanelState(shell, true)
      expect(result.timeBudgetPanelOpen).toBe(true)
      expect(result.mainlineShell).toEqual(shell)
    })
  })
})
