import { readRunStateNextStep } from "../../chat/chatResultModel";
import { isBusyRunState, useRuntimeStore } from "../../runtime/state";
import { ConfirmationRequest, RunEvent, SettingsResponse } from "../../shared/contracts";
import type { RuntimeView } from "../../App";
import { HOME_EXAMPLES } from "../workspaceViewModel";
import type {
  AppModel,
  HomeAction,
  HomeActivity,
  HomeBlock,
  HomeStateKind,
  HomeViewModel,
  ResumeItem,
} from "../workspaceViewModel";
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
