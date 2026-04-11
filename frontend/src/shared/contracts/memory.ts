export type MemoryEntry = {
  id: string;
  kind: string;
  memory_kind?: string;
  title: string;
  summary: string;
  content: string;
  reason: string;
  scope: string;
  workspace_id: string;
  session_id: string;
  source_run_id: string;
  source: string;
  source_type: string;
  source_title?: string;
  source_event_type?: string;
  source_artifact_path?: string;
  governance_version?: string;
  governance_reason?: string;
  governance_source?: string;
  governance_at?: string;
  governance_status?: string;
  memory_action?: string;
  archive_reason?: string;
  verified: boolean;
  priority: number;
  archived: boolean;
  archived_at?: string;
  created_at: string;
  updated_at: string;
  timestamp: string;
};

export type MemoryListResponse = {
  items: MemoryEntry[];
};
