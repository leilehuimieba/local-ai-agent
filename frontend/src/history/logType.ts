import { LogEntry, MemoryEntry, RunEvent } from "../shared/contracts";

export type ReviewLogType =
  | "result"
  | "tool"
  | "memory"
  | "knowledge"
  | "verification"
  | "confirmation"
  | "error"
  | "system";

type ReviewRecord = {
  category?: string;
  completion_status?: string;
  confirmation_id?: string;
  context_snapshot?: LogEntry["context_snapshot"];
  error?: LogEntry["error"];
  event_type?: string;
  final_answer?: string;
  level?: string;
  metadata?: Record<string, string>;
  output_kind?: string;
  record_type?: string;
  source_type?: string;
  tool_call_snapshot?: LogEntry["tool_call_snapshot"];
  tool_category?: string;
  tool_name?: string;
  verification_snapshot?: LogEntry["verification_snapshot"];
  verification_summary?: string;
};

type MemoryLike = {
  archived?: boolean;
  event_type?: string;
  governance_status?: string;
  kind?: string;
  memory_action?: string;
  metadata?: Record<string, string>;
  priority?: number;
  reason?: string;
  source_event_type?: string;
  source_type?: string;
  summary?: string;
  title?: string;
  verified?: boolean;
};

export function readLogType(log: LogEntry) {
  return classifyReviewRecord(log);
}

export function readRunEventType(event: RunEvent) {
  return classifyReviewRecord(event);
}

export function readReviewTypeLabel(type: ReviewLogType) {
  if (type === "result") return "结果";
  if (type === "tool") return "工具";
  if (type === "memory") return "记忆";
  if (type === "knowledge") return "知识";
  if (type === "verification") return "验证";
  if (type === "confirmation") return "确认";
  if (type === "error") return "错误";
  return "系统";
}

export function readMemoryFacetLabel(memory?: MemoryLike | null) {
  if (!memory) return "无记忆";
  if (isLessonMemory(memory)) return "失败教训";
  if (isPreferenceMemory(memory)) return "用户偏好";
  if (isKnowledgeMemory(memory)) return "知识沉淀";
  return "运行记忆";
}

export function readMemoryGovernanceLabel(memory?: MemoryLike | null) {
  if (!memory) return "无记忆";
  const explicit = readExplicitGovernanceLabel(memory);
  if (explicit) return explicit;
  if (memory.archived) return "已归档";
  if (readMemoryEventType(memory) === "memory_write_skipped") return "已跳过";
  if (memory.verified) return "已验证";
  if (isGovernanceCandidate(memory)) return "待治理";
  return "待补充";
}

export function readMemoryGovernanceClass(memory?: MemoryLike | null) {
  const label = readMemoryGovernanceLabel(memory);
  if (["已归档", "已验证", "已写入", "已召回", "生效中"].includes(label)) return "status-completed";
  if (label === "待治理" || label === "已跳过") return "status-awaiting";
  return "status-idle";
}

export function readMemoryActivityLabel(memory?: MemoryLike | null) {
  const action = readMemoryAction(memory);
  const eventType = readMemoryEventType(memory);
  if (action === "recall" || eventType === "memory_recalled") return "最近召回";
  if (action === "skip" || eventType === "memory_write_skipped") return "最近跳过";
  if (action === "write" || eventType === "memory_written") return "最近写入";
  if (action === "archive") return "最近归档";
  if (isLessonMemory(memory)) return "失败教训沉淀";
  if (isPreferenceMemory(memory)) return "偏好沉淀";
  return memory ? "记忆记录" : "暂无动作";
}

export function countMemoryFacets(memories: MemoryEntry[]) {
  return {
    archived: memories.filter((memory) => memory.archived).length,
    lessons: memories.filter((memory) => isLessonMemory(memory)).length,
    pending: memories.filter((memory) => needsGovernanceAttention(memory)).length,
    preferences: memories.filter((memory) => isPreferenceMemory(memory)).length,
  };
}

function classifyReviewRecord(record: ReviewRecord): ReviewLogType {
  if (hasErrorSignal(record)) return "error";
  if (hasResultSignal(record)) return "result";
  if (hasConfirmationSignal(record)) return "confirmation";
  if (hasMemorySignal(record)) return "memory";
  if (hasKnowledgeSignal(record)) return "knowledge";
  if (hasVerificationSignal(record)) return "verification";
  if (hasToolSignal(record)) return "tool";
  return "system";
}

function hasErrorSignal(record: ReviewRecord) {
  return record.level === "error"
    || Boolean(record.error)
    || record.event_type === "run_failed"
    || record.completion_status === "failed"
    || Boolean(record.metadata?.error_code);
}

function hasConfirmationSignal(record: ReviewRecord) {
  return Boolean(
    record.confirmation_id
      || record.event_type === "confirmation_required"
      || record.category === "risk"
      || record.completion_status === "confirmation_required",
  );
}

function hasMemorySignal(record: ReviewRecord) {
  const recordType = readRecordType(record);
  return Boolean(
    record.category === "memory"
      || record.metadata?.memory_kind
      || record.context_snapshot?.memory_digest
      || record.event_type === "memory_recalled"
      || record.event_type === "memory_written"
      || record.event_type === "memory_write_skipped"
      || recordType === "session_state"
      || recordType === "lesson_learned",
  );
}

function hasKnowledgeSignal(record: ReviewRecord) {
  const recordType = readRecordType(record);
  const sourceType = readSourceType(record);
  return Boolean(
    record.category === "knowledge"
      || record.output_kind === "knowledge_digest"
      || recordType === "document_digest"
      || sourceType === "knowledge",
  );
}

function hasVerificationSignal(record: ReviewRecord) {
  return Boolean(
    record.verification_snapshot?.summary
      || readVerificationSummary(record)
      || record.event_type === "verification_completed",
  );
}

function hasToolSignal(record: ReviewRecord) {
  return Boolean(
    record.category === "tool"
      || record.tool_category
      || record.tool_call_snapshot?.tool_name
      || record.tool_name
      || record.event_type === "action_requested"
      || record.event_type === "action_completed",
  );
}

function hasResultSignal(record: ReviewRecord) {
  return Boolean(
    record.final_answer
      || record.output_kind === "final_answer"
      || record.event_type === "run_finished"
      || record.completion_status === "completed",
  );
}

function readRecordType(record: ReviewRecord) {
  return record.record_type || record.metadata?.record_type || "";
}

function readSourceType(record: ReviewRecord) {
  return record.source_type || record.metadata?.source_type || "";
}

function readVerificationSummary(record: ReviewRecord) {
  return record.verification_summary || record.metadata?.verification_summary || "";
}

function isKnowledgeMemory(memory?: MemoryLike | null) {
  return hasKeyword(memory?.kind, "knowledge") || hasKeyword(memory?.source_type, "knowledge");
}

function isPreferenceMemory(memory?: MemoryLike | null) {
  return hasAnyKeyword(readMemoryKeywords(memory), ["prefer", "preference", "偏好", "习惯", "风格", "约定"]);
}

function isLessonMemory(memory?: MemoryLike | null) {
  return hasAnyKeyword(readMemoryKeywords(memory), ["lesson", "教训", "失败", "踩坑", "复盘", "pitfall", "avoid"]);
}

function isGovernanceCandidate(memory?: MemoryLike | null) {
  return Boolean(memory && !memory.archived && !memory.verified && (isPreferenceMemory(memory) || isLessonMemory(memory) || (memory.priority || 0) >= 80));
}

function needsGovernanceAttention(memory?: MemoryLike | null) {
  return Boolean(memory && !readExplicitGovernanceLabel(memory) && isGovernanceCandidate(memory));
}

function readMemoryEventType(memory?: MemoryLike | null) {
  return memory?.event_type || memory?.source_event_type || "";
}

function readMemoryAction(memory?: MemoryLike | null) {
  return memory?.memory_action || memory?.metadata?.memory_action || "";
}

function readExplicitGovernanceLabel(memory?: MemoryLike | null) {
  const status = memory?.governance_status || memory?.metadata?.governance_status || "";
  if (status === "archived") return "已归档";
  if (status === "verified") return "已验证";
  if (status === "written") return "已写入";
  if (status === "skipped") return "已跳过";
  if (status === "recalled") return "已召回";
  if (status === "active") return "生效中";
  return "";
}

function readMemoryKeywords(memory?: MemoryLike | null) {
  return [
    memory?.kind,
    memory?.title,
    memory?.summary,
    memory?.reason,
    memory?.source_type,
    memory?.source_event_type,
    memory?.metadata?.memory_kind,
  ];
}

function hasAnyKeyword(values: Array<string | undefined>, keywords: string[]) {
  return values.some((value) => keywords.some((keyword) => hasKeyword(value, keyword)));
}

function hasKeyword(value: string | undefined, keyword: string) {
  return (value || "").toLowerCase().includes(keyword.toLowerCase());
}
