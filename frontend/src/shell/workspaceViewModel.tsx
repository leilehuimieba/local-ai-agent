import { ChatPanel } from "../chat/ChatPanel";
import { LogsPanel } from "../logs/LogsPanel";
import {
  getLatestFailureEvent,
  isBusyRunState,
  useRuntimeStore,
} from "../runtime/state";
import { SettingsPanel } from "../settings/SettingsPanel";
import { BottomLogsDrawer } from "../workspace/BottomLogsDrawer";
import { BottomPanel } from "../workspace/BottomPanel";
import { ContextSidebar } from "../workspace/ContextSidebar";
import { TopBar } from "../workspace/TopBar";
import { WorkbenchOverview } from "../workspace/WorkbenchOverview";

type AppModel = ReturnType<typeof import("../App").useWorkspaceApp>;

export function renderWorkspaceContent(app: AppModel) {
  if (app.view.currentView === "home") return renderHomeView(app);
  if (app.view.currentView === "agent") return renderAgentView(app);
  if (app.view.currentView === "logs") return renderLogsView(app);
  return renderSettingsView(app);
}

export function renderTopBar(app: AppModel) {
  return <TopBar {...getTopBarProps(app)} />;
}

export function renderHomeView(app: AppModel) {
  return (
    <section className="single-view page-container home-layout">
      <div className="home-main">
        {/* 首页只负责恢复上下文和继续入口，不承接任务推进或复盘详情。 */}
        <WorkbenchOverview {...getWorkbenchProps(app)} />
      </div>
      <ContextSidebar {...getSidebarProps(app)} />
    </section>
  );
}

export function renderBottomPanel(app: AppModel) {
  if (app.view.currentView !== "agent" && app.view.currentView !== "home") return null;
  return <BottomPanel {...getBottomPanelProps(app)} />;
}

export function renderGlobalLayers(app: AppModel) {
  const connectionBanner = renderConnectionBanner(app);
  const errorBanner = renderCriticalErrorBanner(app);
  const confirmationBanner = renderConfirmationBanner(app);
  const bannerLayer = (connectionBanner || errorBanner || confirmationBanner) ? (
    <section className="global-layer-stack">
      {connectionBanner}
      {errorBanner}
      {confirmationBanner}
    </section>
  ) : null;

  return (
    <>
      {bannerLayer}
      <BottomLogsDrawer
        isOpen={app.view.logsDrawerOpen}
        events={app.runtime.events}
        onClose={app.actions.toggleLogsDrawer}
      />
    </>
  );
}

function renderAgentView(app: AppModel) {
  return (
    <section className="single-view page-container home-layout agent-page">
      {/* 任务页是主工作区，新增任务推进能力优先接在主线程、检查器和调查抽屉。 */}
      <ChatPanel {...getChatPanelProps(app)} />
      <ContextSidebar {...getSidebarProps(app)} />
    </section>
  );
}

function renderLogsView(app: AppModel) {
  return (
    <section className="single-view">
      {/* 记录页只处理跨会话调查与复盘，不承接当前会话主线程交互。 */}
      <LogsPanel logs={app.logs.logs} />
    </section>
  );
}

function renderSettingsView(app: AppModel) {
  return (
    <section className="single-view">
      {/* 设置页只承接运行环境、权限和诊断控制。 */}
      <SettingsPanel {...getSettingsPanelProps(app)} />
    </section>
  );
}

function renderConnectionBanner(app: AppModel) {
  if (shouldHideConnectionBanner(app.runtime.connectionState)) return null;
  return (
    <div className={`global-banner banner-${app.runtime.connectionState}`} role="status" aria-live="polite">
      <div>
        <strong>连接状态</strong>
        <p>{app.connectionLabel}</p>
      </div>
      {app.runtime.canReconnect ? (
        <button type="button" className="secondary-button" onClick={app.actions.handleReconnect}>
          重新连接
        </button>
      ) : null}
    </div>
  );
}

function renderCriticalErrorBanner(app: AppModel) {
  const message = app.runtime.criticalError || app.settingsApi.bootstrapError;
  if (!message) return null;
  return (
    <div className="global-banner banner-error" role="alert">
      <div>
        <strong>严重错误</strong>
        <p>{message}</p>
      </div>
      <button
        type="button"
        className="secondary-button"
        onClick={app.actions.dismissCriticalError}
      >
        关闭提示
      </button>
    </div>
  );
}

function renderConfirmationBanner(app: AppModel) {
  if (!app.runtime.confirmation || app.view.currentView === "agent") return null;
  return (
    <div className="global-banner banner-confirmation" role="status" aria-live="assertive">
      <div>
        <strong>高优先级确认</strong>
        <p>{app.runtime.currentTaskTitle}</p>
      </div>
      <button type="button" className="secondary-button" onClick={app.actions.openAgentPage}>
        前往处理确认
      </button>
    </div>
  );
}

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
  };
}

export function getSettingsPanelProps(app: AppModel) {
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
    onDeleteMemory: (memoryId: string) =>
      void app.settingsApi.removeMemory(memoryId),
    onDirectoryPromptEnabledChange: (enabled: boolean) =>
      void app.settingsApi.changeDirectoryPromptEnabled(enabled),
    onModeChange: app.actions.handleModeChange,
    onModelChange: app.actions.handleModelChange,
    onRevokeDirectoryApproval: (rootPath: string) =>
      void app.settingsApi.revokeDirectoryApproval(rootPath),
    onRunExternalConnectionAction: (slotId: string, action: "validate" | "recheck") =>
      void app.settingsApi.runExternalConnectionAction(slotId, action),
    onCheckDiagnostics: () =>
      void app.settingsApi.checkDiagnostics(),
    onShowRiskLevelChange: (enabled: boolean) =>
      void app.settingsApi.changeShowRiskLevel(enabled),
    onRefreshMemories: () =>
      void app.settingsApi.refreshMemories(),
    onWorkspaceChange: app.actions.handleWorkspaceChange,
    pendingAction: app.settingsApi.pendingAction,
    settings: app.settingsApi.settings,
  };
}

function getTopBarProps(app: AppModel) {
  return {
    connectionLabel: app.connectionLabel,
    currentRunId: app.runtime.currentRunId,
    currentView: app.view.currentView,
    onModelChange: app.actions.handleModelChange,
    onViewChange: app.view.setCurrentView,
    onToggleLogsDrawer: app.actions.toggleLogsDrawer,
    runState: app.runtime.runState,
    sessionId: app.runtime.sessionId,
    settings: app.settingsApi.settings,
    statusLine: app.statusLine,
  };
}

function getWorkbenchProps(app: AppModel) {
  return {
    confirmation: app.runtime.confirmation,
    connectionLabel: app.connectionLabel,
    currentRunId: app.runtime.currentRunId,
    currentTaskTitle: app.runtime.currentTaskTitle,
    eventCount: app.runtime.events.length,
    latestEvent: app.runtime.events[app.runtime.events.length - 1],
    logCount: app.logs.logs.length,
    onOpenAgent: app.actions.openAgentPage,
    onOpenLogsPage: app.actions.openLogsPage,
    sessionId: app.runtime.sessionId,
    settings: app.settingsApi.settings,
    statusLine: app.statusLine,
  };
}

function getBottomPanelProps(app: AppModel) {
  return {
    currentTaskTitle: app.runtime.currentTaskTitle,
    events: app.runtime.events,
    isOpen: app.view.bottomPanelOpen,
    onOpenChange: app.view.setBottomPanelOpen,
    runState: app.runtime.runState,
    submitError: app.runtime.submitError,
  };
}

function getSidebarProps(app: AppModel) {
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
  };
}

function shouldHideConnectionBanner(connectionState: AppModel["runtime"]["connectionState"]) {
  return connectionState === "connected" || connectionState === "closed";
}
