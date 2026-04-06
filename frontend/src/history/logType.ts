import { LogEntry, RunEvent } from "../shared/contracts";

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
  record_type?: string;
  source_type?: string;
  tool_category?: string;
  tool_call_snapshot?: LogEntry["tool_call_snapshot"];
  tool_name?: string;
  verification_summary?: string;
  verification_snapshot?: LogEntry["verification_snapshot"];
  output_kind?: string;
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
  return record.level === "error" || Boolean(record.error) || record.event_type === "run_failed" || record.completion_status === "failed" || Boolean(record.metadata?.error_code);
}

function hasConfirmationSignal(record: ReviewRecord) {
  return Boolean(record.confirmation_id || record.event_type === "confirmation_required" || record.category === "risk" || record.completion_status === "confirmation_required");
}

function hasMemorySignal(record: ReviewRecord) {
  const recordType = readRecordType(record);
  return Boolean(
    record.category === "memory" ||
    record.metadata?.memory_kind ||
    record.context_snapshot?.memory_digest ||
    record.event_type === "memory_recalled" ||
    record.event_type === "memory_written" ||
    record.event_type === "memory_write_skipped" ||
    recordType === "session_state" ||
    recordType === "lesson_learned",
  );
}

function hasKnowledgeSignal(record: ReviewRecord) {
  const recordType = readRecordType(record);
  const sourceType = readSourceType(record);
  return Boolean(
    record.category === "knowledge" ||
    record.output_kind === "knowledge_digest" ||
    recordType === "document_digest" ||
    sourceType === "knowledge",
  );
}

function hasVerificationSignal(record: ReviewRecord) {
  return Boolean(
    record.verification_snapshot?.summary ||
    readVerificationSummary(record) ||
    record.event_type === "verification_completed",
  );
}

function hasToolSignal(record: ReviewRecord) {
  return Boolean(
    record.category === "tool" ||
    record.tool_category ||
    record.tool_call_snapshot?.tool_name ||
    record.tool_name ||
    record.event_type === "action_requested" ||
    record.event_type === "action_completed",
  );
}

function hasResultSignal(record: ReviewRecord) {
  return Boolean(
    record.final_answer ||
    record.output_kind === "final_answer" ||
    record.event_type === "run_finished" ||
    record.completion_status === "completed",
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
