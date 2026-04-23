import { useMemo, useState, type FormEvent } from "react";

import { readRunStateNextStep, readThreadStatusClass } from "../chat/chatResultModel";
import { ChatPanel } from "../chat/ChatPanel";
import { LogsPanel } from "../logs/LogsPanel";
import {
  getLatestFailureEvent,
  isBusyRunState,
  readUnifiedStatusFromRunState,
  readUnifiedStatusMeta,
  useRuntimeStore,
} from "../runtime/state";
import { SettingsPanel } from "../settings/SettingsPanel";
import { KnowledgeBasePanel } from "../knowledge-base/KnowledgeBasePanel";
import { BottomPanel } from "../workspace/BottomPanel";
import { TopBar } from "../workspace/TopBar";
import { WorkbenchOverview } from "../workspace/WorkbenchOverview";
import { ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";
import type { AppView, HomeIntent, LogsApi, RuntimeView, SettingsApi } from "../App";

type HomeStateKind = "first_use" | "resume" | "blocked";
type HomeAction = "reconnect" | "settings" | "workspace" | "model";
type HomeBlock = { action: HomeAction; title: string; body: string; detail: string };
type HomeActivity = { id: string; kind: "verification" | "memory" | "tool"; label: string; text: string };
type ResumeItem = { label: string; value: string };
type HomeViewState = {
  currentView: AppView;
  bottomPanelOpen: boolean;
  homeIntent: HomeIntent;
  setBottomPanelOpen: (open: boolean) => void;
  setCurrentView: (view: AppView) => void;
  showHomeCompose: () => void;
};
type AppActions = {
  dismissCriticalError: () => void;
  handleConfirmationDecision: (decision: "approve" | "reject" | "cancel") => Promise<void>;
  handleModeChange: (mode: string) => void;
  handleModelChange: (modelId: string) => void;
  handleReconnect: () => void;
  handleSubmit: (event: FormEvent<HTMLFormElement>) => void;
  handleWorkspaceChange: (workspaceId: string) => void;
  openHomeStart: () => void;
  openLogsPage: () => void;
  openSettingsPage: () => void;
  openTaskPage: () => void;
  openTaskPageForConfirmation: () => void;
  openTaskPageWithDraft: (value: string) => void;
};
type HomeViewModel = {
  kind: HomeStateKind;
  navHint: string;
  composeValue: string;
  canSubmit: boolean;
  isSubmitting: boolean;
  eventCount: number;
  hasConfirmation: boolean;
  envItems: Array<{ label: string; value: string }>;
  examples: typeof HOME_EXAMPLES;
  resumeCard: {
    recentTask: string;
    recentStage: string;
    latestSummary: string;
    nextStep: string;
    runId: string;
    sessionId: string;
    contextItems: ResumeItem[];
    evidenceItems: ResumeItem[];
  };
  systemCard: {
    judgement: string;
    connection: string;
    mode: string;
    workspace: string;
  };
  blockCard: HomeBlock | null;
  recentActivities: HomeActivity[];
  confirmationBanner: { title: string; text: string } | null;
  onComposeValueChange: (value: string) => void;
  onOpenLogsPage: () => void;
  onReconnect: () => void;
  onOpenSettingsPage: () => void;
  onOpenTaskPage: () => void;
  onOpenTaskPageForConfirmation: () => void;
  onPrefillExample: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};
type AppModel = {
  actions: AppActions;
  connectionLabel: string;
  home: HomeViewModel;
  logs: LogsApi;
  runtime: RuntimeView;
  settingsApi: SettingsApi;
  statusLine: string;
  view: HomeViewState;
};
type TaskNavEntry = {
  id: string;
  title: string;
  tag: string;
};

const HOME_EXAMPLES = [
  {
    id: "fix-file",
    label: "修改项目文件",
    prompt: "帮我检查当前项目里最需要修的一个问题，并做最小改动修复",
  },
  {
    id: "build-debug",
    label: "执行命令并排错",
    prompt: "帮我运行构建命令，定位失败原因并给出修复建议",
  },
  {
    id: "docs-summary",
    label: "整理项目说明",
    prompt: "根据 docs 和当前代码，说明这个项目现在做到什么程度",
  },
  {
    id: "knowledge-search",
    label: "检索本地知识",
    prompt: "从本地文档中检索当前项目的正式需求和验收口径",
  },
] as const;

export function renderWorkspaceContent(app: AppModel) {
  if (app.view.currentView === "home") return renderHomeView(app);
  if (app.view.currentView === "task") return renderTaskView(app);
  if (app.view.currentView === "logs") return renderLogsView(app);
  if (app.view.currentView === "knowledge") return renderKnowledgeBaseView(app);
  return renderSettingsView(app);
}

function renderKnowledgeBaseView(_app: AppModel) {
  return (
    <section className="single-view">
      <KnowledgeBasePanel />
    </section>
  );
}

export function renderTopBar(app: AppModel, rightPanelOpen: boolean, onToggleRightPanel: () => void) {
  return <TopBar {...getTopBarProps(app, rightPanelOpen, onToggleRightPanel)} />;
}

export function renderHomeView(app: AppModel) {
  return (
    <section className="single-view">
      <WorkbenchOverview {...app.home} />
    </section>
  );
}

export function renderBottomPanel(app: AppModel) {
  if (app.view.currentView !== "task") return null;
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
    </>
  );
}

function renderLogsView(app: AppModel) {
  return (
    <section className="single-view">
      <LogsPanel logs={app.logs.logs} />
    </section>
  );
}

function renderTaskView(app: AppModel) {
  return (
    <section className="single-view">
      <TaskPageToolbar app={app} />
      <ChatPanel {...getChatPanelProps(app)} />
    </section>
  );
}

function TaskPageToolbar(props: { app: AppModel }) {
  const hasConfirmation = Boolean(props.app.runtime.confirmation);
  const actionLabel = readTaskToolbarActionLabel(hasConfirmation, props.app.view.bottomPanelOpen);
  return (
    <section className="task-page-toolbar">
      <div className="task-page-toolbar-head">
        <div className="task-page-toolbar-copy">
          <span className="section-kicker">任务工作区</span>
          <h2>{props.app.runtime.currentTaskTitle || "等待任务"}</h2>
          <p>{readRunStateNextStep({ latestEvent: props.app.runtime.events[props.app.runtime.events.length - 1], runState: props.app.runtime.runState })}</p>
        </div>
        <span className={`status-badge ${readThreadStatusClass(props.app.runtime.runState)}`}>{props.app.statusLine}</span>
      </div>
      <div className="task-page-toolbar-meta">
        <TaskMetaChip label="运行" value={props.app.runtime.currentRunId || "尚未开始"} />
        <TaskMetaChip label="会话" value={props.app.runtime.sessionId || "尚未创建"} />
        <TaskMetaChip label="工作区" value={props.app.settingsApi.settings?.workspace.name || "未加载"} />
      </div>
      <div className="task-page-toolbar-actions">
        <button type="button" className="primary-action" onClick={() => handleTaskToolbarPrimary(props.app, hasConfirmation)}>{actionLabel}</button>
        <button type="button" className="secondary-button" onClick={props.app.actions.openLogsPage}>查看记录页</button>
        <button type="button" className="secondary-button" onClick={props.app.actions.openHomeStart}>新建任务</button>
      </div>
    </section>
  );
}

function handleTaskToolbarPrimary(app: AppModel, hasConfirmation: boolean) {
  if (hasConfirmation) {
    app.actions.openTaskPageForConfirmation();
    return;
  }
  app.view.setBottomPanelOpen(!app.view.bottomPanelOpen);
}

function readTaskToolbarActionLabel(hasConfirmation: boolean, isBottomPanelOpen: boolean) {
  if (hasConfirmation) return "处理待确认";
  return isBottomPanelOpen ? "收起调查层" : "展开调查层";
}

function TaskMetaChip(props: { label: string; value: string }) {
  return (
    <div className="summary-chip">
      <span>{props.label}</span>
      <strong>{props.value}</strong>
    </div>
  );
}

function TaskLeftNav(props: { app: AppModel }) {
  const [expanded, setExpanded] = useState(true);
  const [search, setSearch] = useState("");
  const groups = useMemo(() => buildTaskNavGroups(props.app), [props.app]);
  const pinnedItems = useMemo(
    () => filterTaskNavEntries(groups.pinned, search),
    [groups.pinned, search],
  );
  const recentItems = useMemo(
    () => filterTaskNavEntries(groups.recent, search),
    [groups.recent, search],
  );
  return (
    <aside className={readTaskLeftNavClass(expanded)} aria-label="任务页导航">
      <TaskNavRail app={props.app} expanded={expanded} onToggleExpand={() => setExpanded((value) => !value)} />
      {expanded ? (
        <TaskNavPanel
          app={props.app}
          search={search}
          pinnedItems={pinnedItems}
          recentItems={recentItems}
          onSearchChange={setSearch}
        />
      ) : null}
    </aside>
  );
}

function readTaskNavClass(currentView: AppView, nav: AppView) {
  return currentView === nav ? "task-nav-button icon-only active" : "task-nav-button icon-only";
}

function readTaskLeftNavClass(expanded: boolean) {
  return expanded ? "task-left-nav expanded" : "task-left-nav";
}

function TaskNavRail(props: { app: AppModel; expanded: boolean; onToggleExpand: () => void }) {
  return (
    <nav className="task-nav-rail" aria-label="任务快捷导航">
      <button type="button" className="task-nav-button icon-only" data-label={props.expanded ? "收起侧栏" : "展开侧栏"} aria-label={props.expanded ? "收起侧栏" : "展开侧栏"} onClick={props.onToggleExpand}><span className="task-nav-icon" aria-hidden="true">☰</span></button>
      <NavIconButton app={props.app} nav="home" icon="⌂" label="首页" />
      <NavIconButton app={props.app} nav="task" icon="◉" label="任务" />
      <NavIconButton app={props.app} nav="logs" icon="≡" label="记录" />
      <NavIconButton app={props.app} nav="knowledge" icon="📚" label="知识库" />
      <NavIconButton app={props.app} nav="settings" icon="⚙" label="设置" />
      <button type="button" className="task-nav-button icon-only task-nav-action" data-label="新任务" aria-label="新任务" onClick={props.app.actions.openHomeStart}><span className="task-nav-icon" aria-hidden="true">＋</span></button>
    </nav>
  );
}

function NavIconButton(props: { app: AppModel; nav: AppView; icon: string; label: string }) {
  return (
    <button
      type="button"
      className={readTaskNavClass(props.app.view.currentView, props.nav)}
      data-label={props.label}
      aria-label={props.label}
      onClick={() => props.app.view.setCurrentView(props.nav)}
    >
      <span className="task-nav-icon" aria-hidden="true">{props.icon}</span>
    </button>
  );
}

function TaskNavPanel(props: {
  app: AppModel;
  search: string;
  pinnedItems: TaskNavEntry[];
  recentItems: TaskNavEntry[];
  onSearchChange: (value: string) => void;
}) {
  return (
    <section className="task-nav-panel">
      <header className="task-nav-panel-head"><strong>任务</strong><span>会话导航</span></header>
      <button type="button" className="task-nav-primary" onClick={props.app.actions.openHomeStart}>新建任务</button>
      <input id="task_nav_search" name="task_nav_search" className="task-nav-search" type="search" value={props.search} placeholder="搜索任务与历史" aria-label="搜索任务与历史" autoComplete="off" onChange={(event) => props.onSearchChange(event.target.value)} />
      <TaskNavGroup title="置顶" empty="暂无置顶任务" items={props.pinnedItems} onPick={(value) => props.app.actions.openTaskPageWithDraft(value)} />
      <TaskNavGroup title="最近" empty="暂无历史记录" items={props.recentItems} onPick={(value) => props.app.actions.openTaskPageWithDraft(value)} />
    </section>
  );
}

function TaskNavGroup(props: { title: string; empty: string; items: TaskNavEntry[]; onPick: (value: string) => void }) {
  return (
    <section className="task-nav-group">
      <header>{props.title}</header>
      {props.items.length === 0 ? <p className="task-nav-empty">{props.empty}</p> : props.items.map((item) => (
        <button key={item.id} type="button" className="task-history-item" onClick={() => props.onPick(item.title)}>
          <span>{item.title}</span>
          <em>{item.tag}</em>
        </button>
      ))}
    </section>
  );
}

function buildTaskNavGroups(app: AppModel) {
  return {
    pinned: buildPinnedTaskEntries(app),
    recent: buildRecentTaskEntries(app),
  };
}

function buildPinnedTaskEntries(app: AppModel) {
  const items: TaskNavEntry[] = [];
  if (hasUsefulTaskTitle(app.runtime.currentTaskTitle)) items.push({ id: "current", title: app.runtime.currentTaskTitle, tag: readRunStateTag(app.runtime.runState) });
  if (app.runtime.confirmation) items.push({ id: `confirm-${app.runtime.confirmation.confirmation_id || "pending"}`, title: app.runtime.confirmation.action_summary || "待确认操作", tag: "待确认" });
  if (app.runtime.runState === "failed") items.push({ id: "failed", title: "上次执行失败，建议重试或调整描述", tag: "恢复建议" });
  return dedupeTaskNavEntries(items).slice(0, 4);
}

function buildRecentTaskEntries(app: AppModel) {
  const userItems = app.runtime.messages.filter((item) => item.role === "user").map((item) => ({
    id: item.id,
    tag: "历史",
    title: readTaskNavTitle(item.content),
  }));
  const suggestions = HOME_EXAMPLES.map((item) => ({ id: `example-${item.id}`, tag: "模板", title: item.prompt }));
  const fallback = hasUsefulTaskTitle(app.runtime.currentTaskTitle)
    ? [{ id: "recent-current", title: app.runtime.currentTaskTitle, tag: "当前" }]
    : suggestions;
  return dedupeTaskNavEntries([...userItems, ...fallback]).slice(0, 12);
}

function filterTaskNavEntries(items: TaskNavEntry[], search: string) {
  const keyword = search.trim().toLowerCase();
  if (!keyword) return items;
  return items.filter((item) => item.title.toLowerCase().includes(keyword) || item.tag.toLowerCase().includes(keyword));
}

function dedupeTaskNavEntries(items: TaskNavEntry[]) {
  const map = new Map<string, TaskNavEntry>();
  items.forEach((item) => {
    const key = item.title.trim().toLowerCase();
    if (!key || map.has(key)) return;
    map.set(key, item);
  });
  return [...map.values()];
}

function readTaskNavTitle(value: string) {
  const text = value.replace(/\s+/g, " ").trim();
  if (!text) return "未命名任务";
  return text.length > 46 ? `${text.slice(0, 46)}…` : text;
}

function readRunStateTag(runState: RuntimeView["runState"]) {
  if (runState === "archived") return "最新";
  if (runState === "idle") return "最新";
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(runState)).label;
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
  const message = readCriticalErrorBannerMessage(app);
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

function readCriticalErrorBannerMessage(
  app: Pick<AppModel, "runtime" | "settingsApi">,
) {
  if (app.runtime.criticalError === "home_preview_blocked") return app.settingsApi.bootstrapError;
  return app.runtime.criticalError || app.settingsApi.bootstrapError;
}

function renderConfirmationBanner(app: AppModel) {
  if (!app.home.confirmationBanner || app.view.currentView !== "home") return null;
  return (
    <div className="global-banner banner-confirmation" role="status" aria-live="assertive">
      <div>
        <strong>{app.home.confirmationBanner.title}</strong>
        <p>{app.home.confirmationBanner.text}</p>
      </div>
      <button
        type="button"
        className="secondary-button"
        onClick={app.actions.openTaskPageForConfirmation}
      >
        前往任务页处理
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
    onTestProvider: (providerId: string, apiKey: string, baseURL?: string) => app.settingsApi.testProvider(providerId, apiKey, baseURL),
    onRefreshMemories: () => void app.settingsApi.refreshMemories(),
    onWorkspaceChange: app.actions.handleWorkspaceChange,
  };
}

function getTopBarProps(app: AppModel, rightPanelOpen: boolean, onToggleRightPanel: () => void) {
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

function getBottomPanelProps(app: AppModel) {
  return {
    currentRunId: app.runtime.currentRunId,
    currentTaskTitle: app.runtime.currentTaskTitle,
    events: app.runtime.events,
    isOpen: app.view.bottomPanelOpen,
    onOpenChange: app.view.setBottomPanelOpen,
    runState: app.runtime.runState,
    submitError: app.runtime.submitError,
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

function shouldHideConnectionBanner(connectionState: AppModel["runtime"]["connectionState"]) {
  return connectionState === "connected" || connectionState === "closed";
}

export function buildHomeViewModel(
  app: Pick<AppModel, "actions" | "connectionLabel" | "runtime" | "settingsApi" | "statusLine" | "view">,
): HomeViewModel {
  const block = readHomeBlock(app);
  const kind = readHomeStateKind(app, block);
  return {
    kind,
    navHint: readHomeNavHint(kind),
    envItems: buildHomeEnvItems(app),
    examples: HOME_EXAMPLES,
    composeValue: app.runtime.composeValue,
    canSubmit: Boolean(app.settingsApi.settings),
    eventCount: app.runtime.events.length,
    hasConfirmation: Boolean(app.runtime.confirmation),
    isSubmitting: isBusyRunState(app.runtime.runState),
    confirmationBanner: readConfirmationBannerModel(app.runtime.confirmation),
    resumeCard: buildResumeCard(app),
    systemCard: buildSystemCard(app, block),
    blockCard: block,
    recentActivities: readRecentActivities(app.runtime.events),
    onComposeValueChange: useRuntimeStore.getState().setComposeValue,
    onOpenLogsPage: app.actions.openLogsPage,
    onReconnect: app.actions.handleReconnect,
    onOpenSettingsPage: app.actions.openSettingsPage,
    onOpenTaskPage: app.actions.openTaskPage,
    onOpenTaskPageForConfirmation: app.actions.openTaskPageForConfirmation,
    onPrefillExample: app.actions.openTaskPageWithDraft,
    onSubmit: app.actions.handleSubmit,
  };
}

function readHomeStateKind(
  app: Pick<AppModel, "runtime" | "view">,
  block: HomeBlock | null,
): HomeStateKind {
  if (block) return "blocked" as HomeStateKind;
  if (app.view.homeIntent === "compose") return "first_use" as HomeStateKind;
  return hasRecoverableContext(app.runtime) ? "resume" : "first_use";
}

function readHomeBlock(
  app: Pick<AppModel, "connectionLabel" | "runtime" | "settingsApi">,
): HomeBlock | null {
  const settings = app.settingsApi.settings;
  if (!settings && app.settingsApi.bootstrapError) {
    return createBlock("settings", "设置当前未成功加载", "当前基础配置没有正常加载，建议先检查设置页和运行环境。", app.settingsApi.bootstrapError);
  }
  if (app.runtime.criticalError === "home_preview_blocked") {
    return createBlock("reconnect", "运行时当前不可达", "系统当前无法建立执行链路，新任务会提交失败或无法继续。", "预览模式：模拟 Runtime 不可达");
  }
  if (app.runtime.criticalError) {
    return createBlock("settings", "当前存在需要优先处理的错误", "系统当前存在需要优先处理的全局错误，建议先检查设置页和运行环境。", app.runtime.criticalError);
  }
  if (!settings) return null;
  if (!settings.runtime_status.ok) {
    return createBlock("reconnect", "运行时当前不可达", "系统当前无法建立执行链路，新任务会提交失败或无法继续。", app.connectionLabel);
  }
  if (!isWorkspaceReady(settings)) {
    return createBlock("workspace", "当前工作区不可访问", "当前工作区路径不可访问，继续任务可能失败或产生错误结果。", settings.workspace.root_path || "工作区未配置");
  }
  if (!isModelReady(settings)) {
    return createBlock("model", "当前模型配置不可用", "当前模型不可用于继续任务，建议先切换到可用模型。", settings.model.display_name || "模型未配置");
  }
  return null;
}

function createBlock(action: HomeAction, title: string, body: string, detail: string) {
  return { action, body, detail, title };
}

function isWorkspaceReady(settings: SettingsResponse) {
  return Boolean(settings.workspace.root_path)
    && settings.available_workspaces.some((item) => item.workspace_id === settings.workspace.workspace_id);
}

function isModelReady(settings: SettingsResponse) {
  return settings.model.available !== false && settings.model.enabled !== false;
}

function hasRecoverableContext(runtime: RuntimeView) {
  return Boolean(
    runtime.currentRunId
    || runtime.sessionId
    || runtime.events.length
    || runtime.confirmation
    || hasUsefulTaskTitle(runtime.currentTaskTitle),
  );
}

function hasUsefulTaskTitle(title: string) {
  return Boolean(title && title !== "等待第一条任务");
}

function readHomeNavHint(kind: HomeStateKind) {
  if (kind === "blocked") return "首页阻塞处理";
  if (kind === "resume") return "首页恢复入口";
  return "首页快速开始";
}

function buildHomeEnvItems(
  app: Pick<AppModel, "runtime" | "settingsApi" | "statusLine">,
): HomeViewModel["envItems"] {
  const settings = app.settingsApi.settings;
  return [
    { label: "模型", value: settings?.model.display_name || "未加载" },
    { label: "模式", value: readModeLabel(settings?.mode) },
    { label: "工作区", value: settings?.workspace.name || "未加载" },
    { label: "Runtime", value: readRuntimeLabel(settings, app.statusLine) },
  ];
}

function buildResumeCard(app: Pick<AppModel, "runtime">): HomeViewModel["resumeCard"] {
  const latestEvent = app.runtime.events[app.runtime.events.length - 1];
  const latestContextEvent = findLatestEvent(app.runtime.events, hasResumeContextSignal);
  const snapshot = latestContextEvent?.context_snapshot;
  return {
    latestSummary: latestEvent?.summary || "当前还没有新的运行摘要。",
    nextStep: readResumeNextStep(app.runtime, latestEvent),
    recentTask: hasUsefulTaskTitle(app.runtime.currentTaskTitle) ? app.runtime.currentTaskTitle : "等待继续的任务",
    recentStage: latestEvent?.stage || "等待运行",
    runId: app.runtime.currentRunId || "等待生成",
    sessionId: app.runtime.sessionId || "尚未创建",
    contextItems: buildResumeContextItems(snapshot),
    evidenceItems: buildResumeEvidenceItems(latestEvent),
  };
}

function buildResumeContextItems(snapshot?: RunEvent["context_snapshot"]) {
  return [
    {
      label: "会话与记忆",
      value: readCompactResumeValue(
        snapshot?.session_summary,
        snapshot?.memory_digest,
        "当前还没有会话或记忆摘要。",
      ),
    },
    {
      label: "知识与思考",
      value: readCompactResumeValue(
        snapshot?.knowledge_digest,
        snapshot?.reasoning_summary,
        "当前还没有知识或思考线索。",
      ),
    },
  ];
}

function buildResumeEvidenceItems(event?: RunEvent) {
  return [
    {
      label: "结果与验证",
      value: readCompactResumeValue(
        event?.result_summary || event?.summary,
        event?.verification_summary || event?.verification_snapshot?.summary,
        "当前还没有结果与验证摘要。",
      ),
    },
  ];
}

function readCompactResumeValue(primary?: string, secondary?: string, fallback?: string) {
  if (primary && secondary) return `${primary} / ${secondary}`;
  return primary || secondary || fallback || "暂无信息。";
}

function readResumeNextStep(runtime: Pick<RuntimeView, "confirmation" | "runState">, event?: RunEvent) {
  if (runtime.confirmation) return runtime.confirmation.action_summary;
  if (event?.metadata?.next_step) return event.metadata.next_step;
  if (!event) return "进入任务页后，系统会给出下一步建议。";
  return readRunStateNextStep({ latestEvent: event, runState: runtime.runState });
}

function buildSystemCard(
  app: Pick<AppModel, "connectionLabel" | "runtime" | "settingsApi">,
  block: HomeBlock | null,
): HomeViewModel["systemCard"] {
  return {
    judgement: readSystemJudgement(app.runtime, app.settingsApi.settings, block),
    mode: readModeLabel(app.settingsApi.settings?.mode),
    workspace: app.settingsApi.settings?.workspace.name || "未加载",
    connection: app.connectionLabel,
  };
}

function readSystemJudgement(
  runtime: RuntimeView,
  settings: SettingsResponse | null,
  block: HomeBlock | null,
) {
  if (block) return block.body;
  if (runtime.confirmation) return runtime.confirmation.action_summary;
  if (!settings?.runtime_status.ok) return "运行时不可达，新的任务执行会失败。";
  return "当前没有新的风险或阻塞。";
}

function readConfirmationBannerModel(
  confirmation: ConfirmationRequest | null,
): HomeViewModel["confirmationBanner"] {
  if (!confirmation) return null;
  return {
    title: "当前有待处理确认",
    text: confirmation.action_summary,
  };
}

function readRecentActivities(events: RunEvent[]): HomeActivity[] {
  const items: HomeActivity[] = [];
  for (const event of [...events].reverse()) {
    const activity = toRecentActivity(event);
    if (!activity || items.some((item) => item.kind === activity.kind)) continue;
    items.push(activity);
    if (items.length === 3) break;
  }
  return items;
}

function toRecentActivity(event: RunEvent): HomeActivity | null {
  if (hasVerificationActivity(event)) {
    return { id: `${event.event_id}-verification`, kind: "verification", label: "最近验证结果", text: event.verification_summary || event.verification_snapshot?.summary || event.summary };
  }
  if (hasMemoryActivity(event)) {
    return { id: `${event.event_id}-memory`, kind: "memory", label: "最近记忆痕迹", text: event.context_snapshot?.memory_digest || event.context_snapshot?.knowledge_digest || event.summary };
  }
  if (hasToolActivity(event)) {
    return { id: `${event.event_id}-tool`, kind: "tool", label: "最近工具动作", text: event.tool_display_name || event.tool_call_snapshot?.display_name || event.tool_name || event.summary };
  }
  return null;
}

function hasVerificationActivity(event: RunEvent) {
  return Boolean(event.verification_summary || event.verification_snapshot?.summary);
}

function hasMemoryActivity(event: RunEvent) {
  return Boolean(event.context_snapshot?.memory_digest || event.context_snapshot?.knowledge_digest);
}

function hasToolActivity(event: RunEvent) {
  return Boolean(event.tool_display_name || event.tool_call_snapshot?.display_name || event.tool_name);
}

function findLatestEvent(events: RunEvent[], predicate: (event: RunEvent) => boolean) {
  return [...events].reverse().find(predicate);
}

function hasResumeContextSignal(event: RunEvent) {
  return Boolean(
    event.context_snapshot?.session_summary
      || event.context_snapshot?.memory_digest
      || event.context_snapshot?.knowledge_digest
      || event.context_snapshot?.reasoning_summary,
  );
}

function readRuntimeLabel(settings: SettingsResponse | null, fallback: string) {
  if (!settings) return fallback;
  return settings.runtime_status.ok
    ? settings.runtime_status.name
    : `${settings.runtime_status.name} 不可达`;
}

function readModeLabel(mode?: string) {
  if (mode === "observe") return "观察模式";
  if (mode === "full_access") return "全权限模式";
  if (mode === "standard") return "标准模式";
  return "未加载";
}
