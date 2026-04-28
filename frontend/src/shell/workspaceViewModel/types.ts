import type { FormEvent } from "react";
import type { ConfirmationRequest } from "../../shared/contracts";
import type { AppView, LogsApi, RuntimeView, SettingsApi } from "../../App";
export type { AppView } from "../../App";

export type HomeStateKind = "first_use" | "resume" | "blocked";
export type HomeAction = "reconnect" | "settings" | "workspace" | "model";
export type HomeBlock = { action: HomeAction; title: string; body: string; detail: string };
export type HomeActivity = { id: string; kind: "verification" | "memory" | "tool"; label: string; text: string };
export type ResumeItem = { label: string; value: string };

export type HomeViewState = {
  currentView: AppView;
  setCurrentView: (view: AppView) => void;
  showHomeCompose: () => void;
};

export type AppActions = {
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

export type TaskNavEntry = {
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
