import { KeyboardEvent, useEffect, useMemo, useRef } from "react";

import { RunEvent, RuntimeContextSnapshot } from "../shared/contracts";
import { readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel, readReviewTypeLabel, readRunEventType } from "../history/logType";
import { readUnifiedStatusMeta, UnifiedStatusKey } from "../runtime/state";

type EventTimelineProps = {
  autoFollow?: boolean;
  events: RunEvent[];
  onLeaveLatest?: () => void;
  selectedEventId?: string;
  onSelectEvent?: (eventId: string) => void;
};

type EventDetail = {
  key: string;
  text: string;
};

export function EventTimeline(props: EventTimelineProps) {
  if (props.events.length === 0) return <EmptyTimeline />;
  return <TimelineList props={props} />;
}

function EmptyTimeline() {
  return (
    <div className="empty-state compact">
      <h3>没有事件记录</h3>
      <p>任务开始后，这里会显示当前会话的阶段推进和关键动作。</p>
    </div>
  );
}

function TimelineList({ props }: { props: EventTimelineProps }) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const lockAutoFollowRef = useRef(false);
  const latestEventId = props.events[props.events.length - 1]?.event_id || "";
  const orderedEvents = useMemo(() => [...props.events].reverse(), [props.events]);
  useEffect(() => {
    if (!props.autoFollow || !latestEventId || !containerRef.current) return;
    lockAutoFollowRef.current = true;
    const latestNode = containerRef.current.querySelector<HTMLElement>(`[data-event-id="${latestEventId}"]`);
    latestNode?.scrollIntoView({ block: "nearest", behavior: "smooth" });
    const timer = window.setTimeout(() => { lockAutoFollowRef.current = false; }, 220);
    return () => window.clearTimeout(timer);
  }, [latestEventId, props.autoFollow]);
  return (
    <div
      ref={containerRef}
      className="investigation-list"
      role="listbox"
      aria-label="任务事件时间线"
      aria-activedescendant={props.selectedEventId || undefined}
      onScroll={(event) => handleTimelineScroll(event.currentTarget, props.autoFollow, lockAutoFollowRef.current, props.onLeaveLatest)}
    >
      {orderedEvents.map((event, index) => (
        <EventCard
          key={event.event_id}
          event={event}
          isLatest={index === 0}
          previousEventId={orderedEvents[index + 1]?.event_id}
          nextEventId={orderedEvents[index - 1]?.event_id}
          selected={event.event_id === props.selectedEventId}
          onSelect={props.onSelectEvent}
        />
      ))}
    </div>
  );
}

function EventCard(props: {
  event: RunEvent;
  isLatest: boolean;
  nextEventId?: string;
  previousEventId?: string;
  selected: boolean;
  onSelect?: (eventId: string) => void;
}) {
  const details = buildEventDetails(props.event);
  return (
    <article
      id={props.event.event_id}
      className={buildEventCardClassName(props.selected, props.isLatest, props.event)}
      data-event-id={props.event.event_id}
      role="option"
      aria-selected={props.selected}
      tabIndex={props.selected ? 0 : -1}
      onClick={() => props.onSelect?.(props.event.event_id)}
      onKeyDown={(event) => handleEventCardKeyDown(event, props)}
    >
      <EventCardHeader event={props.event} isLatest={props.isLatest} />
      <EventSummary event={props.event} />
      <EventDetailList details={details} />
      <EventTagRow event={props.event} />
    </article>
  );
}

function EventCardHeader(props: { event: RunEvent; isLatest: boolean }) {
  const eventType = readRunEventType(props.event);
  return (
    <div className="investigation-item-head">
      <div className="timeline-head-copy">
        <strong>{readReviewTypeLabel(eventType)}</strong>
        <p>{props.event.stage}</p>
      </div>
      <div className="timeline-chip-row">
        <span className={`status-badge ${readStatusClass(props.event)}`}>{readStatusLabel(props.event)}</span>
        {props.isLatest ? <span className="timeline-focus-badge">最新</span> : null}
      </div>
    </div>
  );
}

function EventSummary(props: { event: RunEvent }) {
  return (
    <div className="timeline-detail-group">
      <p className="timeline-detail timeline-summary">{props.event.summary}</p>
      {readPrimaryDetail(props.event) ? <p className="timeline-detail">{readPrimaryDetail(props.event)}</p> : null}
    </div>
  );
}

function EventDetailList(props: { details: EventDetail[] }) {
  if (props.details.length === 0) return null;
  return (
    <div className="timeline-detail-group">
      {props.details.map((detail) => <p key={detail.key} className="timeline-detail">{detail.text}</p>)}
    </div>
  );
}

function EventTagRow(props: { event: RunEvent }) {
  return (
    <div className="timeline-tags">
      <span>{readReviewTypeLabel(readRunEventType(props.event))}</span>
      <span>{props.event.source || "runtime"}</span>
      {props.event.tool_category ? <span>{props.event.tool_category}</span> : null}
      {readToolTag(props.event) ? <span>{readToolTag(props.event)}</span> : null}
      {props.event.risk_level ? <span>{props.event.risk_level}</span> : null}
      {isMemoryEvent(props.event) ? <span>{readMemoryFacetLabel(eventLikeMemory(props.event))}</span> : null}
      {isMemoryEvent(props.event) ? <span>{readMemoryGovernanceLabel(eventLikeMemory(props.event))}</span> : null}
    </div>
  );
}

function buildEventDetails(event: RunEvent) {
  const details = snapshotDetails(event);
  return compactDetails([
    { key: "detail", text: readPrimaryDetail(event) },
    { key: "action", text: readActionDetail(event) },
    { key: "memory", text: isMemoryEvent(event) ? `记忆动作：${readMemoryActivityLabel(eventLikeMemory(event))}` : "" },
    { key: "governance", text: isMemoryEvent(event) ? `治理状态：${readMemoryGovernanceLabel(eventLikeMemory(event))}` : "" },
    { key: "artifact", text: event.artifact_path ? `产物：${event.artifact_path}` : "" },
    { key: "workspace", text: details.workspace ? `工作区：${details.workspace}` : "" },
    { key: "verification", text: details.verification ? `验证：${details.verification}` : "" },
    { key: "next", text: event.metadata?.next_step ? `下一步：${event.metadata.next_step}` : "" },
    { key: "reason", text: readMemoryReason(event) },
  ]);
}

function compactDetails(details: EventDetail[]) {
  return details.filter((detail) => detail.text).slice(0, 4);
}

function snapshotDetails(event: RunEvent) {
  return {
    cache: cacheDetail(event.context_snapshot),
    completion: event.completion_reason,
    knowledgeDigest: event.context_snapshot?.knowledge_digest,
    memoryDigest: event.context_snapshot?.memory_digest,
    reasoning: event.context_snapshot?.reasoning_summary,
    repoSummary: event.metadata?.repo_context_summary,
    verification: event.verification_snapshot?.summary,
    workspace: event.context_snapshot?.workspace_root,
  };
}

function readPrimaryDetail(event: RunEvent) {
  if (event.event_type === "run_failed") return event.detail || event.metadata?.result_summary || "运行失败。";
  if (event.event_type === "confirmation_required") return event.detail || "当前动作需要确认后继续。";
  if (event.event_type === "memory_recalled") return event.result_summary || event.detail || "已召回相关记忆。";
  if (event.event_type === "memory_write_skipped") return event.detail || event.summary || "本次记忆写入已跳过。";
  return event.result_summary || event.metadata?.result_summary || event.detail || "";
}

function readToolTag(event: RunEvent) {
  return event.tool_display_name || event.tool_call_snapshot?.display_name || event.tool_name || event.metadata?.tool_name || "";
}

function readActionDetail(event: RunEvent) {
  if (event.tool_display_name && event.tool_category) return `动作：${event.tool_display_name} / ${event.tool_category}`;
  return event.tool_display_name || event.tool_name || event.tool_category ? `动作：${event.tool_display_name || event.tool_name || event.tool_category}` : "";
}

function readStatusLabel(event: RunEvent) {
  return readUnifiedStatusMeta(readEventStatusKey(event)).label;
}

function readStatusClass(event: RunEvent) {
  return readUnifiedStatusMeta(readEventStatusKey(event)).className;
}

function readEventStatusKey(event: RunEvent): UnifiedStatusKey {
  const eventType = readRunEventType(event);
  if (eventType === "error") return "failed";
  if (eventType === "confirmation") return "awaiting_confirmation";
  if (eventType === "memory" || eventType === "verification" || eventType === "result") return "completed";
  return "running";
}

function cacheDetail(context?: RuntimeContextSnapshot) {
  if (!context?.cache_status) return "";
  return context.cache_reason ? `${context.cache_status} | ${context.cache_reason}` : context.cache_status;
}

function buildEventCardClassName(selected: boolean, isLatest: boolean, event: RunEvent) {
  const parts = ["investigation-item", `tone-${readEventTone(event)}`];
  if (selected) parts.push("selected");
  if (isLatest) parts.push("latest");
  return parts.join(" ");
}

function readEventTone(event: RunEvent) {
  const eventType = readRunEventType(event);
  if (eventType === "error") return "danger";
  if (eventType === "confirmation") return "warning";
  if (eventType === "memory" || eventType === "verification") return "calm";
  return "neutral";
}

function isMemoryEvent(event: RunEvent) {
  return readRunEventType(event) === "memory";
}

function readMemoryReason(event: RunEvent) {
  if (!isMemoryEvent(event)) return "";
  return event.detail || event.summary ? `原因：${event.detail || event.summary}` : "";
}

function eventLikeMemory(event: RunEvent) {
  return {
    event_type: event.event_type,
    kind: event.metadata?.memory_kind || event.record_type || event.output_kind,
    metadata: event.metadata,
    reason: event.detail || event.summary,
    source_type: event.source_type,
    summary: event.summary,
    title: event.metadata?.task_title || event.summary,
    verified: event.verification_snapshot?.passed,
  };
}

function handleTimelineScroll(
  node: HTMLDivElement,
  autoFollow = false,
  locked = false,
  onLeaveLatest?: () => void,
) {
  if (locked || !autoFollow || !onLeaveLatest) return;
  if (node.scrollTop > 48) onLeaveLatest();
}

function handleEventCardKeyDown(
  event: KeyboardEvent<HTMLElement>,
  props: {
    event: RunEvent;
    nextEventId?: string;
    previousEventId?: string;
    onSelect?: (eventId: string) => void;
  },
) {
  if (event.key === "Enter" || event.key === " ") {
    event.preventDefault();
    props.onSelect?.(props.event.event_id);
    return;
  }
  if (event.key === "ArrowDown" && props.previousEventId) {
    event.preventDefault();
    props.onSelect?.(props.previousEventId);
    focusTimelineItem(props.previousEventId);
    return;
  }
  if (event.key === "ArrowUp" && props.nextEventId) {
    event.preventDefault();
    props.onSelect?.(props.nextEventId);
    focusTimelineItem(props.nextEventId);
  }
}

function focusTimelineItem(eventId: string) {
  requestAnimationFrame(() => {
    const node = document.getElementById(eventId);
    node?.focus();
  });
}
