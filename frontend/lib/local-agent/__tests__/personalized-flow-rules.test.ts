import { describe, expect, it } from "vitest"
import { buildPersonalizedDraftState, buildPersonalizedSubmitState } from "../personalized-flow-rules"
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

describe("personalized-flow-rules", () => {
  describe("buildPersonalizedDraftState", () => {
    it("should set personalizedEntry fields", () => {
      const shell = makeShell()
      const result = buildPersonalizedDraftState(shell, { taskType: "reading", resultNote: "阅读笔记" })
      expect(result.mainlineShell.personalizedEntry.taskType).toBe("reading")
      expect(result.mainlineShell.personalizedEntry.resultNote).toBe("阅读笔记")
    })

    it("should reset personalizedFeedback", () => {
      const shell = makeShell({ personalizedFeedback: "旧反馈" })
      const result = buildPersonalizedDraftState(shell, {})
      expect(result.mainlineShell.personalizedFeedback).toBeNull()
    })
  })

  describe("buildPersonalizedSubmitState", () => {
    it("should save record and update insight when entry is complete", () => {
      const shell = makeShell({
        personalizedEntry: {
          taskType: "reading",
          isCoreTask: false,
          completed: true,
          hasShortTermGain: true,
          resultNote: "很有收获",
        },
      })
      const result = buildPersonalizedSubmitState(shell)
      expect(result.personalizedPanelOpen).toBe(false)
      expect(result.mainlineShell.personalizedHistory.length).toBe(1)
      expect(result.mainlineShell.personalizedHistory[0].taskType).toBe("reading")
      expect(result.mainlineShell.personalizedInsight.basedOnCount).toBe(1)
      expect(result.mainlineShell.personalizedEntry.resultNote).toBe("")
    })

    it("should not append record when entry is incomplete", () => {
      const shell = makeShell({
        personalizedEntry: {
          taskType: "reading",
          isCoreTask: false,
          completed: null,
          hasShortTermGain: null,
          resultNote: "",
        },
      })
      const result = buildPersonalizedSubmitState(shell)
      expect(result.mainlineShell.personalizedHistory.length).toBe(0)
      expect(result.mainlineShell.personalizedFeedback).toContain("草稿")
    })

    it("should accumulate history across submissions", () => {
      const existing = {
        taskType: "vocabulary" as const,
        isCoreTask: true,
        completed: true,
        hasShortTermGain: true,
        resultNote: "",
        submittedAt: "2024-01-01T00:00:00.000Z",
      }
      const shell = makeShell({
        personalizedHistory: [existing],
        personalizedEntry: {
          taskType: "listening",
          isCoreTask: false,
          completed: true,
          hasShortTermGain: false,
          resultNote: "",
        },
      })
      const result = buildPersonalizedSubmitState(shell)
      expect(result.mainlineShell.personalizedHistory.length).toBe(2)
      expect(result.mainlineShell.personalizedInsight.basedOnCount).toBe(2)
    })
  })
})
