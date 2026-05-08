import { describe, expect, it } from "vitest"
import { applyPlanTracking } from "../plan-history-rules"
import { syncPlanShell } from "../plan-sync-flow-rules"
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
      coreTaskLabel: "test",
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

describe("plan-history-rules", () => {
  describe("applyPlanTracking", () => {
    it("should set todayPlanStatusText based on takeoverStatus", () => {
      const shell = makeShell({
        nextDayPlan: { ...makeShell().nextDayPlan, takeoverStatus: "taken_over", targetDate: "2024-01-01" },
        execution: { ...makeShell().execution, activeDate: "2024-01-01" },
      })
      const result = applyPlanTracking(shell)
      expect(result.todayPlanStatusText).toBe("今天计划已接管")
    })

    it("should update recentPlanHistory for tracked status", () => {
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          takeoverStatus: "completed",
          targetDate: "2024-01-01",
          coreTaskLabel: "阅读训练",
        },
        execution: { ...makeShell().execution, activeDate: "2024-01-01" },
        evidenceSubmitted: true,
      })
      const result = applyPlanTracking(shell)
      expect(result.recentPlanHistory.length).toBeGreaterThan(0)
      expect(result.recentPlanHistory[0].targetDate).toBe("2024-01-01")
      expect(result.recentPlanHistory[0].status).toBe("completed")
    })

    it("should not add history when targetDate is missing", () => {
      const shell = makeShell({
        nextDayPlan: { ...makeShell().nextDayPlan, takeoverStatus: "completed", targetDate: null },
      })
      const result = applyPlanTracking(shell)
      expect(result.recentPlanHistory).toEqual([])
    })

    it("should keep existing history and prepend new item", () => {
      const existing = {
        targetDate: "2024-01-02",
        taskLabel: "听力训练",
        status: "closed" as const,
        statusText: "已收尾",
        updatedAt: "2024-01-02",
        reconciled: true,
        reconcileText: "已对账",
      }
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          takeoverStatus: "taken_over",
          targetDate: "2024-01-01",
          coreTaskLabel: "阅读训练",
        },
        execution: { ...makeShell().execution, activeDate: "2024-01-01" },
        recentPlanHistory: [existing],
      })
      const result = applyPlanTracking(shell)
      expect(result.recentPlanHistory.length).toBe(2)
      expect(result.recentPlanHistory[0].targetDate).toBe("2024-01-01")
      expect(result.recentPlanHistory[1].targetDate).toBe("2024-01-02")
    })

    it("should replace same-date history entry", () => {
      const existing = {
        targetDate: "2024-01-01",
        taskLabel: "旧任务",
        status: "taken_over" as const,
        statusText: "旧状态",
        updatedAt: "2024-01-01",
        reconciled: false,
        reconcileText: "待对账",
      }
      const shell = makeShell({
        nextDayPlan: {
          ...makeShell().nextDayPlan,
          takeoverStatus: "completed",
          targetDate: "2024-01-01",
          coreTaskLabel: "阅读训练",
        },
        execution: { ...makeShell().execution, activeDate: "2024-01-01" },
        recentPlanHistory: [existing],
      })
      const result = applyPlanTracking(shell)
      expect(result.recentPlanHistory.length).toBe(1)
      expect(result.recentPlanHistory[0].status).toBe("completed")
    })
  })

  describe("syncPlanShell", () => {
    it("should sync plan-related fields", () => {
      const shell = makeShell({
        nextDayPlan: { ...makeShell().nextDayPlan, takeoverStatus: "taken_over", targetDate: "2024-01-01" },
        execution: { ...makeShell().execution, activeDate: "2024-01-01" },
      })
      const result = syncPlanShell(shell)
      expect(result.todayPlanStatusText).toBe("今天计划已接管")
    })
  })
})
