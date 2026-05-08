import type {
  PersonalizedFollowupEntry,
  PersonalizedFollowupInsight,
  PersonalizedFollowupRecord,
  PersonalizedTaskType,
} from "./types"

type FollowupEvaluation = {
  feedback: string
  record: PersonalizedFollowupRecord | null
  insight: PersonalizedFollowupInsight
}

const TASK_LABELS: Record<PersonalizedTaskType, string> = {
  vocabulary: "词汇记忆",
  mock_exam: "完整模考",
  reading: "阅读",
  listening: "听力",
  writing_translation: "写作与翻译",
  professional_skill: "专业技能",
  algorithm: "算法复习",
  other: "其他任务",
}

export function createEmptyPersonalizedEntry(): PersonalizedFollowupEntry {
  return {
    taskType: "vocabulary",
    isCoreTask: true,
    completed: null,
    hasShortTermGain: null,
    resultNote: "",
  }
}

export function createEmptyPersonalizedInsight(): PersonalizedFollowupInsight {
  return {
    updatedAt: null,
    basedOnCount: 0,
    preferredTaskType: null,
    boostTaskType: null,
    riskTaskType: null,
    recommendation: "先通用、后个性化：先连续记录几次任务结果，再判断你更适合哪种推进方式。",
  }
}

export function evaluatePersonalizedEntry(
  entry: PersonalizedFollowupEntry,
  history: PersonalizedFollowupRecord[],
  now = new Date(),
): FollowupEvaluation {
  const record = parseEntry(entry, now)
  if (!record) {
    return {
      feedback: "这次记录已保存为草稿：请至少补全是否完成与是否带来短期收益。",
      record: null,
      insight: buildInsight(history),
    }
  }
  const nextHistory = [...history, record]
  return {
    feedback: buildFeedback(record),
    record,
    insight: buildInsight(nextHistory),
  }
}

function parseEntry(entry: PersonalizedFollowupEntry, now: Date) {
  if (entry.completed === null || entry.hasShortTermGain === null) return null
  return {
    taskType: entry.taskType,
    isCoreTask: entry.isCoreTask,
    completed: entry.completed,
    hasShortTermGain: entry.hasShortTermGain,
    resultNote: entry.resultNote.trim(),
    submittedAt: now.toISOString(),
  } satisfies PersonalizedFollowupRecord
}

function buildFeedback(record: PersonalizedFollowupRecord) {
  const task = TASK_LABELS[record.taskType]
  if (record.completed && record.hasShortTermGain) return `已记录：${task} 这类任务对你当前阶段有正收益。`
  if (record.completed) return `已记录：${task} 能完成，但短期提分收益暂不明显。`
  return `已记录：${task} 当前执行阻力偏高，后续应谨慎加码。`
}

function buildInsight(history: PersonalizedFollowupRecord[]) {
  if (history.length === 0) return createEmptyPersonalizedInsight()
  const preferred = pickTaskType(history, (item) => item.completed)
  const boost = pickTaskType(history, (item) => item.completed && item.hasShortTermGain)
  const risk = pickTaskType(history, (item) => !item.completed)
  return {
    updatedAt: history.at(-1)?.submittedAt || null,
    basedOnCount: history.length,
    preferredTaskType: preferred,
    boostTaskType: boost,
    riskTaskType: risk,
    recommendation: buildRecommendation(preferred, boost, risk),
  }
}

function pickTaskType(
  history: PersonalizedFollowupRecord[],
  matcher: (item: PersonalizedFollowupRecord) => boolean,
) {
  const counts = new Map<PersonalizedTaskType, number>()
  history.filter(matcher).forEach((item) => {
    counts.set(item.taskType, (counts.get(item.taskType) || 0) + 1)
  })
  return [...counts.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] || null
}

function buildRecommendation(
  preferred: PersonalizedTaskType | null,
  boost: PersonalizedTaskType | null,
  risk: PersonalizedTaskType | null,
) {
  const preferredText = preferred ? `你更容易完成的是：${TASK_LABELS[preferred]}。` : ""
  const boostText = boost ? `短期收益最高的是：${TASK_LABELS[boost]}。` : ""
  const riskText = risk ? `当前阻力偏高的是：${TASK_LABELS[risk]}。` : ""
  return [preferredText, boostText, riskText].filter(Boolean).join(" ")
    || "目前证据还不够，继续记录 2~3 次任务结果后再做个性化收敛。"
}

export function getTaskTypeLabel(taskType: PersonalizedTaskType) {
  return TASK_LABELS[taskType]
}
