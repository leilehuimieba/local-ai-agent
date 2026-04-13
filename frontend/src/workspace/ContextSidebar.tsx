import { cloneElement, ReactElement } from "react";
import { readRunStateBody, readRunStateNextStep } from "../chat/chatResultModel";
import { readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel } from "../history/logType";
import { ConnectionState, readUnifiedStatusFromRunState, readUnifiedStatusMeta, RunState } from "../runtime/state";
import { ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";

type ContextSidebarProps = {
  settings: SettingsResponse | null;
  statusLine: string;
  variant: "home" | "task";
  runState: RunState;
  connectionState: ConnectionState;
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  events: RunEvent[];
  confirmation: ConfirmationRequest | null;
  bootstrapError: string | null;
};
type InspectorSectionKey = "task" | "action" | "context" | "repo" | "risk";

export function ContextSidebar(props: ContextSidebarProps) {
  const model = buildHubModel(props);
  return (
    <aside className={readSidebarClass(props.variant)}>
      {buildInspectorSections(model).map(renderInspectorSection)}
    </aside>
  );
}

function readSidebarClass(variant: ContextSidebarProps["variant"]) {
  if (variant === "task") return "context-sidebar context-sidebar-compact";
  return "context-sidebar";
}

function buildHubModel(props: ContextSidebarProps) {
  const latestEvent = props.events[props.events.length - 1];
  const latestContextEvent = findLatest(props.events, hasContextSignal);
  const latestToolEvent = findLatest(props.events, hasToolSignal);
  const latestMemoryEvent = findLatest(props.events, isMemoryEvent);
  const latestRepoEvent = findLatest(props.events, hasRepoSignal);
  return {
    ...props,
    currentTask: latestEvent?.metadata?.task_title || latestEvent?.summary || "等待任务",
    docPaths: splitLines(latestRepoEvent?.metadata?.doc_paths),
    latestContextEvent,
    latestEvent,
    latestMemoryEvent,
    latestRepoEvent,
    latestToolEvent,
    nextAction: getNextAction(props.runState, latestEvent, props.confirmation),
    repoWarnings: splitLines(latestRepoEvent?.metadata?.repo_context_warnings),
  };
}

function TaskGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title="当前任务" status={props.model.statusLine} />
      <p className="sidebar-title">{props.model.currentTask}</p>
      <InfoRows rows={buildTaskRows(props.model)} />
    </section>
  );
}

function buildInspectorSections(model: ReturnType<typeof buildHubModel>) {
  return readInspectorSectionOrder(model.variant).map((key) => createInspectorSection(key, model));
}

function renderInspectorSection(section: { key: string; node: ReactElement }) {
  return cloneElement(section.node, { key: section.key });
}

function createInspectorSection(key: InspectorSectionKey, model: ReturnType<typeof buildHubModel>) {
  if (key === "task") return { key, node: <TaskGroup model={model} /> };
  if (key === "action") return { key, node: <ActionGroup model={model} /> };
  if (key === "context") return { key, node: <ContextGroup model={model} /> };
  if (key === "repo") return { key, node: <RepoGroup model={model} /> };
  return { key, node: <RiskGroup model={model} /> };
}

function readInspectorSectionOrder(variant: ContextSidebarProps["variant"]) {
  if (variant === "home") return ["task", "context", "risk"] as InspectorSectionKey[];
  return ["action", "context", "risk"] as InspectorSectionKey[];
}

function ActionGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title={readActionGroupTitle(props.model.variant)} status={readActionStatus(props.model)} />
      <InfoRows rows={buildActionRows(props.model)} />
      <ActionNotes model={props.model} />
    </section>
  );
}

function readActionGroupTitle(variant: ContextSidebarProps["variant"]) {
  return variant === "task" ? "关键动作与下一步" : "下一步与最近动作";
}

function ContextGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title={readContextGroupTitle(props.model.variant)} status={readContextStatus(props.model)} />
      <InfoRows rows={buildContextRows(props.model)} />
    </section>
  );
}

function readContextGroupTitle(variant: ContextSidebarProps["variant"]) {
  return variant === "task" ? "状态沉淀" : "状态沉淀与续接依据";
}

function RepoGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title="仓库上下文" status={readRepoStatus(props.model)} />
      <InfoRows rows={buildRepoRows(props.model)} />
      <RepoPathList items={props.model.docPaths} />
      {props.model.repoWarnings.length > 0 ? <InlineNote title="降级说明" text={props.model.repoWarnings.join("；")} /> : null}
    </section>
  );
}

function RiskGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title="风险与记忆" status={readRiskStatus(props.model)} />
      <InfoRows rows={buildRiskRows(props.model)} />
      {props.model.bootstrapError ? <InlineError title="初始化异常" body={props.model.bootstrapError} /> : null}
    </section>
  );
}

function InspectorHeader(props: { title: string; status: string }) {
  return (
    <div className="section-head">
      <div>
        <span className="section-kicker">检查器</span>
        <h3>{props.title}</h3>
      </div>
      <span className={`status-badge ${readInspectorStatusClass(props.status)}`}>{props.status}</span>
    </div>
  );
}

function InfoRows(props: { rows: Array<{ label: string; value: string }> }) {
  return <div className="detail-list">{props.rows.map((row) => <Row key={row.label} label={row.label} value={row.value} />)}</div>;
}

function Row(props: { label: string; value: string }) {
  return (
    <div className="sidebar-row">
      <strong>{props.label}</strong>
      <span title={props.value}>{props.value}</span>
    </div>
  );
}

function RepoPathList(props: { items: string[] }) {
  if (props.items.length === 0) return null;
  return (
    <div className="repo-path-list">
      {props.items.map((item) => (
        <span key={item}>
          <strong>命中文档</strong>
          <code>{item}</code>
        </span>
      ))}
    </div>
  );
}

function InlineNote(props: { title: string; text: string }) {
  return (
    <div className="inline-note">
      <strong>{props.title}</strong>
      <p>{props.text}</p>
    </div>
  );
}

function InlineError(props: { title: string; body: string }) {
  return (
    <div className="error">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}

function ActionNotes(props: { model: ReturnType<typeof buildHubModel> }) {
  const evidence = readActionEvidence(props.model);
  if (props.model.variant === "task") {
    return evidence ? <InlineNote title="最近收口依据" text={evidence} /> : null;
  }
  return (
    <>
      <InlineNote title="建议先接这里" text={props.model.nextAction} />
      {evidence ? <InlineNote title="最近收口依据" text={evidence} /> : null}
    </>
  );
}

function ContextHighlights(props: { rows: Array<{ label: string; value: string }> }) {
  if (props.rows.length === 0) return null;
  return (
    <div className="context-highlight-grid">
      {props.rows.map((row) => <ContextHighlightCard key={row.label} row={row} />)}
    </div>
  );
}

function ContextHighlightCard(props: { row: { label: string; value: string } }) {
  return (
    <article className="detail-card context-highlight-card">
      <strong>{props.row.label}</strong>
      <p>{props.row.value}</p>
    </article>
  );
}

function buildTaskRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "运行", value: model.currentRunId || "尚未开始" },
    { label: "工作区", value: model.settings?.workspace.name || "未加载" },
    { label: "连接", value: model.connectionLabel },
  ];
}

function buildActionRows(model: ReturnType<typeof buildHubModel>) {
  if (model.variant === "task") {
    const rows = [
      { label: "当前状态", value: readTaskStatusLabel(model) },
      { label: "当前动作", value: readCurrentAction(model) },
      { label: "下一步线索", value: model.nextAction },
    ];
    if (hasVerificationSummary(model)) rows.push({ label: "最近验证", value: readVerification(model) });
    return rows;
  }
  return [
    { label: "当前状态", value: readTaskStatusLabel(model) },
    { label: "当前阶段", value: model.latestEvent?.stage || "等待事件" },
    { label: "下一步线索", value: model.nextAction },
    { label: "验证依据", value: readVerification(model) },
    { label: "当前动作", value: readCurrentAction(model) },
  ];
}

function readTaskStatusLabel(model: ReturnType<typeof buildHubModel>) {
  if (model.confirmation) return readUnifiedStatusMeta("awaiting_confirmation").label;
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(model.runState)).label;
}

function buildRepoRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "工作区根", value: readWorkspaceRoot(model) },
    { label: "仓库根", value: model.latestRepoEvent?.metadata?.repo_root || "非 Git 或未识别" },
    { label: "分支", value: model.latestRepoEvent?.metadata?.current_branch || "未识别" },
    { label: "工作树", value: getTreeState(model.latestRepoEvent) },
  ];
}

function buildContextRows(model: ReturnType<typeof buildHubModel>) {
  const snapshot = model.latestContextEvent?.context_snapshot;
  if (model.variant === "task") {
    return [
      { label: "沉淀依据", value: readContextEvidence(snapshot) },
      { label: "思考线索", value: snapshot?.reasoning_summary || snapshot?.assembly_profile || "当前没有额外思考线索。" },
    ];
  }
  return [
    { label: "会话续接", value: snapshot?.session_summary || "当前没有会话摘要。" },
    { label: "沉淀依据", value: readContextEvidence(snapshot) },
    { label: "思考线索", value: snapshot?.reasoning_summary || snapshot?.assembly_profile || "当前没有额外思考线索。" },
  ];
}

function buildRiskRows(model: ReturnType<typeof buildHubModel>) {
  if (model.variant === "task") {
    const rows = [
      { label: "当前阻塞", value: model.confirmation?.action_summary || "当前没有待确认项或显式阻塞。" },
      { label: "系统", value: readSystemState(model) },
    ];
    if (model.latestMemoryEvent) rows.push({ label: "最近记忆", value: readMemoryCopy(model) });
    return rows;
  }
  const rows = [
    { label: "当前阻塞", value: model.confirmation?.action_summary || "当前没有待确认项或显式阻塞。" },
    { label: "系统", value: readSystemState(model) },
    { label: "最近验证", value: readVerification(model) },
  ];
  return model.latestMemoryEvent
    ? [...rows, { label: "最近记忆", value: readMemoryCopy(model) }]
    : rows;
}

function findLatest(events: RunEvent[], predicate: (event: RunEvent) => boolean) {
  return [...events].reverse().find(predicate);
}

function hasToolSignal(event: RunEvent) {
  return Boolean(event.tool_name || event.tool_display_name || event.tool_category || event.result_summary || event.artifact_path || event.metadata?.tool_name);
}

function isMemoryEvent(event: RunEvent) {
  return event.event_type === "memory_written" || event.event_type === "memory_recalled" || event.event_type === "memory_write_skipped";
}

function hasContextSignal(event: RunEvent) {
  return Boolean(
    event.context_snapshot?.session_summary ||
      event.context_snapshot?.memory_digest ||
      event.context_snapshot?.knowledge_digest ||
      event.context_snapshot?.reasoning_summary,
  );
}

function hasRepoSignal(event: RunEvent) {
  return Boolean(event.metadata?.repo_context_status || event.metadata?.workspace_root);
}

function splitLines(value?: string) {
  return value?.split("\n").filter(Boolean) || [];
}

function getNextAction(runState: RunState, event?: RunEvent, confirmation?: ConfirmationRequest | null) {
  if (confirmation) return readRunStateNextStep({ runState: "awaiting_confirmation" });
  if (!event) return readRunStateNextStep({ runState });
  if (runState === "streaming" || runState === "resuming") return readRunStateNextStep({ latestEvent: event, runState });
  if (runState === "completed") return readRunStateNextStep({ latestEvent: event, runState });
  if (runState === "failed") return readRunStateNextStep({ latestFailureEvent: event, runState });
  return event.metadata?.next_step || readRunStateNextStep({ latestEvent: event, runState });
}

function readRecentSummary(model: ReturnType<typeof buildHubModel>) {
  if (model.runState === "awaiting_confirmation") {
    return readRunStateBody({ runState: "awaiting_confirmation" });
  }
  if (model.runState === "failed") {
    return readRunStateBody({ latestFailureEvent: model.latestEvent, runState: "failed", submitError: null });
  }
  return model.latestToolEvent?.result_summary || model.latestToolEvent?.detail || "最近动作还没有独立摘要。";
}

function readActionStatus(model: ReturnType<typeof buildHubModel>) {
  if (model.confirmation) return readUnifiedStatusMeta("awaiting_confirmation").label;
  if (model.runState === "failed") return readUnifiedStatusMeta("failed").label;
  if (model.runState === "completed") return readUnifiedStatusMeta("completed").label;
  if (model.runState === "idle") return readUnifiedStatusMeta("idle").label;
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(model.runState)).label;
}

function readRepoStatus(model: ReturnType<typeof buildHubModel>) {
  if (!model.latestRepoEvent) return readUnifiedStatusMeta("idle").label;
  return model.latestRepoEvent.metadata?.repo_context_status === "degraded" ? "降级" : "就绪";
}

function readContextStatus(model: ReturnType<typeof buildHubModel>) {
  const snapshot = model.latestContextEvent?.context_snapshot;
  if (!snapshot) return readUnifiedStatusMeta("idle").label;
  if (snapshot.knowledge_digest || snapshot.memory_digest) return "已沉淀";
  if (snapshot.session_summary || snapshot.reasoning_summary) return "有上下文";
  return readUnifiedStatusMeta("idle").label;
}

function readWorkspaceRoot(model: ReturnType<typeof buildHubModel>) {
  return model.latestRepoEvent?.metadata?.workspace_root || model.settings?.workspace.root_path || "未加载";
}

function readRiskStatus(model: ReturnType<typeof buildHubModel>) {
  if (model.confirmation) return model.confirmation.risk_level;
  if (model.latestMemoryEvent) return readMemoryGovernance(model.latestMemoryEvent);
  return "稳定";
}

function readMemoryCopy(model: ReturnType<typeof buildHubModel>) {
  return model.latestMemoryEvent?.detail || model.latestMemoryEvent?.summary || model.latestMemoryEvent?.context_snapshot?.memory_digest || "当前没有新的记忆记录。";
}

function readCurrentAction(model: ReturnType<typeof buildHubModel>) {
  const event = model.latestToolEvent;
  if (!event) return "当前没有独立动作事件。";
  if (event.tool_display_name && event.tool_category) return `${event.tool_display_name} / ${event.tool_category}`;
  return event.tool_display_name || event.tool_name || event.tool_category || event.event_type || "当前没有独立动作事件。";
}

function readActionEvidence(model: ReturnType<typeof buildHubModel>) {
  const parts = [readRecentSummary(model), readArtifactPath(model)].filter((item) => item && !item.startsWith("当前没有"));
  return parts.join("；");
}

function readVerification(model: ReturnType<typeof buildHubModel>) {
  return model.latestEvent?.verification_snapshot?.summary || "当前没有验证摘要。";
}

function hasVerificationSummary(model: ReturnType<typeof buildHubModel>) {
  return Boolean(model.latestEvent?.verification_snapshot?.summary);
}

function readArtifactPath(model: ReturnType<typeof buildHubModel>) {
  return model.latestToolEvent?.artifact_path || model.latestEvent?.artifact_path || "当前没有产物路径。";
}

function readContextEvidence(snapshot?: RunEvent["context_snapshot"]) {
  const parts = [snapshot?.memory_digest, snapshot?.knowledge_digest].filter(Boolean);
  return parts.length > 0 ? parts.join("；") : "当前没有记忆或知识沉淀。";
}

function readSystemState(model: ReturnType<typeof buildHubModel>) {
  if (model.bootstrapError) return "初始化异常";
  if (model.settings && !model.settings.runtime_status.ok) return "运行时不可达";
  return model.connectionState === "connected" ? "连接正常" : model.connectionLabel;
}

function getTreeState(event?: RunEvent) {
  if (event?.metadata?.git_available !== "true") return "Git 不可用";
  return event.metadata?.git_dirty === "true" ? "有未提交修改" : "干净";
}

function readMemoryActivity(event?: RunEvent) {
  if (!event) return "暂无动作";
  return readMemoryActivityLabel(event);
}

function readMemoryFacet(event?: RunEvent) {
  if (!event) return "无记忆";
  return readMemoryFacetLabel(eventLikeMemory(event));
}

function readMemoryGovernance(event?: RunEvent) {
  if (!event) return "暂无治理";
  return readMemoryGovernanceLabel(eventLikeMemory(event));
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

function readInspectorStatusClass(status: string) {
  if (status === "失败" || status === "降级") return "status-failed";
  if (status === "待确认" || status === "high" || status === "medium" || status === "low") return "status-awaiting";
  if (status === "完成" || status === "已完成" || status === "就绪" || status === "稳定" || status === "已验证" || status === "已归档") return "status-completed";
  if (status === "待治理" || status === "已跳过") return "status-awaiting";
  if (status === "等待任务" || status === "等待中") return "status-idle";
  if (status === "处理中" || status === "进行中") return "status-running";
  return "status-idle";
}
