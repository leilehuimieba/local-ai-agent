import type {
  KeyEvidenceEntry,
  KeyEvidenceRecord,
  KeyEvidenceSourceType,
  MainlineRiskLevel,
} from "./types"

const EXPIRE_MS = 48 * 60 * 60 * 1000

type ProbabilityEvaluation = {
  status: "strong" | "weak"
  feedback: string
  probabilityValue: number | null
  riskLevel?: MainlineRiskLevel
  adjustment: string
  record?: KeyEvidenceRecord
}

export function createEmptyKeyEvidenceEntry(): KeyEvidenceEntry {
  return {
    sourceType: "mock_exam",
    completedDate: "",
    durationMinutes: "",
    underExamCondition: false,
    totalScore: "",
    listeningScore: "",
    readingScore: "",
    writingTranslationScore: "",
    notes: "",
  }
}

export function isCriticalEvidenceExpired(timestamp: string | null, now = Date.now()) {
  if (!timestamp) return true
  const submittedAt = new Date(timestamp).getTime()
  return !Number.isFinite(submittedAt) || now - submittedAt > EXPIRE_MS
}

export function evaluateKeyEvidenceEntry(
  entry: KeyEvidenceEntry,
  history: KeyEvidenceRecord[],
  currentProbability: number | null,
  now = new Date(),
): ProbabilityEvaluation {
  const parsed = parseStrongRecord(entry, now)
  if (!parsed) return buildWeakEvaluation(entry)
  return buildStrongEvaluation(parsed, history, currentProbability)
}

function parseStrongRecord(entry: KeyEvidenceEntry, now: Date) {
  const numbers = parseScores(entry)
  if (!numbers || entry.underExamCondition !== true || !entry.completedDate) return null
  return {
    ...numbers,
    sourceType: entry.sourceType,
    completedDate: entry.completedDate,
    underExamCondition: true,
    notes: entry.notes.trim(),
    submittedAt: now.toISOString(),
  } satisfies Omit<KeyEvidenceRecord, "probabilityAfter">
}

function parseScores(entry: KeyEvidenceEntry) {
  const values = [
    Number(entry.durationMinutes),
    Number(entry.totalScore),
    Number(entry.listeningScore),
    Number(entry.readingScore),
    Number(entry.writingTranslationScore),
  ]
  return values.every((value) => Number.isFinite(value) && value > 0)
    ? {
        durationMinutes: values[0],
        totalScore: values[1],
        listeningScore: values[2],
        readingScore: values[3],
        writingTranslationScore: values[4],
      }
    : null
}

function buildWeakEvaluation(entry: KeyEvidenceEntry): ProbabilityEvaluation {
  return {
    status: "weak",
    probabilityValue: null,
    adjustment: "先补完整考试条件、总分和分项得分，再决定是否调整押题模块。",
    feedback: buildWeakFeedback(entry),
  }
}

function buildWeakFeedback(entry: KeyEvidenceEntry) {
  const hasScores = Boolean(parseScores(entry) && entry.completedDate)
  if (!hasScores) return "已记录但暂不改概率：请补齐做题日期、用时、总分和分项得分。"
  return entry.underExamCondition === false
    ? "已记录为弱参考：未按完整考试条件完成，本次不直接改动安全通过概率。"
    : "已记录但暂不改概率：请补齐做题日期、用时、总分和分项得分。"
}

function buildStrongEvaluation(
  record: Omit<KeyEvidenceRecord, "probabilityAfter">,
  history: KeyEvidenceRecord[],
  currentProbability: number | null,
): ProbabilityEvaluation {
  const probabilityValue = computeProbability(record, history, currentProbability)
  return {
    status: "strong",
    probabilityValue,
    riskLevel: probabilityToRisk(probabilityValue),
    adjustment: buildAdjustment(record),
    feedback: `已按关键证据更新安全通过概率：${probabilityValue}%`,
    record: { ...record, probabilityAfter: probabilityValue },
  }
}

function computeProbability(
  record: Omit<KeyEvidenceRecord, "probabilityAfter">,
  history: KeyEvidenceRecord[],
  currentProbability: number | null,
) {
  const base = scoreToProbability(record.totalScore)
  const baseline = typeof currentProbability === "number" ? currentProbability : base
  const blended = Math.round((baseline + base) / 2)
  return applyTrend(blended, record.totalScore, lastStrongRecord(history)?.totalScore)
}

function scoreToProbability(totalScore: number) {
  if (totalScore >= 520) return 92
  if (totalScore >= 480) return 82
  if (totalScore >= 450) return 70
  if (totalScore >= 425) return 58
  if (totalScore >= 390) return 42
  return 28
}

function applyTrend(probability: number, totalScore: number, previousScore?: number) {
  if (typeof previousScore !== "number") return probability
  const delta = totalScore - previousScore
  if (delta >= 20) return Math.min(95, probability + 6)
  if (delta <= -20) return Math.max(15, probability - 8)
  return probability
}

function probabilityToRisk(probability: number): MainlineRiskLevel {
  if (probability >= 80) return "green"
  if (probability >= 60) return "yellow"
  if (probability >= 45) return "orange"
  return "red"
}

function buildAdjustment(record: Omit<KeyEvidenceRecord, "probabilityAfter">) {
  const weakest = findWeakestModule(record)
  if (record.totalScore < 425) return `先保短期提分效率最高项：${weakest.label}，下次继续围绕该模块做完整题复盘。`
  if (record.totalScore < 470) return `继续围绕 ${weakest.label} 做模块级补强，同时保留词汇记忆主轴。`
  return `保持当前主线节奏，优先稳住 ${weakest.label}，再用完整题验证是否进入更安全区间。`
}

function findWeakestModule(record: Omit<KeyEvidenceRecord, "probabilityAfter">) {
  const modules = [
    { label: "听力", score: record.listeningScore },
    { label: "阅读", score: record.readingScore },
    { label: "写作与翻译", score: record.writingTranslationScore },
  ]
  return modules.sort((a, b) => a.score - b.score)[0]
}

function lastStrongRecord(history: KeyEvidenceRecord[]) {
  return [...history].reverse().find((item) => item.probabilityAfter !== null)
}

export function buildWeakEvidenceSummary(entry: KeyEvidenceEntry, sourceType: KeyEvidenceSourceType) {
  const label = sourceType === "real_exam" ? "真题" : "模考"
  return `${label}结果已记录，等待补齐关键证据。`
}
