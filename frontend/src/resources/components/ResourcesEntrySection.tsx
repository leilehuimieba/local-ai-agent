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
  );
}
