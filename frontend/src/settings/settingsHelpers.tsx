import { ExternalConnectionSlot, MemoryEntry, ProviderSettingsResponse, SettingsResponse } from "../shared/contracts";
import { readUnifiedStatusFromLabel, readUnifiedStatusMeta, UnifiedStatusKey } from "../runtime/state";
import { countMemoryFacets } from "../history/logType";
import { SettingsActionFeedback, SettingsActionKind, ProviderActionState } from "./useSettings";

export type DiagnosticsGroupKey = "status" | "impact" | "actions";
export type DiagnosticsGroupModel = {
  key: DiagnosticsGroupKey;
  title: string;
  summary: string;
  details: string[];
  status: UnifiedStatusKey;
};

export type SettingsModulesProps = {
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
  onEmbeddingProviderChange: (providerId: string) => void;
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
export function readModelValue(settings: SettingsResponse) {
  return `${settings.model.provider_id}:${settings.model.model_id}`;
}

export function renderModelOption(model: SettingsResponse["available_models"][number]) {
  return <option key={`${model.provider_id}:${model.model_id}`} value={`${model.provider_id}:${model.model_id}`} disabled={!model.available || !model.enabled}>{model.display_name}</option>;
}

export function renderWorkspaceOption(workspace: SettingsResponse["available_workspaces"][number]) {
  return <option key={workspace.workspace_id} value={workspace.workspace_id}>{workspace.name}</option>;
}

export function modeDescription(mode: string) {
  if (mode === "observe") return "只读观察，不执行修改动作。";
  if (mode === "full_access") return "开放全部已注册能力，但高危动作仍需确认。";
  return "允许常见开发读写与任务推进。";
}

export function readRuntimeLabel(settings: SettingsResponse) {
  return settings.runtime_status.ok ? "可达" : "不可达";
}

export function readRuntimeVersion(settings: SettingsResponse) {
  return settings.runtime_status.version || settings.runtime_status.name || "未提供";
}

export function buildRuntimeRows(settings: SettingsResponse) {
  return [
    { label: "应用", value: settings.app_name || "未提供" },
    { label: "运行时", value: readRuntimeLabel(settings) },
    { label: "版本", value: readRuntimeVersion(settings) },
    { label: "网关端口", value: String(settings.ports.gateway || 0) },
    { label: "运行时端口", value: String(settings.ports.runtime || 0) },
    { label: "工作区", value: settings.workspace.name || "未提供" },
  ];
}

export function buildMemoryOverviewRows(memories: MemoryEntry[]) {
  const summary = countMemoryFacets(memories);
  return [
    { label: "用户偏好", value: `${summary.preferences} 条` },
    { label: "失败教训", value: `${summary.lessons} 条` },
    { label: "待治理", value: `${summary.pending} 条` },
    { label: "已归档", value: `${summary.archived} 条` },
  ];
}

export function buildDiagnosticsRows(settings: SettingsResponse) {
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

export function buildDiagnosticsGroupModels(settings: SettingsResponse): DiagnosticsGroupModel[] {
  return [
    buildDiagnosticsStatusGroup(settings),
    buildDiagnosticsImpactGroup(settings),
    buildDiagnosticsActionGroup(settings),
  ];
}

export function buildDiagnosticsStatusGroup(settings: SettingsResponse): DiagnosticsGroupModel {
  const warnings = (settings.diagnostics.warnings || []).length;
  const errors = (settings.diagnostics.errors || []).length;
  return {
    key: "status",
    title: "当前状态",
    summary: readDiagnosticsRuntimeSummary(settings),
    details: [
      readDiagnosticsCheckTime(settings),
      `警告 ${warnings} 条`,
      `错误 ${errors} 条`,
      `运行时版本：${settings.diagnostics.runtime_version || "未提供"}`,
    ],
    status: readDiagnosticsOverallStatusKey(settings),
  };
}

export function buildDiagnosticsImpactGroup(settings: SettingsResponse): DiagnosticsGroupModel {
  const diagnostics = settings.diagnostics;
  return {
    key: "impact",
    title: "影响范围",
    summary: "本次诊断覆盖运行时连通性、配置落点、模型与工作区库存。",
    details: [
      `服务方 ${diagnostics.provider_count} 个 / 模型 ${diagnostics.model_count} 个`,
      `工作区 ${diagnostics.workspace_count} 个 / 授权目录 ${diagnostics.approved_directory_count} 个`,
      `设置文件：${diagnostics.settings_path || "未提供"}`,
      `运行日志：${diagnostics.run_log_path || "未提供"}`,
    ],
    status: readDiagnosticsImpactStatusKey(settings),
  };
}

export function buildDiagnosticsActionGroup(settings: SettingsResponse): DiagnosticsGroupModel {
  return {
    key: "actions",
    title: "建议动作",
    summary: readDiagnosticsActionSummary(settings),
    details: buildDiagnosticsActionItems(settings),
    status: readDiagnosticsActionStatusKey(settings),
  };
}

export function readDiagnosticsActionSummary(settings: SettingsResponse) {
  const errors = (settings.diagnostics.errors || []).length;
  if (errors > 0) return "存在诊断错误，建议先处理高优先级阻塞项。";
  if (!settings.diagnostics.runtime_reachable) return "运行时不可达，建议先恢复连接链路。";
  if ((settings.diagnostics.warnings || []).length > 0) return "存在警告，建议按风险顺序逐项核对。";
  return "诊断链路整体稳定，建议保持当前配置并定期复检。";
}

export function buildDiagnosticsActionItems(settings: SettingsResponse) {
  const diagnostics = settings.diagnostics;
  const items = collectDiagnosticsActionItems(diagnostics);
  items.push("处理后点击“重新检测”，确认状态回到完成态。");
  return items;
}

export function collectDiagnosticsActionItems(diagnostics: SettingsResponse["diagnostics"]) {
  const items: string[] = [];
  if (!diagnostics.runtime_reachable) items.push("恢复运行时可达性，再执行导出与外部连接校验。");
  diagnostics.errors?.slice(0, 2).forEach((item) => items.push(`优先处理错误：${item}`));
  diagnostics.warnings?.slice(0, 2).forEach((item) => items.push(`排查警告：${item}`));
  if (!diagnostics.siyuan_auto_write_enabled) items.push("如需自动沉淀，启用思源自动写入并重新检测。");
  if (items.length === 0) items.push("当前无需额外处置，可继续按常规节奏复检。");
  return items;
}

export function readDiagnosticsOverallStatusKey(settings: SettingsResponse): UnifiedStatusKey {
  if (!settings.diagnostics.runtime_reachable) return "failed";
  if ((settings.diagnostics.errors || []).length > 0) return "failed";
  if ((settings.diagnostics.warnings || []).length > 0) return "awaiting_confirmation";
  return "completed";
}

export function readDiagnosticsImpactStatusKey(settings: SettingsResponse): UnifiedStatusKey {
  if ((settings.diagnostics.errors || []).length > 0) return "failed";
  if ((settings.diagnostics.warnings || []).length > 0) return "awaiting_confirmation";
  return "completed";
}

export function readDiagnosticsActionStatusKey(settings: SettingsResponse): UnifiedStatusKey {
  if (!settings.diagnostics.runtime_reachable) return "failed";
  if ((settings.diagnostics.errors || []).length > 0) return "failed";
  if ((settings.diagnostics.warnings || []).length > 0) return "awaiting_confirmation";
  return "completed";
}

export function readDiagnosticsRuntimeSummary(settings: SettingsResponse) {
  const status = settings.diagnostics.runtime_reachable ? "运行时当前可达" : "运行时当前不可达";
  return `${status}，版本 ${settings.diagnostics.runtime_version || "未提供"}。`;
}

export function readDiagnosticsInventorySummary(settings: SettingsResponse) {
  const diagnostics = settings.diagnostics;
  return `已发现 ${diagnostics.provider_count} 个服务方、${diagnostics.model_count} 个模型、${diagnostics.workspace_count} 个工作区。`;
}

export function readDiagnosticsCheckTime(settings: SettingsResponse) {
  return `最近检测：${formatDiagnosticsTime(settings.diagnostics.checked_at)}`;
}

export function formatDiagnosticsTime(value?: string) {
  if (!value) return "未提供";
  const time = new Date(value);
  if (Number.isNaN(time.getTime())) return value;
  return time.toLocaleString("zh-CN", { hour12: false });
}

export function readSiyuanSummary(settings: SettingsResponse) {
  const autoWrite = settings.diagnostics.siyuan_auto_write_enabled ? "自动写入已启用" : "自动写入未启用";
  const sync = settings.diagnostics.siyuan_sync_enabled ? "同步已启用" : "同步未启用";
  return `${autoWrite}，${sync}。`;
}

export function buildExternalConnectionRows(
  slot: SettingsResponse["external_connections"][number],
  model: ReturnType<typeof buildExternalConnectionModel>,
) {
  return [
    { label: "状态", value: model.statusLabel },
    { label: "范围", value: slot.scope },
    { label: "已绑定工具", value: model.toolSummary },
  ];
}

export function buildExternalConnectionModel(slot: ExternalConnectionSlot) {
  const action = readExternalConnectionAction(slot);
  if (slot.status === "active" && slot.current_tools.length > 0) {
    return createExternalConnectionModel("已可用", "completed", "当前已发现可用接点。", slot.current_tools.join(" / "), readNextStep(slot, "当前不需要额外前端操作。"), action, readExternalConnectionActionLabel(action, true));
  }
  if (slot.status === "limited") {
    return createExternalConnectionModel("当前受限", "awaiting_confirmation", "连接位置已登记，但当前能力受限。", readToolSummary(slot), readNextStep(slot, "等待运行环境或工具能力恢复。"), action, readExternalConnectionActionLabel(action, false));
  }
  if (slot.current_tools.length > 0) {
    return createExternalConnectionModel("已预留未接入", "idle", "当前只保留接入位置，还没有进入可用态。", slot.current_tools.join(" / "), readNextStep(slot, "等待后端接入后再转为可用。"));
  }
  return createExternalConnectionModel("未绑定工具", "disconnected", "当前没有绑定可执行工具。", "无", readNextStep(slot, "等待相关工具注册或接入信息返回。"));
}

export function createExternalConnectionModel(
  statusLabel: string,
  status: UnifiedStatusKey | "disconnected",
  summary: string,
  toolSummary: string,
  nextStep: string,
  action?: "validate" | "recheck",
  actionLabel?: string,
) {
  const statusClass = status === "disconnected"
    ? "status-disconnected"
    : readUnifiedStatusMeta(status).className;
  return { action, actionLabel, nextStep, statusClass, statusLabel, summary, toolSummary };
}

export function readToolSummary(slot: ExternalConnectionSlot) {
  return slot.current_tools.join(" / ") || "无";
}

export function readNextStep(slot: ExternalConnectionSlot, fallback: string) {
  return slot.next_step || fallback;
}

export function readToggleState(checked: boolean, isRunning: boolean) {
  if (isRunning) return readUnifiedStatusMeta("running").label;
  return checked ? readUnifiedStatusMeta("completed").label : readUnifiedStatusMeta("idle").label;
}

export function matchActionFeedback(
  feedback: SettingsActionFeedback | null,
  actions: SettingsActionKind[],
) {
  if (!feedback) return null;
  return actions.includes(feedback.action) ? feedback : null;
}

export function readControlBadge(props: SettingsModulesProps, actions: SettingsActionKind[]) {
  if (matchActionFeedback(props.pendingAction, actions)) return readUnifiedStatusMeta("running").label;
  if (matchActionFeedback(props.actionError, actions)) return "失败";
  if (matchActionFeedback(props.lastSuccess, actions)) return readUnifiedStatusMeta("completed").label;
  if (actions.includes("model") || actions.includes("mode")) return props.isRunning ? readUnifiedStatusMeta("running").label : readUnifiedStatusMeta("idle").label;
  return "就绪";
}

export function readModuleStatusClass(status: string) {
  if (status === "已断开") return "status-disconnected";
  return readUnifiedStatusMeta(readUnifiedStatusFromLabel(status)).className;
}

export function readMemoryActionState(props: SettingsModulesProps) {
  if (props.memoryPendingAction) return { message: props.memoryPendingAction.message, tone: "running" as const };
  if (props.memoryActionError) return { message: props.memoryActionError.message, tone: "failed" as const };
  if (props.memoryActionSuccess) return { message: props.memoryActionSuccess.message, tone: "completed" as const };
  return null;
}

export function readDiagnosticsBadge(props: SettingsModulesProps) {
  if (props.isActionPending("diagnosticsCheck")) return readUnifiedStatusMeta("running").label;
  return readUnifiedStatusMeta(readDiagnosticsOverallStatusKey(props.settings)).label;
}

export function readExternalConnectionAction(slot: ExternalConnectionSlot) {
  if (slot.supported_actions?.includes("recheck")) return "recheck";
  if (slot.supported_actions?.includes("validate")) return "validate";
  return undefined;
}

export function readExternalConnectionActionLabel(
  action: "validate" | "recheck" | undefined,
  isActive: boolean,
) {
  if (!action) return undefined;
  if (action === "validate") return "校验";
  return isActive ? "重新校验" : "重新检测";
}