import { buildFollowthroughAfterClose, buildFollowthroughAfterCoreDone, buildFollowthroughAfterEvidence, buildFollowthroughAfterPlan, buildFollowthroughAfterTimeBudget } from "./followthrough-rules"
import { buildNextDayPlan, syncNextDayPlan } from "./next-day-plan-rules"
import { applyPlanTracking } from "./plan-history-rules"
import type { MainlineShellState } from "./types"

export function syncPlanShell(mainlineShell: MainlineShellState) {
  const nextShell = {
    ...mainlineShell,
    nextDayPlan: syncNextDayPlan(mainlineShell),
  }
  return applyPlanTracking(nextShell)
}

export function buildEvidenceSubmittedState(mainlineShell: MainlineShellState) {
  const syncedShell = syncPlanShell(mainlineShell)
  return {
    evidencePanelOpen: false,
    mainlineShell: {
      ...syncedShell,
      followthrough: buildFollowthroughAfterEvidence(syncedShell),
    },
  }
}

export function buildTimeBudgetSubmittedState(mainlineShell: MainlineShellState) {
  return {
    timeBudgetPanelOpen: false,
    mainlineShell: syncPlanShell(mainlineShell),
  }
}

export function buildCoreTaskCompletedState(mainlineShell: MainlineShellState) {
  const nextShell = syncPlanShell(mainlineShell)
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterCoreDone(nextShell),
  }
}

export function buildTodayClosedPlanState(mainlineShell: MainlineShellState) {
  const nextShell = syncPlanShell(mainlineShell)
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterClose(nextShell),
  }
}

export function buildNextDayPlanGeneratedState(mainlineShell: MainlineShellState) {
  const syncedShell = syncPlanShell({
    ...mainlineShell,
    nextDayPlan: buildNextDayPlan(mainlineShell),
  })
  return {
    ...syncedShell,
    followthrough: buildFollowthroughAfterPlan(syncedShell),
  }
}

export function buildTimeBudgetPanelState(
  mainlineShell: MainlineShellState,
  timeBudgetPanelOpen: boolean,
) {
  if (!timeBudgetPanelOpen) return { timeBudgetPanelOpen }
  return {
    timeBudgetPanelOpen,
    mainlineShell: buildTimeBudgetTakeoverDraft(mainlineShell),
  }
}

function buildTimeBudgetTakeoverDraft(mainlineShell: MainlineShellState) {
  const task = pickTodayPlanTask(mainlineShell)
  if (!task) return mainlineShell
  return {
    ...mainlineShell,
    timeBudgetEntry: {
      ...mainlineShell.timeBudgetEntry,
      coreTaskLabel: task,
    },
  }
}

function pickTodayPlanTask(mainlineShell: MainlineShellState) {
  const plan = mainlineShell.nextDayPlan
  if (!plan.ready || plan.targetDate !== todayKey()) return null
  return plan.coreTaskLabel
}

function todayKey(now = new Date()) {
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, "0")
  const day = String(now.getDate()).padStart(2, "0")
  return `${year}-${month}-${day}`
}
