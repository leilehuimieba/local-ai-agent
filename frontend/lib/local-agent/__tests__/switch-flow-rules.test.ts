import { describe, expect, it } from "vitest"
import { buildSwitchDraftState, buildTemporarySwitchState, restoreOriginalMainlineState } from "../switch-flow-rules"
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

describe("switch-flow-rules", () => {
  describe("buildSwitchDraftState", () => {
    it("should set switchForm fields", () => {
      const shell = makeShell()
      const result = buildSwitchDraftState(shell, { temporaryGoalLabel: "临时目标", reason: "紧急任务" })
      expect(result.switchForm.temporaryGoalLabel).toBe("临时目标")
      expect(result.switchForm.reason).toBe("紧急任务")
    })

    it("should reset switchFeedback and temporaryMainline review", () => {
      const shell = makeShell({ switchFeedback: "旧的反馈", temporaryMainline: { ...makeShell().temporaryMainline, reviewStatus: "rejected", reviewFeedback: "被拒绝" } })
      const result = buildSwitchDraftState(shell, {})
      expect(result.switchFeedback).toBeNull()
      expect(result.temporaryMainline.reviewStatus).toBe("idle")
      expect(result.temporaryMainline.reviewFeedback).toBeNull()
    })
  })

  describe("buildTemporarySwitchState", () => {
    it("should activate temporary mainline when approved", () => {
      const shell = makeShell({
        switchForm: { temporaryGoalLabel: "紧急复习", reason: "考试", dueDate: "", urgent: true, important: true, insistAfterReject: false },
      })
      const result = buildTemporarySwitchState(shell)
      expect(result.mainlineShell.currentGoalLabel).toBe("紧急复习")
      expect(result.mainlineShell.temporaryMainline.isActive).toBe(true)
      expect(result.mainlineShell.temporaryMainline.currentTemporaryGoal).toBe("紧急复习")
      expect(result.mainlineShell.temporaryMainline.reviewStatus).toBe("approved")
      expect(result.switchPanelOpen).toBe(false)
    })

    it("should reject when not urgent and important and not insisted", () => {
      const shell = makeShell({
        switchForm: { temporaryGoalLabel: "普通任务", reason: "", dueDate: "", urgent: false, important: false, insistAfterReject: false },
      })
      const result = buildTemporarySwitchState(shell)
      expect(result.mainlineShell.temporaryMainline.reviewStatus).toBe("rejected")
      expect(result.mainlineShell.temporaryMainline.isActive).toBe(false)
      expect(result.switchPanelOpen).toBe(false)
    })

    it("should allow when user insisted after rejection", () => {
      const shell = makeShell({
        switchForm: { temporaryGoalLabel: "坚持任务", reason: "", dueDate: "", urgent: false, important: false, insistAfterReject: true },
      })
      const result = buildTemporarySwitchState(shell)
      expect(result.mainlineShell.temporaryMainline.reviewStatus).toBe("rejected")
      expect(result.mainlineShell.temporaryMainline.isActive).toBe(true)
      expect(result.mainlineShell.temporaryMainline.currentTemporaryGoal).toBe("坚持任务")
    })
  })

  describe("restoreOriginalMainlineState", () => {
    it("should restore original snapshot and reset temporary mainline", () => {
      const snapshot = {
        currentGoalLabel: "原目标",
        probabilityValue: 70,
        probabilityState: "known" as const,
        riskLevel: "green" as const,
        evidenceExpired: false,
        lastCriticalEvidenceAt: "2024-01-01T00:00:00.000Z",
        latestAdjustment: "原调整",
      }
      const shell = makeShell({
        temporaryMainline: {
          isActive: true,
          reviewStatus: "approved",
          reviewFeedback: null,
          currentTemporaryGoal: "临时目标",
          originalSnapshot: snapshot,
          currentSwitchStartedAt: "2024-01-01T00:00:00.000Z",
          activeRecordId: "2024-01-01T00:00:00.000Z",
          history: [{
            temporaryGoalLabel: "临时目标",
            originalGoalLabel: "原目标",
            reason: "",
            dueDate: "",
            approvedByReview: true,
            userInsisted: false,
            switchedAt: "2024-01-01T00:00:00.000Z",
            restoredAt: null,
          }],
        },
      })
      const result = restoreOriginalMainlineState(shell)
      expect(result.mainlineShell.currentGoalLabel).toBe("原目标")
      expect(result.mainlineShell.probabilityValue).toBe(70)
      expect(result.mainlineShell.temporaryMainline.isActive).toBe(false)
      expect(result.mainlineShell.temporaryMainline.currentTemporaryGoal).toBeNull()
      expect(result.mainlineShell.temporaryMainline.originalSnapshot).toBeNull()
      expect(result.mainlineShell.switchFeedback).toContain("已自动恢复")
    })

    it("should return unchanged when temporary mainline is not active", () => {
      const shell = makeShell({ temporaryMainline: { ...makeShell().temporaryMainline, isActive: false } })
      const result = restoreOriginalMainlineState(shell)
      expect(result.mainlineShell).toEqual(shell)
    })

    it("should restore history item with restoredAt", () => {
      const shell = makeShell({
        temporaryMainline: {
          isActive: true,
          reviewStatus: "approved",
          reviewFeedback: null,
          currentTemporaryGoal: "临时目标",
          originalSnapshot: {
            currentGoalLabel: "原目标",
            probabilityValue: null,
            probabilityState: "unknown",
            riskLevel: "yellow",
            evidenceExpired: true,
            lastCriticalEvidenceAt: null,
            latestAdjustment: "test",
          },
          currentSwitchStartedAt: "2024-01-01T00:00:00.000Z",
          activeRecordId: "2024-01-01T00:00:00.000Z",
          history: [{
            temporaryGoalLabel: "临时目标",
            originalGoalLabel: "原目标",
            reason: "",
            dueDate: "",
            approvedByReview: true,
            userInsisted: false,
            switchedAt: "2024-01-01T00:00:00.000Z",
            restoredAt: null,
          }],
        },
      })
      const result = restoreOriginalMainlineState(shell)
      expect(result.mainlineShell.temporaryMainline.history[0].restoredAt).not.toBeNull()
    })
  })
})
