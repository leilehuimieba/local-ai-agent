import { KeyboardEvent } from "react";

import { LogEntry } from "../../shared/contracts";
import { EmptyStateBlock, MetricChip, SectionHeader, StatusPill } from "../../ui/primitives";
import { readAuditTags } from "../auditSignals";
import { readLogType, readReviewTypeLabel } from "../logType";
import { readUnifiedStatusMeta, UnifiedStatusKey } from "../../runtime/state";
import { isPermissionAwaiting, isPermissionBlocked, isPermissionResolved, readPermissionSummary } from "../../shared/permissionFlow";

type HistoryTimelineSectionProps = {
  logs: LogEntry[];
  selectedLogId: string;
  onSelectLog: (logId: string) => void;
};

export function HistoryTimelineSection(props: HistoryTimelineSectionProps) {
  return (
    <section className="page-section logs-timeline-panel">
      <SectionHeader
        kicker="Timeline"
        title="稳定记录流"
        description="按时间顺序扫读稳定记录，优先看状态、阶段、摘要和关键信号。"
        action={<MetricChip label="记录" value={`${props.logs.length} 条`} />}
      />
      <div className="investigation-list" role="listbox" aria-label="稳定记录时间线">
        {renderTimeline(props.logs, props.selectedLogId, props.onSelectLog)}
      </div>
    </section>
  );
}

function renderTimeline(logs: LogEntry[], selectedLogId: string, onSelectLog: (logId: string) => void) {
  if (logs.length === 0) return <EmptyTimeline />;
  return logs.map((log) => (
    <HistoryTimelineItem
      key={log.log_id}
      log={log}
      selected={log.log_id === selectedLogId}
      onSelect={onSelectLog}
    />
  ));
}

function EmptyTimeline() {
  return <EmptyStateBlock compact title="当前筛选下没有记录" text="调整搜索或筛选条件后，这里会显示匹配结果。" />;
}

function HistoryTimelineItem(props: {
  log: LogEntry;
  selected: boolean;
  onSelect: (logId: string) => void;
}) {
  const itemClass = buildItemClass(props.selected, props.log);
  return (
    <article
      id={props.log.log_id}
      className={itemClass}
      role="option"
      aria-selected={props.selected}
      tabIndex={props.selected ? 0 : -1}
      onClick={() => props.onSelect(props.log.log_id)}
      onKeyDown={(event) => handleHistoryItemKeyDown(event, props.log.log_id, props.onSelect)}
    >
      <TimelineItemHeader log={props.log} selected={props.selected} />
      <TimelineItemDetails log={props.log} />
      <TimelineItemMeta log={props.log} />
    </article>
  );
}

function TimelineItemHeader(props: { log: LogEntry; selected: boolean }) {
  const logType = readLogType(props.log);
  return (
    <div className="investigation-item-head">
      <div className="timeline-head-copy">
        <strong>{readReviewTypeLabel(logType)}</strong>
        <p>{props.log.stage || "无阶段"}</p>
      </div>
      <div className={buildTimelineChipRowClass(props.log, props.selected)}>
        <StatusPill className={readStatusClass(props.log)} label={readStatusLabel(props.log)} />
        {readRiskTag(props.log) ? <span className="risk-pill risk-medium">{readRiskTag(props.log)}</span> : null}
        {props.log.error?.error_code ? <span className="sidebar-chip-muted">{props.log.error.error_code}</span> : null}
      </div>
    </div>
  );
}

function TimelineItemDetails(props: { log: LogEntry }) {
  return (
    <div className="timeline-detail-group">
      {readTimelineDetails(props.log).map((item, index) => (
        <p key={`${props.log.log_id}-${index}`} className={index === 0 ? "timeline-detail timeline-summary" : "timeline-detail"}>{item}</p>
      ))}
    </div>
  );
}

function TimelineItemMeta(props: { log: LogEntry }) {
  const tags = readTimelineTags(props.log);
  return (
    <div className="timeline-tags">
      {tags.map((item) => <span key={`${props.log.log_id}-${item}`}>{item}</span>)}
    </div>
  );
}

function buildItemClass(selected: boolean, log: LogEntry) {
  const tone = readItemTone(log);
  return selected ? `investigation-item selected tone-${tone}` : `investigation-item tone-${tone}`;
}

function buildTimelineChipRowClass(log: LogEntry, selected: boolean) {
  const tone = readItemTone(log);
  const stateClass = `timeline-chip-row timeline-chip-row-${tone}`;
  return selected ? `${stateClass} timeline-chip-row-selected` : stateClass;
}

function readItemTone(log: LogEntry) {
  const status = readHistoryStatusKey(log);
  if (status === "failed") return "danger";
  if (status === "awaiting_confirmation") return "warning";
  if (status === "completed") return "calm";
  return "neutral";
}

function readToolName(log: LogEntry) {
  return log.tool_call_snapshot?.tool_name || log.tool_name || "";
}

function readRiskTag(log: LogEntry) {
  return log.risk_level || (log.confirmation_id ? "确认" : "");
}

function readTimelineDetails(log: LogEntry) {
  return [
    readTimelineSummary(log),
    readToolName(log) ? `工具：${readToolName(log)}` : "",
    readKeyDetail(log),
  ].filter(Boolean) as string[];
}

function readTimelineTags(log: LogEntry) {
  const tags = [
    readReviewTypeLabel(readLogType(log)),
    log.tool_category || log.category || "",
    log.level || "",
    log.source || "runtime",
    ...readAuditTags(log),
  ].filter(Boolean);
  const unique = Array.from(new Set(tags)).slice(0, 3);
  return [...unique, formatTimestamp(log.timestamp)];
}

function readKeyDetail(log: LogEntry) {
  const permissionSummary = readPermissionSummary(log);
  if (log.error) return `${log.error.error_code} / ${log.error.message}`;
  if (readLogType(log) === "confirmation") {
    return permissionSummary || log.metadata?.reason || log.detail || "当前记录要求人工确认后才能继续。";
  }
  if (log.detail_preview) return log.detail_preview;
  if (permissionSummary) return permissionSummary;
  if (log.metadata?.confirmation_chain_step) return `确认链：${log.metadata.confirmation_chain_step}`;
  if (log.metadata?.tool_elapsed_ms) return `工具耗时：${log.metadata.tool_elapsed_ms}ms`;
  return log.result_summary || log.verification_snapshot?.summary || log.detail || "";
}

function readStatusLabel(log: LogEntry) {
  return readUnifiedStatusMeta(readHistoryStatusKey(log)).label;
}

function readStatusClass(log: LogEntry) {
  return readUnifiedStatusMeta(readHistoryStatusKey(log)).className;
}

function readHistoryStatusKey(log: LogEntry): UnifiedStatusKey {
  if (hasHistoryFailedSignal(log)) return "failed";
  if (hasHistoryAwaitingSignal(log)) return "awaiting_confirmation";
  if (hasHistoryCompletedSignal(log)) return "completed";
  const type = readLogType(log);
  if (type === "result" || type === "memory" || type === "verification") return "completed";
  return "running";
}

function hasHistoryFailedSignal(log: LogEntry) {
  return readLogType(log) === "error"
    || log.completion_status === "failed"
    || Boolean(log.error || log.metadata?.error_code)
    || isPermissionBlocked(log);
}

function hasHistoryAwaitingSignal(log: LogEntry) {
  if (isPermissionAwaiting(log)) return true;
  if (isPermissionResolved(log) || hasHistoryCompletedSignal(log)) return false;
  return readLogType(log) === "confirmation"
    || log.completion_status === "confirmation_required"
    || Boolean(log.confirmation_id);
}

function hasHistoryCompletedSignal(log: LogEntry) {
  return log.completion_status === "completed"
    || Boolean(log.final_answer)
    || isPermissionResolved(log);
}

function handleHistoryItemKeyDown(
  event: KeyboardEvent<HTMLElement>,
  logId: string,
  onSelect: (logId: string) => void,
) {
  if (event.key !== "Enter" && event.key !== " ") return;
  event.preventDefault();
  onSelect(logId);
}

function formatTimestamp(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleString("zh-CN", { hour12: false });
}

function readTimelineSummary(log: LogEntry) {
  const type = readLogType(log);
  if (type === "result") return log.final_answer || log.summary;
  if (type === "error") return log.error?.summary || log.summary;
  if (type === "confirmation") return log.metadata?.action_summary || log.summary;
  return log.summary;
}
