import { FormEvent, startTransition, useEffect, useMemo, useState } from "react";
import { useShallow } from "zustand/react/shallow";

import { submitChatRun } from "./chat/api";
import { submitConfirmationDecision } from "./confirmations/api";
import { useSessionEventStream } from "./events/useSessionEventStream";
import { useLogs } from "./logs/useLogs";
import {
  getConnectionStateLabel,
  getRunStateLabel,
  useRuntimeStore,
} from "./runtime/state";
import { AppShell } from "./shell/AppShell";
import {
  buildHomeViewModel,
  renderBottomPanel,
  renderGlobalLayers,
  renderTopBar,
  renderWorkspaceContent,
} from "./shell/workspaceViewModel";
import { useSettings } from "./settings/useSettings";

export type AppView = "home" | "task" | "logs" | "settings";
export type RuntimeView = ReturnType<typeof useRuntimeView>;
export type ViewState = ReturnType<typeof useViewState>;
export type SettingsApi = ReturnType<typeof useSettings>;
export type LogsApi = ReturnType<typeof useLogs>;
export type RuntimeActions = ReturnType<typeof useRuntimeActions>;
type ConfirmationDecision = "approve" | "reject" | "cancel";
export type HomeIntent = "auto" | "compose";
export type WorkspaceAppModel = ReturnType<typeof buildAppModel>;

function App() {
  const app = useWorkspaceApp();
  return <AppLayout app={app} />;
}

function AppLayout({ app }: { app: ReturnType<typeof useWorkspaceApp> }) {
  return (
    <AppShell
      topbar={renderTopBar(app)}
      overlays={renderGlobalLayers(app)}
      content={renderWorkspaceContent(app)}
      bottomPanel={renderBottomPanel(app)}
    />
  );
}

export function useWorkspaceApp(): WorkspaceAppModel {
  const settingsApi = useSettings();
  const runtime = useRuntimeView();
  const view = useViewState();
  const logs = useLogs(view.currentView === "logs");
  useHomePreview();
  const stream = useRuntimeStream(runtime, view);
  const actions = useAppActions(settingsApi, runtime, view, logs, stream.reconnect);
  return buildAppModel(settingsApi, runtime, view, logs, actions);
}

function useRuntimeView() {
  return useRuntimeStore(useShallow(selectRuntimeView));
}

function useRuntimeActions() {
  return useRuntimeStore(useShallow(selectRuntimeActions));
}

function useRuntimeStream(runtime: RuntimeView, view: ViewState) {
  const actions = useRuntimeActions();
  return useSessionEventStream(runtime.sessionId, {
    onConnectionChange: actions.syncConnection,
    onEvent: (payload) => applyIncomingEvent(payload, actions.applyEvent, view),
    onStreamError: actions.setCriticalError,
  });
}

function useHomePreview() {
  useEffect(() => applyHomePreview(), []);
}

function useViewState() {
  const [currentView, setCurrentView] = useState<AppView>("home");
  const [bottomPanelOpen, setBottomPanelOpen] = useState(false);
  const [homeIntent, setHomeIntent] = useState<HomeIntent>("auto");
  return {
    bottomPanelOpen,
    currentView,
    homeIntent,
    setBottomPanelOpen,
    setCurrentView: (nextView: AppView) => updateCurrentView(nextView, setCurrentView, setHomeIntent),
    showHomeCompose: () => showHomeCompose(setCurrentView, setHomeIntent),
  };
}

function useAppActions(
  settingsApi: SettingsApi,
  runtime: RuntimeView,
  view: ViewState,
  logs: LogsApi,
  reconnect: () => void,
) {
  const actions = useRuntimeActions();
  return useMemo(() => buildActions(settingsApi, runtime, view, logs, reconnect, actions), [actions, logs, reconnect, runtime, settingsApi, view]);
}

function selectRuntimeView(state: ReturnType<typeof useRuntimeStore.getState>) {
  return {
    canReconnect: state.canReconnect,
    composeValue: state.composeValue,
    confirmation: state.confirmation,
    connectionState: state.connectionState,
    criticalError: state.criticalError,
    currentRunId: state.currentRunId,
    currentTaskTitle: state.currentTaskTitle,
    events: state.events,
    latestEventAt: state.latestEventAt,
    messages: state.messages,
    rememberChoice: state.rememberChoice,
    runState: state.runState,
    sessionId: state.sessionId,
    submitError: state.submitError,
  };
}

function selectRuntimeActions(state: ReturnType<typeof useRuntimeStore.getState>) {
  return {
    acceptRun: state.acceptRun,
    applyEvent: state.applyEvent,
    clearCriticalError: state.clearCriticalError,
    failSubmission: state.failSubmission,
    resolveConfirmationDecision: state.resolveConfirmationDecision,
    setComposeValue: state.setComposeValue,
    setCriticalError: state.setCriticalError,
    setRememberChoice: state.setRememberChoice,
    setSubmitError: state.setSubmitError,
    startSubmission: state.startSubmission,
    syncConnection: state.syncConnection,
  };
}

function buildAppModel(
  settingsApi: SettingsApi,
  runtime: RuntimeView,
  view: ViewState,
  logs: LogsApi,
  actions: ReturnType<typeof buildActions>,
): {
  actions: ReturnType<typeof buildActions>;
  connectionLabel: string;
  home: ReturnType<typeof buildHomeViewModel>;
  logs: LogsApi;
  runtime: RuntimeView;
  settingsApi: SettingsApi;
  statusLine: string;
  view: ViewState;
} {
  const statusLine = getRunStateLabel(runtime.runState, runtime.events.length);
  const connectionLabel = getConnectionStateLabel(runtime.connectionState);
  const home = buildHomeViewModel({ actions, connectionLabel, runtime, settingsApi, statusLine, view });
  return { actions, connectionLabel, home, logs, runtime, settingsApi, statusLine, view };
}

function buildActions(
  settingsApi: SettingsApi,
  runtime: RuntimeView,
  view: ViewState,
  logs: LogsApi,
  reconnect: () => void,
  actions: RuntimeActions,
) {
  return {
    dismissCriticalError: () => actions.clearCriticalError(),
    handleConfirmationDecision: (decision: ConfirmationDecision) =>
      resolveConfirmation(runtime, actions, decision),
    handleModeChange: (mode: string) => void settingsApi.changeMode(mode),
    handleModelChange: (modelId: string) => void settingsApi.changeModel(modelId),
    handleReconnect: () => reconnect(),
    handleSubmit: (event: FormEvent<HTMLFormElement>) =>
      void submitTask(event, runtime, settingsApi, actions, view),
    handleWorkspaceChange: (workspaceId: string) =>
      void settingsApi.changeWorkspace(workspaceId),
    openHomeStart: () => openHomeStart(actions.setComposeValue, view),
    openLogsPage: () => openLogsPage(logs, view),
    openSettingsPage: () => openSettingsPage(view),
    openTaskPage: () => openTaskPage(runtime, view),
    openTaskPageForConfirmation: () => openTaskPageForConfirmation(runtime, view),
    openTaskPageWithDraft: (value: string) =>
      openTaskPageWithDraft(value, actions.setComposeValue, view),
  };
}

function applyIncomingEvent(
  payload: Parameters<RuntimeActions["applyEvent"]>[0],
  applyEvent: RuntimeActions["applyEvent"],
  view: ViewState,
) {
  startTransition(() => applyEvent(payload));
  if (payload.event_type === "confirmation_required" || payload.event_type === "run_failed") {
    view.setBottomPanelOpen(true);
  }
    if (payload.event_type === "confirmation_required") {
    view.setCurrentView("task");
  }
}

async function submitTask(
  event: FormEvent<HTMLFormElement>,
  runtime: RuntimeView,
  settingsApi: SettingsApi,
  actions: RuntimeActions,
  view: ViewState,
) {
  event.preventDefault();
  const userInput = runtime.composeValue.trim();
  if (!userInput || !settingsApi.settings) return;
  actions.startSubmission(userInput);
  view.setCurrentView("task");
  try {
    const payload = await submitChatRun(buildChatRunPayload(runtime, settingsApi, userInput));
    actions.acceptRun(payload.session_id, payload.run_id);
  } catch (error) {
    actions.failSubmission(readRuntimeError(error, "未知提交错误"));
  }
}

function buildChatRunPayload(
  runtime: RuntimeView,
  settingsApi: SettingsApi,
  userInput: string,
) {
  const settings = settingsApi.settings!;
  return {
    mode: settings.mode,
    model: settings.model,
    sessionId: runtime.sessionId,
    userInput,
    workspace: settings.workspace,
  };
}

async function resolveConfirmation(
  runtime: RuntimeView,
  actions: RuntimeActions,
  decision: ConfirmationDecision,
) {
  if (!runtime.confirmation) return;
  try {
    await submitConfirmationDecision(buildConfirmationPayload(runtime, decision));
    actions.resolveConfirmationDecision(decision);
  } catch (error) {
    actions.setSubmitError(readRuntimeError(error, "提交确认失败"));
    throw error;
  }
}

function buildConfirmationPayload(
  runtime: RuntimeView,
  decision: ConfirmationDecision,
) {
  return {
    confirmationId: runtime.confirmation!.confirmation_id,
    decision,
    remember: decision === "approve" ? runtime.rememberChoice : false,
    runId: runtime.confirmation!.run_id,
  };
}

function readRuntimeError(error: unknown, fallback: string) {
  return error instanceof Error ? error.message : fallback;
}

function openLogsPage(logs: LogsApi, view: ViewState) {
  logs.refresh();
  view.setCurrentView("logs");
}

function openHomeStart(
  setComposeValue: RuntimeActions["setComposeValue"],
  view: ViewState,
) {
  setComposeValue("");
  view.showHomeCompose();
}

function openSettingsPage(view: ViewState) {
  view.setCurrentView("settings");
}

function openTaskPage(runtime: RuntimeView, view: ViewState) {
  view.setCurrentView("task");
  if (shouldOpenBottomPanel(runtime)) view.setBottomPanelOpen(true);
}

function openTaskPageForConfirmation(runtime: RuntimeView, view: ViewState) {
  view.setCurrentView("task");
  view.setBottomPanelOpen(true);
  focusConfirmationCard();
}

function openTaskPageWithDraft(
  value: string,
  setComposeValue: RuntimeActions["setComposeValue"],
  view: ViewState,
) {
  setComposeValue(value);
  view.setCurrentView("task");
}

function shouldOpenBottomPanel(runtime: RuntimeView) {
  return Boolean(
    runtime.confirmation
    || runtime.runState === "failed"
    || runtime.events.length > 0,
  );
}

function updateCurrentView(
  nextView: AppView,
  setCurrentView: (view: AppView) => void,
  setHomeIntent: (value: HomeIntent) => void,
) {
  setHomeIntent("auto");
  setCurrentView(nextView);
}

function showHomeCompose(
  setCurrentView: (view: AppView) => void,
  setHomeIntent: (value: HomeIntent) => void,
) {
  setHomeIntent("compose");
  setCurrentView("home");
}

function applyHomePreview() {
  const preview = readHomePreview();
  if (!preview) return;
  if (preview === "first_use") return applyFirstUsePreview();
  if (preview === "resume") return applyResumePreview();
  if (preview === "confirmation") return applyConfirmationPreview();
  applyBlockedPreview();
}

function readHomePreview() {
  const value = new URLSearchParams(window.location.search).get("home_preview");
  return value === "first_use" || value === "resume" || value === "blocked" || value === "confirmation"
    ? value
    : null;
}

function applyFirstUsePreview() {
  useRuntimeStore.setState({
    composeValue: "",
    confirmation: null,
    connectionState: "closed",
    criticalError: null,
    currentRunId: "",
    currentTaskTitle: "等待第一条任务",
    events: [],
    latestEventAt: null,
    messages: [],
    runState: "idle",
    sessionId: "",
    submitError: null,
  });
}

function applyResumePreview() {
  useRuntimeStore.setState({
    composeValue: "",
    confirmation: null,
    connectionState: "connected",
    criticalError: null,
    currentRunId: "preview-run-001",
    currentTaskTitle: "帮我检查当前项目里最需要修的一个问题",
    events: buildResumePreviewEvents(),
    latestEventAt: "2026-04-08T10:10:10Z",
    messages: [],
    runState: "completed",
    sessionId: "preview-session-001",
    submitError: null,
  });
}

function applyConfirmationPreview() {
  useRuntimeStore.setState({
    composeValue: "",
    confirmation: buildPreviewConfirmation(),
    connectionState: "connected",
    criticalError: null,
    currentRunId: "preview-run-001",
    currentTaskTitle: "帮我检查当前项目里最需要修的一个问题",
    events: [],
    latestEventAt: null,
    messages: [],
    runState: "awaiting_confirmation",
    sessionId: "preview-session-001",
    submitError: null,
  });
}

function applyBlockedPreview() {
  useRuntimeStore.setState({
    composeValue: "",
    confirmation: null,
    connectionState: "disconnected",
    criticalError: "home_preview_blocked",
    currentRunId: "",
    currentTaskTitle: "等待第一条任务",
    events: [],
    latestEventAt: null,
    messages: [],
    runState: "idle",
    sessionId: "",
    submitError: null,
  });
}

function buildResumePreviewEvents() {
  return [
    {
      event_id: "preview-tool-1",
      event_type: "tool_started",
      session_id: "preview-session-001",
      run_id: "preview-run-001",
      sequence: 1,
      timestamp: "2026-04-08T10:00:00Z",
      stage: "analysis",
      summary: "正在读取工作区文档",
      tool_name: "read_docs",
      tool_display_name: "读取文档",
    },
    {
      event_id: "preview-verify-1",
      event_type: "run_finished",
      session_id: "preview-session-001",
      run_id: "preview-run-001",
      sequence: 2,
      timestamp: "2026-04-08T10:06:00Z",
      stage: "verification",
      summary: "构建验证通过",
      verification_summary: "npm run build 通过",
      verification_snapshot: { summary: "构建成功", passed: true },
      completion_status: "completed",
    },
    {
      event_id: "preview-memory-1",
      event_type: "run_finished",
      session_id: "preview-session-001",
      run_id: "preview-run-001",
      sequence: 3,
      timestamp: "2026-04-08T10:08:00Z",
      stage: "summary",
      summary: "已记录项目当前执行入口",
      context_snapshot: { memory_digest: "已写入首页状态化实现要点" },
      completion_status: "completed",
    },
  ];
}

function buildPreviewConfirmation() {
  return {
    confirmation_id: "preview-confirmation-001",
    run_id: "preview-run-001",
    risk_level: "high",
    action_summary: "需要确认是否覆盖当前工作区中的配置文件",
    reason: "该操作会修改已有配置",
    impact_scope: "frontend/src/index.css",
    target_paths: ["frontend/src/index.css"],
    reversible: true,
    hazards: ["可能覆盖现有样式修改"],
    alternatives: ["先查看 diff 再确认"],
    kind: "high_risk_action",
  };
}

function focusConfirmationCard() {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      document.getElementById("task-confirmation-anchor")?.focus();
    });
  });
}

export default App;
