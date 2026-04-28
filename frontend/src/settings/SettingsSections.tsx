import { useState } from "react";
import { countMemoryFacets, readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel } from "../history/logType";
import { ResourcesEntrySection } from "../resources/components";
import { readUnifiedStatusFromLabel, readUnifiedStatusMeta, UnifiedStatusKey } from "../runtime/state";
import { ExternalConnectionSlot, MemoryEntry, ProviderSettingsResponse, SettingsResponse } from "../shared/contracts";
import { EmptyStateBlock, MetaGrid, SectionHeader, StatusPill } from "../ui/primitives";
import { exportRunLogs, exportSettingsSnapshot, openDiagnosticsSnapshot } from "./api";
import { ProviderCredentialsSection } from "./ProviderCredentialsSection";
import { ProviderActionState, SettingsActionFeedback, SettingsActionKind } from "./useSettings";
import {
  SettingsModulesProps,
  buildExternalConnectionModel,
  buildExternalConnectionRows,
  buildMemoryOverviewRows,
  buildRuntimeRows,
  matchActionFeedback,
  modeDescription,
  readControlBadge,
  readDiagnosticsBadge,
  readDiagnosticsCheckTime,
  readDiagnosticsOverallStatusKey,
  readMemoryActionState,
  readModelValue,
  readModuleStatusClass,
  readToggleState,
  renderModelOption,
  renderWorkspaceOption,
} from "./settingsHelpers";

const SETTINGS_MODULE_ORDER = ["runtime", "model", "provider", "embedding", "workspace", "risk", "resources", "diagnostics"] as const;
const MODULE_TITLES: Record<(typeof SETTINGS_MODULE_ORDER)[number], string> = {
  runtime: "运行环境",
  model: "模型与模式",
  provider: "Provider 凭证",
  embedding: "向量化与嵌入",
  workspace: "工作区与授权",
  risk: "风险与权限",
  resources: "记忆与资源",
  diagnostics: "诊断与导出",
};
function readModuleBadge(key: (typeof SETTINGS_MODULE_ORDER)[number], props: SettingsModulesProps): string {
  if (key === "runtime") return props.settings.runtime_status.ok ? "正常" : "已断开";
  if (key === "model") return readControlBadge(props, ["model", "mode"]);
  if (key === "provider") return props.providerSettings ? `${props.providerSettings.providers.length} 个` : "加载中";
  if (key === "embedding") return props.settings.embedding?.model_name ? "已配置" : "未配置";
  if (key === "workspace") return readControlBadge(props, ["workspace", "revokeApproval"]);
  if (key === "risk") return readControlBadge(props, ["directoryPrompt", "riskLevel"]);
  if (key === "resources") return props.settings.memory_policy.enabled ? "已启用" : "未启用";
  return readDiagnosticsBadge(props);
}
type DiagnosticsActionKey = "logs" | "settings" | "snapshot";
type DiagnosticsFeedback = { tone: "running" | "failed" | "completed"; detail: string };

export function SettingsModules(props: SettingsModulesProps) {
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});
  const toggle = (key: string) => setCollapsed((prev) => ({ ...prev, [key]: !prev[key] }));
  return (
    <div className="settings-stack">
      {buildSettingsModules(props, collapsed, toggle).map((item) => item.node)}
    </div>
  );
}

export function SettingsEmptyState() {
  return <EmptyStateBlock compact title="设置加载中" text="环境状态返回后显示控制项。" />;
}

function buildSettingsModules(
  props: SettingsModulesProps,
  collapsed: Record<string, boolean>,
  toggle: (key: string) => void,
) {
  return SETTINGS_MODULE_ORDER.map((key) => createSettingsModule(key, props, collapsed[key], toggle));
}

function createSettingsModule(
  key: typeof SETTINGS_MODULE_ORDER[number],
  props: SettingsModulesProps,
  isCollapsed: boolean,
  toggle: (key: string) => void,
) {
  const header = <CollapsibleModuleHeader title={MODULE_TITLES[key]} badge={readModuleBadge(key, props)} collapsed={!!isCollapsed} onToggle={() => toggle(key)} />;
  let content: React.ReactNode = null;
  if (key === "runtime") content = <RuntimeModule settings={props.settings} />;
  if (key === "model") content = <ModelModule props={props} />;
  if (key === "provider") content = <ProviderModule props={props} />;
  if (key === "embedding") content = <EmbeddingModule props={props} />;
  if (key === "workspace") content = <WorkspaceModule props={props} />;
  if (key === "risk") content = <RiskModule props={props} />;
  if (key === "resources") content = <ResourcesModule props={props} />;
  if (key === "diagnostics") content = <DiagnosticsModule props={props} />;
  return {
    key,
    node: (
      <section key={key} id={`settings-module-${key}`} className="settings-module control-module">
        {header}
        {!isCollapsed ? content : null}
      </section>
    ),
  };
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


function EmbeddingModule(props: { props: SettingsModulesProps }) {
  const embedding = props.props.settings.embedding;
  const pending = props.props.isActionPending("embedding");
  const providers = props.props.settings.providers.filter((p) => p.embedding_model);
  return (
    <>
      <div className="settings-control-grid">
        <label className="control-field">
          <span>当前嵌入服务方</span>
          <select name="embedding_provider" value={embedding?.provider_id || ""} disabled={pending} onChange={(event) => props.props.onEmbeddingProviderChange(event.target.value)}>
            {providers.map((p) => <option key={p.provider_id} value={p.provider_id}>{p.display_name}</option>)}
          </select>
        </label>
        <div className="detail-card muted-card">
          <strong>当前嵌入模型</strong>
          <p>{embedding?.model_name || "未配置"}</p>
        </div>
      </div>
      <p className="workspace-root" style={{ marginTop: 12 }}>切换嵌入服务方将使用 config/app.json 中配置的对应模型，向量维度变化后需重建知识库索引。</p>
    </>
  );
}
function RuntimeModule(props: { settings: SettingsResponse }) {
  const ok = props.settings.runtime_status.ok;
  return (
    <>
      <div className="settings-control-grid">
        <div className="detail-card">
          <strong>运行时连接</strong>
          <p>{ok ? "运行时正常响应" : "运行时不可达"}</p>
          <p className="workspace-root">{props.settings.runtime_status.name} / {props.settings.runtime_status.version || "未知版本"}</p>
        </div>
        <div className="detail-card muted-card">
          <strong>服务端口</strong>
          <p>网关 {props.settings.ports.gateway} / 运行时 {props.settings.ports.runtime}</p>
          <p className="workspace-root">工作区 {props.settings.workspace.name}</p>
        </div>
      </div>
    </>
  );
}

function ModelModule(props: { props: SettingsModulesProps }) {
  return (
    <>
      <ModelControls props={props.props} />
      <ActionHint props={props.props} actions={["model", "mode"]} />
      <ModelSummary settings={props.props.settings} />
    </>
  );
}

function ModelControls(props: { props: SettingsModulesProps }) {
  const modelPending = props.props.isActionPending("model");
  const modePending = props.props.isActionPending("mode");
  return (
    <div className="settings-control-grid">
      <label className="control-field">
        <span>当前模型</span>
        <select name="model_select" value={readModelValue(props.props.settings)} disabled={modelPending} onChange={(event) => props.props.onModelChange(event.target.value)}>
          {props.props.settings.available_models.map(renderModelOption)}
        </select>
      </label>
      <label className="control-field">
        <span>运行模式</span>
        <select name="mode_select" value={props.props.settings.mode} disabled={modePending} onChange={(event) => props.props.onModeChange(event.target.value)}>
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
  return (
    <>
      <WorkspaceControlGrid props={props.props} />
      <ActionHint props={props.props} actions={["workspace", "revokeApproval"]} />
      <ApprovalList props={props.props} />
    </>
  );
}

function WorkspaceControlGrid(props: { props: SettingsModulesProps }) {
  const workspacePending = props.props.isActionPending("workspace");
  return (
    <div className="settings-control-grid">
      <label className="control-field">
        <span>当前工作区</span>
        <select name="workspace_select" value={props.props.settings.workspace.workspace_id} disabled={workspacePending} onChange={(event) => props.props.onWorkspaceChange(event.target.value)}>
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
  return (
    <>
      <div className="settings-control-grid">
        <ToggleTile name="directory_prompt_enabled" title="新目录首次接触提醒" description="进入新目录时，先提示授权边界。" checked={props.props.settings.directory_prompt_enabled} isRunning={props.props.isActionPending("directoryPrompt")} onChange={props.props.onDirectoryPromptEnabledChange} />
        <ToggleTile name="show_risk_level" title="显示风险等级" description="在确认流中展示风险等级。" checked={props.props.settings.show_risk_level} isRunning={props.props.isActionPending("riskLevel")} onChange={props.props.onShowRiskLevelChange} />
      </div>
      <ActionHint props={props.props} actions={["directoryPrompt", "riskLevel"]} />
    </>
  );
}

function ResourcesModule(props: { props: SettingsModulesProps }) {
  const actionState = readMemoryActionState(props.props);
  const [showMemories, setShowMemories] = useState(false);
  const [showConnections, setShowConnections] = useState(false);
  return (
    <>
      <MemoryOverviewCards memories={props.props.memories} />
      <div className="settings-subsection">
        <button
          type="button"
          className="secondary-button"
          onClick={() => setShowMemories((v) => !v)}
        >
          {showMemories ? "收起记忆列表" : `展开记忆列表 (${props.props.memories.length})`}
        </button>
        {showMemories ? (
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
        ) : null}
      </div>
      <div className="settings-subsection">
        <button
          type="button"
          className="secondary-button"
          onClick={() => setShowConnections((v) => !v)}
        >
          {showConnections ? "收起外部连接" : `展开外部连接 (${(props.props.settings.external_connections || []).length})`}
        </button>
        {showConnections ? <ExternalConnectionsSection props={props.props} /> : null}
      </div>
    </>
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
          {pending ? readUnifiedStatusMeta("running").label : model.actionLabel}
        </button>
      ) : null}
    </div>
  );
}

function DiagnosticsModule(props: { props: SettingsModulesProps }) {
  return (
    <>
      <DiagnosticsGroupedSummary settings={props.props.settings} />
      <DiagnosticsActions props={props.props} />
      <DiagnosticsAlerts settings={props.props.settings} />
      <DiagnosticsPathCard settings={props.props.settings} />
    </>
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

function DiagnosticsGroupedSummary(props: { settings: SettingsResponse }) {
  const diagnostics = props.settings.diagnostics;
  const status = readUnifiedStatusMeta(readDiagnosticsOverallStatusKey(props.settings));
  const warnings = (diagnostics.warnings || []).length;
  const errors = (diagnostics.errors || []).length;
  return (
    <section className="detail-card muted-card">
      <div className="diagnostics-group-head">
        <strong>诊断摘要</strong>
        <StatusPill className={status.className} label={status.label} />
      </div>
      <MetaGrid items={[
        { label: "检测时间", value: readDiagnosticsCheckTime(props.settings) },
        { label: "运行时", value: diagnostics.runtime_reachable ? "可达" : "不可达" },
        { label: "版本", value: diagnostics.runtime_version || "未提供" },
        { label: "服务方/模型", value: `${diagnostics.provider_count} 个 / ${diagnostics.model_count} 个` },
        { label: "工作区/授权目录", value: `${diagnostics.workspace_count} 个 / ${diagnostics.approved_directory_count} 个` },
        { label: "警告/错误", value: `${warnings} 条 / ${errors} 条` },
      ]} />
    </section>
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
  name: string;
  title: string;
  description: string;
  checked: boolean;
  isRunning: boolean;
  onChange: (enabled: boolean) => void;
}) {
  return (
    <label className="toggle-tile">
      <input name={props.name} type="checkbox" checked={props.checked} disabled={props.isRunning} onChange={(event) => props.onChange(event.target.checked)} />
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
  return <SectionHeader kind="head" kicker="Module" title={props.title} action={<StatusPill className={readModuleStatusClass(props.badge)} label={props.badge} />} />;
}

function CollapsibleModuleHeader(props: { title: string; badge: string; collapsed: boolean; onToggle: () => void }) {
  return (
    <button type="button" className="settings-module-header" onClick={props.onToggle} aria-expanded={!props.collapsed}>
      <span className="settings-module-header-title">
        <span className={`settings-module-chevron ${props.collapsed ? "" : "expanded"}`}>▸</span>
        {props.title}
      </span>
      <StatusPill className={readModuleStatusClass(props.badge)} label={props.badge} />
    </button>
  );
}

function PlaceholderRow(props: { title: string; text: string }) {
  return (
    <div className="detail-card muted-card">
      <strong>{props.title}</strong>
      <p>{props.text}</p>
    </div>
  );
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
