import { MemoryEntry, ProviderSettingsResponse, SettingsResponse } from "../shared/contracts";
import { MetricChip, SectionHeader } from "../ui/primitives";
import { SettingsEmptyState, SettingsModules } from "./SettingsSections";
import { StatusCard } from "./StatusCard";
import { ProviderActionState, SettingsActionFeedback, SettingsActionKind } from "./useSettings";

type SettingsPanelProps = {
  settings: SettingsResponse | null;
  providerSettings: ProviderSettingsResponse | null;
  bootstrapError: string | null;
  providerBootstrapError: string | null;
  providerActions: Record<string, ProviderActionState>;
  isRunning: boolean;
  pendingAction: SettingsActionFeedback | null;
  actionError: SettingsActionFeedback | null;
  lastSuccess: SettingsActionFeedback | null;
  memories: MemoryEntry[];
  memoryError: string | null;
  memoryPendingAction: { action: "refresh" | "delete"; message: string } | null;
  memoryActionError: { action: "refresh" | "delete"; message: string } | null;
  memoryActionSuccess: { action: "refresh" | "delete"; message: string } | null;
  deletingMemoryId: string;
  onModeChange: (mode: string) => void;
  onModelChange: (modelKey: string) => void;
  onWorkspaceChange: (workspaceId: string) => void;
  onDirectoryPromptEnabledChange: (enabled: boolean) => void;
  onShowRiskLevelChange: (enabled: boolean) => void;
  onRevokeDirectoryApproval: (rootPath: string) => void;
  onRunExternalConnectionAction: (slotId: string, action: "validate" | "recheck") => void;
  onCheckDiagnostics: () => void;
  onRefreshProviderSettings: () => Promise<unknown>;
  onTestProvider: (providerId: string, apiKey: string, baseURL?: string) => Promise<unknown>;
  onSaveProvider: (providerId: string, apiKey: string) => Promise<unknown>;
  onApplyProvider: (providerId: string) => Promise<unknown>;
  onRemoveProvider: (providerId: string) => Promise<unknown>;
  onDeleteMemory: (memoryId: string) => void;
  onRefreshMemories: () => void;
  isActionPending: (action: SettingsActionKind) => boolean;
};

export function SettingsPanel(props: SettingsPanelProps) {
  return (
    <article className="panel settings-page settings-workspace">
      <SettingsWorkspaceHero settings={props.settings} memories={props.memories} />
      <div className="settings-workspace-stack">
        <StatusCard
          settings={props.settings}
          bootstrapError={props.bootstrapError}
          pendingAction={props.pendingAction}
          actionError={props.actionError}
          lastSuccess={props.lastSuccess}
        />
        {!props.settings ? <SettingsEmptyState /> : null}
        {props.settings ? <SettingsModules {...props} settings={props.settings} /> : null}
      </div>
    </article>
  );
}

function SettingsWorkspaceHero(props: {
  settings: SettingsResponse | null;
  memories: MemoryEntry[];
}) {
  return (
    <section className="settings-workspace-hero">
      <SectionHeader
        kind="page"
        kicker="Workspace"
        level="h2"
        title="Settings Workspace"
        description="统一查看运行环境、控制项、诊断动作与资源状态，保持与任务页、Logs 页一致的工作台语义。"
        action={<SettingsWorkspaceMeta settings={props.settings} memories={props.memories} />}
      />
    </section>
  );
}

function SettingsWorkspaceMeta(props: {
  settings: SettingsResponse | null;
  memories: MemoryEntry[];
}) {
  return (
    <div className="page-header-meta">
      <MetricChip label="模式" value={props.settings?.mode || "读取中"} />
      <MetricChip label="工作区" value={props.settings?.workspace.name || "未加载"} />
      <MetricChip label="记忆" value={`${props.memories.length} 条`} />
    </div>
  );
}
