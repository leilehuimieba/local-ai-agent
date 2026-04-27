import { KeyboardEvent, useEffect, useMemo, useRef } from "react";

import { RunEvent, RuntimeContextSnapshot } from "../shared/contracts";
import { eventLikeMemory, readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel, readReviewTypeLabel, readRunEventType } from "../history/logType";
import { readUnifiedStatusMeta, UnifiedStatusKey } from "../runtime/state";
import { isPermissionAwaiting, isPermissionBlocked, isPermissionResolved, readPermissionSummary } from "../shared/permissionFlow";
import { EmptyStateBlock, InfoCard, StatusPill } from "../ui/primitives";

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
    <EmptyStateBlock compact title="没有事件记录" text="任务开始后显示阶段推进。" />
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
  const tone = readEventTone(props.event);
  return (
    <InfoCard
      id={props.event.event_id}
      className={buildEventCardClassName(props.selected, props.isLatest, props.event)}
      data-event-id={props.event.event_id}
      role="option"
      aria-selected={props.selected}
      tabIndex={props.selected ? 0 : -1}
      onClick={() => props.onSelect?.(props.event.event_id)}
      onKeyDown={(event: KeyboardEvent<HTMLElement>) => handleEventCardKeyDown(event, props)}
    >
      <EventCardHeader event={props.event} isLatest={props.isLatest} tone={tone} selected={props.selected} />
      <EventSummary event={props.event} tone={tone} />
      <EventDetailList details={details} tone={tone} />
      <EventTagRow event={props.event} />
    </InfoCard>
  );
}

function EventCardHeader(props: { event: RunEvent; isLatest: boolean; tone: string; selected: boolean }) {
  const title = readReviewTypeLabel(readRunEventType(props.event));
  return (
    <div className="investigation-item-head">
      <div className="timeline-head-copy">
        <strong>{title}</strong>
        <p>{props.event.stage}</p>
      </div>
      <div className={readTimelineChipRowClass(props.tone, props.selected)}>
        <StatusPill className={readStatusClass(props.event)} label={readStatusLabel(props.event)} />
        {props.isLatest ? <span className={readFocusBadgeClass(props.tone, props.selected)}>最新</span> : null}
      </div>
    </div>
  );
}

function EventSummary(props: { event: RunEvent; tone: string }) {
  const detail = readPrimaryDetail(props.event);
  return (
    <div className={`timeline-detail-group timeline-detail-group-${props.tone}`}>
      <p className={`timeline-detail timeline-summary timeline-summary-${props.tone}`}>{props.event.summary}</p>
      {detail ? <p className={`timeline-detail timeline-primary-detail timeline-primary-detail-${props.tone}`}>{detail}</p> : null}
    </div>
  );
}

function EventDetailList(props: { details: EventDetail[]; tone: string }) {
  if (props.details.length === 0) return null;
  return (
    <div className={`timeline-detail-list timeline-detail-list-${props.tone}`}>
      {props.details.map((detail) => <DetailRow key={detail.key} detail={detail} tone={props.tone} />)}
    </div>
  );
}

function EventTagRow(props: { event: RunEvent }) {
  const tags = buildEventTags(props.event);
  return (
    <div className="timeline-tags">
      {tags.map((tag) => <span key={tag}>{tag}</span>)}
    </div>
  );
}

function buildEventTags(event: RunEvent) {
  const tags = [readReviewTypeLabel(readRunEventType(event)), readToolTag(event), event.risk_level, event.source || "runtime"];
  if (isMemoryEvent(event)) tags.push(readMemoryFacetLabel(eventLikeMemory(event)));
  if (isMemoryEvent(event)) tags.push(readMemoryGovernanceLabel(eventLikeMemory(event)));
  return compactTags(tags);
}

function buildEventDetails(event: RunEvent) {
  const tokenBudget = observationTokenBudget(event.context_snapshot);
  const details = snapshotDetails(event);
  const permissionSummary = readPermissionSummary(event);
  return compactDetails([
    { key: "action", text: readActionDetail(event) },
    { key: "permission", text: permissionSummary },
    { key: "next", text: readNextActionHint(event) },
    { key: "failure-route", text: readFailureRoute(event) },
    { key: "stall", text: readStallDetail(event) },
    { key: "waiting-reason", text: event.waiting_reason ? `等待原因：${event.waiting_reason}` : "" },
    { key: "artifact", text: event.artifact_path ? `产物：${event.artifact_path}` : "" },
    { key: "verification", text: details.verification ? `验证：${details.verification}` : "" },
    { key: "observation-token-budget", text: tokenBudget },
    { key: "activity-state", text: event.activity_state ? `执行态：${event.activity_state}` : "" },
    { key: "raw-output", text: readRawOutputRef(event) },
    { key: "evidence-ref", text: readEvidenceRef(event) },
    { key: "memory", text: isMemoryEvent(event) ? `记忆动作：${readMemoryActivityLabel(eventLikeMemory(event))}` : "" },
    { key: "governance", text: isMemoryEvent(event) ? `治理状态：${readMemoryGovernanceLabel(eventLikeMemory(event))}` : "" },
    { key: "workspace", text: details.workspace ? `工作区：${details.workspace}` : "" },
    { key: "reason", text: readMemoryReason(event) },
  ]);
}

function observationTokenBudget(snapshot?: RuntimeContextSnapshot) {
  if (!snapshot || !snapshot.observation_budget_total_tokens) return "";
  const used = snapshot.observation_budget_used_tokens || 0;
  const total = snapshot.observation_budget_total_tokens;
  const hit = snapshot.observation_budget_hit_tokens ? "，触顶" : "";
  return `Observation Token(估算)：${used}/${total}${hit}`;
}

function readNextActionHint(event: RunEvent) {
  const hint = event.next_action_hint || event.metadata?.next_step || "";
  return hint ? `下一步：${hint}` : "";
}

function compactDetails(details: EventDetail[]) {
  return details.filter((detail) => detail.text).slice(0, 4);
}

function compactTags(tags: Array<string | undefined>) {
  return [...new Set(tags.filter(Boolean))].slice(0, 3) as string[];
}

function DetailRow(props: { detail: EventDetail; tone: string }) {
  return <p className={`timeline-detail timeline-detail-row timeline-detail-row-${props.tone}`}>{props.detail.text}</p>;
}

function readTimelineChipRowClass(tone: string, selected: boolean) {
  const state = selected ? "selected" : "plain";
  return `timeline-chip-row timeline-chip-row-${tone} timeline-chip-row-${state}`;
}

function readFocusBadgeClass(tone: string, selected: boolean) {
  const state = selected ? "selected" : "plain";
  return `timeline-focus-badge timeline-focus-badge-${tone} timeline-focus-badge-${state}`;
}

function readRawOutputRef(event: RunEvent) {
  const value = event.raw_output_ref || event.metadata?.raw_output_ref || "";
  return value ? `原文引用：${value}` : "";
}

function readEvidenceRef(event: RunEvent) {
  const value = event.evidence_ref || event.metadata?.evidence_ref || "";
  return value ? `证据引用：${value}` : "";
}

function readStallDetail(event: RunEvent) {
  const stall = parseStallSeconds(event.stall_seconds);
  if (stall < 30) return "";
  if (stall >= 120) return `卡住检测：${stall}s（需人工接管）`;
  if (stall >= 60) return `卡住检测：${stall}s（可能卡住）`;
  return `卡住检测：${stall}s（处理中）`;
}

function parseStallSeconds(value?: string) {
  const parsed = Number(value || "0");
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 0;
}

function readFailureRoute(event: RunEvent) {
  const route = event.failure_route || event.metadata?.failure_route || "";
  if (!route) return "";
  return `失败分流：${route}`;
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
  const permissionSummary = readPermissionSummary(event);
  if (event.event_type === "run_failed") return event.detail || event.metadata?.result_summary || "运行失败。";
  if (event.event_type === "confirmation_required") {
    return permissionSummary || event.detail || "当前动作需要确认后继续。";
  }
  if (event.event_type === "memory_recalled") return event.result_summary || event.detail || "已召回相关记忆。";
  if (event.event_type === "memory_write_skipped") return event.detail || event.summary || "本次记忆写入已跳过。";
  if (permissionSummary) return permissionSummary;
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
  if (hasFailedSignal(event)) return "failed";
  if (hasAwaitingSignal(event)) return "awaiting_confirmation";
  if (hasCompletedSignal(event)) return "completed";
  const eventType = readRunEventType(event);
  if (eventType === "memory" || eventType === "verification" || eventType === "result") {
    return "completed";
  }
  return "running";
}

function hasFailedSignal(event: RunEvent) {
  return event.event_type === "run_failed"
    || event.completion_status === "failed"
    || Boolean(event.metadata?.error_code)
    || isPermissionBlocked(event);
}

function hasAwaitingSignal(event: RunEvent) {
  if (isPermissionResolved(event) || hasCompletedSignal(event)) return false;
  return isPermissionAwaiting(event);
}

function hasCompletedSignal(event: RunEvent) {
  return event.completion_status === "completed"
    || event.event_type === "run_finished"
    || isPermissionResolved(event);
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
  const status = readEventStatusKey(event);
  if (status === "failed") return "danger";
  if (status === "awaiting_confirmation") return "warning";
  if (status === "completed") return "calm";
  return "neutral";
}

function isMemoryEvent(event: RunEvent) {
  return readRunEventType(event) === "memory";
}

function readMemoryReason(event: RunEvent) {
  if (!isMemoryEvent(event)) return "";
  return event.detail || event.summary ? `原因：${event.detail || event.summary}` : "";
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
