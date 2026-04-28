// API layer for Local Agent - connects to backend gateway

const API_BASE = "";

async function readError(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || `HTTP ${response.status}`;
}

// ========== Chat APIs ==========

export type ModelRef = {
  model_id: string;
  display_name: string;
  provider_id: string;
};

export type WorkspaceRef = {
  id: string;
  name: string;
  root_path: string;
};

export type ChatRunAccepted = {
  accepted: boolean;
  session_id: string;
  run_id: string;
  initial_status: string;
};

export type ChatRetryRequest = {
  session_id: string;
  run_id: string;
  checkpoint_id?: string;
};

export type SubmitChatRunPayload = {
  sessionId: string;
  userInput: string;
  mode: string;
  model: ModelRef;
  workspace: WorkspaceRef;
  knowledgeBaseId: string;
};

export async function submitChatRun(payload: SubmitChatRunPayload): Promise<ChatRunAccepted> {
  const response = await fetch(`${API_BASE}/api/v1/chat/run`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      session_id: payload.sessionId,
      user_input: payload.userInput,
      mode: payload.mode,
      model: payload.model,
      workspace: payload.workspace,
      knowledge_base_id: payload.knowledgeBaseId || undefined,
    }),
  });
  if (!response.ok) throw new Error(`提交任务失败: ${await readError(response)}`);
  return (await response.json()) as ChatRunAccepted;
}

export async function submitChatRetry(payload: ChatRetryRequest): Promise<ChatRunAccepted> {
  const response = await fetch(`${API_BASE}/api/v1/chat/retry`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error(`提交重试失败: ${await readError(response)}`);
  return (await response.json()) as ChatRunAccepted;
}

export async function submitChatCancel(sessionId: string, runId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/chat/cancel`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ session_id: sessionId, run_id: runId }),
  });
  if (!response.ok) throw new Error(`取消任务失败: ${await readError(response)}`);
}

// ========== Confirmation APIs ==========

export type SubmitConfirmationPayload = {
  confirmationId: string;
  runId: string;
  decision: "approve" | "reject" | "cancel";
  remember: boolean;
};

export async function submitConfirmationDecision(payload: SubmitConfirmationPayload): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/chat/confirm`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      confirmation_id: payload.confirmationId,
      run_id: payload.runId,
      decision: payload.decision,
      remember: payload.remember,
    }),
  });
  if (!response.ok) throw new Error(`提交确认失败: ${await readError(response)}`);
}

// ========== Settings APIs ==========

export type SettingsResponse = {
  mode: string;
  model: ModelRef;
  workspace: WorkspaceRef;
  embedding_provider_id: string;
  providers: Array<{
    provider_id: string;
    display_name: string;
    base_url: string;
    api_key?: string;
    embedding_model?: string;
  }>;
};

export async function fetchSettings(): Promise<SettingsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/settings`);
  if (!response.ok) throw new Error(`获取设置失败: ${await readError(response)}`);
  return (await response.json()) as SettingsResponse;
}

export async function updateSettings(patch: Partial<SettingsResponse>): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/settings`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(patch),
  });
  if (!response.ok) throw new Error(`更新设置失败: ${await readError(response)}`);
}

export type ProviderSettingsResponse = {
  providers: Array<{
    provider_id: string;
    display_name: string;
    base_url: string;
    api_key?: string;
    embedding_model?: string;
  }>;
};

export async function fetchProviderSettings(): Promise<ProviderSettingsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers`);
  if (!response.ok) throw new Error(`获取服务商设置失败: ${await readError(response)}`);
  return (await response.json()) as ProviderSettingsResponse;
}

// ========== Knowledge APIs ==========

export type KnowledgeItem = {
  id: string;
  title: string;
  summary: string;
  content: string;
  category: string;
  tags: string[];
  citationCount: number;
  source: string;
  createdAt: string;
  updatedAt: string;
};

export type KnowledgeItemsResponse = {
  items: KnowledgeItem[];
  categories: string[];
  tags: string[];
};

export async function fetchKnowledgeItems(): Promise<KnowledgeItemsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items`);
  if (!response.ok) throw new Error(`获取知识库失败: ${await readError(response)}`);
  return (await response.json()) as KnowledgeItemsResponse;
}

export async function createKnowledgeItem(item: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">): Promise<KnowledgeItem> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(item),
  });
  if (!response.ok) throw new Error(`创建知识库条目失败: ${await readError(response)}`);
  return (await response.json()) as KnowledgeItem;
}

export async function updateKnowledgeItem(id: string, patch: Partial<KnowledgeItem>): Promise<KnowledgeItem> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(patch),
  });
  if (!response.ok) throw new Error(`更新知识库条目失败: ${await readError(response)}`);
  return (await response.json()) as KnowledgeItem;
}

export async function deleteKnowledgeItem(id: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items/${id}`, {
    method: "DELETE",
  });
  if (!response.ok) throw new Error(`删除知识库条目失败: ${await readError(response)}`);
}

export async function uploadKnowledgeFile(file: File): Promise<KnowledgeItem> {
  const formData = new FormData();
  formData.append("file", file);
  const response = await fetch(`${API_BASE}/api/v1/knowledge/upload`, {
    method: "POST",
    body: formData,
  });
  if (!response.ok) throw new Error(`上传文件失败: ${await readError(response)}`);
  return (await response.json()) as KnowledgeItem;
}

export async function askKnowledgeBase(question: string, workspaceId?: string): Promise<{ answer: string; sources: string[] }> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/ask`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ question, workspace_id: workspaceId }),
  });
  if (!response.ok) throw new Error(`知识库问答失败: ${await readError(response)}`);
  return (await response.json()) as { answer: string; sources: string[] };
}

// ========== Logs APIs ==========

export type LogEntry = {
  log_id: string;
  session_id: string;
  run_id: string;
  timestamp: string;
  level: string;
  category: string;
  source: string;
  summary: string;
};

export type LogRun = {
  run_id: string;
  session_id: string;
  title: string;
  status: "completed" | "failed" | "running";
  started_at: string;
  completed_at?: string;
  duration_ms: number;
  event_count: number;
};

export async function fetchLogs(view: "runs" | "events", params?: { session_id?: string; limit?: number }): Promise<{ items: LogEntry[]; runs?: LogRun[] }> {
  const search = new URLSearchParams({ view });
  if (params?.session_id) search.set("session_id", params.session_id);
  if (params?.limit) search.set("limit", String(params.limit));
  const response = await fetch(`${API_BASE}/api/v1/logs?${search.toString()}`);
  if (!response.ok) throw new Error(`获取日志失败: ${await readError(response)}`);
  return (await response.json()) as { items: LogEntry[]; runs?: LogRun[] };
}

// ========== System APIs ==========

export type SystemInfo = {
  repo_root: string;
  runtime_status: string;
  version: string;
};

export async function fetchSystemInfo(): Promise<SystemInfo> {
  const response = await fetch(`${API_BASE}/api/v1/system/info`);
  if (!response.ok) throw new Error(`获取系统信息失败: ${await readError(response)}`);
  return (await response.json()) as SystemInfo;
}
