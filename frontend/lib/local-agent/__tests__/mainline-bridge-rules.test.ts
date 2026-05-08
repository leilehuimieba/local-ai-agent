import { describe, expect, it } from "vitest"
import {
  applyProbabilityState,
  buildEvidenceSubmitState,
  buildKeyEvidenceState,
  buildTimeBudgetSubmitState,
} from "../mainline-bridge-rules"
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

describe("mainline-bridge-rules", () => {
  describe("applyProbabilityState", () => {
    it("should update probabilityValue, probabilityState, riskLevel and set evidenceExpired to false", () => {
      const shell = makeShell()
      const result = applyProbabilityState(shell, 75, "green")
      expect(result.probabilityValue).toBe(75)
      expect(result.probabilityState).toBe("known")
      expect(result.riskLevel).toBe("green")
      expect(result.evidenceExpired).toBe(false)
    })
  })

  describe("buildEvidenceSubmitState", () => {
    it("should return evidencePanelOpen false and update mainlineShell", () => {
      const shell = makeShell()
      const result = buildEvidenceSubmitState(shell)
      expect(result.evidencePanelOpen).toBe(false)
      expect(result.mainlineShell.evidenceSubmitted).toBe(true)
      expect(result.mainlineShell.followthrough.lastActionLabel).toBe("已提交晚间证据")
    })
  })

  describe("buildKeyEvidenceState", () => {
    it("should return keyEvidencePanelOpen false and feedback for weak evidence", () => {
      const shell = makeShell({
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
      })
      const result = buildKeyEvidenceState(shell)
      expect(result.keyEvidencePanelOpen).toBe(false)
      expect(result.mainlineShell.keyEvidenceFeedback).not.toBeNull()
    })

    it("should update probability for strong evidence", () => {
      const shell = makeShell({
        keyEvidenceEntry: {
          sourceType: "mock_exam",
          completedDate: "2024-01-01",
          durationMinutes: "120",
          underExamCondition: true,
          totalScore: "480",
          listeningScore: "150",
          readingScore: "170",
          writingTranslationScore: "160",
          notes: "",
        },
      })
      const result = buildKeyEvidenceState(shell)
      expect(result.keyEvidencePanelOpen).toBe(false)
      expect(result.mainlineShell.probabilityValue).not.toBeNull()
      expect(result.mainlineShell.probabilityState).toBe("known")
      expect(result.mainlineShell.keyEvidenceHistory.length).toBe(1)
    })
  })

  describe("buildTimeBudgetSubmitState", () => {
    it("should return timeBudgetPanelOpen false and update time budget related fields", () => {
      const shell = makeShell({
        timeBudgetEntry: {
          coreTaskLabel: "阅读训练",
          budgetChangeNote: "",
          timeBlockDraft: { startTime: "09:00", endTime: "11:00", label: "上午学习" },
        },
        timeBlocks: [],
      })
      const result = buildTimeBudgetSubmitState(shell)
      expect(result.timeBudgetPanelOpen).toBe(false)
      expect(result.mainlineShell.timeBlocks.length).toBe(1)
      expect(result.mainlineShell.timeBudgetInsight.totalAvailableMinutes).toBe(120)
      expect(result.mainlineShell.execution.status).toBe("executing")
    })
  })
})
