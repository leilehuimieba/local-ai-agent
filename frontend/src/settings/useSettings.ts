import { useEffect, useState } from "react";

import {
  DiagnosticsCheckResponse,
  ExternalConnectionActionResponse,
  ProviderSettingsResponse,
  SettingsResponse,
} from "../shared/contracts";
import { useMemories } from "../resources/useMemories";
import {
  applyDiagnosticsResult,
  applyNextSettings,
  buildActionRunner,
  createFeedback,
  isActionPending,
  mergeExternalConnections,
  runProviderAction,
  runSettingsAction,
  type ActionRunner,
  type ProviderActionState,
  type SettingsActionFeedback,
  type SettingsActionKind,
} from "./actionRunner";
import {
  applyProviderCredential,
  checkDiagnostics,
  fetchProviderSettings,
  fetchSettings,
  removeProviderCredential,
  runExternalConnectionAction,
  saveProviderCredential,
  testProviderConnection,
  updateSettings,
} from "./api";

type PendingActions = Partial<Record<SettingsActionKind, true>>;
type ProviderActions = Record<string, ProviderActionState>;

export type { SettingsActionKind, SettingsActionFeedback, ProviderActionState };

export function useSettings() {
  const state = useSettingsState();
  const feedback = useSettingsFeedback();
  const memoriesApi = useMemories((count) => syncMemoryCount(state.setSettings, count));
  useSettingsBootstrap(state);
  return buildSettingsApi(state, feedback, memoriesApi);
}

function useSettingsState() {
  const [settings, setSettings] = useState<SettingsResponse | null>(null);
  const [providerSettings, setProviderSettings] = useState<ProviderSettingsResponse | null>(null);
  const [bootstrapError, setBootstrapError] = useState<string | null>(null);
  const [providerBootstrapError, setProviderBootstrapError] = useState<string | null>(null);
  const [providerActions, setProviderActions] = useState<ProviderActions>({});
  return {
    settings, setSettings, providerSettings, setProviderSettings,
    bootstrapError, setBootstrapError, providerBootstrapError, setProviderBootstrapError,
    providerActions, setProviderActions,
  };
}

function useSettingsFeedback() {
  const [pendingAction, setPendingAction] = useState<SettingsActionFeedback | null>(null);
  const [pendingActions, setPendingActions] = useState<PendingActions>({});
  const [actionError, setActionError] = useState<SettingsActionFeedback | null>(null);
  const [lastSuccess, setLastSuccess] = useState<SettingsActionFeedback | null>(null);
  return {
    actionError,
    isActionPending: (action: SettingsActionKind) => isActionPending(pendingActions, action),
    lastSuccess,
    pendingAction,
    setActionError,
    setLastSuccess,
    setPendingAction,
    setPendingActions,
  };
}

function useSettingsBootstrap(
  state: ReturnType<typeof useSettingsState>,
) {
  useEffect(() => {
    const controller = new AbortController();
    bootstrapSettings(state, controller.signal);
    bootstrapProviderSettings(state, controller.signal);
    return () => controller.abort();
  }, []);
}

function buildSettingsApi(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
  memoriesApi: ReturnType<typeof useMemories>,
) {
  return {
    ...buildSettingsSnapshot(state, feedback, memoriesApi),
    ...buildSettingsActions(state, feedback, memoriesApi),
  };
}

function syncMemoryCount(
  setSettings: React.Dispatch<React.SetStateAction<SettingsResponse | null>>,
  count: number,
) {
  setSettings((current) => current ? ({
    ...current,
    memory_policy: { ...current.memory_policy, long_term_memory_count: count },
  }) : current);
}

function buildSettingsSnapshot(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
  memoriesApi: ReturnType<typeof useMemories>,
) {
  return {
    settings: state.settings,
    providerSettings: state.providerSettings,
    bootstrapError: state.bootstrapError,
    providerBootstrapError: state.providerBootstrapError,
    providerActions: state.providerActions,
    pendingAction: feedback.pendingAction,
    actionError: feedback.actionError,
    lastSuccess: feedback.lastSuccess,
    memories: memoriesApi.memories,
    memoryError: memoriesApi.memoryError,
    memoryPendingAction: memoriesApi.memoryPendingAction,
    memoryActionError: memoriesApi.memoryActionError,
    memoryActionSuccess: memoriesApi.memoryActionSuccess,
    deletingMemoryId: memoriesApi.deletingId,
  };
}

function buildSettingsActions(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
  memoriesApi: ReturnType<typeof useMemories>,
) {
  return {
    changeWorkspace: createWorkspaceChange(state, feedback, memoriesApi),
    changeMode: createModeChange(state, feedback),
    changeModel: createModelChange(state, feedback),
    changeDirectoryPromptEnabled: createDirectoryPromptChange(state, feedback),
    changeShowRiskLevel: createRiskLevelChange(state, feedback),
    revokeDirectoryApproval: createApprovalRevoke(state, feedback),
    runExternalConnectionAction: createExternalConnectionAction(state, feedback),
    checkDiagnostics: createDiagnosticsCheck(state, feedback),
    refreshProviderSettings: createProviderRefresh(state),
    testProvider: createProviderTest(state),
    saveProvider: createProviderSave(state),
    applyProvider: createProviderApply(state),
    removeProvider: createProviderRemove(state),
    isActionPending: feedback.isActionPending,
    refreshMemories: memoriesApi.refreshMemories,
    removeMemory: memoriesApi.removeMemory,
  };
}

function bootstrapSettings(
  state: ReturnType<typeof useSettingsState>,
  signal: AbortSignal,
) {
  fetchSettings(signal)
    .then((data) => state.setSettings(data))
    .catch((error: Error) => {
      if (!signal.aborted) state.setBootstrapError(error.message);
    });
}

function bootstrapProviderSettings(
  state: ReturnType<typeof useSettingsState>,
  signal: AbortSignal,
) {
  fetchProviderSettings(signal)
    .then((data) => state.setProviderSettings(data))
    .catch((error: Error) => {
      if (!signal.aborted) state.setProviderBootstrapError(error.message);
    });
}

function createProviderRefresh(state: ReturnType<typeof useSettingsState>) {
  return async () => {
    const data = await fetchProviderSettings();
    state.setProviderSettings(data);
    state.setProviderBootstrapError(null);
    return data;
  };
}

function createProviderTest(state: ReturnType<typeof useSettingsState>) {
  return async (providerId: string, apiKey: string, baseURL?: string) => {
    return runProviderAction(state, providerId, async () => {
      const result = await testProviderConnection({ provider_id: providerId, api_key: apiKey, base_url: baseURL });
      if (!result.ok) throw new Error(result.message);
      return result;
    });
  };
}

function createProviderSave(state: ReturnType<typeof useSettingsState>) {
  return async (providerId: string, apiKey: string) => {
    return runProviderAction(state, providerId, async () => {
      const result = await saveProviderCredential({ provider_id: providerId, api_key: apiKey });
      await createProviderRefresh(state)();
      return result;
    });
  };
}

function createProviderApply(state: ReturnType<typeof useSettingsState>) {
  return async (providerId: string) => {
    return runProviderAction(state, providerId, async () => {
      const result = await applyProviderCredential({ provider_id: providerId });
      await createProviderRefresh(state)();
      return result;
    });
  };
}

function createProviderRemove(state: ReturnType<typeof useSettingsState>) {
  return async (providerId: string) => {
    return runProviderAction(state, providerId, async () => {
      const result = await removeProviderCredential({ provider_id: providerId });
      await createProviderRefresh(state)();
      return result;
    });
  };
}

function createWorkspaceChange(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
  memoriesApi: ReturnType<typeof useMemories>,
) {
  return async (workspaceId: string) => {
    const settings = state.settings;
    if (!settings || workspaceId === settings.workspace.workspace_id || feedback.isActionPending("workspace")) return settings;
    const name = readWorkspaceName(settings, workspaceId);
    return runSettingsAction(buildActionRunner({
      action: createFeedback("workspace", "工作区切换", `正在切换到 ${name}。`),
      execute: async () => updateSettings({ workspace_id: workspaceId }),
      onSuccess: async () => { await memoriesApi.refreshMemories(); },
      successDetail: `已切换到 ${name}。`,
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createModeChange(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (mode: string) => {
    const settings = state.settings;
    if (!settings || mode === settings.mode || feedback.isActionPending("mode")) return settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("mode", "运行模式切换", `正在切换到${readModeLabel(mode)}。`),
      execute: async () => updateSettings({ mode }),
      successDetail: `当前模式已更新为${readModeLabel(mode)}。`,
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createModelChange(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (modelKey: string) => {
    const settings = state.settings;
    if (!settings || feedback.isActionPending("model")) return settings;
    const targetModel = findTargetModel(settings, modelKey);
    if (isCurrentModel(settings, targetModel)) return settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("model", "模型切换", `正在切换到 ${targetModel.display_name}。`),
      execute: async () => updateSettings({ model: targetModel }),
      successDetail: `当前模型已切换为 ${targetModel.display_name}。`,
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createDirectoryPromptChange(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (enabled: boolean) => {
    const settings = state.settings;
    if (!settings || enabled === settings.directory_prompt_enabled || feedback.isActionPending("directoryPrompt")) return settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("directoryPrompt", "目录提醒开关", readTogglePendingText("新目录首次接触提醒", enabled)),
      execute: async () => updateSettings({ directory_prompt_enabled: enabled }),
      successDetail: readToggleSuccessText("新目录首次接触提醒", enabled),
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createRiskLevelChange(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (enabled: boolean) => {
    const settings = state.settings;
    if (!settings || enabled === settings.show_risk_level || feedback.isActionPending("riskLevel")) return settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("riskLevel", "风险等级展示开关", readTogglePendingText("显示风险等级", enabled)),
      execute: async () => updateSettings({ show_risk_level: enabled }),
      successDetail: readToggleSuccessText("显示风险等级", enabled),
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createApprovalRevoke(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (rootPath: string) => {
    if (!state.settings || feedback.isActionPending("revokeApproval")) return state.settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("revokeApproval", "移除目录授权", `正在移除目录授权：${rootPath}`),
      execute: async () => updateSettings({ revoke_directory_root: rootPath }),
      successDetail: `已移除目录授权：${rootPath}`,
    }, feedback, state)).then(applyNextSettings(state.setSettings));
  };
}

function createExternalConnectionAction(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async (slotId: string, action: "validate" | "recheck") => {
    if (!state.settings || feedback.isActionPending("externalConnection")) return state.settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("externalConnection", "外部连接动作", `正在处理 ${slotId} 的${readExternalActionLabel(action)}。`),
      execute: async () => {
        const result = await runExternalConnectionAction({ slot_id: slotId, action });
        state.setSettings((current) => current ? mergeExternalConnections(current, result) : current);
        if (!result.ok) throw new Error(result.message);
        return result;
      },
      successDetail: `${slotId} ${readExternalActionLabel(action)}完成。`,
    }, feedback, state));
  };
}

function createDiagnosticsCheck(
  state: ReturnType<typeof useSettingsState>,
  feedback: ReturnType<typeof useSettingsFeedback>,
) {
  return async () => {
    if (!state.settings || feedback.isActionPending("diagnosticsCheck")) return state.settings;
    return runSettingsAction(buildActionRunner({
      action: createFeedback("diagnosticsCheck", "环境重新检测", "正在重新检测当前环境。"),
      execute: checkDiagnostics,
      successDetail: "诊断结果已更新。",
    }, feedback, state)).then(applyDiagnosticsResult(state.setSettings));
  };
}

function readWorkspaceName(settings: SettingsResponse, workspaceId: string) {
  return settings.available_workspaces.find((item) => item.workspace_id === workspaceId)?.name || workspaceId;
}

function readModeLabel(mode: string) {
  if (mode === "observe") return "观察模式";
  if (mode === "full_access") return "全权限模式";
  return "标准模式";
}

function readTogglePendingText(label: string, enabled: boolean) {
  return `正在${enabled ? "开启" : "关闭"}${label}。`;
}

function readToggleSuccessText(label: string, enabled: boolean) {
  return `${label}已${enabled ? "开启" : "关闭"}。`;
}

function readExternalActionLabel(action: "validate" | "recheck") {
  return action === "validate" ? "校验" : "重检";
}

function findTargetModel(settings: SettingsResponse, modelKey: string) {
  const [providerId, modelId] = modelKey.split(":", 2);
  const targetModel = settings.available_models.find(
    (item) => item.provider_id === providerId && item.model_id === modelId,
  );
  if (!targetModel) throw new Error("目标模型不存在");
  if (!targetModel.available || !targetModel.enabled) {
    throw new Error(`模型 ${targetModel.display_name} 当前不可用`);
  }
  return targetModel;
}

function isCurrentModel(settings: SettingsResponse, targetModel: SettingsResponse["model"]) {
  return targetModel.model_id === settings.model.model_id
    && targetModel.provider_id === settings.model.provider_id;
}
