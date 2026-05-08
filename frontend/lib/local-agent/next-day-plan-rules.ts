import type {
  MainlineShellState,
  NextDayPlanState,
} from "./types"

export function createEmptyNextDayPlan(): NextDayPlanState {
  return {
    generatedAt: null,
    ready: false,
    coreTaskLabel: "请先补关键证据后再生成明日计划",
    supportTaskLabel: "暂无补充任务",
    rationale: "当前证据不足，先保守处理。",
    restoreNote: null,
    targetDate: null,
    basedOnEvidenceDate: null,
    needsRefresh: false,
    statusText: "暂无明日计划",
    refreshReason: null,
    todayTaskLabel: null,
    takeoverStatus: "idle",
    takeoverHint: null,
  }
}

export function buildNextDayPlan(mainlineShell: MainlineShellState, now = new Date()) {
  const coreTaskLabel = pickCoreTask(mainlineShell)
  const targetDate = localDay(addDays(now, 1))
  return {
    generatedAt: now.toISOString(),
    ready: true,
    coreTaskLabel,
    supportTaskLabel: pickSupportTask(mainlineShell),
    rationale: buildRationale(mainlineShell),
    restoreNote: buildRestoreNote(mainlineShell),
    targetDate,
    basedOnEvidenceDate: toEvidenceDate(mainlineShell.lastCriticalEvidenceAt),
    needsRefresh: false,
    statusText: `计划目标日：${targetDate}`,
    refreshReason: null,
    todayTaskLabel: null,
    takeoverStatus: "idle",
    takeoverHint: null,
  } satisfies NextDayPlanState
}

export function syncNextDayPlan(mainlineShell: MainlineShellState, now = new Date()) {
  const plan = mainlineShell.nextDayPlan
  if (!plan.ready) return plan
  const today = localDay(now)
  const withTakeover = syncPlanTakeover(plan, mainlineShell, today)
  return syncPlanFreshness(withTakeover, mainlineShell, today)
}

function pickCoreTask(mainlineShell: MainlineShellState) {
  if (mainlineShell.temporaryMainline.currentTemporaryGoal) {
    return `恢复后优先回到：${mainlineShell.currentGoalLabel}`
  }
  if (mainlineShell.probabilityState === "unknown") {
    return "先补 1 份完整关键证据，再决定是否扩任务量"
  }
  if (mainlineShell.timeBudgetInsight.level === "critical") {
    return mainlineShell.timeBudgetEntry.coreTaskLabel
  }
  return mainlineShell.latestAdjustment
}

function pickSupportTask(mainlineShell: MainlineShellState) {
  if (mainlineShell.personalizedInsight.boostTaskType) {
    return `补充项：继续安排 ${formatTaskType(mainlineShell.personalizedInsight.boostTaskType)}`
  }
  if (mainlineShell.timeBudgetInsight.level === "ample") {
    return "补充项：主线完成后再补 1 个次目标"
  }
  return "补充项：无，先保唯一核心任务"
}

function buildRationale(mainlineShell: MainlineShellState) {
  const parts = [
    `概率状态：${mainlineShell.probabilityState === "known" ? `${mainlineShell.probabilityValue}%` : "未知"}`,
    `个性化短期收益：${mainlineShell.personalizedInsight.boostTaskType ? formatTaskType(mainlineShell.personalizedInsight.boostTaskType) : "尚无判断"}`,
    `时间预算：${formatBudgetLevel(mainlineShell.timeBudgetInsight.level)}`,
  ]
  return parts.join("；")
}

function buildRestoreNote(mainlineShell: MainlineShellState) {
  const temp = mainlineShell.temporaryMainline
  if (temp.isActive) return "当前仍处于临时主线中，结束后应恢复原主线。"
  const restored = temp.history.find((item) => item.restoredAt)
  return restored ? `已恢复原主线：${restored.originalGoalLabel}` : null
}

function markRefresh(plan: NextDayPlanState, refreshReason: string, statusText: string) {
  return {
    ...plan,
    needsRefresh: true,
    statusText,
    refreshReason,
  } satisfies NextDayPlanState
}

function finalizePlan(plan: NextDayPlanState, statusText: string) {
  return {
    ...plan,
    needsRefresh: false,
    statusText,
    refreshReason: null,
  } satisfies NextDayPlanState
}

function syncPlanTakeover(plan: NextDayPlanState, mainlineShell: MainlineShellState, today: string) {
  if (plan.targetDate !== today) return updateTakeover(plan, "idle", null, null)
  if (mainlineShell.execution.todayClosed) return updateTakeover(plan, "closed", pickTodayTaskLabel(plan, mainlineShell), "今日已按计划收尾。")
  if (mainlineShell.execution.todayCoreTaskCompleted) return updateTakeover(plan, "completed", pickTodayTaskLabel(plan, mainlineShell), "计划核心任务已完成。")
  if (mainlineShell.execution.status === "executing") return updateTakeover(plan, "taken_over", pickTodayTaskLabel(plan, mainlineShell), "今天主线已承接这份计划。")
  if (mainlineShell.execution.needsReopen) return updateTakeover(plan, "pending_today", plan.coreTaskLabel, `今日重开将优先承接：${plan.coreTaskLabel}`)
  return updateTakeover(plan, "idle", null, null)
}

function syncPlanFreshness(plan: NextDayPlanState, mainlineShell: MainlineShellState, today: string) {
  if (!plan.targetDate) return markRefresh(plan, "计划缺少目标日期，请重新生成。", "计划信息不完整")
  if (plan.targetDate < today) return markRefresh(plan, "该计划目标日已过，请按今天状态重新生成。", "计划已过目标日")
  if (plan.targetDate === today && mainlineShell.execution.needsReopen) return markRefresh(plan, "已进入计划目标日，请结合今天现实时间预算重新生成。", "今天计划待重算")
  if (mainlineShell.evidenceExpired) return markRefresh(plan, "计划依据的关键证据已过期，请先补关键证据后再重算。", "计划依据已过期")
  if (hasNewerEvidence(plan.basedOnEvidenceDate, mainlineShell.lastCriticalEvidenceAt)) return markRefresh(plan, "关键证据已更新，原计划已不是最新版本。", "计划依据已变化")
  return finalizePlan(plan, plan.targetDate === today ? "这是今天的计划" : `计划目标日：${plan.targetDate}`)
}

function updateTakeover(
  plan: NextDayPlanState,
  takeoverStatus: NextDayPlanState["takeoverStatus"],
  todayTaskLabel: string | null,
  takeoverHint: string | null,
) {
  return { ...plan, takeoverStatus, todayTaskLabel, takeoverHint } satisfies NextDayPlanState
}

function pickTodayTaskLabel(plan: NextDayPlanState, mainlineShell: MainlineShellState) {
  return mainlineShell.execution.currentTaskLabel || mainlineShell.timeBudgetEntry.coreTaskLabel || plan.coreTaskLabel
}

function hasNewerEvidence(basedOnEvidenceDate: string | null, lastCriticalEvidenceAt: string | null) {
  const latestDate = toEvidenceDate(lastCriticalEvidenceAt)
  return Boolean(basedOnEvidenceDate && latestDate && latestDate !== basedOnEvidenceDate)
}

function toEvidenceDate(timestamp: string | null) {
  if (!timestamp) return null
  const date = new Date(timestamp)
  return Number.isNaN(date.getTime()) ? null : localDay(date)
}

function addDays(date: Date, days: number) {
  const next = new Date(date)
  next.setDate(next.getDate() + days)
  return next
}

function localDay(date: Date) {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, "0")
  const day = String(date.getDate()).padStart(2, "0")
  return `${year}-${month}-${day}`
}

function formatTaskType(taskType: MainlineShellState["personalizedInsight"]["boostTaskType"]) {
  const labels = {
    vocabulary: "词汇记忆",
    mock_exam: "完整模考",
    reading: "阅读",
    listening: "听力",
    writing_translation: "写作与翻译",
    professional_skill: "专业技能",
    algorithm: "算法复习",
    other: "其他任务",
  }
  return taskType ? labels[taskType] : "暂无"
}

function formatBudgetLevel(level: MainlineShellState["timeBudgetInsight"]["level"]) {
  return {
    ample: "宽松",
    steady: "正常",
    tight: "紧张",
    critical: "极紧",
  }[level]
}
