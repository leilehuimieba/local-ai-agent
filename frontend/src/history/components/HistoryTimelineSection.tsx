import { KeyboardEvent } from "react";

import { LogEntry } from "../../shared/contracts";
import { EmptyStateBlock, SectionHeader, StatusPill } from "../../ui/primitives";
import { readLogType, readReviewTypeLabel } from "../logType";

type HistoryTimelineSectionProps = {
  logs: LogEntry[];
  selectedLogId: string;
  onSelectLog: (logId: string) => void;
};

export function HistoryTimelineSection(props: HistoryTimelineSectionProps) {
  return (
    <section className="page-section logs-timeline-panel">
      <SectionHeader title="时间线" description="按时间顺序扫读稳定记录，优先看类型、阶段、摘要和风险。" />
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
      <TimelineItemHeader log={props.log} />
      <TimelineItemDetails log={props.log} />
      <TimelineItemMeta log={props.log} />
    </article>
  );
}

function TimelineItemHeader(props: { log: LogEntry }) {
  const logType = readLogType(props.log);
  return (
    <div className="investigation-item-head">
      <div className="timeline-head-copy">
        <strong>{readReviewTypeLabel(logType)}</strong>
        <p>{props.log.stage || "无阶段"}</p>
      </div>
      <div className="timeline-chip-row">
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
      <p className="timeline-detail timeline-summary">{readTimelineSummary(props.log)}</p>
      {readToolName(props.log) ? <p className="timeline-detail">工具：{readToolName(props.log)}</p> : null}
      {readKeyDetail(props.log) ? <p className="timeline-detail">{readKeyDetail(props.log)}</p> : null}
    </div>
  );
}

function TimelineItemMeta(props: { log: LogEntry }) {
  return (
    <div className="timeline-tags">
      <span>{readReviewTypeLabel(readLogType(props.log))}</span>
      <span>{props.log.tool_category || props.log.category}</span>
      <span>{props.log.level}</span>
      <span>{props.log.source || "runtime"}</span>
      <span>{formatTimestamp(props.log.timestamp)}</span>
    </div>
  );
}

function buildItemClass(selected: boolean, log: LogEntry) {
  const tone = readItemTone(log);
  return selected ? `investigation-item selected tone-${tone}` : `investigation-item tone-${tone}`;
}

function readItemTone(log: LogEntry) {
  const type = readLogType(log);
  if (type === "error") return "danger";
  if (type === "confirmation") return "warning";
  if (type === "memory" || type === "verification") return "calm";
  return "neutral";
}

function readToolName(log: LogEntry) {
  return log.tool_call_snapshot?.tool_name || log.tool_name || "";
}

function readRiskTag(log: LogEntry) {
  return log.risk_level || (log.confirmation_id ? "确认" : "");
}

function readKeyDetail(log: LogEntry) {
  if (log.error) return `${log.error.error_code} / ${log.error.message}`;
  if (readLogType(log) === "confirmation") return log.metadata?.reason || log.detail || "当前记录要求人工确认后才能继续。";
  return log.result_summary || log.verification_snapshot?.summary || log.detail || "";
}

function readStatusLabel(log: LogEntry) {
  const type = readLogType(log);
  if (type === "error") return "失败";
  if (type === "confirmation") return "待确认";
  if (log.completion_status) return log.completion_status;
  if (log.final_answer) return "已完成";
  return "处理中";
}

function readStatusClass(log: LogEntry) {
  const type = readLogType(log);
  if (type === "error") return "status-failed";
  if (type === "confirmation") return "status-awaiting";
  if (log.completion_status === "completed" || log.final_answer) return "status-completed";
  return "status-running";
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
