import { createEmptyEveningReviewState } from "./evening-review-rules"
import { createEmptyExecutionState } from "./execution-state-rules"
import { createEmptyFollowthroughState } from "./followthrough-rules"
import { createEmptyKeyEvidenceEntry } from "./mainline-rules"
import { createEmptySwitchForm } from "./mainline-switch-rules"
import { createEmptyNextDayPlan } from "./next-day-plan-rules"
import {
  createEmptyPersonalizedEntry,
  createEmptyPersonalizedInsight,
} from "./personalized-followup-rules"
import {
  createEmptyTimeBudgetEntry,
  createEmptyTimeBudgetInsight,
} from "./time-budget-rules"
import type { MainlineShellState } from "./types"

export function createUIMainlineState() {
  return {
    mainlineExpanded: false,
    evidencePanelOpen: false,
    keyEvidencePanelOpen: false,
    switchPanelOpen: false,
    personalizedPanelOpen: false,
    timeBudgetPanelOpen: false,
    mainlineShell: createDefaultMainlineShell(),
  }
}

export function createDefaultMainlineShell(): MainlineShellState {
  return {
    ...createDefaultMainlineCore(),
    ...createDefaultMainlineProgress(),
    ...createDefaultMainlinePlanState(),
  }
}

function createDefaultMainlineCore() {
  return {
    currentGoalLabel: "四级冲刺",
    probabilityValue: null,
    probabilityState: "unknown" as const,
    riskLevel: "yellow" as const,
    evidenceExpired: true,
    evidenceSubmitted: false,
    lastCriticalEvidenceAt: null,
    latestAdjustment: "请先补充一份完整模考 / 真题结果，再决定模块级调整。",
    keyEvidenceFeedback: null,
    switchFeedback: null,
    evidencePacket: createDefaultEvidencePacket(),
    keyEvidenceEntry: createEmptyKeyEvidenceEntry(),
    keyEvidenceHistory: [],
    switchForm: createEmptySwitchForm(),
    temporaryMainline: createDefaultTemporaryState(),
    personalizedFeedback: null,
    personalizedEntry: createEmptyPersonalizedEntry(),
    personalizedHistory: [],
    personalizedInsight: createEmptyPersonalizedInsight(),
  }
}

function createDefaultMainlineProgress() {
  return {
    timeBudgetFeedback: null,
    timeBudgetEntry: createEmptyTimeBudgetEntry(),
    timeBlocks: [],
    timeBudgetInsight: createEmptyTimeBudgetInsight(),
    eveningReview: createEmptyEveningReviewState(),
    followthrough: createEmptyFollowthroughState(),
    execution: createEmptyExecutionState(),
  }
}

function createDefaultMainlinePlanState() {
  return {
    nextDayPlan: createEmptyNextDayPlan(),
    todayPlanStatusText: "今日计划尚未接管",
    todayPlanReconcileText: null,
    recentPlanHistory: [],
  }
}

function createDefaultEvidencePacket() {
  return { didWhat: "", resultSummary: "", blockers: "", nextAdjustment: "" }
}

function createDefaultTemporaryState(): MainlineShellState["temporaryMainline"] {
  return {
    isActive: false,
    reviewStatus: "idle",
    reviewFeedback: null,
    currentTemporaryGoal: null,
    originalSnapshot: null,
    currentSwitchStartedAt: null,
    activeRecordId: null,
    history: [],
  }
}
