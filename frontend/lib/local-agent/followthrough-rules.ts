import { buildMainlineEntryRecommendation } from "./mainline-entry-rules"
import type { FollowthroughState, MainlineShellState } from "./types"

export function createEmptyFollowthroughState(): FollowthroughState {
  return {
    lastActionLabel: null,
    nextActionLabel: "先交今晚证据",
    nextActionHelper: "先完成晚间对账，再决定是否补关键证据或生成明日计划。",
    planUsesLatestEvidence: null,
  }
}

export function buildFollowthroughAfterEvidence(mainlineShell: MainlineShellState) {
  const nextStep = buildMainlineEntryRecommendation(mainlineShell)
  return {
    lastActionLabel: "已提交晚间证据",
    nextActionLabel: nextStep.label,
    nextActionHelper: mainlineShell.probabilityState === "unknown"
      ? "晚间证据已交，但关键判断仍未知；下一步优先补关键证据。"
      : "晚间证据已交，关键判断已知；下一步直接生成明日计划。",
    planUsesLatestEvidence: mainlineShell.probabilityState === "known",
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterKeyEvidence(mainlineShell: MainlineShellState) {
  const nextStep = pickPostEvidenceStep(mainlineShell)
  return {
    lastActionLabel: "已更新关键证据",
    nextActionLabel: nextStep.label,
    nextActionHelper: mainlineShell.timeBudgetInsight.level === "critical"
      ? "关键证据已更新，但今天时间极紧；下一步优先保今天核心任务。"
      : "关键证据已更新；下一步优先基于最新判断生成明日计划。",
    planUsesLatestEvidence: true,
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterPlan(mainlineShell: MainlineShellState) {
  const usesLatestEvidence = mainlineShell.probabilityState === "known" && !mainlineShell.evidenceExpired
  return {
    lastActionLabel: "已生成明日计划",
    nextActionLabel: "按计划执行",
    nextActionHelper: usesLatestEvidence
      ? "这份明日计划已基于最新关键证据，可按当前方案执行。"
      : "这份明日计划未拿到最新关键证据兜底，优先尽快补关键证据。",
    planUsesLatestEvidence: usesLatestEvidence,
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterTimeBudget(mainlineShell: MainlineShellState) {
  return {
    lastActionLabel: "已接管今天主线",
    nextActionLabel: "标记今天核心任务完成",
    nextActionHelper: `今天只保 ${mainlineShell.execution.currentTaskLabel || "唯一核心任务"}，完成后再决定是否收尾。`,
    planUsesLatestEvidence: mainlineShell.probabilityState === "known" && !mainlineShell.evidenceExpired,
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterCoreDone(mainlineShell: MainlineShellState) {
  return {
    lastActionLabel: "已完成今天核心任务",
    nextActionLabel: "进入今天收尾",
    nextActionHelper: "核心任务已经完成；如果没有新变量，今天到此为止，不再临时扩任务。",
    planUsesLatestEvidence: mainlineShell.probabilityState === "known" && !mainlineShell.evidenceExpired,
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterClose(mainlineShell: MainlineShellState) {
  return {
    lastActionLabel: "已进入今天收尾",
    nextActionLabel: "等待新的关键证据",
    nextActionHelper: "今天收尾后不再继续强推动作，直到出现新的关键证据或现实约束变化。",
    planUsesLatestEvidence: mainlineShell.probabilityState === "known" && !mainlineShell.evidenceExpired,
  } satisfies FollowthroughState
}

export function buildFollowthroughAfterDayReset(mainlineShell: MainlineShellState) {
  return {
    lastActionLabel: mainlineShell.execution.todayClosed ? "昨天已收尾" : "昨天未收尾",
    nextActionLabel: "重开今天主线",
    nextActionHelper: mainlineShell.execution.todayClosed ? "已进入新的一天，昨天收尾状态不再沿用；今天需要重新接管时间预算。" : "已进入新的一天，昨天未收口状态不能直接延续；今天先重新接管时间预算。",
    planUsesLatestEvidence: mainlineShell.probabilityState === "known" && !mainlineShell.evidenceExpired,
  } satisfies FollowthroughState
}

function pickPostEvidenceStep(mainlineShell: MainlineShellState) {
  if (mainlineShell.timeBudgetInsight.level === "critical") {
    return {
      label: "优先保今天核心任务",
      helperText: "关键证据已更新，但今天时间极紧；下一步优先保今天核心任务。",
    }
  }
  return {
    label: mainlineShell.nextDayPlan.ready ? "更新明日计划" : "优先生成明日计划",
    helperText: "关键证据已更新；下一步优先基于最新判断生成明日计划。",
  }
}
