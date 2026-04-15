import { useEffect, useRef, useState } from "react";

import {
  readPendingAdvice,
  readPendingBody,
  readPendingHeadline,
  readRunStateBody,
  readRunStateNextStep,
} from "../chat/chatResultModel";
import { EventTimeline } from "../events/EventTimeline";
import { readUnifiedStatusFromRunState, readUnifiedStatusMeta, RunState } from "../runtime/state";
import { RunEvent } from "../shared/contracts";
import { MetaGrid, SectionHeader } from "../ui/primitives";

type BottomPanelProps = {
  currentRunId: string;
  isOpen: boolean;
  events: RunEvent[];
  currentTaskTitle: string;
  runState: RunState;
  submitError: string | null;
  onOpenChange: (open: boolean) => void;
};

export function BottomPanel(props: BottomPanelProps) {
  const focus = useInvestigationFocus(props.events);
  const progress = useRunCycleProgress(props.runState, props.events.length);
  const model = buildInvestigationModel(props.events, focus.selectedEventId, focus.autoFollow);
  return (
    <section className={props.isOpen ? "bottom-panel open" : "bottom-panel"} aria-label="当前任务调查层">
      <BottomPanelHeader isOpen={props.isOpen} onOpenChange={props.onOpenChange} />
      <BottomPanelSummary props={props} model={model} progress={progress} />
      {props.isOpen ? <BottomPanelBody props={props} model={model} focus={focus} progress={progress} /> : null}
    </section>
  );
}

function useRunCycleProgress(runState: RunState, eventCount: number) {
  const [baselineCount, setBaselineCount] = useState<number | null>(null);
  const previousRunState = useRef(runState);
  useEffect(() => {
    const wasBusy = isBusyRunState(previousRunState.current);
    if (isBusyRunState(runState) && !wasBusy) setBaselineCount(eventCount);
    if (runState === "idle") setBaselineCount(null);
    previousRunState.current = runState;
  }, [eventCount, runState]);
  const newEventCount = baselineCount === null ? eventCount : Math.max(eventCount - baselineCount, 0);
  return {
    baselineCount,
    hasNewEvent: newEventCount > 0,
    newEventCount,
  };
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
  progress: ReturnType<typeof useRunCycleProgress>;
}) {
  const summaryStatus = readSummaryStatusClass(props.props.runState);
  const summaryLabel = readSummaryStatusLabel(props.props.runState);
  return (
    <div className="bottom-panel-summary" aria-live="polite">
      <div className="bottom-panel-summary-copy">
        <strong>{readBottomPanelHeadline(props.props, props.progress)}</strong>
        <p>{readBottomPanelSummary(props.props, props.progress)}</p>
      </div>
      <div className="bottom-panel-summary-meta">
        <span className={`status-badge ${summaryStatus}`}>{summaryLabel}</span>
        <span className={`status-badge ${props.model.autoFollow ? "status-running" : "status-idle"}`}>{props.model.autoFollow ? "自动跟随最新" : "手动查看历史"}</span>
        <span className="sidebar-chip-muted">{readEventCountLabel(props.props.events.length, props.progress)}</span>
      </div>
    </div>
  );
}

function BottomPanelBody(props: {
  props: BottomPanelProps;
  model: ReturnType<typeof buildInvestigationModel>;
  focus: ReturnType<typeof useInvestigationFocus>;
  progress: ReturnType<typeof useRunCycleProgress>;
}) {
  if (shouldShowPendingInvestigation(props.props.runState, props.progress.hasNewEvent)) {
    return (
      <PendingInvestigationState
        currentRunId={props.props.currentRunId}
        runState={props.props.runState}
        taskTitle={props.props.currentTaskTitle}
      />
    );
  }
  if (shouldShowFailedInvestigation(props.props.runState, props.progress.hasNewEvent)) {
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
    progress: ReturnType<typeof useRunCycleProgress>;
  };
}) {
  return (
    <section className="investigation-lane">
      <LaneHeader title="事件流" text="当前会话过程。" />
      <FollowModeNotice autoFollow={props.props.model.autoFollow} onResumeFollow={props.props.focus.resumeFollow} />
      {shouldShowInlineInvestigationFailure(props.props.props.runState, props.props.progress.hasNewEvent) ? (
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
      {!props.props.model.autoFollow ? <button type="button" className="secondary-button" onClick={props.props.focus.resumeFollow}>回到最新事件</button> : null}
      <InspectionFocusCard model={props.props.model} />
      <InspectionMetaGrid model={props.props.model} />
      <InspectionNextCard model={props.props.model} />
    </aside>
  );
}

function PendingInvestigationState(props: {
  currentRunId: string;
  taskTitle: string;
  runState: RunState;
}) {
  const pendingBody = readPendingBody({ currentRunId: props.currentRunId, taskTitle: props.taskTitle });
  return (
    <div id="investigation-panel-body" className="bottom-panel-body investigation-board">
      <section className="investigation-lane">
        <LaneHeader title="事件流" text={pendingBody} />
        <StateCard title={readPendingHeadline(props.runState)} body={pendingBody} advice={readPendingAdvice(props.runState)} />
      </section>
      <aside className="inspection-lane">
        <LaneHeader title="焦点详情" text="首个事件返回前，焦点区保留等待态。" />
        <StateCard title="等待首个事件" body="系统正在建立本轮任务的事件流。" advice="收到首个事件后将自动切换到最新焦点。" />
      </aside>
    </div>
  );
}

function FailedInvestigationState(props: { submitError: string | null }) {
  return (
    <div id="investigation-panel-body" className="bottom-panel-body investigation-board">
      <section className="investigation-lane">
        <LaneHeader title="事件流" text="当前没有进入事件流。" />
        <StateCard title="任务失败且无事件" body={readRunStateBody({ runState: "failed", submitError: props.submitError })} advice="建议先检查运行时连接和任务输入，再重新提交任务。" />
      </section>
      <aside className="inspection-lane">
        <LaneHeader title="焦点详情" text="本轮尚未形成可调查事件，先完成恢复再继续。" />
        <StateCard title="恢复建议" body="排查模型可用性、工作区权限和网络后重试。" advice={readRunStateNextStep({ runState: "failed" })} />
      </aside>
    </div>
  );
}

function InlineInvestigationFailure(props: { submitError: string | null }) {
  return <StateCard title={readUnifiedStatusMeta("failed").label} body={readRunStateBody({ runState: "failed", submitError: props.submitError })} advice="当前仍显示上一轮历史事件，可继续查看已有过程。" />;
}

function FollowModeNotice(props: { autoFollow: boolean; onResumeFollow: () => void }) {
  if (props.autoFollow) {
    return <StateCard title="跟随模式：自动跟随" body="新事件到达时，时间线和焦点会自动跳到最新事件。" advice="如果要阅读历史事件，点击旧事件或滚动离开最新位置即可切换为手动查看。" />;
  }
  return (
    <div className="detail-card state-card">
      <strong>跟随模式：手动查看</strong>
      <p>当前焦点不会被新事件抢走，你可以稳定阅读历史节点。</p>
      <p>查看完历史后可一键返回最新事件。</p>
      <button type="button" className="secondary-button" onClick={props.onResumeFollow}>回到最新事件</button>
    </div>
  );
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
    { label: "执行态", value: focusEvent.activity_state || "未附带" },
    { label: "等待原因", value: focusEvent.waiting_reason || "未附带" },
    { label: "失败分流", value: focusEvent.failure_route || "未附带" },
    { label: "卡住秒数", value: focusEvent.stall_seconds || "0" },
    { label: "产物", value: focusEvent.artifact_path || "未附带" },
    { label: "原文引用", value: focusEvent.raw_output_ref || "未附带" },
    { label: "证据引用", value: focusEvent.evidence_ref || "未附带" },
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

function shouldShowPendingInvestigation(runState: RunState, hasNewEvent: boolean) {
  return isBusyRunState(runState) && !hasNewEvent;
}

function shouldShowFailedInvestigation(runState: RunState, hasNewEvent: boolean) {
  return runState === "failed" && !hasNewEvent;
}

function shouldShowInlineInvestigationFailure(runState: RunState, hasNewEvent: boolean) {
  return runState === "failed" && hasNewEvent;
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

function readSummaryStatusClass(runState: RunState) {
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(runState)).className;
}

function readSummaryStatusLabel(runState: RunState) {
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(runState)).label;
}

function readBottomPanelSummary(
  props: BottomPanelProps,
  progress: ReturnType<typeof useRunCycleProgress>,
) {
  if (props.runState === "failed" && !progress.hasNewEvent) {
    return readRunStateBody({ runState: "failed", submitError: props.submitError });
  }
  if (isBusyRunState(props.runState) && !progress.hasNewEvent) {
    return readPendingBody({ currentRunId: props.currentRunId, taskTitle: props.currentTaskTitle });
  }
  if (props.events.length === 0) return "任务进入运行后，这里会显示事件过程。";
  if (props.runState === "failed" || props.runState === "completed" || props.runState === "awaiting_confirmation") {
    return readRunStateBody({ latestFailureEvent: props.events[props.events.length - 1], runState: props.runState });
  }
  return readPreviewSummary(props.events[props.events.length - 1]);
}

function readBottomPanelHeadline(props: BottomPanelProps, progress: ReturnType<typeof useRunCycleProgress>) {
  if (props.runState === "idle") return readUnifiedStatusMeta("idle").label;
  if (props.runState === "archived") return "已归档";
  if (props.runState === "failed" && !progress.hasNewEvent) return readUnifiedStatusMeta("failed").label;
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(props.runState)).label;
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

function readEventCountLabel(totalCount: number, progress: ReturnType<typeof useRunCycleProgress>) {
  if (progress.baselineCount === null) return `${totalCount} 条事件`;
  return `${progress.newEventCount} 条本轮事件`;
}

function isBusyRunState(runState: RunState) {
  return runState === "submitting" || runState === "streaming" || runState === "resuming";
}
