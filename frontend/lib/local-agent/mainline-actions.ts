import type { StateCreator } from "zustand"
import { buildEvidenceDraftState, buildEvidenceRefreshState, buildKeyEvidenceDraftState } from "./evidence-flow-rules"
import { buildTimeBlockDraftState, buildTimeBudgetDraftState } from "./execution-flow-rules"
import {
  applyProbabilityState,
  buildCoreTaskDoneState,
  buildEvidenceSubmitState,
  buildKeyEvidenceState,
  buildNextDayPlanState,
  buildTimeBudgetSubmitState,
  buildTodayClosedState,
} from "./mainline-bridge-rules"
import { buildPersonalizedDraftState, buildPersonalizedSubmitState } from "./personalized-flow-rules"
import { syncPlanShell } from "./plan-sync-flow-rules"
import { buildSwitchDraftState, buildTemporarySwitchState, restoreOriginalMainlineState } from "./switch-flow-rules"
import type { KeyEvidenceEntry, MainlineRiskLevel, MainlineShellState } from "./types"

type UISet = Parameters<StateCreator<MainlineActionState>>[0]

type MainlineActionState = {
  evidencePanelOpen: boolean
  keyEvidencePanelOpen: boolean
  switchPanelOpen: boolean
  personalizedPanelOpen: boolean
  timeBudgetPanelOpen: boolean
  mainlineShell: MainlineShellState
}

export function createMainlineActions(set: UISet) {
  return {
    ...createGoalActions(set),
    ...createEvidenceActions(set),
    ...createSwitchActions(set),
    ...createPersonalizedActions(set),
    ...createTimeBudgetActions(set),
    ...createExecutionActions(set),
    ...createNextDayPlanActions(set),
  }
}

function createGoalActions(set: UISet) {
  return {
    setMainlineGoal: (label: string) => set((state: MainlineActionState) => ({ mainlineShell: { ...state.mainlineShell, currentGoalLabel: label } })),
    setMainlineProbability: (value: number, riskLevel: MainlineRiskLevel) =>
      set((state: MainlineActionState) => ({ mainlineShell: applyProbabilityState(state.mainlineShell, value, riskLevel) })),
    refreshMainlineEvidenceState: () =>
      set((state: MainlineActionState) => ({ mainlineShell: syncPlanShell(buildEvidenceRefreshState(state.mainlineShell)) })),
  }
}

function createEvidenceActions(set: UISet) {
  return {
    updateEvidencePacket: (patch: Partial<MainlineShellState["evidencePacket"]>) =>
      set((state: MainlineActionState) => ({ mainlineShell: buildEvidenceDraftState(state.mainlineShell, patch) })),
    submitEvidencePacket: () => set((state: MainlineActionState) => buildEvidenceSubmitState(state.mainlineShell)),
    updateKeyEvidenceEntry: (patch: Partial<KeyEvidenceEntry>) =>
      set((state: MainlineActionState) => ({ mainlineShell: buildKeyEvidenceDraftState(state.mainlineShell, patch) })),
    submitKeyEvidenceEntry: () => set((state: MainlineActionState) => buildKeyEvidenceState(state.mainlineShell)),
  }
}

function createSwitchActions(set: UISet) {
  return {
    updateSwitchForm: (patch: Partial<MainlineShellState["switchForm"]>) =>
      set((state: MainlineActionState) => ({ mainlineShell: buildSwitchDraftState(state.mainlineShell, patch) })),
    submitTemporaryMainlineSwitch: () =>
      set((state: MainlineActionState) => buildTemporarySwitchState(state.mainlineShell)),
    restoreOriginalMainline: () =>
      set((state: MainlineActionState) => restoreOriginalMainlineState(state.mainlineShell)),
  }
}

function createPersonalizedActions(set: UISet) {
  return {
    updatePersonalizedEntry: (patch: Partial<MainlineShellState["personalizedEntry"]>) =>
      set((state: MainlineActionState) => buildPersonalizedDraftState(state.mainlineShell, patch)),
    submitPersonalizedEntry: () =>
      set((state: MainlineActionState) => buildPersonalizedSubmitState(state.mainlineShell)),
  }
}

function createTimeBudgetActions(set: UISet) {
  return {
    updateTimeBudgetEntry: (patch: Partial<MainlineShellState["timeBudgetEntry"]>) =>
      set((state: MainlineActionState) => ({ mainlineShell: buildTimeBudgetDraftState(state.mainlineShell, patch) })),
    updateTimeBlockDraft: (patch: Partial<MainlineShellState["timeBudgetEntry"]["timeBlockDraft"]>) =>
      set((state: MainlineActionState) => ({ mainlineShell: buildTimeBlockDraftState(state.mainlineShell, patch) })),
    submitTimeBudgetEntry: () =>
      set((state: MainlineActionState) => buildTimeBudgetSubmitState(state.mainlineShell)),
  }
}

function createExecutionActions(set: UISet) {
  return {
    markTodayCoreTaskCompleted: () =>
      set((state: MainlineActionState) => ({ mainlineShell: buildCoreTaskDoneState(state.mainlineShell) })),
    closeToday: () =>
      set((state: MainlineActionState) => ({ mainlineShell: buildTodayClosedState(state.mainlineShell) })),
  }
}

function createNextDayPlanActions(set: UISet) {
  return {
    generateNextDayPlan: () =>
      set((state: MainlineActionState) => ({ mainlineShell: buildNextDayPlanState(state.mainlineShell) })),
  }
}
