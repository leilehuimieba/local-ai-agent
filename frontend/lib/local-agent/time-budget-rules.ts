import type {
  TimeBlockDraft,
  TimeBlockItem,
  TimeBudgetEntry,
  TimeBudgetInsight,
  TimeBudgetLevel,
} from "./types"

type TimeBudgetResult = {
  feedback: string
  nextBlocks: TimeBlockItem[]
  insight: TimeBudgetInsight
}

export function createEmptyTimeBlockDraft(): TimeBlockDraft {
  return { startTime: "", endTime: "", label: "" }
}

export function createEmptyTimeBudgetEntry(): TimeBudgetEntry {
  return {
    coreTaskLabel: "今天先保词汇记忆主轴",
    budgetChangeNote: "",
    timeBlockDraft: createEmptyTimeBlockDraft(),
  }
}

export function createEmptyTimeBudgetInsight(): TimeBudgetInsight {
  return {
    updatedAt: null,
    totalAvailableMinutes: 0,
    level: "steady",
    recommendation: "先录入今天的真实可用时间块，再判断是否需要缩量保主线。",
  }
}

export function evaluateTimeBudgetEntry(
  entry: TimeBudgetEntry,
  timeBlocks: TimeBlockItem[],
  now = new Date(),
): TimeBudgetResult {
  const block = parseTimeBlock(entry.timeBlockDraft)
  const nextBlocks = block ? [...timeBlocks, block] : timeBlocks
  return {
    feedback: buildBudgetFeedback(entry, block),
    nextBlocks,
    insight: buildTimeBudgetInsight(nextBlocks, entry.coreTaskLabel, now),
  }
}

function parseTimeBlock(draft: TimeBlockDraft) {
  const start = parseMinutes(draft.startTime)
  const end = parseMinutes(draft.endTime)
  if (start === null || end === null || end <= start) return null
  return {
    startTime: draft.startTime,
    endTime: draft.endTime,
    label: draft.label.trim() || "未命名时间块",
    durationMinutes: end - start,
  } satisfies TimeBlockItem
}

function parseMinutes(value: string) {
  const parts = value.split(":").map(Number)
  if (parts.length !== 2 || parts.some((part) => !Number.isFinite(part))) return null
  return parts[0] * 60 + parts[1]
}

function buildBudgetFeedback(entry: TimeBudgetEntry, block: TimeBlockItem | null) {
  if (!block) return "已更新今日核心任务；时间块未新增，请补全开始和结束时间。"
  return `已接管今日主线：核心任务为“${entry.coreTaskLabel}”，并新增 1 个可用时间块。`
}

function buildTimeBudgetInsight(
  blocks: TimeBlockItem[],
  coreTaskLabel: string,
  now: Date,
) {
  const totalAvailableMinutes = blocks.reduce((sum, item) => sum + item.durationMinutes, 0)
  const level = toBudgetLevel(totalAvailableMinutes)
  return {
    updatedAt: now.toISOString(),
    totalAvailableMinutes,
    level,
    recommendation: buildBudgetRecommendation(level, coreTaskLabel),
  }
}

function toBudgetLevel(totalAvailableMinutes: number): TimeBudgetLevel {
  if (totalAvailableMinutes >= 240) return "ample"
  if (totalAvailableMinutes >= 120) return "steady"
  if (totalAvailableMinutes >= 60) return "tight"
  return "critical"
}

function buildBudgetRecommendation(level: TimeBudgetLevel, coreTaskLabel: string) {
  if (level === "ample") return `今天时间相对宽松，先完成核心任务“${coreTaskLabel}”，再考虑补充次目标。`
  if (level === "steady") return `今天时间基本够用，优先锁定核心任务“${coreTaskLabel}”。`
  if (level === "tight") return "今天时间偏紧，先降任务量，保主目标最核心部分。"
  return "今天时间极紧，只保留 1 个不可协商核心任务，其余全部降级。"
}
