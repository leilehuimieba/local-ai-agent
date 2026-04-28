import { useRuntimeStore, isBusyRunState, getLatestFailureEvent } from "../../runtime/state";
import type { AppModel } from "./types";

export function getChatPanelProps(app: AppModel) {
  const state = useRuntimeStore.getState();
  return {
    composeValue: app.runtime.composeValue,
    confirmation: app.runtime.confirmation,
    currentRunId: app.runtime.currentRunId,
    currentTaskTitle: app.runtime.currentTaskTitle,
    events: app.runtime.events,
    isRunning: isBusyRunState(app.runtime.runState),
    latestFailureEvent: getLatestFailureEvent(app.runtime.events),
    messages: app.runtime.messages,
    onComposeValueChange: state.setComposeValue,
    onConfirmationDecision: app.actions.handleConfirmationDecision,
    onRememberChoiceChange: state.setRememberChoice,
    onSubmit: app.actions.handleSubmit,
    rememberChoice: app.runtime.rememberChoice,
    runState: app.runtime.runState,
    settings: app.settingsApi.settings,
    showRiskLevel: app.settingsApi.settings?.show_risk_level ?? true,
    statusLine: app.statusLine,
    submitError: app.runtime.submitError,
    onExampleClick: app.actions.openTaskPageWithDraft,
  };
}

export function getSettingsPanelProps(app: AppModel) {
  return {
    ...buildSettingsPanelState(app),
    ...buildSettingsPanelHandlers(app),
  };
}

function buildSettingsPanelState(app: AppModel) {
  return {
    actionError: app.settingsApi.actionError,
    bootstrapError: app.settingsApi.bootstrapError,
    deletingMemoryId: app.settingsApi.deletingMemoryId,
    isRunning: isBusyRunState(app.runtime.runState),
    isActionPending: app.settingsApi.isActionPending,
    lastSuccess: app.settingsApi.lastSuccess,
    memoryActionError: app.settingsApi.memoryActionError,
    memoryActionSuccess: app.settingsApi.memoryActionSuccess,
    memoryPendingAction: app.settingsApi.memoryPendingAction,
    memories: app.settingsApi.memories,
    memoryError: app.settingsApi.memoryError,
    pendingAction: app.settingsApi.pendingAction,
    providerActions: app.settingsApi.providerActions,
    providerBootstrapError: app.settingsApi.providerBootstrapError,
    providerSettings: app.settingsApi.providerSettings,
    settings: app.settingsApi.settings,
  };
}

function buildSettingsPanelHandlers(app: AppModel) {
  return {
    onApplyProvider: (providerId: string) => app.settingsApi.applyProvider(providerId),
    onDeleteMemory: (memoryId: string) => void app.settingsApi.removeMemory(memoryId),
    onDirectoryPromptEnabledChange: (enabled: boolean) => void app.settingsApi.changeDirectoryPromptEnabled(enabled),
    onModeChange: app.actions.handleModeChange,
    onModelChange: app.actions.handleModelChange,
    onRefreshProviderSettings: () => app.settingsApi.refreshProviderSettings(),
    onRevokeDirectoryApproval: (rootPath: string) => void app.settingsApi.revokeDirectoryApproval(rootPath),
    onRemoveProvider: (providerId: string) => app.settingsApi.removeProvider(providerId),
    onRunExternalConnectionAction: (slotId: string, action: "validate" | "recheck") => void app.settingsApi.runExternalConnectionAction(slotId, action),
    onCheckDiagnostics: () => void app.settingsApi.checkDiagnostics(),
    onSaveProvider: (providerId: string, apiKey: string) => app.settingsApi.saveProvider(providerId, apiKey),
    onShowRiskLevelChange: (enabled: boolean) => void app.settingsApi.changeShowRiskLevel(enabled),
    onEmbeddingProviderChange: (providerId: string) => void app.settingsApi.changeEmbeddingProvider(providerId),
    onTestProvider: (providerId: string, apiKey: string, baseURL?: string) => app.settingsApi.testProvider(providerId, apiKey, baseURL),
    onRefreshMemories: () => void app.settingsApi.refreshMemories(),
    onWorkspaceChange: app.actions.handleWorkspaceChange,
  };
}

export function getTopBarProps(app: AppModel, rightPanelOpen: boolean, onToggleRightPanel: () => void) {
  return {
    connectionLabel: app.connectionLabel,
    currentRunId: app.runtime.currentRunId,
    currentView: app.view.currentView,
    homeStateHint: app.home.navHint,
    onOpenHomeStart: app.actions.openHomeStart,
    onViewChange: app.view.setCurrentView,
    rightPanelOpen,
    onToggleRightPanel,
    runState: app.runtime.runState,
    sessionId: app.runtime.sessionId,
    settings: app.settingsApi.settings,
    statusLine: app.statusLine,
  };
}

export function getSidebarProps(app: AppModel, variant: "home" | "task") {
  return {
    bootstrapError: app.settingsApi.bootstrapError,
    confirmation: app.runtime.confirmation,
    connectionLabel: app.connectionLabel,
    connectionState: app.runtime.connectionState,
    currentRunId: app.runtime.currentRunId,
    events: app.runtime.events,
    runState: app.runtime.runState,
    sessionId: app.runtime.sessionId,
    settings: app.settingsApi.settings,
    statusLine: app.statusLine,
    variant,
  };
}
