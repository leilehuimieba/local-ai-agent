import { create } from "zustand";

import { ChatMessage, ConfirmationRequest, RunEvent } from "../shared/contracts";

export type RunState =
  | "idle"
  | "submitting"
  | "streaming"
  | "awaiting_confirmation"
  | "resuming"
  | "completed"
  | "failed"
  | "archived";

export type ConnectionState =
  | "connecting"
  | "connected"
  | "reconnecting"
  | "disconnected"
  | "closed";

export type UnifiedStatusKey =
  | "idle"
  | "running"
  | "awaiting_confirmation"
  | "completed"
  | "failed";

const FAILED_STATUS_LABELS = ["失败", "降级", "failed", "error"];
const AWAITING_STATUS_LABELS = ["待确认", "high", "medium", "low", "待治理", "已跳过"];
const COMPLETED_STATUS_LABELS = ["完成", "已完成", "就绪", "稳定", "已验证", "已归档"];
const RUNNING_STATUS_LABELS = ["处理中", "进行中", "running"];
const IDLE_STATUS_LABELS = ["等待任务", "等待中", "idle", "空闲"];

type RuntimeState = {
  composeValue: string;
  sessionId: string;
  currentRunId: string;
  messages: ChatMessage[];
  events: RunEvent[];
  submitError: string | null;
  criticalError: string | null;
  confirmation: ConfirmationRequest | null;
  rememberChoice: boolean;
  runState: RunState;
  connectionState: ConnectionState;
  latestEventAt: string | null;
  canReconnect: boolean;
  currentTaskTitle: string;
};

type RuntimeActions = {
  setComposeValue: (value: string) => void;
  setRememberChoice: (checked: boolean) => void;
  setSubmitError: (value: string | null) => void;
  setCriticalError: (value: string | null) => void;
  startSubmission: (userInput: string) => void;
  acceptRun: (sessionId: string, runId: string) => void;
  failSubmission: (message: string) => void;
  resolveConfirmationDecision: (decision: "approve" | "reject" | "cancel") => void;
  applyEvent: (event: RunEvent) => void;
  syncConnection: (
    connectionState: ConnectionState,
    latestEventAt: string | null,
    canReconnect: boolean,
  ) => void;
  clearCriticalError: () => void;
};

type RuntimeStore = RuntimeState & RuntimeActions;

const INITIAL_STATE: RuntimeState = {
  canReconnect: false,
  composeValue: "",
  confirmation: null,
  connectionState: "closed",
  criticalError: null,
  currentRunId: "",
  currentTaskTitle: "等待第一条任务",
  events: [],
  latestEventAt: null,
  messages: [],
  rememberChoice: true,
  runState: "idle",
  sessionId: "",
  submitError: null,
};

export const useRuntimeStore = create<RuntimeStore>((set) => ({
  ...INITIAL_STATE,
  acceptRun: (sessionId, runId) => set(buildAcceptedRunState(sessionId, runId)),
  applyEvent: (event) => set((state) => buildEventUpdate(state, event)),
  clearCriticalError: () => set({ criticalError: null }),
  failSubmission: (message) => set(buildSubmissionFailureState(message)),
  resolveConfirmationDecision: (decision) => set((state) => buildConfirmationState(state, decision)),
  setComposeValue: (composeValue) => set({ composeValue }),
  setCriticalError: (criticalError) => set({ criticalError }),
  setRememberChoice: (rememberChoice) => set({ rememberChoice }),
  setSubmitError: (submitError) => set({ submitError }),
  startSubmission: (userInput) => set((state) => buildSubmissionState(state, userInput)),
  syncConnection: (connectionState, latestEventAt, canReconnect) =>
    set({ canReconnect, connectionState, latestEventAt }),
}));

export function isBusyRunState(runState: RunState) {
  return runState === "submitting" || runState === "streaming" || runState === "resuming";
}

export function isTerminalRunState(runState: RunState) {
  return runState === "completed" || runState === "failed" || runState === "archived";
}

export function getRunStateLabel(runState: RunState) {
  if (runState === "archived") return "已归档";
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(runState)).label;
}

export function getConnectionStateLabel(connectionState: ConnectionState) {
  if (connectionState === "connecting") return "连接建立中";
  if (connectionState === "connected") return "连接正常";
  if (connectionState === "reconnecting") return "连接恢复中";
  if (connectionState === "disconnected") return "连接已断开";
  return "连接已关闭";
}

export function getRunTone(runState: RunState) {
  const status = readUnifiedStatusFromRunState(runState);
  if (status === "running") return "running";
  if (status === "awaiting_confirmation") return "awaiting";
  if (status === "failed") return "failed";
  if (status === "completed") return "completed";
  return "idle";
}

export function readUnifiedStatusMeta(status: UnifiedStatusKey) {
  if (status === "failed") return { className: "status-failed", label: "失败" };
  if (status === "awaiting_confirmation") return { className: "status-awaiting", label: "待确认" };
  if (status === "completed") return { className: "status-completed", label: "完成" };
  if (status === "running") return { className: "status-running", label: "进行中" };
  return { className: "status-idle", label: "等待中" };
}

export function readUnifiedStatusFromRunState(runState: RunState): UnifiedStatusKey {
  if (runState === "failed") return "failed";
  if (runState === "awaiting_confirmation") return "awaiting_confirmation";
  if (runState === "completed") return "completed";
  if (runState === "submitting" || runState === "streaming" || runState === "resuming") return "running";
  return "idle";
}

export function readUnifiedStatusFromLabel(label: string): UnifiedStatusKey {
  const normalized = label.trim().toLowerCase();
  if (FAILED_STATUS_LABELS.includes(normalized)) return "failed";
  if (AWAITING_STATUS_LABELS.includes(normalized)) return "awaiting_confirmation";
  if (COMPLETED_STATUS_LABELS.includes(normalized)) return "completed";
  if (RUNNING_STATUS_LABELS.includes(normalized)) return "running";
  if (IDLE_STATUS_LABELS.includes(normalized)) return "idle";
  return "idle";
}

export function getLatestFailureEvent(events: RunEvent[]) {
  return [...events].reverse().find((item) => item.event_type === "run_failed");
}

function buildAcceptedRunState(sessionId: string, runId: string) {
  return {
    canReconnect: false,
    connectionState: "connecting" as ConnectionState,
    currentRunId: runId,
    runState: "streaming" as RunState,
    sessionId,
    submitError: null,
  };
}

function buildSubmissionState(state: RuntimeState, userInput: string) {
  return {
    composeValue: "",
    confirmation: null,
    connectionState: state.sessionId ? "connecting" : state.connectionState,
    criticalError: null,
    currentTaskTitle: userInput,
    messages: [createUserMessage(userInput), ...state.messages],
    runState: "submitting" as RunState,
    submitError: null,
  };
}

function buildSubmissionFailureState(message: string) {
  return {
    connectionState: "closed" as ConnectionState,
    criticalError: message,
    runState: "failed" as RunState,
    submitError: message,
  };
}

function buildConfirmationState(
  state: RuntimeState,
  decision: "approve" | "reject" | "cancel",
) {
  return {
    confirmation: null,
    criticalError: null,
    runState: decision === "approve" ? ("resuming" as RunState) : state.runState,
    submitError: null,
  };
}

function buildEventUpdate(state: RuntimeState, event: RunEvent) {
  if (hasEvent(state.events, event.event_id)) {
    return {};
  }
  const nextState = getRunStateFromEvent(state.runState, event);
  return {
    confirmation: getConfirmationFromEvent(event, nextState, state.confirmation),
    criticalError: getCriticalError(event, nextState, state.criticalError),
    currentRunId: event.run_id || state.currentRunId,
    currentTaskTitle: getTaskTitle(state.currentTaskTitle, event),
    events: [...state.events, event],
    latestEventAt: event.timestamp,
    messages: appendAssistantMessage(state.messages, event, nextState),
    runState: nextState,
    submitError: getSubmitError(event, nextState, state.submitError),
  };
}

function hasEvent(events: RunEvent[], eventId: string) {
  return events.some((item) => item.event_id === eventId);
}

function getRunStateFromEvent(currentState: RunState, event: RunEvent): RunState {
  if (event.event_type === "confirmation_required") return "awaiting_confirmation";
  if (event.event_type === "run_failed" || event.metadata?.error_code) return "failed";
  if (event.event_type === "run_finished") return isFailedCompletion(event) ? "failed" : "completed";
  if (currentState === "submitting" || currentState === "resuming" || currentState === "idle") {
    return "streaming";
  }
  return currentState === "archived" ? "streaming" : currentState;
}

function isFailedCompletion(event: RunEvent) {
  return event.completion_status === "failed" || Boolean(event.metadata?.error_code);
}

function getConfirmationFromEvent(
  event: RunEvent,
  nextState: RunState,
  current: ConfirmationRequest | null,
) {
  if (event.event_type === "confirmation_required") return createConfirmationRequest(event);
  return nextState === "awaiting_confirmation" ? current : null;
}

function getCriticalError(event: RunEvent, nextState: RunState, current: string | null) {
  if (nextState !== "failed") return current;
  return event.detail || event.summary || current;
}

function getTaskTitle(currentTaskTitle: string, event: RunEvent) {
  return event.metadata?.task_title || event.summary || currentTaskTitle;
}

function getSubmitError(event: RunEvent, nextState: RunState, current: string | null) {
  if (nextState !== "failed") return null;
  return event.detail || event.summary || current;
}

function appendAssistantMessage(messages: ChatMessage[], event: RunEvent, nextState: RunState) {
  if (nextState !== "completed" && nextState !== "failed") return messages;
  if (!shouldAppendAssistantMessage(event, nextState)) return messages;
  const answer = getAssistantAnswer(event);
  return hasAssistantMessage(messages, event.run_id) ? messages : [...messages, createAssistantMessage(answer, event.run_id)];
}

function shouldAppendAssistantMessage(event: RunEvent, nextState: RunState) {
  if (nextState === "completed") return true;
  return Boolean(event.final_answer || event.metadata?.final_answer);
}

function getAssistantAnswer(event: RunEvent) {
  return (
    event.final_answer ||
    event.metadata?.final_answer ||
    event.result_summary ||
    event.summary ||
    "任务已结束，但当前事件没有携带最终答复。"
  );
}

function hasAssistantMessage(messages: ChatMessage[], runId: string) {
  return messages.some((item) => item.role === "assistant" && item.runId === runId);
}

function createAssistantMessage(content: string, runId: string) {
  return { content, id: `${runId}-assistant`, role: "assistant" as const, runId };
}

function createUserMessage(content: string) {
  return { content, id: `user-${Date.now()}`, role: "user" as const };
}

function createConfirmationRequest(event: RunEvent): ConfirmationRequest {
  return {
    action_summary: event.metadata?.action_summary || event.summary,
    alternatives: splitMetadataList(event.metadata?.alternatives),
    confirmation_id: event.metadata?.confirmation_id || "",
    hazards: splitMetadataList(event.metadata?.hazards),
    impact_scope: event.metadata?.impact_scope || "",
    kind: event.metadata?.kind || "high_risk_action",
    reason: event.metadata?.reason || event.detail || event.summary,
    reversible: event.metadata?.reversible === "true",
    risk_level: event.metadata?.risk_level || "medium",
    run_id: event.run_id,
    target_paths: splitMetadataList(event.metadata?.target_paths),
  };
}

function splitMetadataList(value?: string) {
  return value?.split("\n").filter(Boolean) || [];
}
