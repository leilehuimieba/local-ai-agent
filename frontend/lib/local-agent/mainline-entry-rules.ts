import type { MainlineShellState } from "./types"

export type MainlineEntryAction =
  | "evidence_packet"
  | "key_evidence"
  | "next_day_plan"
  | "time_budget"
  | "complete_core_task"
  | "close_day"
  | "noop"

export interface MainlineEntryRecommendation {
  action: MainlineEntryAction
  label: string
  helperText: string
}

export function buildMainlineEntryRecommendation(mainlineShell: MainlineShellState) {
  if (mainlineShell.execution.todayClosed) return buildClosedRecommendation()
  if (mainlineShell.execution.todayCoreTaskCompleted) return buildCloseDayRecommendation()
  if (mainlineShell.execution.status === "executing") {
    return buildCompleteTaskRecommendation(mainlineShell.execution.currentTaskLabel)
  }
  if (shouldMakeUpCriticalEvidence(mainlineShell)) return buildMakeUpEvidenceRecommendation()
  if (mainlineShell.execution.needsReopen) return buildReopenRecommendation(mainlineShell.execution.helperText)
  if (mainlineShell.probabilityState === "unknown") return buildUnknownRecommendation(mainlineShell.eveningReview.status)
  if (shouldFollowKeyEvidence(mainlineShell)) return buildPostEvidenceRecommendation(mainlineShell)
  if (shouldProtectToday(mainlineShell)) return buildTodayRecommendation()
  if (mainlineShell.evidenceSubmitted) return buildPlanRecommendation(mainlineShell.nextDayPlan.ready)
  if (mainlineShell.eveningReview.status === "late_allowed") return buildLatePacketRecommendation()
  return buildDefaultRecommendation()
}

function buildUnknownRecommendation(status: MainlineShellState["eveningReview"]["status"]) {
  return createRecommendation("key_evidence", "优先补关键证据", pickUnknownHelper(status))
}

function pickUnknownHelper(status: MainlineShellState["eveningReview"]["status"]) {
  if (status === "late_allowed") return "概率未知时，先补会改变判断的模拟题 / 真题结果；昨晚证据可作为次级补交。"
  if (status === "reroute") return "补交窗口已过，至少先补会改变判断的关键证据，不再沿用旧概率。"
  return "当前概率未知，晚间入口优先直达最新模拟题 / 真题结果录入。"
}

function shouldProtectToday(mainlineShell: MainlineShellState) {
  return mainlineShell.eveningReview.status === "reroute"
    && mainlineShell.eveningReview.branchDecision === "continue_today"
}

function shouldFollowKeyEvidence(mainlineShell: MainlineShellState) {
  return mainlineShell.followthrough.lastActionLabel === "已更新关键证据"
}

function shouldMakeUpCriticalEvidence(mainlineShell: MainlineShellState) {
  return mainlineShell.eveningReview.status === "reroute"
    && mainlineShell.eveningReview.branchDecision === "make_up_yesterday"
}

function buildTodayRecommendation() {
  return createRecommendation("time_budget", "优先保今天核心任务", "补交窗口已过，优先按现实时间预算保住今天唯一核心任务。")
}

function buildReopenRecommendation(helperText: string) {
  return createRecommendation("time_budget", "重开今天主线", helperText)
}

function buildMakeUpEvidenceRecommendation() {
  return createRecommendation("key_evidence", "优先补关键证据", "昨天的关键判断仍会影响今天安排；先补会改变判断的关键证据。")
}

function buildCompleteTaskRecommendation(taskLabel: string | null) {
  const helperText = taskLabel ? `当前执行项：${taskLabel}。完成后立刻回写，不再让系统继续分散注意力。` : "今天已进入执行态，先做完唯一核心任务，再决定是否收尾。"
  return createRecommendation("complete_core_task", "标记今天核心任务完成", helperText)
}

function buildCloseDayRecommendation() {
  return createRecommendation("close_day", "进入今天收尾", "今天核心任务已经完成，若无新变量，直接结束今天，不再追加次优动作。")
}

function buildClosedRecommendation() {
  return createRecommendation("noop", "今天已收尾", "今天已经进入收尾态；只有拿到新的关键证据或现实时间预算变化时才重新打开。")
}

function buildPostEvidenceRecommendation(mainlineShell: MainlineShellState) {
  if (mainlineShell.timeBudgetInsight.level === "critical") return buildTodayRecommendation()
  return buildPlanRecommendation(mainlineShell.nextDayPlan.ready)
}

function buildPlanRecommendation(ready: boolean) {
  const label = ready ? "更新明日计划" : "优先生成明日计划"
  const helper = ready ? "晚间证据与关键判断已齐，可按最新状态重算明日方案。" : "晚间证据与关键判断已齐，下一步直接生成明日计划。"
  return createRecommendation("next_day_plan", label, helper)
}

function buildLatePacketRecommendation() {
  return createRecommendation("evidence_packet", "先补交昨晚证据", "先把昨晚证据补齐，再决定是否继续补关键证据或直接生成明日计划。")
}

function buildDefaultRecommendation() {
  return createRecommendation("evidence_packet", "先交今晚证据", "先完成晚间对账，再决定是否补关键证据或生成明日计划。")
}

function createRecommendation(action: MainlineEntryAction, label: string, helperText: string) {
  return { action, label, helperText } satisfies MainlineEntryRecommendation
}
