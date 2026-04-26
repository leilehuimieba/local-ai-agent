import { useMemo, useState, type FormEvent } from "react";


import { ChatPanel } from "../chat/ChatPanel";
import { LogsPanel } from "../logs/LogsPanel";
import { ReleaseWizardPanel } from "../release/ReleaseWizardPanel";
import {
  getLatestFailureEvent,
  isBusyRunState,
  readUnifiedStatusFromRunState,
  readUnifiedStatusMeta,
  useRuntimeStore,
} from "../runtime/state";
import { SettingsPanel } from "../settings/SettingsPanel";
import { KnowledgeBasePanel } from "../knowledge-base/KnowledgeBasePanel";
import { TopBar } from "../workspace/TopBar";
import { WorkbenchOverview } from "../workspace/WorkbenchOverview";
import { ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";
import type { AppView, LogsApi, RuntimeView, SettingsApi } from "../App";

export type HomeStateKind = "first_use" | "resume" | "blocked";
export type HomeAction = "reconnect" | "settings" | "workspace" | "model";
export type HomeBlock = { action: HomeAction; title: string; body: string; detail: string };
export type HomeActivity = { id: string; kind: "verification" | "memory" | "tool"; label: string; text: string };
export type ResumeItem = { label: string; value: string };
type HomeViewState = {
  currentView: AppView;
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
  openReleasePage: () => void;
  openSettingsPage: () => void;
  openTaskPage: () => void;
  openTaskPageForConfirmation: () => void;
  openTaskPageWithDraft: (value: string) => void;
};
export type HomeViewModel = {
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
  onOpenReleasePage: () => void;
  onReconnect: () => void;
  onOpenSettingsPage: () => void;
  onOpenTaskPage: () => void;
  onOpenTaskPageForConfirmation: () => void;
  onPrefillExample: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};
export type AppModel = {
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

export const HOME_EXAMPLES = [
  {
    id: "project-status",
    label: "检查项目状态",
    prompt: "检查当前项目状态，告诉我现在做到哪、卡在哪里、下一步最小动作是什么",
  },
  {
    id: "prelaunch-test",
    label: "上线前检查",
    prompt: "运行上线前检查；如果失败，请说明原因、影响范围、不影响什么和建议修复",
  },
  {
    id: "safe-change",
    label: "安全修改功能",
    prompt: "帮我修改一个最有价值的小功能；先说明会影响哪些文件，再做最小改动并验证",
  },
  {
    id: "continue-work",
    label: "继续上次任务",
    prompt: "继续上次任务：先读取当前状态和活跃 change，再给出下一步建议",
  },
] as const;

export function renderWorkspaceContent(app: AppModel) {
  // Home view merged into task view; drawer handles non-task views
  return renderTaskView(app);
}

export function renderDrawerContent(app: AppModel) {
  if (app.view.currentView === "settings") return <SettingsPanel {...getSettingsPanelProps(app)} />;
  if (app.view.currentView === "knowledge") return <KnowledgeBasePanel />;
  return null;
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

export function renderGlobalLayers(app: AppModel) {
  const errorBanner = renderCriticalErrorBanner(app);
  if (!errorBanner) return null;
  return <section className="global-layer-stack">{errorBanner}</section>;
}


export function renderMainView(app: AppModel) {
  if (app.view.currentView === "logs") return renderLogsView(app);
  if (app.view.currentView === "release") return renderReleaseView();
  return renderTaskView(app);
}

function renderReleaseView() {
  return (
    <section className="single-view">
      <ReleaseWizardPanel />
    </section>
  );
}

export function renderLogsView(app: AppModel) {
  return (
    <section className="single-view">
      <LogsPanel logs={app.logs.logs} />
    </section>
  );
}

export function renderTaskView(app: AppModel) {
  return (
    <section className="single-view task-chat-layout">
      <ChatPanel {...getChatPanelProps(app)} />
    </section>
  );
}

export function TaskLeftNav(props: { app: AppModel }) {
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
  const isTaskView = props.app.view.currentView === "task";
  return (
    <aside className={readTaskLeftNavClass(expanded)} aria-label="任务页导航">
      <TaskNavRail app={props.app} expanded={expanded} onToggleExpand={() => setExpanded((value) => !value)} />
      {expanded && isTaskView ? (
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
      <button type="button" className="task-nav-button icon-only" title={props.expanded ? "收起侧栏" : "展开侧栏"} aria-label={props.expanded ? "收起侧栏" : "展开侧栏"} onClick={props.onToggleExpand}><span className="task-nav-icon" aria-hidden="true">☰</span></button>
      <NavIconButton app={props.app} nav="task" icon="◉" label="任务" />
      <NavIconButton app={props.app} nav="logs" icon="≡" label="历史" />
      <NavIconButton app={props.app} nav="release" icon="⇧" label="上线" />
      <NavIconButton app={props.app} nav="knowledge" icon="📚" label="知识库" />
      <NavIconButton app={props.app} nav="settings" icon="⚙" label="设置" />
      <button type="button" className="task-nav-button icon-only task-nav-action" title="新任务" aria-label="新任务" onClick={props.app.actions.openHomeStart}><span className="task-nav-icon" aria-hidden="true">＋</span></button>
    </nav>
  );
}

function NavIconButton(props: { app: AppModel; nav: AppView; icon: string; label: string }) {
  return (
    <button
      type="button"
      className={readTaskNavClass(props.app.view.currentView, props.nav)}
      title={props.label}
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

function hasUsefulTaskTitle(title: string) {
  return Boolean(title && title !== "等待第一条任务");
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


