import {
  applyStrongEvidence,
  applyWeakEvidence,
  buildEvidenceSubmitShell,
} from "./evidence-flow-rules"
import {
  buildCoreTaskDoneShell,
  buildTimeBudgetCommittedShell,
  buildTodayClosedShell,
} from "./execution-flow-rules"
import { evaluateKeyEvidenceEntry } from "./mainline-rules"
import {
  buildCoreTaskCompletedState,
  buildEvidenceSubmittedState,
  buildNextDayPlanGeneratedState,
  buildTimeBudgetPanelState,
  buildTimeBudgetSubmittedState,
  buildTodayClosedPlanState,
} from "./plan-sync-flow-rules"
import { evaluateTimeBudgetEntry } from "./time-budget-rules"
import type { MainlineRiskLevel, MainlineShellState } from "./types"

export function applyProbabilityState(
  mainlineShell: MainlineShellState,
  value: number,
  riskLevel: MainlineRiskLevel,
) {
  return {
    ...mainlineShell,
    probabilityValue: value,
    probabilityState: "known" as const,
    riskLevel,
    evidenceExpired: false,
  }
}

export function buildEvidenceSubmitState(mainlineShell: MainlineShellState) {
  return buildEvidenceSubmittedState(buildEvidenceSubmitShell(mainlineShell))
}

export function buildKeyEvidenceState(mainlineShell: MainlineShellState) {
  const result = evaluateKeyEvidenceEntry(
    mainlineShell.keyEvidenceEntry,
    mainlineShell.keyEvidenceHistory,
    mainlineShell.probabilityValue,
  )
  return {
    keyEvidencePanelOpen: false,
    mainlineShell: result.status === "strong"
      ? applyStrongEvidence(mainlineShell, result)
      : applyWeakEvidence(mainlineShell, result.feedback),
  }
}

export function buildTimeBudgetSubmitState(mainlineShell: MainlineShellState) {
  const result = evaluateTimeBudgetEntry(
    mainlineShell.timeBudgetEntry,
    mainlineShell.timeBlocks,
  )
  return buildTimeBudgetSubmittedState(
    buildTimeBudgetCommittedShell(
      mainlineShell,
      result.insight,
      result.nextBlocks,
      result.feedback,
    ),
  )
}

export function buildCoreTaskDoneState(mainlineShell: MainlineShellState) {
  return buildCoreTaskCompletedState(buildCoreTaskDoneShell(mainlineShell))
}

export function buildTodayClosedState(mainlineShell: MainlineShellState) {
  return buildTodayClosedPlanState(buildTodayClosedShell(mainlineShell))
}

export function buildNextDayPlanState(mainlineShell: MainlineShellState) {
  return buildNextDayPlanGeneratedState(mainlineShell)
}

export function buildTimeBudgetPanelOpenState(
  mainlineShell: MainlineShellState,
  timeBudgetPanelOpen: boolean,
) {
  return buildTimeBudgetPanelState(mainlineShell, timeBudgetPanelOpen)
}
