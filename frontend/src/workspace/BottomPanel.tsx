import { useEffect, useState } from "react";

import { readRunStateBody, readRunStateHeadline, readRunStateNextStep } from "../chat/chatResultModel";
import { EventTimeline } from "../events/EventTimeline";
import { RunState } from "../runtime/state";
import { RunEvent } from "../shared/contracts";
import { MetaGrid, SectionHeader } from "../ui/primitives";

type BottomPanelProps = {
  isOpen: boolean;
  events: RunEvent[];
  currentTaskTitle: string;
  runState: RunState;
  submitError: string | null;
  onOpenChange: (open: boolean) => void;
};

export function BottomPanel(props: BottomPanelProps) {
  const focus = useInvestigationFocus(props.events);
  const model = buildInvestigationModel(props.events, focus.selectedEventId, focus.autoFollow);
  return (
    <section className={props.isOpen ? "bottom-panel open" : "bottom-panel"} aria-label="当前任务调查层">
      <BottomPanelHeader isOpen={props.isOpen} onOpenChange={props.onOpenChange} />
      <BottomPanelSummary props={props} model={model} />
      {props.isOpen ? <BottomPanelBody props={props} model={model} focus={focus} /> : null}
    </section>
  );
}

function useInvestigationFocus(events: RunEvent[]) {
  const latestEventId = events[events.length - 1]?.event_id || "";
  const [selectedEventId, setSelectedEventId] = useState("");
  const [autoFollow, setAutoFollow] = useState(true);
  useEffect(() => {
    if (!latestEventId) return resetFocus(setSelectedEventId, setAutoFollow);
    if (autoFollow) setSelectedEventId(latestEventId);
  }, [autoFollow, latestEventId]);
  return {
    autoFollow,
    pauseFollow: () => setAutoFollow(false),
    resumeFollow: () => resumeFocus(latestEventId, setAutoFollow, setSelectedEventId),
    selectedEventId,
    selectEvent: (eventId: string) => selectFocusEventId(eventId, latestEventId, setAutoFollow, setSelectedEventId),
  };
}

function buildInvestigationModel(events: RunEvent[], selectedEventId: string, autoFollow: boolean) {
  const focusEvent = resolveFocusEvent(events, selectedEventId, autoFollow);
  return {
    autoFollow,
    events,
    focusEvent,
    inspection: getEventInspection(events, focusEvent),
    preview: getEventPreview(focusEvent, autoFollow),
    selectedEventId: focusEvent?.event_id || "",
  };
}

function BottomPanelHeader(props: { isOpen: boolean; onOpenChange: (open: boolean) => void }) {
  return (
    <div className="bottom-panel-header">
      <div>
        <span className="bottom-panel-kicker">调查</span>
        <h2>当前会话调查</h2>
      </div>
      <button
        type="button"
        className="panel-toggle"
        aria-controls="investigation-panel-body"
        aria-expanded={props.isOpen}
        onClick={() => props.onOpenChange(!props.isOpen)}
      >
        {props.isOpen ? "收起调查" : "展开调查"}
      </button>
    </div>
  );
}

function BottomPanelSummary(props: {
  props: BottomPanelProps;
  model: ReturnType<typeof buildInvestigationModel>;
}) {
  const summaryStatus = readSummaryStatusClass(props.props.runState, props.model.autoFollow);
  const summaryLabel = readSummaryStatusLabel(props.props.runState, props.model.autoFollow);
  return (
    <div className="bottom-panel-summary" aria-live="polite">
      <div className="bottom-panel-summary-copy">
        <strong>{readRunStateHeadline(props.props.runState, props.model.focusEvent || undefined)}</strong>
        <p>{readBottomPanelSummary(props.props.runState, props.model)}</p>
      </div>
      <div className="bottom-panel-summary-meta">
        <span className={`status-badge ${summaryStatus}`}>{summaryLabel}</span>
        <span className="sidebar-chip-muted">{props.props.events.length} 条事件</span>
      </div>
    </div>
  );
}

function BottomPanelBody(props: {
  props: BottomPanelProps;
  model: ReturnType<typeof buildInvestigationModel>;
  focus: ReturnType<typeof useInvestigationFocus>;
}) {
  if (shouldShowPendingInvestigation(props.props.runState, props.model.events)) {
    return <PendingInvestigationState taskTitle={props.props.currentTaskTitle} />;
  }
  if (shouldShowFailedInvestigation(props.props.runState, props.model.events, props.props.submitError)) {
    return <FailedInvestigationState submitError={props.props.submitError} />;
  }
  return (
    <div id="investigation-panel-body" className="bottom-panel-body investigation-board">
      <InvestigationLane props={props} />
      <FocusLane props={props} />
    </div>
  );
}

function InvestigationLane(props: {
  props: {
    props: BottomPanelProps;
    model: ReturnType<typeof buildInvestigationModel>;
    focus: ReturnType<typeof useInvestigationFocus>;
  };
}) {
  return (
    <section className="investigation-lane">
      <LaneHeader title="事件流" text="当前会话过程。" />
      {shouldShowInlineInvestigationFailure(props.props.props.runState, props.props.model.events, props.props.props.submitError) ? (
        <InlineInvestigationFailure submitError={props.props.props.submitError} />
      ) : null}
      <EventTimeline
        autoFollow={props.props.model.autoFollow}
        events={props.props.model.events}
        onLeaveLatest={props.props.focus.pauseFollow}
        selectedEventId={props.props.model.selectedEventId}
        onSelectEvent={props.props.focus.selectEvent}
      />
    </section>
  );
}

function FocusLane(props: {
  props: {
    props: BottomPanelProps;
    model: ReturnType<typeof buildInvestigationModel>;
    focus: ReturnType<typeof useInvestigationFocus>;
  };
}) {
  return (
    <aside className="inspection-lane">
      <LaneHeader title="焦点详情" text="阶段、证据、下一步。" />
      {!props.props.model.autoFollow ? <button type="button" className="secondary-button" onClick={props.props.focus.resumeFollow}>返回最新事件</button> : null}
      <InspectionFocusCard model={props.props.model} />
      <InspectionMetaGrid model={props.props.model} />
      <InspectionNextCard model={props.props.model} />
    </aside>
  );
}

function PendingInvestigationState(props: { taskTitle: string }) {
  return (
    <div id="investigation-panel-body" className="bottom-panel-body investigation-board">
      <section className="investigation-lane">
        <LaneHeader title="事件流" text={readRunStateBody({ currentTaskTitle: props.taskTitle, runState: "submitting" })} />
        <StateCard title={readRunStateHeadline("submitting")} body={props.taskTitle || "当前任务"} advice={readRunStateNextStep({ runState: "submitting" })} />
      </section>
      <aside className="inspection-lane">
        <LaneHeader title="焦点详情" text="首个事件返回后，这里会显示重点信息。" />
        <StateCard title="等待第一条事件" body="系统正在建立运行流。" advice="焦点详情会在首个事件到达后同步更新。" />
      </aside>
    </div>
  );
}

function FailedInvestigationState(props: { submitError: string | null }) {
  return (
    <div id="investigation-panel-body" className="bottom-panel-body investigation-board">
      <section className="investigation-lane">
        <LaneHeader title="事件流" text="当前没有进入事件流。" />
        <StateCard title={readRunStateHeadline("failed")} body={readRunStateBody({ runState: "failed", submitError: props.submitError })} advice={readRunStateNextStep({ runState: "failed" })} />
      </section>
      <aside className="inspection-lane">
        <LaneHeader title="焦点详情" text="恢复可提交状态后再继续查看过程。" />
        <StateCard title="下一步建议" body="先恢复提交链路。" advice={readRunStateNextStep({ runState: "failed" })} />
      </aside>
    </div>
  );
}

function InlineInvestigationFailure(props: { submitError: string | null }) {
  return <StateCard title={readRunStateHeadline("failed")} body={readRunStateBody({ runState: "failed", submitError: props.submitError })} advice="当前仍显示上一轮历史事件，可继续查看已有过程。" />;
}

function InspectionFocusCard(props: { model: ReturnType<typeof buildInvestigationModel> }) {
  return (
    <div className="detail-card inspection-focus-card">
      <strong>{props.model.inspection.title}</strong>
      <p>{props.model.inspection.copy}</p>
      <div className="sidebar-inline-meta">
        <span>{props.model.autoFollow ? "自动跟随" : "手动焦点"}</span>
        <span>{props.model.focusEvent?.stage || "当前焦点"}</span>
      </div>
    </div>
  );
}

function InspectionMetaGrid(props: { model: ReturnType<typeof buildInvestigationModel> }) {
  return <MetaGrid items={props.model.inspection.meta} />;
}

function InspectionNextCard(props: { model: ReturnType<typeof buildInvestigationModel> }) {
  return (
    <div className="detail-card muted-card">
      <strong>下一步建议</strong>
      <ul>{props.model.inspection.next.map((item) => <li key={item}>{item}</li>)}</ul>
    </div>
  );
}

function StateCard(props: { title: string; body: string; advice: string }) {
  return (
    <div className="detail-card state-card">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
      <p>{props.advice}</p>
    </div>
  );
}

function LaneHeader(props: { title: string; text: string }) {
  return <SectionHeader title={props.title} description={props.text} />;
}

function getEventPreview(focusEvent: RunEvent | null, autoFollow: boolean) {
  if (!focusEvent) return { title: "等待事件", summary: "任务开始后，这里会出现当前会话的过程推进。", meta: ["当前会话"] };
  return {
    title: focusEvent.tool_display_name || focusEvent.event_type,
    summary: readPreviewSummary(focusEvent),
    meta: [focusEvent.stage, focusEvent.tool_category || focusEvent.source || "runtime", autoFollow ? "自动跟随" : "手动焦点"],
  };
}

function resolveFocusEvent(events: RunEvent[], selectedEventId: string, autoFollow: boolean) {
  if (autoFollow || !selectedEventId) return events[events.length - 1] || null;
  return events.find((event) => event.event_id === selectedEventId) || events[events.length - 1] || null;
}

function getEventInspection(events: RunEvent[], focusEvent: RunEvent | null) {
  if (!focusEvent) return buildEmptyInspection();
  return {
    copy: readInspectionCopy(focusEvent),
    meta: buildInspectionMeta(events, focusEvent),
    next: buildNextObservations(focusEvent),
    title: focusEvent.tool_display_name || focusEvent.event_type,
  };
}

function buildEmptyInspection() {
  return {
    copy: "任务开始后，这里会显示当前会话的调查焦点。",
    meta: [{ label: "来源", value: "当前会话" }, { label: "状态", value: "等待生成" }],
    next: ["提交任务后查看阶段推进", "观察是否出现确认请求"],
    title: "等待事件",
  };
}

function buildInspectionMeta(events: RunEvent[], focusEvent: RunEvent) {
  return [
    { label: "阶段", value: focusEvent.stage },
    { label: "来源", value: focusEvent.source || "runtime" },
    { label: "动作", value: readToolLabel(focusEvent) },
    { label: "摘要", value: focusEvent.result_summary || "未附带" },
    { label: "验证", value: focusEvent.verification_snapshot?.summary || "未附带" },
    { label: "产物", value: focusEvent.artifact_path || "未附带" },
    { label: "Run ID", value: focusEvent.run_id },
    { label: "事件数", value: String(events.length) },
  ];
}

function buildNextObservations(focusEvent: RunEvent) {
  return [
    focusEvent.metadata?.next_step || (focusEvent.event_type === "confirmation_required" ? readRunStateNextStep({ runState: "awaiting_confirmation" }) : "继续观察后续阶段是否推进"),
    focusEvent.verification_snapshot?.summary ? "对照验证快照判断这一步是否已收口" : "需要稳定记录时进入日志页查看",
    focusEvent.completion_reason || "",
  ];
}

function shouldShowPendingInvestigation(runState: RunState, events: RunEvent[]) {
  return events.length === 0 && (runState === "submitting" || runState === "streaming" || runState === "resuming");
}

function shouldShowFailedInvestigation(runState: RunState, events: RunEvent[], submitError: string | null) {
  return events.length === 0 && runState === "failed" && Boolean(submitError);
}

function shouldShowInlineInvestigationFailure(runState: RunState, events: RunEvent[], submitError: string | null) {
  return events.length > 0 && runState === "failed" && Boolean(submitError);
}

function resetFocus(setSelectedEventId: (value: string) => void, setAutoFollow: (value: boolean) => void) {
  setSelectedEventId("");
  setAutoFollow(true);
}

function resumeFocus(
  latestEventId: string,
  setAutoFollow: (value: boolean) => void,
  setSelectedEventId: (value: string) => void,
) {
  setAutoFollow(true);
  setSelectedEventId(latestEventId);
}

function selectFocusEventId(
  eventId: string,
  latestEventId: string,
  setAutoFollow: (value: boolean) => void,
  setSelectedEventId: (value: string) => void,
) {
  setSelectedEventId(eventId);
  setAutoFollow(eventId === latestEventId);
}

function readSummaryStatusClass(runState: RunState, autoFollow: boolean) {
  if (runState === "failed") return "status-failed";
  if (runState === "awaiting_confirmation") return "status-awaiting";
  if (runState === "completed") return "status-completed";
  return autoFollow ? "status-running" : "status-idle";
}

function readSummaryStatusLabel(runState: RunState, autoFollow: boolean) {
  if (runState === "failed") return "执行失败";
  if (runState === "awaiting_confirmation") return "等待确认";
  if (runState === "completed") return "本轮完成";
  return autoFollow ? "自动跟随" : "手动查看";
}

function readBottomPanelSummary(
  runState: RunState,
  model: ReturnType<typeof buildInvestigationModel>,
) {
  if (model.events.length === 0) return "任务进入运行后，这里会显示事件过程。";
  if (runState === "failed" || runState === "completed" || runState === "awaiting_confirmation") {
    return readRunStateBody({ latestFailureEvent: model.focusEvent || undefined, runState });
  }
  return model.preview.summary;
}

function readPreviewSummary(event: RunEvent) {
  return event.result_summary || event.summary || event.detail || "当前事件没有额外摘要。";
}

function readInspectionCopy(event: RunEvent) {
  return event.result_summary || event.summary || event.detail || "当前事件没有额外摘要。";
}

function readToolLabel(event: RunEvent) {
  if (event.tool_display_name && event.tool_category) return `${event.tool_display_name} / ${event.tool_category}`;
  return event.tool_display_name || event.tool_name || event.tool_category || "未附带";
}
