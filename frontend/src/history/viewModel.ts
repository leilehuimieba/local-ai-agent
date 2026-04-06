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
