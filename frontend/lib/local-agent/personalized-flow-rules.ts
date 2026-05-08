import {
  createEmptyPersonalizedEntry,
  evaluatePersonalizedEntry,
} from "./personalized-followup-rules"
import type { MainlineShellState } from "./types"

export function buildPersonalizedDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["personalizedEntry"]>,
) {
  return {
    mainlineShell: {
      ...mainlineShell,
      personalizedFeedback: null,
      personalizedEntry: {
        ...mainlineShell.personalizedEntry,
        ...patch,
      },
    },
  }
}

export function buildPersonalizedSubmitState(mainlineShell: MainlineShellState) {
  const result = evaluatePersonalizedEntry(
    mainlineShell.personalizedEntry,
    mainlineShell.personalizedHistory,
  )
  return {
    personalizedPanelOpen: false,
    mainlineShell: {
      ...mainlineShell,
      personalizedFeedback: result.feedback,
      personalizedEntry: createEmptyPersonalizedEntry(),
      personalizedHistory: appendPersonalizedRecord(mainlineShell, result.record),
      personalizedInsight: result.insight,
    },
  }
}

function appendPersonalizedRecord(
  mainlineShell: MainlineShellState,
  record: MainlineShellState["personalizedHistory"][number] | null,
) {
  return record
    ? [...mainlineShell.personalizedHistory, record]
    : mainlineShell.personalizedHistory
}
