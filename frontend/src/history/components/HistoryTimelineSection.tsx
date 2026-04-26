import { KeyboardEvent } from "react";

import { LogEntry } from "../../shared/contracts";
import { EmptyStateBlock, SectionHeader, StatusPill } from "../../ui/primitives";
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
    <section className="page-section logs-timeline-panel simple-logs-timeline">
      <SectionHeader
        kicker="Timeline"
        title="工作时间线"
        description="按时间查看最近做过什么，默认隐藏技术细节，点击后查看工具、验证与耗时。"
      />
      <div className="simple-timeline-list" role="listbox" aria-label="工作时间线">
        {renderTimeline(props.logs, props.selectedLogId, props.onSelectLog)}
      </div>
    </section>
  );
}

function renderTimeline(logs: LogEntry[], selectedLogId: string, onSelectLog: (logId: string) => void) {
  if (logs.length === 0) return <EmptyTimeline />;
  return logs.map((log) => <HistoryTimelineItem key={log.log_id} log={log} selected={log.log_id === selectedLogId} onSelect={onSelectLog} />);
}

function EmptyTimeline() {
  return <EmptyStateBlock compact title="还没有记录" text="完成一次任务后，这里会显示结果摘要。" />;
}

function HistoryTimelineItem(props: { log: LogEntry; selected: boolean; onSelect: (logId: string) => void }) {
  return (
    <article className={buildItemClass(props.selected, props.log)} role="option" aria-selected={props.selected} tabIndex={0} onClick={() => props.onSelect(props.log.log_id)} onKeyDown={(event) => handleHistoryItemKeyDown(event, props.log.log_id, props.onSelect)}>
      <TimelineItemHeader log={props.log} selected={props.selected} />
      <p className="simple-timeline-summary">{readTimelineSummary(props.log)}</p>
      {props.selected ? <TimelineExpandedDetails log={props.log} /> : null}
    </article>
  );
}

function TimelineItemHeader(props: { log: LogEntry; selected: boolean }) {
  return (
    <div className="simple-timeline-head">
      <div>
        <time>{formatTimestamp(props.log.timestamp)}</time>
        <strong>{readTimelineTitle(props.log)}</strong>
      </div>
      <StatusPill className={readStatusClass(props.log)} label={readStatusLabel(props.log)} />
      <span className="simple-expand-hint">{props.selected ? "收起" : "展开"}</span>
    </div>
  );
}

function TimelineExpandedDetails(props: { log: LogEntry }) {
  return (
    <dl className="simple-timeline-details">
      {readDetailRows(props.log).map((item) => <TimelineDetailRow key={item.label} {...item} />)}
    </dl>
  );
}

function TimelineDetailRow(props: { label: string; value: string }) {
  return (
    <div>
      <dt>{props.label}</dt>
      <dd>{props.value}</dd>
    </div>
  );
}

function readDetailRows(log: LogEntry) {
  return [
    { label: "结果摘要", value: readKeyDetail(log) || readTimelineSummary(log) || "未附带" },
    { label: "工具调用", value: readToolName(log) || "无" },
    { label: "验证结果", value: readVerification(log) },
    { label: "运行耗时", value: readElapsed(log) },
  ];
}

function buildItemClass(selected: boolean, log: LogEntry) {
  const tone = readItemTone(log);
  return selected ? `simple-timeline-item selected tone-${tone}` : `simple-timeline-item tone-${tone}`;
}

function readItemTone(log: LogEntry) {
  const status = readHistoryStatusKey(log);
  if (status === "failed") return "danger";
  if (status === "awaiting_confirmation") return "warning";
  if (status === "completed") return "calm";
  return "neutral";
}

function readTimelineTitle(log: LogEntry) {
  return log.metadata?.task_title || log.task_title || log.summary || readReviewTypeLabel(readLogType(log));
}

function readToolName(log: LogEntry) {
  return log.tool_call_snapshot?.tool_name || log.tool_display_name || log.tool_name || "";
}

function readVerification(log: LogEntry) {
  if (log.verification_snapshot?.summary) return log.verification_snapshot.summary;
  if (log.verification_summary) return log.verification_summary;
  if (log.verification_snapshot?.passed === true) return "通过";
  if (log.verification_snapshot?.passed === false) return "未通过";
  return "未附带";
}

function readElapsed(log: LogEntry) {
  const value = log.metadata?.tool_elapsed_ms || log.metadata?.elapsed_ms || "";
  return value ? `${value}ms` : "未附带";
}

function readKeyDetail(log: LogEntry) {
  const permissionSummary = readPermissionSummary(log);
  if (log.error) return `${log.error.error_code} / ${log.error.message}`;
  if (readLogType(log) === "confirmation") return permissionSummary || log.metadata?.reason || log.detail || "当前记录要求人工确认后才能继续。";
  if (log.detail_preview) return log.detail_preview;
  if (permissionSummary) return permissionSummary;
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
  return readLogType(log) === "error" || log.completion_status === "failed" || Boolean(log.error || log.metadata?.error_code) || isPermissionBlocked(log);
}

function hasHistoryAwaitingSignal(log: LogEntry) {
  if (isPermissionAwaiting(log)) return true;
  if (isPermissionResolved(log) || hasHistoryCompletedSignal(log)) return false;
  return readLogType(log) === "confirmation" || log.completion_status === "confirmation_required" || Boolean(log.confirmation_id);
}

function hasHistoryCompletedSignal(log: LogEntry) {
  return log.completion_status === "completed" || Boolean(log.final_answer) || isPermissionResolved(log);
}

function handleHistoryItemKeyDown(event: KeyboardEvent<HTMLElement>, logId: string, onSelect: (logId: string) => void) {
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
