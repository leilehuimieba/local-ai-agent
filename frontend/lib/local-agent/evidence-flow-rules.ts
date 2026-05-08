import {
  buildWeakEvidenceSummary,
  createEmptyKeyEvidenceEntry,
  isCriticalEvidenceExpired,
} from "./mainline-rules"
import { refreshEveningReview, submitEveningReview } from "./evening-review-rules"
import {
  buildFollowthroughAfterDayReset,
  buildFollowthroughAfterEvidence,
  buildFollowthroughAfterKeyEvidence,
} from "./followthrough-rules"
import { hasExecutionRolledOver, buildExecutionAfterDayReset } from "./execution-state-rules"
import type { MainlineShellState } from "./types"

export type KeyEvidenceResult = {
  status: "strong" | "weak"
  feedback: string
  probabilityValue: number | null
  riskLevel?: MainlineShellState["riskLevel"]
  adjustment: string
  record?: MainlineShellState["keyEvidenceHistory"][number]
}

export function buildEvidenceRefreshState(mainlineShell: MainlineShellState) {
  const expired = shouldExpireEvidence(mainlineShell)
  const resetShell = hasExecutionRolledOver(mainlineShell)
    ? applyDayReset(mainlineShell)
    : mainlineShell
  const eveningReview = refreshEveningReview(resetShell)
  return {
    ...resetShell,
    probabilityValue: expired ? null : resetShell.probabilityValue,
    probabilityState: expired ? "unknown" : resetShell.probabilityState,
    evidenceExpired: expired,
    evidenceSubmitted: eveningReview.status === "submitted",
    eveningReview,
    followthrough: hasExecutionRolledOver(mainlineShell)
      ? buildFollowthroughAfterDayReset(resetShell)
      : resetShell.followthrough,
  }
}

export function buildEvidenceDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["evidencePacket"]>,
) {
  return {
    ...mainlineShell,
    evidenceSubmitted: false,
    evidencePacket: {
      ...mainlineShell.evidencePacket,
      ...patch,
    },
  }
}

export function buildEvidenceSubmitShell(mainlineShell: MainlineShellState) {
  const eveningReview = submitEveningReview(mainlineShell)
  const nextShell = {
    ...mainlineShell,
    evidenceSubmitted: true,
    eveningReview,
  }
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterEvidence(nextShell),
  }
}

export function buildKeyEvidenceDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["keyEvidenceEntry"]>,
) {
  return {
    ...mainlineShell,
    keyEvidenceFeedback: null,
    keyEvidenceEntry: {
      ...mainlineShell.keyEvidenceEntry,
      ...patch,
    },
  }
}

export function applyStrongEvidence(mainlineShell: MainlineShellState, result: KeyEvidenceResult) {
  const nextShell = {
    ...mainlineShell,
    probabilityValue: result.probabilityValue,
    probabilityState: "known" as const,
    riskLevel: result.riskLevel || mainlineShell.riskLevel,
    evidenceExpired: false,
    lastCriticalEvidenceAt: result.record?.submittedAt || new Date().toISOString(),
    latestAdjustment: result.adjustment,
    keyEvidenceFeedback: result.feedback,
    keyEvidenceEntry: createEmptyKeyEvidenceEntry(),
    keyEvidenceHistory: result.record ? [...mainlineShell.keyEvidenceHistory, result.record] : mainlineShell.keyEvidenceHistory,
  }
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterKeyEvidence(nextShell),
  }
}

export function applyWeakEvidence(mainlineShell: MainlineShellState, feedback: string) {
  const entry = mainlineShell.keyEvidenceEntry
  return {
    ...mainlineShell,
    keyEvidenceFeedback: `${feedback} ${buildWeakEvidenceSummary(entry, entry.sourceType)}`,
    latestAdjustment: "先补齐完整关键证据，再决定是否调整模块押注。",
    keyEvidenceEntry: createEmptyKeyEvidenceEntry(),
  }
}

function applyDayReset(mainlineShell: MainlineShellState) {
  const execution = buildExecutionAfterDayReset(mainlineShell)
  return {
    ...mainlineShell,
    evidenceSubmitted: false,
    execution,
    followthrough: buildFollowthroughAfterDayReset({
      ...mainlineShell,
      execution,
    }),
  }
}

function shouldExpireEvidence(mainlineShell: MainlineShellState) {
  if (!mainlineShell.lastCriticalEvidenceAt) {
    return mainlineShell.evidenceExpired
  }
  return isCriticalEvidenceExpired(mainlineShell.lastCriticalEvidenceAt)
}
