import {
  buildExecutionAfterClose,
  buildExecutionAfterCoreDone,
  buildExecutionAfterTimeBudget,
} from "./execution-state-rules"
import {
  buildFollowthroughAfterClose,
  buildFollowthroughAfterCoreDone,
  buildFollowthroughAfterTimeBudget,
} from "./followthrough-rules"
import { createEmptyTimeBudgetEntry } from "./time-budget-rules"
import type { MainlineShellState } from "./types"

export function buildTimeBudgetDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["timeBudgetEntry"]>,
) {
  return {
    ...mainlineShell,
    timeBudgetFeedback: null,
    timeBudgetEntry: {
      ...mainlineShell.timeBudgetEntry,
      ...patch,
    },
  }
}

export function buildTimeBlockDraftState(
  mainlineShell: MainlineShellState,
  patch: Partial<MainlineShellState["timeBudgetEntry"]["timeBlockDraft"]>,
) {
  return {
    ...mainlineShell,
    timeBudgetFeedback: null,
    timeBudgetEntry: {
      ...mainlineShell.timeBudgetEntry,
      timeBlockDraft: {
        ...mainlineShell.timeBudgetEntry.timeBlockDraft,
        ...patch,
      },
    },
  }
}

export function buildTimeBudgetCommittedShell(
  mainlineShell: MainlineShellState,
  insight: MainlineShellState["timeBudgetInsight"],
  nextBlocks: MainlineShellState["timeBlocks"],
  feedback: string,
) {
  const nextShell = {
    ...mainlineShell,
    timeBudgetFeedback: feedback,
    latestAdjustment: insight.recommendation,
    timeBlocks: nextBlocks,
    timeBudgetInsight: insight,
    timeBudgetEntry: {
      ...mainlineShell.timeBudgetEntry,
      timeBlockDraft: createEmptyTimeBudgetEntry().timeBlockDraft,
    },
  }
  const execution = buildExecutionAfterTimeBudget(nextShell)
  return {
    ...nextShell,
    execution,
    followthrough: buildFollowthroughAfterTimeBudget({ ...nextShell, execution }),
  }
}

export function buildCoreTaskDoneShell(mainlineShell: MainlineShellState) {
  const execution = buildExecutionAfterCoreDone(mainlineShell)
  const nextShell = { ...mainlineShell, execution }
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterCoreDone(nextShell),
  }
}

export function buildTodayClosedShell(mainlineShell: MainlineShellState) {
  const execution = buildExecutionAfterClose(mainlineShell)
  const nextShell = { ...mainlineShell, execution }
  return {
    ...nextShell,
    followthrough: buildFollowthroughAfterClose(nextShell),
  }
}
