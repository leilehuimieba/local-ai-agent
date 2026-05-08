import type {
  EveningReviewState,
  MainlineShellState,
  LateEvidenceDecision,
} from "./types"

const EVENING_HOUR = 19
const NOON_HOUR = 12

export function createEmptyEveningReviewState(): EveningReviewState {
  return {
    scheduledLabel: "晚上固定查看",
    status: "pending",
    statusText: "今晚待查看",
    guidance: "晚上固定时间回插件，先交今天的证据包。",
    deadlineLabel: null,
    branchDecision: "none",
    branchText: "暂无补交流转",
    lastReviewedAt: null,
    lastSubmittedForDate: null,
  }
}

export function refreshEveningReview(mainlineShell: MainlineShellState, now = new Date()) {
  if (isSubmittedToday(mainlineShell.eveningReview.lastSubmittedForDate, now)) {
    return buildSubmittedState(mainlineShell.eveningReview, now)
  }
  if (!hasReviewHistory(mainlineShell.eveningReview)) return buildPendingState(mainlineShell.eveningReview, now)
  if (canMakeUpYesterday(mainlineShell.eveningReview.lastSubmittedForDate, now)) {
    return buildLateAllowedState(mainlineShell)
  }
  if (shouldReroute(mainlineShell.eveningReview.lastSubmittedForDate, now)) return buildRerouteState(mainlineShell)
  return buildPendingState(mainlineShell.eveningReview, now)
}

export function submitEveningReview(
  mainlineShell: MainlineShellState,
  now = new Date(),
): EveningReviewState {
  const targetDate = resolveSubmissionDate(mainlineShell.eveningReview.lastSubmittedForDate, now)
  return {
    ...mainlineShell.eveningReview,
    status: "submitted",
    statusText: targetDate === localDay(now) ? "今晚证据已提交" : "昨晚证据已补交",
    guidance: targetDate === localDay(now) ? "下一步先生成明日计划。" : "已补昨天证据，今晚仍按固定时间回插件。",
    deadlineLabel: null,
    branchDecision: "none",
    branchText: targetDate === localDay(now) ? "今日晚间闭环已完成" : "昨日证据已补齐",
    lastReviewedAt: now.toISOString(),
    lastSubmittedForDate: targetDate,
  }
}

function buildSubmittedState(state: EveningReviewState, now: Date): EveningReviewState {
  return {
    ...state,
    status: "submitted",
    statusText: "今晚证据已提交",
    guidance: "今晚闭环已完成，下一步先看明日计划。",
    deadlineLabel: null,
    branchDecision: "none",
    branchText: `已完成 ${localDay(now)} 的晚间对账`,
  }
}

function buildPendingState(state: EveningReviewState, now: Date): EveningReviewState {
  const inWindow = now.getHours() >= EVENING_HOUR
  return {
    ...state,
    status: "pending",
    statusText: inWindow ? "现在进入固定查看窗口" : "今晚待查看",
    guidance: inWindow ? "先交今天的证据包，再决定下一步调整。" : "今晚固定时间回插件，先交今天的证据包。",
    deadlineLabel: null,
    branchDecision: "none",
    branchText: "暂无补交流转",
  }
}

function buildLateAllowedState(mainlineShell: MainlineShellState): EveningReviewState {
  return {
    ...mainlineShell.eveningReview,
    status: "late_allowed",
    statusText: "昨晚证据可补交",
    guidance: "先允许补交，次日中午前先补昨天的证据包。",
    deadlineLabel: "补交截止：次日中午前",
    branchDecision: "make_up_yesterday",
    branchText: "当前裁决：先补昨天",
  }
}

function buildRerouteState(mainlineShell: MainlineShellState): EveningReviewState {
  const decision = decideLateEvidence(mainlineShell)
  return {
    ...mainlineShell.eveningReview,
    status: "reroute",
    statusText: "补交窗口已过",
    guidance: decision === "make_up_yesterday" ? "先补昨天会改变判断的关键证据，再决定今天是否扩任务量。" : "先保今天唯一核心任务，昨晚证据改为可追记。",
    deadlineLabel: null,
    branchDecision: decision,
    branchText: decision === "make_up_yesterday" ? "当前裁决：先补昨天" : "当前裁决：继续今天",
  }
}

function decideLateEvidence(mainlineShell: MainlineShellState): LateEvidenceDecision {
  if (mainlineShell.probabilityState === "unknown") return "make_up_yesterday"
  if (mainlineShell.riskLevel === "orange" || mainlineShell.riskLevel === "red") return "make_up_yesterday"
  if (mainlineShell.timeBudgetInsight.level === "critical") return "continue_today"
  return "continue_today"
}

function hasReviewHistory(state: EveningReviewState) {
  return Boolean(state.lastReviewedAt || state.lastSubmittedForDate)
}

function resolveSubmissionDate(lastSubmittedForDate: string | null, now: Date) {
  return canMakeUpYesterday(lastSubmittedForDate, now) ? localDay(addDays(now, -1)) : localDay(now)
}

function canMakeUpYesterday(lastSubmittedForDate: string | null, now: Date) {
  return now.getHours() < NOON_HOUR && lastSubmittedForDate !== localDay(addDays(now, -1))
}

function shouldReroute(lastSubmittedForDate: string | null, now: Date) {
  return now.getHours() >= NOON_HOUR && lastSubmittedForDate !== localDay(addDays(now, -1))
}

function isSubmittedToday(lastSubmittedForDate: string | null, now: Date) {
  return lastSubmittedForDate === localDay(now)
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
