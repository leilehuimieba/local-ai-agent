import { MemoryEntry, SettingsResponse } from "../../shared/contracts";
import { MemoryResourcesSection } from "./MemoryResourcesSection";

type ResourcesEntrySectionProps = {
  settings: SettingsResponse;
  memories: MemoryEntry[];
  error: string | null;
  actionState: {
    tone: "running" | "failed" | "completed";
    message: string;
  } | null;
  deletingId: string;
  isRunning: boolean;
  isRefreshing: boolean;
  onDeleteMemory: (memoryId: string) => void;
  onRefreshMemories: () => void;
};

export function ResourcesEntrySection(props: ResourcesEntrySectionProps) {
  return (
    <div className="resources-entry-section">
      <ResourcesWorkbenchNote />
      <MemoryResourcesSection
        actionState={props.actionState}
        deletingId={props.deletingId}
        settings={props.settings}
        memories={props.memories}
        error={props.error}
        isRunning={props.isRunning}
        isRefreshing={props.isRefreshing}
        onDeleteMemory={props.onDeleteMemory}
        onRefresh={props.onRefreshMemories}
      />
    </div>
  );
}

function ResourcesWorkbenchNote() {
  return (
    <div className="logs-filter-note resources-entry-note">
      <strong>资源工作区</strong>
      <p>先看记忆策略与治理摘要，再进入列表筛选和单条详情，避免把资源区读成普通设置表单。</p>
    </div>
  );
}
