import { useMemo, useState } from "react";

import { LogEntry } from "../shared/contracts";
import { AuditFilter, hasAuditSignal } from "./auditSignals";
import { readLogType, readReviewTypeLabel } from "./logType";

export type HistoryStats = {
  confirmationCount: number;
  errorCount: number;
  sessionCount: number;
  toolCount: number;
};
export type ReviewFocusFilter = "all" | "result" | "risk" | "verification" | "governance";
export type HistoryDetailFocusSection = "summary" | "risk" | "verification" | "metadata" | null;

export function useHistoryReview(logs: LogEntry[]) {
  const filters = useReviewFilters();
  const filteredLogs = useMemo(
    () => filterLogs(logs, filters.values),
    [filters.values, logs],
  );
  const recentLogs = useMemo(() => sortLogsByTimestamp(filteredLogs).slice(0, 48), [filteredLogs]);
  const stats = useMemo(() => getLogStats(logs), [logs]);
  const focusLog = useFocusLog(recentLogs, filters.selectedLogId);
  return {
    focusLog,
    recentLogs,
    selectLog: filters.setSelectedLogId,
    stats,
    detailFocusSection: readDetailFocusSection(filters.values.focusFilter),
    toolbarProps: buildToolbarProps(filteredLogs.length, filters),
  };
}

function useReviewFilters() {
  const [auditFilter, setAuditFilter] = useState<AuditFilter>("all");
  const [focusFilter, setFocusFilter] = useState<ReviewFocusFilter>("all");
  const [query, setQuery] = useState("");
  const [levelFilter, setLevelFilter] = useState("all");
  const [typeFilter, setTypeFilter] = useState("all");
  const [onlyErrors, setOnlyErrors] = useState(false);
  const [onlyConfirmations, setOnlyConfirmations] = useState(false);
  const [selectedLogId, setSelectedLogId] = useState("");
  return {
    selectedLogId,
    setSelectedLogId,
    values: { auditFilter, focusFilter, levelFilter, onlyConfirmations, onlyErrors, query, typeFilter },
    setters: { setAuditFilter, setFocusFilter, setLevelFilter, setOnlyConfirmations, setOnlyErrors, setQuery, setTypeFilter },
  };
}

function useFocusLog(logs: LogEntry[], selectedLogId: string) {
  return useMemo(() => selectFocusLog(logs, selectedLogId), [logs, selectedLogId]);
}

function buildToolbarProps(resultCount: number, filters: ReturnType<typeof useReviewFilters>) {
  return {
    auditFilter: filters.values.auditFilter,
    focusFilter: filters.values.focusFilter,
    onAuditFilterChange: filters.setters.setAuditFilter,
    onFocusFilterChange: filters.setters.setFocusFilter,
    levelFilter: filters.values.levelFilter,
    onLevelFilterChange: filters.setters.setLevelFilter,
    onOnlyConfirmationsChange: filters.setters.setOnlyConfirmations,
    onOnlyErrorsChange: filters.setters.setOnlyErrors,
    onQueryChange: filters.setters.setQuery,
    onTypeFilterChange: filters.setters.setTypeFilter,
    onlyConfirmations: filters.values.onlyConfirmations,
    onlyErrors: filters.values.onlyErrors,
    query: filters.values.query,
    resultCount,
    typeFilter: filters.values.typeFilter,
  };
}

function getLogStats(logs: LogEntry[]): HistoryStats {
  return {
    confirmationCount: logs.filter((log) => readLogType(log) === "confirmation").length,
    errorCount: logs.filter((log) => readLogType(log) === "error").length,
    sessionCount: new Set(logs.map((log) => log.session_id).filter(Boolean)).size,
    toolCount: new Set(logs.map(readToolName).filter(Boolean)).size,
  };
}

function sortLogsByTimestamp(logs: LogEntry[]) {
  return [...logs].sort((left, right) => readTimestamp(right.timestamp) - readTimestamp(left.timestamp));
}

function selectFocusLog(logs: LogEntry[], selectedLogId: string) {
  if (selectedLogId) {
    const selected = logs.find((log) => log.log_id === selectedLogId);
    if (selected) return selected;
  }
  return selectDefaultFocusLog(logs);
}

function selectDefaultFocusLog(logs: LogEntry[]) {
  if (logs.length === 0) return null;
  return logs.find(isCompletedResultLog) || logs.find(isReplayPriorityLog) || logs[0];
}

function filterLogs(
  logs: LogEntry[],
  filters: {
    auditFilter: AuditFilter;
    focusFilter: ReviewFocusFilter;
    query: string;
    levelFilter: string;
    typeFilter: string;
    onlyErrors: boolean;
    onlyConfirmations: boolean;
  },
) {
  const keyword = filters.query.trim().toLowerCase();
  return logs.filter((log) => matchesAllFilters(log, filters, keyword));
}

function matchesAllFilters(
  log: LogEntry,
  filters: {
    auditFilter: AuditFilter;
    focusFilter: ReviewFocusFilter;
    levelFilter: string;
    typeFilter: string;
    onlyErrors: boolean;
    onlyConfirmations: boolean;
  },
  keyword: string,
) {
  return matchesAudit(log, filters.auditFilter) &&
    matchesFocus(log, filters.focusFilter) &&
    matchesLevel(log, filters.levelFilter) &&
    matchesType(log, filters.typeFilter) &&
    matchesErrorOnly(log, filters.onlyErrors) &&
    matchesConfirmationOnly(log, filters.onlyConfirmations) &&
    matchesKeyword(log, keyword);
}

function matchesAudit(log: LogEntry, auditFilter: AuditFilter) {
  return hasAuditSignal(log, auditFilter);
}

function matchesFocus(log: LogEntry, focusFilter: ReviewFocusFilter) {
  if (focusFilter === "all") return true;
  if (focusFilter === "result") return hasResultFocus(log);
  if (focusFilter === "risk") return hasRiskFocus(log);
  if (focusFilter === "verification") return hasVerificationFocus(log);
  return hasGovernanceFocus(log);
}

function matchesLevel(log: LogEntry, levelFilter: string) {
  return levelFilter === "all" || log.level === levelFilter;
}

function matchesType(log: LogEntry, typeFilter: string) {
  return typeFilter === "all" || readLogType(log) === typeFilter;
}

function matchesErrorOnly(log: LogEntry, onlyErrors: boolean) {
  return !onlyErrors || readLogType(log) === "error";
}

function matchesConfirmationOnly(log: LogEntry, onlyConfirmations: boolean) {
  return !onlyConfirmations || readLogType(log) === "confirmation";
}

function matchesKeyword(log: LogEntry, keyword: string) {
  return !keyword || searchableText(log).includes(keyword);
}

function isReplayPriorityLog(log: LogEntry) {
  const type = readLogType(log);
  if (type === "result") return true;
  if (type === "error") return true;
  return type === "confirmation";
}

function isCompletedResultLog(log: LogEntry) {
  return readLogType(log) === "result" && log.completion_status === "completed";
}

function readToolName(log: LogEntry) {
  return log.tool_call_snapshot?.tool_name || log.tool_name || "";
}

function readTimestamp(value: string) {
  const numeric = Number(value);
  if (!Number.isNaN(numeric)) return numeric;
  const parsed = Date.parse(value);
  return Number.isNaN(parsed) ? 0 : parsed;
}

function searchableText(log: LogEntry) {
  return [
    ...readBaseSearchFields(log),
    ...readErrorSearchFields(log),
    ...readAuditSearchFields(log),
    ...readLocalizedSearchFields(log),
  ]
    .filter(Boolean)
    .join(" ")
    .toLowerCase();
}

function readBaseSearchFields(log: LogEntry) {
  return [
    log.summary,
    log.detail,
    log.level,
    log.category,
    log.tool_category,
    log.source,
    log.event_type,
    log.stage,
    log.run_id,
    log.session_id,
    readToolName(log),
    log.output_kind,
    log.completion_status,
    log.completion_reason,
    readRecordType(log),
    readSourceType(log),
    log.metadata?.reason,
    log.final_answer,
    log.result_summary,
    log.context_snapshot?.workspace_root,
    log.context_snapshot?.memory_digest,
    log.verification_summary,
    log.verification_snapshot?.summary,
    log.metadata?.verification_summary,
  ];
}

function readErrorSearchFields(log: LogEntry) {
  return [
    log.error?.error_code,
    log.error?.message,
    log.error?.summary,
    log.metadata?.error_code,
    log.metadata?.error_message,
    log.metadata?.failure_recovery_hint,
  ];
}

function readAuditSearchFields(log: LogEntry) {
  return [
    log.metadata?.confirmation_chain_step,
    log.metadata?.confirmation_decision,
    log.metadata?.confirmation_resume_strategy,
    log.metadata?.checkpoint_id,
    log.metadata?.tool_elapsed_ms,
    log.metadata?.governance_status,
    log.metadata?.governance_reason,
    log.metadata?.archive_reason,
  ];
}

function readLocalizedSearchFields(log: LogEntry) {
  return [
    readReviewTypeLabel(readLogType(log)),
    readLevelLabel(log.level),
    readCompletionLabel(log.completion_status),
  ];
}

function readLevelLabel(level: string) {
  if (level === "error") return "错误";
  if (level === "warn") return "警告";
  if (level === "info") return "信息";
  return "";
}

function readCompletionLabel(status?: string) {
  if (status === "completed") return "完成";
  if (status === "failed") return "失败";
  if (status === "confirmation_required") return "待确认";
  return "";
}

function hasResultFocus(log: LogEntry) {
  return readLogType(log) === "result" || Boolean(log.result_summary || log.final_answer || log.completion_status === "completed");
}

function hasRiskFocus(log: LogEntry) {
  return readLogType(log) === "error"
    || readLogType(log) === "confirmation"
    || Boolean(log.error || log.risk_level || log.confirmation_id || log.metadata?.failure_recovery_hint);
}

function hasVerificationFocus(log: LogEntry) {
  return readLogType(log) === "verification" || Boolean(log.verification_summary || log.verification_snapshot?.summary);
}

function hasGovernanceFocus(log: LogEntry) {
  return hasAuditSignal(log, "governance") || readLogType(log) === "memory";
}

function readDetailFocusSection(focusFilter: ReviewFocusFilter): HistoryDetailFocusSection {
  if (focusFilter === "result") return "summary";
  if (focusFilter === "risk") return "risk";
  if (focusFilter === "verification") return "verification";
  if (focusFilter === "governance") return "metadata";
  return null;
}

function readRecordType(log: LogEntry) {
  return log.record_type || log.metadata?.record_type || "";
}

function readSourceType(log: LogEntry) {
  return log.source_type || log.metadata?.source_type || "";
}
