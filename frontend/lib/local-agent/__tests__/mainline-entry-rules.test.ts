import { describe, expect, it } from "vitest"
import { buildMainlineEntryRecommendation } from "../mainline-entry-rules"
import type { MainlineShellState } from "../types"

function makeShell(patch: Partial<MainlineShellState> = {}): MainlineShellState {
  return {
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
    ...patch,
  } as MainlineShellState
}

describe("mainline-entry-rules", () => {
  it("returns noop when today is closed", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({ execution: { ...makeShell().execution, todayClosed: true } }))
    expect(rec.action).toBe("noop")
    expect(rec.label).toBe("今天已收尾")
  })

  it("returns close_day when core task completed", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({ execution: { ...makeShell().execution, todayCoreTaskCompleted: true } }))
    expect(rec.action).toBe("close_day")
    expect(rec.label).toBe("进入今天收尾")
  })

  it("returns complete_core_task when executing", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({ execution: { ...makeShell().execution, status: "executing", currentTaskLabel: "阅读训练" } }))
    expect(rec.action).toBe("complete_core_task")
    expect(rec.label).toBe("标记今天核心任务完成")
  })

  it("returns key_evidence when probability is unknown", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({ probabilityState: "unknown", execution: { ...makeShell().execution, needsReopen: false } }))
    expect(rec.action).toBe("key_evidence")
    expect(rec.label).toBe("优先补关键证据")
  })

  it("returns next_day_plan when evidence submitted and probability known", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      probabilityState: "known",
      evidenceSubmitted: true,
      nextDayPlan: { ...makeShell().nextDayPlan, ready: true },
    }))
    expect(rec.action).toBe("next_day_plan")
    expect(rec.label).toBe("更新明日计划")
  })

  it("returns key_evidence for make_up_yesterday branch", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      eveningReview: { ...makeShell().eveningReview, status: "reroute", branchDecision: "make_up_yesterday" },
    }))
    expect(rec.action).toBe("key_evidence")
    expect(rec.label).toBe("优先补关键证据")
  })

  it("returns time_budget for continue_today branch", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      probabilityState: "known",
      eveningReview: { ...makeShell().eveningReview, status: "reroute", branchDecision: "continue_today" },
    }))
    expect(rec.action).toBe("time_budget")
    expect(rec.label).toBe("优先保今天核心任务")
  })

  it("returns evidence_packet for late_allowed with known probability", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      eveningReview: { ...makeShell().eveningReview, status: "late_allowed" },
      probabilityState: "known",
      evidenceSubmitted: false,
    }))
    expect(rec.action).toBe("evidence_packet")
    expect(rec.label).toBe("先补交昨晚证据")
  })

  it("returns next_day_plan when followthrough says key evidence updated and plan not ready", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      followthrough: { ...makeShell().followthrough, lastActionLabel: "已更新关键证据" },
      probabilityState: "known",
      nextDayPlan: { ...makeShell().nextDayPlan, ready: false },
    }))
    expect(rec.action).toBe("next_day_plan")
    expect(rec.label).toBe("优先生成明日计划")
  })

  it("returns time_budget when needsReopen is true", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({
      execution: { ...makeShell().execution, needsReopen: true, helperText: "test" },
    }))
    expect(rec.action).toBe("time_budget")
    expect(rec.label).toBe("重开今天主线")
  })

  it("returns default evidence_packet when nothing special", () => {
    const rec = buildMainlineEntryRecommendation(makeShell({ probabilityState: "known" }))
    expect(rec.action).toBe("evidence_packet")
    expect(rec.label).toBe("先交今晚证据")
  })
})
