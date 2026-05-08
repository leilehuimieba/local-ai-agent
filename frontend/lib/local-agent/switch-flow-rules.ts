import {
  buildMainlineSnapshot,
  buildRestoreAdjustment,
  buildSwitchHistory,
  buildTemporaryAdjustment,
  createEmptySwitchForm,
  reviewTemporaryMainline,
} from "./mainline-switch-rules"
import type { MainlineShellState, MainlineSnapshot } from "./types"

type SwitchReviewResult = ReturnType<typeof reviewTemporaryMainline>

export function buildSwitchDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["switchForm"]>,
) {
  return {
    ...mainlineShell,
    switchFeedback: null,
    temporaryMainline: resetSwitchReview(mainlineShell),
    switchForm: {
      ...mainlineShell.switchForm,
      ...patch,
    },
  }
}

export function buildTemporarySwitchState(mainlineShell: MainlineShellState) {
  const review = reviewTemporaryMainline(mainlineShell.switchForm)
  if (!review.approvedByReview && !review.userInsisted) {
    return buildRejectedSwitchState(mainlineShell, review)
  }
  return buildAcceptedSwitchState(mainlineShell, review)
}

export function restoreOriginalMainlineState(mainlineShell: MainlineShellState) {
  const temp = mainlineShell.temporaryMainline
  if (!temp.isActive || !temp.originalSnapshot) return { mainlineShell }
  const now = new Date().toISOString()
  return {
    mainlineShell: {
      ...mainlineShell,
      ...temp.originalSnapshot,
      latestAdjustment: buildRestoreAdjustment(temp.originalSnapshot),
      switchFeedback: "临时主线已结束，已自动恢复原主线优先级结构。",
      temporaryMainline: buildRestoredTemporaryState(mainlineShell, now),
    },
  }
}

function resetSwitchReview(mainlineShell: MainlineShellState) {
  return {
    ...mainlineShell.temporaryMainline,
    reviewStatus: "idle" as const,
    reviewFeedback: null,
  }
}

function buildRejectedSwitchState(
  mainlineShell: MainlineShellState,
  review: SwitchReviewResult,
) {
  return {
    switchPanelOpen: false,
    mainlineShell: {
      ...mainlineShell,
      switchFeedback: review.reviewFeedback,
      temporaryMainline: {
        ...mainlineShell.temporaryMainline,
        reviewStatus: review.reviewStatus,
        reviewFeedback: review.reviewFeedback,
      },
    },
  }
}

function buildAcceptedSwitchState(
  mainlineShell: MainlineShellState,
  review: SwitchReviewResult,
) {
  const now = new Date().toISOString()
  const goalLabel = mainlineShell.switchForm.temporaryGoalLabel.trim()
  const snapshot = buildSwitchSnapshot(mainlineShell)
  return {
    switchPanelOpen: false,
    mainlineShell: buildAcceptedMainlineShell(mainlineShell, review, snapshot, now, goalLabel),
  }
}

function buildSwitchSnapshot(mainlineShell: MainlineShellState) {
  return buildMainlineSnapshot({
    currentGoalLabel: mainlineShell.currentGoalLabel,
    probabilityValue: mainlineShell.probabilityValue,
    probabilityState: mainlineShell.probabilityState,
    riskLevel: mainlineShell.riskLevel,
    evidenceExpired: mainlineShell.evidenceExpired,
    lastCriticalEvidenceAt: mainlineShell.lastCriticalEvidenceAt,
    latestAdjustment: mainlineShell.latestAdjustment,
  })
}

function buildAcceptedMainlineShell(
  mainlineShell: MainlineShellState,
  review: SwitchReviewResult,
  snapshot: MainlineSnapshot,
  now: string,
  goalLabel: string,
) {
  return {
    ...mainlineShell,
    currentGoalLabel: goalLabel,
    probabilityValue: null,
    probabilityState: "unknown" as const,
    riskLevel: "orange" as const,
    evidenceExpired: true,
    latestAdjustment: buildTemporaryAdjustment(mainlineShell.switchForm),
    switchFeedback: review.reviewFeedback,
    switchForm: createEmptySwitchForm(),
    temporaryMainline: buildActivatedTemporaryState(mainlineShell, review, snapshot, now, goalLabel),
  }
}

function buildActivatedTemporaryState(
  mainlineShell: MainlineShellState,
  review: SwitchReviewResult,
  snapshot: MainlineSnapshot,
  now: string,
  goalLabel: string,
) {
  const record = buildSwitchHistory(mainlineShell.switchForm, snapshot, review, now)
  return {
    ...mainlineShell.temporaryMainline,
    isActive: true,
    reviewStatus: review.reviewStatus,
    reviewFeedback: review.reviewFeedback,
    currentTemporaryGoal: goalLabel,
    originalSnapshot: snapshot,
    currentSwitchStartedAt: now,
    activeRecordId: now,
    history: [...mainlineShell.temporaryMainline.history, record],
  }
}

function buildRestoredTemporaryState(
  mainlineShell: MainlineShellState,
  now: string,
) {
  const temp = mainlineShell.temporaryMainline
  return {
    ...temp,
    isActive: false,
    currentTemporaryGoal: null,
    originalSnapshot: null,
    currentSwitchStartedAt: null,
    activeRecordId: null,
    history: temp.history.map((item) =>
      item.switchedAt === temp.activeRecordId ? { ...item, restoredAt: now } : item,
    ),
  }
}
