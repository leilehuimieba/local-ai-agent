import { FormEvent, startTransition, useMemo, useState } from "react";
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
  renderBottomPanel,
  renderGlobalLayers,
  renderTopBar,
  renderWorkspaceContent,
} from "./shell/workspaceViewModel";
import { useSettings } from "./settings/useSettings";

type AppView = "home" | "agent" | "logs" | "settings";
type RuntimeView = ReturnType<typeof useRuntimeView>;
type ViewState = ReturnType<typeof useViewState>;
type SettingsApi = ReturnType<typeof useSettings>;
type LogsApi = ReturnType<typeof useLogs>;
type RuntimeActions = ReturnType<typeof useRuntimeActions>;
export type WorkspaceAppModel = ReturnType<typeof useWorkspaceApp>;
type ConfirmationDecision = "approve" | "reject" | "cancel";

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

export function useWorkspaceApp() {
  const settingsApi = useSettings();
  const runtime = useRuntimeView();
  const view = useViewState();
  const logs = useLogs(view.currentView === "logs");
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

function useViewState() {
  const [currentView, setCurrentView] = useState<AppView>("agent");
  const [bottomPanelOpen, setBottomPanelOpen] = useState(false);
  const [logsDrawerOpen, setLogsDrawerOpen] = useState(false);
  return { bottomPanelOpen, currentView, logsDrawerOpen, setBottomPanelOpen, setCurrentView, setLogsDrawerOpen };
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
) {
  const statusLine = getRunStateLabel(runtime.runState, runtime.events.length);
  const connectionLabel = getConnectionStateLabel(runtime.connectionState);
  return { actions, connectionLabel, logs, runtime, settingsApi, statusLine, view };
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
    openAgentPage: () => view.setCurrentView("agent"),
    openLogsPage: () => openLogsPage(logs, view),
    toggleLogsDrawer: () => view.setLogsDrawerOpen(!view.logsDrawerOpen),
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
    view.setCurrentView("agent");
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
  view.setCurrentView("agent");
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

export default App;
