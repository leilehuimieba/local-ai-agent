import { useState } from "react";
import { countMemoryFacets, readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel } from "../history/logType";
import { ResourcesEntrySection } from "../resources/components";
import { readUnifiedStatusFromLabel, readUnifiedStatusMeta } from "../runtime/state";
import { ExternalConnectionSlot, MemoryEntry, ProviderSettingsResponse, SettingsResponse } from "../shared/contracts";
import { EmptyStateBlock, MetaGrid, SectionHeader, StatusPill } from "../ui/primitives";
import { exportRunLogs, exportSettingsSnapshot, openDiagnosticsSnapshot } from "./api";
import { ProviderCredentialsSection } from "./ProviderCredentialsSection";
import { ProviderActionState, SettingsActionFeedback, SettingsActionKind } from "./useSettings";

type SettingsModulesProps = {
  settings: SettingsResponse;
  providerSettings: ProviderSettingsResponse | null;
  providerBootstrapError: string | null;
  providerActions: Record<string, ProviderActionState>;
  isRunning: boolean;
  pendingAction: SettingsActionFeedback | null;
  actionError: SettingsActionFeedback | null;
  lastSuccess: SettingsActionFeedback | null;
  memories: MemoryEntry[];
  memoryError: string | null;
  memoryPendingAction: { action: "refresh" | "delete"; message: string } | null;
  memoryActionError: { action: "refresh" | "delete"; message: string } | null;
  memoryActionSuccess: { action: "refresh" | "delete"; message: string } | null;
  deletingMemoryId: string;
  onModeChange: (mode: string) => void;
  onModelChange: (modelKey: string) => void;
  onWorkspaceChange: (workspaceId: string) => void;
  onDirectoryPromptEnabledChange: (enabled: boolean) => void;
  onShowRiskLevelChange: (enabled: boolean) => void;
  onRevokeDirectoryApproval: (rootPath: string) => void;
  onRunExternalConnectionAction: (slotId: string, action: "validate" | "recheck") => void;
  onCheckDiagnostics: () => void;
  onRefreshProviderSettings: () => Promise<unknown>;
  onTestProvider: (providerId: string, apiKey: string, baseURL?: string) => Promise<unknown>;
  onSaveProvider: (providerId: string, apiKey: string) => Promise<unknown>;
  onApplyProvider: (providerId: string) => Promise<unknown>;
  onRemoveProvider: (providerId: string) => Promise<unknown>;
  onDeleteMemory: (memoryId: string) => void;
  onRefreshMemories: () => void;
  isActionPending: (action: SettingsActionKind) => boolean;
};

const SETTINGS_MODULE_ORDER = ["runtime", "model", "provider", "workspace", "risk", "resources", "diagnostics"] as const;
type DiagnosticsActionKey = "logs" | "settings" | "snapshot";
type DiagnosticsFeedback = { tone: "running" | "failed" | "completed"; detail: string };

export function SettingsModules(props: SettingsModulesProps) {
  return <div className="settings-stack">{buildSettingsModules(props).map((item) => item.node)}</div>;
}

export function SettingsEmptyState() {
  return <EmptyStateBlock compact title="正在读取设置" text="环境状态返回后，这里会显示可用控制项。" />;
}

function buildSettingsModules(props: SettingsModulesProps) {
  return SETTINGS_MODULE_ORDER.map((key) => createSettingsModule(key, props));
}

function createSettingsModule(key: typeof SETTINGS_MODULE_ORDER[number], props: SettingsModulesProps) {
  if (key === "runtime") return { key, node: <RuntimeModule key={key} settings={props.settings} /> };
  if (key === "model") return { key, node: <ModelModule key={key} props={props} /> };
  if (key === "provider") return { key, node: <ProviderModule key={key} props={props} /> };
  if (key === "workspace") return { key, node: <WorkspaceModule key={key} props={props} /> };
  if (key === "risk") return { key, node: <RiskModule key={key} props={props} /> };
  if (key === "resources") return { key, node: <ResourcesModule key={key} props={props} /> };
  return { key, node: <DiagnosticsModule key={key} props={props} /> };
}

function ProviderModule(props: { props: SettingsModulesProps }) {
  return (
    <ProviderCredentialsSection
      providerSettings={props.props.providerSettings}
      providerBootstrapError={props.props.providerBootstrapError}
      providerActions={props.props.providerActions}
      onRefreshProviderSettings={props.props.onRefreshProviderSettings}
      onTestProvider={props.props.onTestProvider}
      onSaveProvider={props.props.onSaveProvider}
      onApplyProvider={props.props.onApplyProvider}
      onRemoveProvider={props.props.onRemoveProvider}
    />
  );
}

function RuntimeModule(props: { settings: SettingsResponse }) {
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="运行环境" badge={props.settings.runtime_status.ok ? "已完成" : "已断开"} />
      <MetaGrid items={buildRuntimeRows(props.settings)} />
    </section>
  );
}

function ModelModule(props: { props: SettingsModulesProps }) {
  const badge = readControlBadge(props.props, ["model", "mode"]);
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="模型与模式" badge={badge} />
      <ModelControls props={props.props} />
      <ActionHint props={props.props} actions={["model", "mode"]} />
      <ModelSummary settings={props.props.settings} />
    </section>
  );
}

function ModelControls(props: { props: SettingsModulesProps }) {
  const modelPending = props.props.isActionPending("model");
  const modePending = props.props.isActionPending("mode");
  return (
    <div className="settings-control-grid">
      <label className="control-field">
        <span>当前模型</span>
        <select value={readModelValue(props.props.settings)} disabled={modelPending} onChange={(event) => props.props.onModelChange(event.target.value)}>
          {props.props.settings.available_models.map(renderModelOption)}
        </select>
      </label>
      <label className="control-field">
        <span>运行模式</span>
        <select value={props.props.settings.mode} disabled={modePending} onChange={(event) => props.props.onModeChange(event.target.value)}>
          <option value="observe">观察模式</option>
          <option value="standard">标准模式</option>
          <option value="full_access">全权限模式</option>
        </select>
      </label>
    </div>
  );
}

function ModelSummary(props: { settings: SettingsResponse }) {
  return (
    <div className="settings-control-grid">
      <div className="detail-card">
        <strong>当前服务方</strong>
        <p>{props.settings.model.provider_id}</p>
      </div>
      <div className="detail-card muted-card">
        <strong>当前模式</strong>
        <p>{modeDescription(props.settings.mode)}</p>
      </div>
    </div>
  );
}

function WorkspaceModule(props: { props: SettingsModulesProps }) {
  const badge = readControlBadge(props.props, ["workspace", "revokeApproval"]);
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="工作区与授权" badge={badge} />
      <WorkspaceControlGrid props={props.props} />
      <ActionHint props={props.props} actions={["workspace", "revokeApproval"]} />
      <ApprovalList props={props.props} />
    </section>
  );
}

function WorkspaceControlGrid(props: { props: SettingsModulesProps }) {
  const workspacePending = props.props.isActionPending("workspace");
  return (
    <div className="settings-control-grid">
      <label className="control-field">
        <span>当前工作区</span>
        <select value={props.props.settings.workspace.workspace_id} disabled={workspacePending} onChange={(event) => props.props.onWorkspaceChange(event.target.value)}>
          {props.props.settings.available_workspaces.map(renderWorkspaceOption)}
        </select>
      </label>
      <div className="detail-card">
        <strong>当前路径</strong>
        <p className="workspace-root">{props.props.settings.workspace.root_path}</p>
      </div>
    </div>
  );
}

function ApprovalList(props: { props: SettingsModulesProps }) {
  const approvals = props.props.settings.approved_directories;
  return (
    <div className="settings-subsection">
      <strong>授权目录</strong>
      {approvals.length === 0 ? <p className="workspace-root">当前没有额外记住的目录授权。</p> : null}
      {approvals.length > 0 ? <div className="approval-list">{approvals.map((directory) => <ApprovalItem key={directory.root_path} props={props.props} name={directory.name} rootPath={directory.root_path} />)}</div> : null}
    </div>
  );
}

function ApprovalItem(props: {
  props: SettingsModulesProps;
  name: string;
  rootPath: string;
}) {
  const pending = props.props.isActionPending("revokeApproval");
  return (
    <div className="approval-item">
      <div>
        <strong>{props.name}</strong>
        <div className="workspace-root">{props.rootPath}</div>
      </div>
      <button type="button" className="secondary-button" disabled={pending} onClick={() => props.props.onRevokeDirectoryApproval(props.rootPath)}>
        移除授权
      </button>
    </div>
  );
}

function RiskModule(props: { props: SettingsModulesProps }) {
  const badge = readControlBadge(props.props, ["directoryPrompt", "riskLevel"]);
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="风险与权限" badge={badge} />
      <div className="settings-control-grid">
        <ToggleTile title="新目录首次接触提醒" description="进入新目录时，先提示授权边界。" checked={props.props.settings.directory_prompt_enabled} isRunning={props.props.isActionPending("directoryPrompt")} onChange={props.props.onDirectoryPromptEnabledChange} />
        <ToggleTile title="显示风险等级" description="在确认流中展示风险等级。" checked={props.props.settings.show_risk_level} isRunning={props.props.isActionPending("riskLevel")} onChange={props.props.onShowRiskLevelChange} />
      </div>
      <ActionHint props={props.props} actions={["directoryPrompt", "riskLevel"]} />
    </section>
  );
}

function ResourcesModule(props: { props: SettingsModulesProps }) {
  const actionState = readMemoryActionState(props.props);
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="记忆与资源" badge={props.props.settings.memory_policy.enabled ? "已启用" : "未启用"} />
      <MemoryOverviewCards memories={props.props.memories} />
      <ResourcesEntrySection
        actionState={actionState}
        deletingId={props.props.deletingMemoryId}
        settings={props.props.settings}
        memories={props.props.memories}
        error={props.props.memoryError}
        isRunning={props.props.isRunning}
        isRefreshing={props.props.memoryPendingAction?.action === "refresh"}
        onDeleteMemory={props.props.onDeleteMemory}
        onRefreshMemories={props.props.onRefreshMemories}
      />
      <ExternalConnectionsSection props={props.props} />
    </section>
  );
}

function MemoryOverviewCards(props: { memories: MemoryEntry[] }) {
  const latest = props.memories[0];
  return (
    <div className="settings-control-grid">
      <div className="detail-card muted-card">
        <strong>偏好与教训摘要</strong>
        <MetaGrid items={buildMemoryOverviewRows(props.memories)} />
      </div>
      <div className="detail-card">
        <strong>最近记忆动作</strong>
        <p>{latest ? `${readMemoryActivityLabel(latest)} / ${readMemoryFacetLabel(latest)}` : "当前没有新的记忆动作。"}</p>
        <p>{latest ? `治理状态：${readMemoryGovernanceLabel(latest)}` : "治理状态会在写入、召回和归档后更新。"}</p>
      </div>
    </div>
  );
}

function ExternalConnectionsSection(props: { props: SettingsModulesProps }) {
  const slots = props.props.settings.external_connections || [];
  return (
    <div className="settings-subsection">
      <strong>外部连接状态</strong>
      <ActionHint props={props.props} actions={["externalConnection"]} />
      <div className="memory-list">
        {slots.length === 0 ? <PlaceholderRow title="无外部连接" text="当前设置响应没有返回额外外部连接条目。" /> : null}
        {slots.map((slot) => <ExternalConnectionItem key={slot.slot_id} props={props.props} slot={slot} />)}
      </div>
    </div>
  );
}

function ExternalConnectionItem(props: {
  props: SettingsModulesProps;
  slot: SettingsResponse["external_connections"][number];
}) {
  const model = buildExternalConnectionModel(props.slot);
  const pending = props.props.isActionPending("externalConnection");
  return (
    <div className="memory-item">
      <div className="memory-item-head">
        <div>
          <strong>{`${props.slot.priority}. ${props.slot.display_name}`}</strong>
          <p>{model.summary}</p>
        </div>
        <StatusPill className={model.statusClass} label={model.statusLabel} />
      </div>
      <MetaGrid items={buildExternalConnectionRows(props.slot, model)} />
      <p>{props.slot.boundary}</p>
      <p>{model.nextStep}</p>
      {model.action ? (
        <button
          type="button"
          className="secondary-button"
          disabled={pending}
          onClick={() => props.props.onRunExternalConnectionAction(props.slot.slot_id, model.action!)}
        >
          {pending ? "处理中" : model.actionLabel}
        </button>
      ) : null}
    </div>
  );
}

function DiagnosticsModule(props: { props: SettingsModulesProps }) {
  return (
    <section className="settings-module control-module">
      <ModuleHeader title="诊断与导出" badge={readDiagnosticsBadge(props.props)} />
      <DiagnosticsHealthSummary settings={props.props.settings} />
      <DiagnosticsActions props={props.props} />
      <MetaGrid items={buildDiagnosticsRows(props.props.settings)} />
      <DiagnosticsAlerts settings={props.props.settings} />
      <DiagnosticsPathCard settings={props.props.settings} />
    </section>
  );
}

function DiagnosticsActions(props: { props: SettingsModulesProps }) {
  const [pendingAction, setPendingAction] = useState<DiagnosticsActionKey | "">("");
  const [feedback, setFeedback] = useState<DiagnosticsFeedback | null>(null);
  return (
    <>
      <div className="settings-control-grid">
        <DiagnosticsActionButton title="重新检测" description="调用后端重新生成诊断结果" disabled={props.props.isActionPending("diagnosticsCheck")} onClick={() => props.props.onCheckDiagnostics()} />
        <DiagnosticsActionButton title="导出运行日志" description="导出当前稳定日志快照" disabled={pendingAction === "logs"} onClick={() => void runDiagnosticsAction("logs", "正在导出运行日志…", exportRunLogs, setPendingAction, setFeedback)} />
        <DiagnosticsActionButton title="导出当前配置" description="导出当前设置响应快照" disabled={pendingAction === "settings"} onClick={() => void runDiagnosticsAction("settings", "正在导出当前配置…", () => exportSettingsSnapshot(props.props.settings), setPendingAction, setFeedback)} />
        <DiagnosticsActionButton title="打开诊断信息" description="查看当前诊断摘要 JSON" disabled={pendingAction === "snapshot"} onClick={() => void runDiagnosticsAction("snapshot", "正在打开诊断信息…", () => openDiagnosticsSnapshot(props.props.settings), setPendingAction, setFeedback)} />
      </div>
      <ActionHint props={props.props} actions={["diagnosticsCheck"]} />
      {feedback ? <InlineFeedback tone={feedback.tone} detail={feedback.detail} /> : null}
    </>
  );
}

function DiagnosticsActionButton(props: {
  title: string;
  description: string;
  disabled: boolean;
  onClick: () => void;
}) {
  return (
    <button type="button" className="utility-card" disabled={props.disabled} onClick={props.onClick}>
      <strong>{props.title}</strong>
      <span>{props.description}</span>
    </button>
  );
}

async function runDiagnosticsAction(
  action: DiagnosticsActionKey,
  runningDetail: string,
  task: () => Promise<string>,
  setPendingAction: (value: DiagnosticsActionKey | "") => void,
  setFeedback: (value: DiagnosticsFeedback | null) => void,
) {
  setPendingAction(action);
  setFeedback({ tone: "running", detail: runningDetail });
  try {
    setFeedback({ tone: "completed", detail: await task() });
  } catch (error) {
    setFeedback({ tone: "failed", detail: error instanceof Error ? error.message : "诊断动作执行失败" });
  } finally {
    setPendingAction("");
  }
}

function DiagnosticsHealthSummary(props: { settings: SettingsResponse }) {
  return (
    <div className="settings-control-grid">
      <div className="detail-card">
        <strong>健康摘要</strong>
        <p>{readDiagnosticsRuntimeSummary(props.settings)}</p>
        <p>{readDiagnosticsInventorySummary(props.settings)}</p>
        <p>{readDiagnosticsCheckTime(props.settings)}</p>
        <p>{`已记住 ${props.settings.diagnostics.approved_directory_count} 个授权目录。`}</p>
      </div>
      <div className="detail-card muted-card">
        <strong>思源链路</strong>
        <p>{readSiyuanSummary(props.settings)}</p>
        <p className="workspace-root">{props.settings.diagnostics.siyuan_root || "未提供思源根目录"}</p>
        <p className="workspace-root">{props.settings.diagnostics.siyuan_export_dir || "未提供导出目录"}</p>
      </div>
    </div>
  );
}

function DiagnosticsAlerts(props: { settings: SettingsResponse }) {
  const warnings = props.settings.diagnostics.warnings || [];
  const errors = props.settings.diagnostics.errors || [];
  if (warnings.length === 0 && errors.length === 0) return null;
  return (
    <div className="settings-control-grid">
      <div className="detail-card muted-card">
        <strong>警告</strong>
        {warnings.length === 0 ? <p>当前没有警告。</p> : warnings.map((item) => <p key={item}>{item}</p>)}
      </div>
      <div className="detail-card muted-card">
        <strong>错误</strong>
        {errors.length === 0 ? <p>当前没有错误。</p> : errors.map((item) => <p key={item}>{item}</p>)}
      </div>
    </div>
  );
}

function ToggleTile(props: {
  title: string;
  description: string;
  checked: boolean;
  isRunning: boolean;
  onChange: (enabled: boolean) => void;
}) {
  return (
    <label className="toggle-tile">
      <input type="checkbox" checked={props.checked} disabled={props.isRunning} onChange={(event) => props.onChange(event.target.checked)} />
      <div>
        <strong>{props.title}</strong>
        <p>{props.description}</p>
        <span className="toggle-state">{readToggleState(props.checked, props.isRunning)}</span>
      </div>
    </label>
  );
}

function DiagnosticsPathCard(props: { settings: SettingsResponse }) {
  return (
    <div className="detail-card muted-card">
      <strong>诊断落点</strong>
      <p className="workspace-root">{props.settings.diagnostics.settings_path}</p>
      <p className="workspace-root">{props.settings.diagnostics.run_log_path}</p>
      <p className="workspace-root">{props.settings.diagnostics.event_log_path}</p>
      <p className="workspace-root">{props.settings.diagnostics.repo_root}</p>
    </div>
  );
}

function ModuleHeader(props: { title: string; badge: string }) {
  return <SectionHeader kind="head" kicker="控制" title={props.title} action={<StatusPill className={readModuleStatusClass(props.badge)} label={props.badge} />} />;
}

function PlaceholderRow(props: { title: string; text: string }) {
  return (
    <div className="detail-card muted-card">
      <strong>{props.title}</strong>
      <p>{props.text}</p>
    </div>
  );
}

function readModelValue(settings: SettingsResponse) {
  return `${settings.model.provider_id}:${settings.model.model_id}`;
}

function renderModelOption(model: SettingsResponse["available_models"][number]) {
  return <option key={`${model.provider_id}:${model.model_id}`} value={`${model.provider_id}:${model.model_id}`} disabled={!model.available || !model.enabled}>{model.display_name}</option>;
}

function renderWorkspaceOption(workspace: SettingsResponse["available_workspaces"][number]) {
  return <option key={workspace.workspace_id} value={workspace.workspace_id}>{workspace.name}</option>;
}

function modeDescription(mode: string) {
  if (mode === "observe") return "只读观察，不执行修改动作。";
  if (mode === "full_access") return "开放全部已注册能力，但高危动作仍需确认。";
  return "允许常见开发读写与任务推进。";
}

function readRuntimeLabel(settings: SettingsResponse) {
  return settings.runtime_status.ok ? "可达" : "不可达";
}

function readRuntimeVersion(settings: SettingsResponse) {
  return settings.runtime_status.version || settings.runtime_status.name || "未提供";
}

function buildRuntimeRows(settings: SettingsResponse) {
  return [
    { label: "应用", value: settings.app_name || "未提供" },
    { label: "运行时", value: readRuntimeLabel(settings) },
    { label: "版本", value: readRuntimeVersion(settings) },
    { label: "网关端口", value: String(settings.ports.gateway || 0) },
    { label: "运行时端口", value: String(settings.ports.runtime || 0) },
    { label: "工作区", value: settings.workspace.name || "未提供" },
  ];
}

function buildMemoryOverviewRows(memories: MemoryEntry[]) {
  const summary = countMemoryFacets(memories);
  return [
    { label: "用户偏好", value: `${summary.preferences} 条` },
    { label: "失败教训", value: `${summary.lessons} 条` },
    { label: "待治理", value: `${summary.pending} 条` },
    { label: "已归档", value: `${summary.archived} 条` },
  ];
}

function buildDiagnosticsRows(settings: SettingsResponse) {
  return [
    { label: "检测时间", value: settings.diagnostics.checked_at || "未提供" },
    { label: "运行时", value: settings.diagnostics.runtime_reachable ? "可达" : "不可达" },
    { label: "版本", value: settings.diagnostics.runtime_version || "未提供" },
    { label: "服务方", value: `${settings.diagnostics.provider_count} 个` },
    { label: "模型", value: `${settings.diagnostics.model_count} 个` },
    { label: "工作区", value: `${settings.diagnostics.workspace_count} 个` },
    { label: "授权目录", value: `${settings.diagnostics.approved_directory_count} 个` },
    { label: "警告", value: `${(settings.diagnostics.warnings || []).length} 条` },
    { label: "错误", value: `${(settings.diagnostics.errors || []).length} 条` },
  ];
}

function readDiagnosticsRuntimeSummary(settings: SettingsResponse) {
  const status = settings.diagnostics.runtime_reachable ? "运行时当前可达" : "运行时当前不可达";
  return `${status}，版本 ${settings.diagnostics.runtime_version || "未提供"}。`;
}

function readDiagnosticsInventorySummary(settings: SettingsResponse) {
  const diagnostics = settings.diagnostics;
  return `已发现 ${diagnostics.provider_count} 个服务方、${diagnostics.model_count} 个模型、${diagnostics.workspace_count} 个工作区。`;
}

function readDiagnosticsCheckTime(settings: SettingsResponse) {
  return `最近检测：${settings.diagnostics.checked_at || "未提供"}`;
}

function readSiyuanSummary(settings: SettingsResponse) {
  const autoWrite = settings.diagnostics.siyuan_auto_write_enabled ? "自动写入已启用" : "自动写入未启用";
  const sync = settings.diagnostics.siyuan_sync_enabled ? "同步已启用" : "同步未启用";
  return `${autoWrite}，${sync}。`;
}

function buildExternalConnectionRows(
  slot: SettingsResponse["external_connections"][number],
  model: ReturnType<typeof buildExternalConnectionModel>,
) {
  return [
    { label: "状态", value: model.statusLabel },
    { label: "范围", value: slot.scope },
    { label: "已绑定工具", value: model.toolSummary },
  ];
}

function buildExternalConnectionModel(slot: ExternalConnectionSlot) {
  const action = readExternalConnectionAction(slot);
  if (slot.status === "active" && slot.current_tools.length > 0) {
    return createExternalConnectionModel("已可用", "status-completed", "当前已发现可用接点。", slot.current_tools.join(" / "), readNextStep(slot, "当前不需要额外前端操作。"), action, readExternalConnectionActionLabel(action, true));
  }
  if (slot.status === "limited") {
    return createExternalConnectionModel("当前受限", "status-awaiting", "连接位置已登记，但当前能力受限。", readToolSummary(slot), readNextStep(slot, "等待运行环境或工具能力恢复。"), action, readExternalConnectionActionLabel(action, false));
  }
  if (slot.current_tools.length > 0) {
    return createExternalConnectionModel("已预留未接入", "status-idle", "当前只保留接入位置，还没有进入可用态。", slot.current_tools.join(" / "), readNextStep(slot, "等待后端接入后再转为可用。"));
  }
  return createExternalConnectionModel("未绑定工具", "status-disconnected", "当前没有绑定可执行工具。", "无", readNextStep(slot, "等待相关工具注册或接入信息返回。"));
}

function createExternalConnectionModel(
  statusLabel: string,
  statusClass: string,
  summary: string,
  toolSummary: string,
  nextStep: string,
  action?: "validate" | "recheck",
  actionLabel?: string,
) {
  return { action, actionLabel, nextStep, statusClass, statusLabel, summary, toolSummary };
}

function readToolSummary(slot: ExternalConnectionSlot) {
  return slot.current_tools.join(" / ") || "无";
}

function readNextStep(slot: ExternalConnectionSlot, fallback: string) {
  return slot.next_step || fallback;
}

function readToggleState(checked: boolean, isRunning: boolean) {
  if (isRunning) return "处理中";
  return checked ? "已完成" : "空闲";
}

function ActionHint(props: {
  props: SettingsModulesProps;
  actions: SettingsActionKind[];
}) {
  const pending = matchActionFeedback(props.props.pendingAction, props.actions);
  const failed = matchActionFeedback(props.props.actionError, props.actions);
  const success = matchActionFeedback(props.props.lastSuccess, props.actions);
  if (!pending && !failed && !success) return null;
  if (pending) return <InlineFeedback tone="running" detail={pending.detail} />;
  if (failed) return <InlineFeedback tone="failed" detail={failed.detail} />;
  return <InlineFeedback tone="completed" detail={success!.detail} />;
}

function InlineFeedback(props: { tone: "running" | "failed" | "completed"; detail: string }) {
  return <p className={`settings-inline-feedback settings-inline-feedback-${props.tone}`}>{props.detail}</p>;
}

function matchActionFeedback(
  feedback: SettingsActionFeedback | null,
  actions: SettingsActionKind[],
) {
  if (!feedback) return null;
  return actions.includes(feedback.action) ? feedback : null;
}

function readControlBadge(props: SettingsModulesProps, actions: SettingsActionKind[]) {
  if (matchActionFeedback(props.pendingAction, actions)) return "处理中";
  if (matchActionFeedback(props.actionError, actions)) return "失败";
  if (matchActionFeedback(props.lastSuccess, actions)) return "已完成";
  if (actions.includes("model") || actions.includes("mode")) return props.isRunning ? "处理中" : "空闲";
  return "就绪";
}

function readModuleStatusClass(status: string) {
  if (status === "已断开") return "status-disconnected";
  return readUnifiedStatusMeta(readUnifiedStatusFromLabel(status)).className;
}

function readMemoryActionState(props: SettingsModulesProps) {
  if (props.memoryPendingAction) return { message: props.memoryPendingAction.message, tone: "running" as const };
  if (props.memoryActionError) return { message: props.memoryActionError.message, tone: "failed" as const };
  if (props.memoryActionSuccess) return { message: props.memoryActionSuccess.message, tone: "completed" as const };
  return null;
}

function readDiagnosticsBadge(props: SettingsModulesProps) {
  if (props.isActionPending("diagnosticsCheck")) return "处理中";
  if ((props.settings.diagnostics.errors || []).length > 0) return "失败";
  return "就绪";
}

function readExternalConnectionAction(slot: ExternalConnectionSlot) {
  if (slot.supported_actions?.includes("recheck")) return "recheck";
  if (slot.supported_actions?.includes("validate")) return "validate";
  return undefined;
}

function readExternalConnectionActionLabel(
  action: "validate" | "recheck" | undefined,
  isActive: boolean,
) {
  if (!action) return undefined;
  if (action === "validate") return "校验";
  return isActive ? "重新校验" : "重新检测";
}
