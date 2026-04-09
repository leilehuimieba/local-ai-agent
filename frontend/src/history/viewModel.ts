import { LogEntry } from "../shared/contracts";
import { buildResultFromFields } from "../chat/chatResultModel";
import { readLogType, readReviewTypeLabel } from "./logType";

export type ReviewTone = "neutral" | "danger" | "warning" | "calm";

export type FocusLogDetails = {
  category: string;
  classification: string;
  eventType: string;
  completion?: string;
  cache: string;
  finalAnswer?: string;
  knowledgeDigest?: string;
  memoryDigest?: string;
  reason?: string;
  reasoning?: string;
  recordTitle?: string;
  sourceType?: string;
  verification?: string;
  workspace?: string;
};

export type ReviewCardModel = {
  label: string;
  value: string;
  tone: ReviewTone;
};

export type ReviewSpotlightModel = {
  cards: ReviewCardModel[];
  chips: Array<{ label: string; value: string }>;
  title: string;
};

export type ReplaySummary = {
  conclusion: string;
  evidence: string;
  verification: string;
  nextStep: string;
};

export type LearningContinuation = {
  topic: string;
  grasp: string;
  review: string;
  nextStep: string;
};

const LEARNING_INTENT_RE = /(学习|复习|回顾|梳理|理解|掌握|巩固|练习|知识点|概念|章节|笔记)/;
const LEARNING_GRASP_RE = /(已理解|已掌握|理解了|掌握了|学到|看懂|熟悉|已整理|已回顾|能解释|能说清)/;
const LEARNING_REVIEW_RE = /(待巩固|待补|不稳|薄弱|疑问|没弄清|未掌握|还差|卡住|遗漏|易错|补看)/;
const LEARNING_NEXT_RE = /(下一步|继续|先复习|先回顾|先练习|建议先|补一轮|再看|自测)/;
const PROJECT_DOC_RE = /(项目说明|项目文档|需求文档|产品定义|工作区|构建|运行时|provider|settings|gateway|runtime)/;

export function getFocusLogDetails(log: LogEntry): FocusLogDetails {
  return {
    category: readCategoryLabel(log),
    classification: readReviewTypeLabel(readLogType(log)),
    eventType: log.event_type || log.category,
    completion: log.completion_reason || log.metadata?.completion_reason,
    cache: cacheDetail(log.context_snapshot?.cache_status, log.context_snapshot?.cache_reason),
    finalAnswer: log.final_answer || log.metadata?.final_answer,
    knowledgeDigest: log.context_snapshot?.knowledge_digest,
    memoryDigest: log.context_snapshot?.memory_digest,
    reason: log.metadata?.reason,
    reasoning: log.context_snapshot?.reasoning_summary,
    recordTitle: log.record_type || log.metadata?.title,
    sourceType: readSourceType(log),
    verification: readVerification(log),
    workspace: log.context_snapshot?.workspace_root,
  };
}

export function getHistoryNextSteps(log: LogEntry) {
  const items = buildReplayNextSteps(log);
  return Array.from(new Set(items)).slice(0, 3);
}

export function getReviewSpotlight(log: LogEntry): ReviewSpotlightModel {
  const details = getFocusLogDetails(log);
  return {
    cards: buildReviewCards(log, details),
    chips: buildReviewChips(log, details.cache),
    title: log.metadata?.task_title || log.summary,
  };
}

export function buildLogResult(log: LogEntry) {
  const replay = getReplaySummary(log);
  return buildResultFromFields(log.final_answer || log.summary, [
    { kind: "action", text: replay.evidence },
    { kind: "detail", text: readReplayDetail(log, replay) },
    { kind: "risk", text: buildRiskSummary(log) },
    { kind: "next", text: replay.nextStep },
  ]);
}

export function getReplaySummary(log: LogEntry): ReplaySummary {
  return {
    conclusion: getOutcomeValue(log, getFocusLogDetails(log)),
    evidence: getEvidenceValue(log, getFocusLogDetails(log)),
    verification: readVerificationSummary(log, getFocusLogDetails(log)),
    nextStep: getHistoryNextSteps(log)[0] || "补看相关运行记录确认上下文来源。",
  };
}

export function getLearningContinuation(log: LogEntry): LearningContinuation | null {
  if (!hasLearningContinuationSignal(log)) return null;
  const review = readLearningReview(log);
  return {
    topic: compactReviewText(readLearningTopic(log), "当前记录还没有明确学习主题。"),
    grasp: compactReviewText(readLearningGrasp(log), "当前记录还没有明确掌握情况。"),
    review: compactReviewText(review, "当前记录还没有明确待巩固内容。"),
    nextStep: compactReviewText(readLearningNextStep(log, review), "先回顾本轮学习记录，再决定下一步要继续哪一块。"),
  };
}

function buildReviewCards(log: LogEntry, details: FocusLogDetails) {
  const replay = getReplaySummary(log);
  const memory = details.memoryDigest || "当前记录没有记忆摘要。";
  const knowledge = details.knowledgeDigest || "当前记录没有知识摘要。";
  return [
    createReviewCard("当前结论", replay.conclusion, getOutcomeTone(log)),
    createReviewCard("下一步", replay.nextStep, "warning"),
    createReviewCard("执行依据", replay.evidence, "neutral"),
    createReviewCard("验证结果", replay.verification, getVerificationTone(log)),
    createReviewCard("记忆与知识", `${memory} / ${knowledge}`, "neutral"),
    createReviewCard("缓存状态", details.cache || "当前记录没有缓存命中信息。", "calm"),
  ];
}

function buildReviewChips(log: LogEntry, cache: string) {
  return [
    { label: "类型", value: readReviewTypeLabel(readLogType(log)) },
    { label: "分类", value: readCategoryLabel(log) },
    { label: "结论", value: readCompletionLabel(log) },
    { label: "验证", value: log.verification_snapshot?.passed ? "已通过" : "待确认" },
    { label: "缓存", value: cache || "无" },
  ];
}

function createReviewCard(label: string, value: string, tone: ReviewTone) {
  return { label, tone, value };
}

function cacheDetail(status?: string, reason?: string) {
  if (!status) return "";
  return reason ? `${status} | ${reason}` : status;
}

function getOutcomeValue(log: LogEntry, details: FocusLogDetails) {
  if (readLogType(log) === "confirmation") {
    return log.summary || "当前记录表明任务在这里进入确认。";
  }
  return log.error?.message || details.finalAnswer || log.detail || log.summary;
}

function getEvidenceValue(log: LogEntry, details: FocusLogDetails) {
  if (readLogType(log) === "confirmation") {
    return log.metadata?.reason || log.detail || "当前记录未附带额外确认依据。";
  }
  return log.metadata?.task_title || details.reasoning || log.result_summary || "当前记录未附带额外执行依据。";
}

function getOutcomeTone(log: LogEntry): ReviewTone {
  if (log.error || log.completion_status === "failed") return "danger";
  return log.risk_level ? "warning" : "calm";
}

function getVerificationTone(log: LogEntry): ReviewTone {
  if (log.verification_snapshot?.passed) return "calm";
  return log.error || log.completion_status === "failed" ? "danger" : "warning";
}

function buildRiskSummary(log: LogEntry) {
  if (log.error) return `${log.error.error_code} / ${log.error.message}`;
  if (log.risk_level) return `风险等级：${log.risk_level}`;
  return "";
}

function buildReplayNextSteps(log: LogEntry) {
  if (readLogType(log) === "result") return buildResultNextSteps(log);
  if (readLogType(log) === "error") return buildFailureNextSteps(log);
  if (readLogType(log) === "confirmation") return buildConfirmationNextSteps(log);
  return buildGenericNextSteps(log);
}

function buildResultNextSteps(log: LogEntry) {
  return [
    log.metadata?.next_step,
    log.metadata?.verification_next_step,
    "先核对最终结论与验证结果，再决定是否继续追问。",
    "对照本轮工作区和输出结果，确认这次复盘是否已经收口。",
  ].filter(Boolean) as string[];
}

function buildFailureNextSteps(log: LogEntry) {
  return [
    log.metadata?.failure_recovery_hint,
    log.metadata?.next_step,
    log.error?.retryable ? "当前问题可重试，先核对输入、路径和工具边界后再决定是否重跑。" : "",
    "回看失败前最后一条有效动作，确认阻塞点到底出在输入、工具还是确认链路。",
  ].filter(Boolean) as string[];
}

function buildConfirmationNextSteps(log: LogEntry) {
  return [
    log.metadata?.next_step,
    "先回看确认原因、目标路径和风险等级，再判断是否要继续推进。",
    log.risk_level ? `重点核对 ${log.risk_level} 风险等级与最终确认动作是否一致。` : "",
  ].filter(Boolean) as string[];
}

function buildGenericNextSteps(log: LogEntry) {
  return [
    log.metadata?.next_step,
    log.metadata?.verification_next_step,
    log.context_snapshot?.workspace_root ? "核对上下文工作区与本次任务是否一致。" : "补看相关运行记录确认上下文来源。",
  ].filter(Boolean) as string[];
}

function readVerificationSummary(log: LogEntry, details: FocusLogDetails) {
  if (readLogType(log) === "confirmation") return "当前记录是确认节点，需要结合后续动作判断最终结果。";
  if (log.error || log.completion_status === "failed") {
    return details.verification || "当前记录没有通过验证，需要优先复核失败原因。";
  }
  return details.verification || "当前记录未附带验证摘要。";
}

function readReplayDetail(log: LogEntry, replay: ReplaySummary) {
  if (readLogType(log) === "error") return replay.verification;
  if (readLogType(log) === "confirmation") return `${replay.verification} ${replay.nextStep}`;
  return log.detail || replay.verification;
}

function readCategoryLabel(log: LogEntry) {
  if (log.category === "risk") return "风险";
  if (log.category === "memory") return "记忆";
  if (log.category === "knowledge") return "知识";
  if (log.category === "tool") return log.tool_category || "工具";
  if (log.category) return log.category;
  return "未标注";
}

function readCompletionLabel(log: LogEntry) {
  if (log.completion_status) return log.completion_status;
  return log.level || "未标注";
}

function readSourceType(log: LogEntry) {
  return log.source_type || log.metadata?.source_type;
}

function readVerification(log: LogEntry) {
  return log.verification_summary || log.verification_snapshot?.summary || log.metadata?.verification_summary;
}

function hasLearningContinuationSignal(log: LogEntry) {
  return hasLearningIntent(log) && hasLearningEvidence(log) && !isProjectAnswerFalsePositive(log);
}

function readLearningTopic(log: LogEntry) {
  return findLearningFragment(
    [
      log.metadata?.task_title,
      log.summary,
      log.detail,
      log.final_answer,
      log.result_summary,
      log.context_snapshot?.session_summary,
      log.context_snapshot?.knowledge_digest,
    ],
    [LEARNING_INTENT_RE],
  );
}

function readLearningGrasp(log: LogEntry) {
  return findLearningFragment(
    [
      log.verification_summary,
      log.result_summary,
      log.summary,
      log.detail,
      log.final_answer,
      log.context_snapshot?.session_summary,
    ],
    [LEARNING_GRASP_RE, LEARNING_INTENT_RE],
  );
}

function readLearningReview(log: LogEntry) {
  const explicit = findLearningFragment(
    [log.context_snapshot?.knowledge_digest, log.context_snapshot?.memory_digest, log.detail, log.verification_summary, log.context_snapshot?.session_summary],
    [LEARNING_REVIEW_RE],
  );
  if (explicit) return explicit;
  return findLearningFragment(
    [log.context_snapshot?.knowledge_digest, log.context_snapshot?.memory_digest, log.detail],
    [/概念|知识点|笔记|例题|重点|难点/],
  );
}

function readLearningNextStep(log: LogEntry, review: string) {
  const explicit = findLearningFragment(
    [
      log.metadata?.next_step,
      log.metadata?.verification_next_step,
      log.detail,
      log.final_answer,
      ...getHistoryNextSteps(log),
    ],
    [LEARNING_NEXT_RE, LEARNING_INTENT_RE, LEARNING_REVIEW_RE],
  );
  if (explicit) return explicit;
  if (!review) return "";
  return `先回到“${shortLearningText(review)}”再补一轮回顾或练习。`;
}

function compactReviewText(value: string | undefined, fallback: string) {
  const normalized = cleanLearningFragment(value || "");
  if (!normalized) return fallback;
  return normalized.length > 140 ? `${normalized.slice(0, 137)}...` : normalized;
}

function hasLearningIntent(log: LogEntry) {
  return LEARNING_INTENT_RE.test([
    log.metadata?.task_title,
    log.summary,
    log.detail,
    log.context_snapshot?.session_summary,
  ].filter(Boolean).join(" "));
}

function hasLearningEvidence(log: LogEntry) {
  return Boolean(findLearningFragment(
    [
      log.verification_summary,
      log.result_summary,
      log.detail,
      log.final_answer,
      log.context_snapshot?.knowledge_digest,
      log.context_snapshot?.memory_digest,
      log.metadata?.next_step,
    ],
    [LEARNING_GRASP_RE, LEARNING_REVIEW_RE, LEARNING_NEXT_RE],
  ));
}

function isProjectAnswerFalsePositive(log: LogEntry) {
  const toolName = log.tool_call_snapshot?.tool_name || log.tool_name || "";
  const text = [log.metadata?.task_title, log.summary, log.result_summary, log.verification_summary].filter(Boolean).join(" ");
  return toolName === "project_answer" && PROJECT_DOC_RE.test(text);
}

function findLearningFragment(sources: Array<string | undefined>, matchers: RegExp[]) {
  for (const source of sources) {
    for (const fragment of splitLearningFragments(source)) {
      if (!matchers.some((matcher) => matcher.test(fragment))) continue;
      if (!isUsableLearningFragment(fragment)) continue;
      return fragment;
    }
  }
  return "";
}

function splitLearningFragments(source?: string) {
  return (source || "")
    .split(/[\n。；]/)
    .map((fragment) => cleanLearningFragment(fragment))
    .filter(Boolean);
}

function cleanLearningFragment(fragment: string) {
  return fragment
    .replace(/\s+/g, " ")
    .replace(/^(验证通过[:：]|已验证[:：]|结果[:：]|摘要[:：]|最近摘要[:：]|当前结论[:：]|上一步结果摘要[:：]|最近压缩摘要[:：])/, "")
    .replace(/^(当前目标[:：]|当前计划[:：]|最近观察[:：]|文件[:：]|片段[:：])/, "")
    .trim();
}

function isUsableLearningFragment(fragment: string) {
  if (fragment.length < 6) return false;
  if (/^[A-Z]:\\/.test(fragment)) return false;
  if (fragment.includes("Docs 导航")) return false;
  return !PROJECT_DOC_RE.test(fragment) || LEARNING_INTENT_RE.test(fragment);
}

function shortLearningText(value: string) {
  return value.length > 18 ? `${value.slice(0, 18)}...` : value;
}
