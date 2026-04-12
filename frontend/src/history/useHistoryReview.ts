import { useMemo, useState } from "react";

import { LogEntry } from "../shared/contracts";
import { AuditFilter, hasAuditSignal } from "./auditSignals";
import { readLogType } from "./logType";

export type HistoryStats = {
  confirmationCount: number;
  errorCount: number;
  sessionCount: number;
  toolCount: number;
};

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
    toolbarProps: buildToolbarProps(filteredLogs.length, filters),
  };
}

function useReviewFilters() {
  const [auditFilter, setAuditFilter] = useState<AuditFilter>("all");
  const [query, setQuery] = useState("");
  const [levelFilter, setLevelFilter] = useState("all");
  const [typeFilter, setTypeFilter] = useState("all");
  const [onlyErrors, setOnlyErrors] = useState(false);
  const [onlyConfirmations, setOnlyConfirmations] = useState(false);
  const [selectedLogId, setSelectedLogId] = useState("");
  return {
    selectedLogId,
    setSelectedLogId,
    values: { auditFilter, levelFilter, onlyConfirmations, onlyErrors, query, typeFilter },
    setters: { setAuditFilter, setLevelFilter, setOnlyConfirmations, setOnlyErrors, setQuery, setTypeFilter },
  };
}

function useFocusLog(logs: LogEntry[], selectedLogId: string) {
  return useMemo(() => selectFocusLog(logs, selectedLogId), [logs, selectedLogId]);
}

function buildToolbarProps(resultCount: number, filters: ReturnType<typeof useReviewFilters>) {
  return {
    auditFilter: filters.values.auditFilter,
    onAuditFilterChange: filters.setters.setAuditFilter,
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
    levelFilter: string;
    typeFilter: string;
    onlyErrors: boolean;
    onlyConfirmations: boolean;
  },
  keyword: string,
) {
  return matchesAudit(log, filters.auditFilter) &&
    matchesLevel(log, filters.levelFilter) &&
    matchesType(log, filters.typeFilter) &&
    matchesErrorOnly(log, filters.onlyErrors) &&
    matchesConfirmationOnly(log, filters.onlyConfirmations) &&
    matchesKeyword(log, keyword);
}

function matchesAudit(log: LogEntry, auditFilter: AuditFilter) {
  return hasAuditSignal(log, auditFilter);
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
  return [...readBaseSearchFields(log), ...readAuditSearchFields(log)]
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
    log.context_snapshot?.workspace_root,
    log.context_snapshot?.memory_digest,
    log.verification_summary,
    log.verification_snapshot?.summary,
    log.metadata?.verification_summary,
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

function readRecordType(log: LogEntry) {
  return log.record_type || log.metadata?.record_type || "";
}

function readSourceType(log: LogEntry) {
  return log.source_type || log.metadata?.source_type || "";
}
