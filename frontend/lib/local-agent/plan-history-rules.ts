import type {
  MainlineShellState,
  NextDayPlanState,
  PlanHistoryItem,
} from "./types"

export function applyPlanTracking(mainlineShell: MainlineShellState) {
  const nextShell = {
    ...mainlineShell,
    todayPlanStatusText: buildTodayPlanStatusText(mainlineShell.nextDayPlan),
    todayPlanReconcileText: buildTodayPlanReconcileText(mainlineShell),
  }
  return {
    ...nextShell,
    recentPlanHistory: buildRecentPlanHistory(nextShell),
  }
}

function buildTodayPlanStatusText(plan: NextDayPlanState) {
  if (plan.takeoverStatus === "pending_today") return "今天计划待接管"
  if (plan.takeoverStatus === "taken_over") return "今天计划已接管"
  if (plan.takeoverStatus === "completed") return "今天计划核心任务已完成"
  if (plan.takeoverStatus === "closed") return "今天计划已收尾"
  return "今日计划尚未接管"
}

function buildTodayPlanReconcileText(mainlineShell: MainlineShellState) {
  const status = mainlineShell.nextDayPlan.takeoverStatus
  if (status === "idle") return null
  if (status === "pending_today") return "今晚对账：今天尚未真正按计划启动，需要说明原因。"
  if (status === "taken_over") return pickTakenOverText(mainlineShell.evidenceSubmitted)
  if (status === "completed") return "今晚对账：计划核心任务已完成，补充结果即可。"
  return "今晚对账：今天已按计划收尾，可直接记录结果。"
}

function buildRecentPlanHistory(mainlineShell: MainlineShellState) {
  const current = toHistoryItem(mainlineShell)
  if (!current) return mainlineShell.recentPlanHistory
  const rest = mainlineShell.recentPlanHistory.filter((item) => item.targetDate !== current.targetDate)
  return [current, ...rest].slice(0, 3)
}

function toHistoryItem(mainlineShell: MainlineShellState) {
  const plan = mainlineShell.nextDayPlan
  if (!plan.targetDate) return null
  if (!isTrackedStatus(plan.takeoverStatus)) return null
  return {
    targetDate: plan.targetDate,
    taskLabel: plan.todayTaskLabel || plan.coreTaskLabel,
    status: plan.takeoverStatus,
    statusText: buildTodayPlanStatusText(plan),
    updatedAt: mainlineShell.execution.activeDate || plan.targetDate,
    reconciled: isReconciled(mainlineShell),
    reconcileText: buildHistoryReconcileText(mainlineShell),
  } satisfies PlanHistoryItem
}

function isTrackedStatus(status: NextDayPlanState["takeoverStatus"]) {
  return status === "taken_over" || status === "completed" || status === "closed"
}

function pickTakenOverText(evidenceSubmitted: boolean) {
  return evidenceSubmitted
    ? "今晚对账：今天计划仍在执行中，已提交证据时需说明未完成部分。"
    : "今晚对账：今天计划仍在执行中，提交证据时需说明偏差或未完成部分。"
}

function isReconciled(mainlineShell: MainlineShellState) {
  return mainlineShell.nextDayPlan.takeoverStatus === "closed"
    && mainlineShell.evidenceSubmitted
}

function buildHistoryReconcileText(mainlineShell: MainlineShellState) {
  return isReconciled(mainlineShell) ? "已对账" : "待对账"
}
