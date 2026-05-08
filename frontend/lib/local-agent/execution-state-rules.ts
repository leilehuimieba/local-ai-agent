import type { ExecutionState, MainlineShellState } from "./types"

export function createEmptyExecutionState(): ExecutionState {
  return {
    status: "idle",
    currentTaskLabel: null,
    todayCoreTaskCompleted: false,
    todayClosed: false,
    statusText: "未进入执行态",
    helperText: "先拿到今日主线接管结果，再进入执行态。",
    activeDate: null,
    needsReopen: false,
    staleFromDate: null,
  }
}

export function buildExecutionAfterTimeBudget(mainlineShell: MainlineShellState, now = new Date()): ExecutionState {
  return {
    status: "executing",
    currentTaskLabel: pickExecutionTaskLabel(mainlineShell),
    todayCoreTaskCompleted: false,
    todayClosed: false,
    statusText: "正在执行今天核心任务",
    helperText: "当前只盯这一项，其余动作全部降级。",
    activeDate: localDay(now),
    needsReopen: false,
    staleFromDate: null,
  }
}

export function buildExecutionAfterCoreDone(mainlineShell: MainlineShellState, now = new Date()): ExecutionState {
  return {
    status: "completed",
    currentTaskLabel: pickExecutionTaskLabel(mainlineShell),
    todayCoreTaskCompleted: true,
    todayClosed: false,
    statusText: "今天核心任务已完成",
    helperText: "若没有新的关键证据或现实约束变化，下一步直接进入今天收尾。",
    activeDate: pickActiveDate(mainlineShell, now),
    needsReopen: false,
    staleFromDate: null,
  }
}

export function buildExecutionAfterClose(mainlineShell: MainlineShellState, now = new Date()): ExecutionState {
  return {
    status: "closed",
    currentTaskLabel: pickExecutionTaskLabel(mainlineShell),
    todayCoreTaskCompleted: true,
    todayClosed: true,
    statusText: "今天已收尾",
    helperText: "今天不再继续强推次优动作，等待新的关键证据或新的现实约束变化。",
    activeDate: pickActiveDate(mainlineShell, now),
    needsReopen: false,
    staleFromDate: null,
  }
}

export function hasExecutionRolledOver(mainlineShell: MainlineShellState, now = new Date()) {
  const activeDate = mainlineShell.execution.activeDate
  return Boolean(activeDate && activeDate !== localDay(now))
}

export function buildExecutionAfterDayReset(mainlineShell: MainlineShellState, now = new Date()): ExecutionState {
  const previous = mainlineShell.execution
  return {
    status: "idle",
    currentTaskLabel: previous.currentTaskLabel,
    todayCoreTaskCompleted: false,
    todayClosed: false,
    statusText: previous.todayClosed ? "新的一天待重开" : "昨日未收尾，今天待重开",
    helperText: previous.todayClosed ? "昨天已收尾；今天需要重新录入现实时间预算后再进入执行态。" : "昨天未收口；今天先重新录入现实时间预算，再决定是否继续昨天任务。",
    activeDate: localDay(now),
    needsReopen: true,
    staleFromDate: previous.activeDate,
  }
}

function pickExecutionTaskLabel(mainlineShell: MainlineShellState) {
  const current = mainlineShell.execution.currentTaskLabel?.trim()
  if (current) return current
  const budgetTask = mainlineShell.timeBudgetEntry.coreTaskLabel.trim()
  return budgetTask || mainlineShell.currentGoalLabel
}

function pickActiveDate(mainlineShell: MainlineShellState, now: Date) {
  return mainlineShell.execution.activeDate || localDay(now)
}

function localDay(date: Date) {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, "0")
  const day = String(date.getDate()).padStart(2, "0")
  return `${year}-${month}-${day}`
}
