import { cloneElement, ReactElement } from "react";
import { readRunStateBody, readRunStateHeadline, readRunStateNextStep } from "../chat/chatResultModel";
import { ConnectionState, RunState } from "../runtime/state";
import { ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";

type ContextSidebarProps = {
  settings: SettingsResponse | null;
  statusLine: string;
  runState: RunState;
  connectionState: ConnectionState;
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  events: RunEvent[];
  confirmation: ConfirmationRequest | null;
  bootstrapError: string | null;
};

const INSPECTOR_SECTION_ORDER = ["task", "action", "repo", "risk"] as const;

export function ContextSidebar(props: ContextSidebarProps) {
  const model = buildHubModel(props);
  return (
    <aside className="context-sidebar">
      {buildInspectorSections(model).map(renderInspectorSection)}
    </aside>
  );
}

function buildHubModel(props: ContextSidebarProps) {
  const latestEvent = props.events[props.events.length - 1];
  const latestToolEvent = findLatest(props.events, hasToolSignal);
  const latestMemoryEvent = findLatest(props.events, isMemoryEvent);
  const latestRepoEvent = findLatest(props.events, hasRepoSignal);
  return {
    ...props,
    currentTask: latestEvent?.metadata?.task_title || latestEvent?.summary || "等待任务",
    docPaths: splitLines(latestRepoEvent?.metadata?.doc_paths),
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
  return INSPECTOR_SECTION_ORDER.map((key) => createInspectorSection(key, model));
}

function renderInspectorSection(section: { key: string; node: ReactElement }) {
  return cloneElement(section.node, { key: section.key });
}

function createInspectorSection(key: typeof INSPECTOR_SECTION_ORDER[number], model: ReturnType<typeof buildHubModel>) {
  if (key === "task") return { key, node: <TaskGroup model={model} /> };
  if (key === "action") return { key, node: <ActionGroup model={model} /> };
  if (key === "repo") return { key, node: <RepoGroup model={model} /> };
  return { key, node: <RiskGroup model={model} /> };
}

function ActionGroup(props: { model: ReturnType<typeof buildHubModel> }) {
  return (
    <section className="sidebar-card inspector-card">
      <InspectorHeader title="下一步与最近动作" status={readActionStatus(props.model)} />
      <InfoRows rows={buildActionRows(props.model)} />
    </section>
  );
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

function buildTaskRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "运行", value: model.currentRunId || "尚未开始" },
    { label: "工作区", value: model.settings?.workspace.name || "未加载" },
    { label: "连接", value: model.connectionLabel },
  ];
}

function buildActionRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "当前状态", value: readRunStateHeadline(model.runState, model.latestEvent) },
    { label: "下一步", value: model.nextAction },
    { label: "当前动作", value: readCurrentAction(model) },
    { label: "结果摘要", value: readRecentSummary(model) },
    { label: "验证结果", value: readVerification(model) },
    { label: "产物路径", value: readArtifactPath(model) },
    { label: "阶段", value: model.latestEvent?.stage || "等待事件" },
  ];
}

function buildRepoRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "工作区根", value: readWorkspaceRoot(model) },
    { label: "仓库根", value: model.latestRepoEvent?.metadata?.repo_root || "非 Git 或未识别" },
    { label: "分支", value: model.latestRepoEvent?.metadata?.current_branch || "未识别" },
    { label: "工作树", value: getTreeState(model.latestRepoEvent) },
  ];
}

function buildRiskRows(model: ReturnType<typeof buildHubModel>) {
  return [
    { label: "风险", value: model.confirmation?.action_summary || "当前没有待确认项" },
    { label: "记忆摘要", value: readMemoryCopy(model) },
    { label: "记忆状态", value: memoryStateLabel(model.latestMemoryEvent) },
    { label: "系统", value: readSystemState(model) },
  ];
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
  if (model.confirmation) return "待确认";
  if (model.runState === "failed") return "失败";
  if (model.runState === "completed") return "已完成";
  if (model.runState === "idle") return "等待任务";
  return "处理中";
}

function readRepoStatus(model: ReturnType<typeof buildHubModel>) {
  if (!model.latestRepoEvent) return "空闲";
  return model.latestRepoEvent.metadata?.repo_context_status === "degraded" ? "降级" : "就绪";
}

function readWorkspaceRoot(model: ReturnType<typeof buildHubModel>) {
  return model.latestRepoEvent?.metadata?.workspace_root || model.settings?.workspace.root_path || "未加载";
}

function readRiskStatus(model: ReturnType<typeof buildHubModel>) {
  if (model.confirmation) return model.confirmation.risk_level;
  return model.latestMemoryEvent ? "有记忆" : "稳定";
}

function readMemoryCopy(model: ReturnType<typeof buildHubModel>) {
  return model.latestMemoryEvent?.context_snapshot?.memory_digest || model.latestMemoryEvent?.detail || "当前没有新的记忆记录。";
}

function readCurrentAction(model: ReturnType<typeof buildHubModel>) {
  const event = model.latestToolEvent;
  if (!event) return "当前没有独立动作事件。";
  if (event.tool_display_name && event.tool_category) return `${event.tool_display_name} / ${event.tool_category}`;
  return event.tool_display_name || event.tool_name || event.tool_category || event.event_type || "当前没有独立动作事件。";
}

function readVerification(model: ReturnType<typeof buildHubModel>) {
  return model.latestEvent?.verification_snapshot?.summary || "当前没有验证摘要。";
}

function readArtifactPath(model: ReturnType<typeof buildHubModel>) {
  return model.latestToolEvent?.artifact_path || model.latestEvent?.artifact_path || "当前没有产物路径。";
}

function readSystemState(model: ReturnType<typeof buildHubModel>) {
  if (model.bootstrapError) return "初始化异常";
  if (model.settings && !model.settings.runtime_status.ok) return "Runtime 不可达";
  return model.connectionState === "connected" ? "连接正常" : model.connectionLabel;
}

function getTreeState(event?: RunEvent) {
  if (event?.metadata?.git_available !== "true") return "Git 不可用";
  return event.metadata?.git_dirty === "true" ? "有未提交修改" : "干净";
}

function memoryStateLabel(event?: RunEvent) {
  if (!event) return "暂无痕迹";
  if (event.event_type === "memory_recalled") return "已召回";
  if (event.event_type === "memory_write_skipped") return "已跳过";
  return "已记录";
}

function readInspectorStatusClass(status: string) {
  if (status === "失败" || status === "降级") return "status-failed";
  if (status === "待确认" || status === "high" || status === "medium") return "status-awaiting";
  if (status === "已完成" || status === "就绪" || status === "稳定" || status === "有记忆") return "status-completed";
  if (status === "等待任务") return "status-idle";
  if (status === "处理中") return "status-running";
  return "status-idle";
}
