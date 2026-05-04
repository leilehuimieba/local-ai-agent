// API layer for Local Agent - connects to backend gateway

const API_BASE = process.env.NEXT_PUBLIC_API_BASE ?? "";

function readGatewayToken(): string {
  if (typeof document === "undefined") return "";
  return document.querySelector<HTMLMetaElement>('meta[name="local-agent-token"]')?.content ?? "";
}

function authHeaders(): Record<string, string> {
  const token = readGatewayToken();
  return token ? { "X-Local-Agent-Token": token } : {};
}

function jsonHeaders(): Record<string, string> {
  return { "Content-Type": "application/json", ...authHeaders() };
}

async function readError(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || `HTTP ${response.status}`;
}

// ========== Chat APIs ==========

export type ModelRef = {
  model_id: string;
  display_name: string;
  provider_id: string;
  enabled?: boolean;
  available?: boolean;
};

export type WorkspaceRef = {
  workspace_id: string;
  name: string;
  root_path: string;
  is_active?: boolean;
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
    headers: jsonHeaders(),
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
    headers: jsonHeaders(),
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error(`提交重试失败: ${await readError(response)}`);
  return (await response.json()) as ChatRunAccepted;
}

export async function submitChatCancel(sessionId: string, runId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/chat/cancel`, {
    method: "POST",
    headers: jsonHeaders(),
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
    headers: jsonHeaders(),
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
  app_name: string;
  mode: string;
  model: ModelRef;
  workspace: WorkspaceRef;
  available_models: ModelRef[];
  available_workspaces: WorkspaceRef[];
  approved_directories: DirectoryApproval[];
  directory_prompt_enabled: boolean;
  show_risk_level: boolean;
  ports: Record<string, number>;
  runtime_status: RuntimeStatusInfo;
  embedding: EmbeddingInfo;
  providers: ProviderOption[];
  mcp: MCPInfo;
};

export type MCPServerInfo = {
  id: string;
  name: string;
  type: string;
  url: string;
  enabled: boolean;
  ready: boolean;
  tool_count: number;
  allowed_tool_count: number;
  blocked_tool_count: number;
  requires_policy: boolean;
};

export type MCPInfo = {
  servers: MCPServerInfo[];
  tools: MCPTool[];
};

export type MCPTool = {
  name: string;
  description: string;
  inputSchema?: Record<string, unknown>;
  server_id?: string;
  allowed: boolean;
  risk_level: "low" | "medium" | "high" | "critical";
  requires_confirmation: boolean;
  audit_enabled: boolean;
  policy_source: string;
};

export type RuntimeStatusInfo = {
  ok: boolean;
  name: string;
  version: string;
};

export type EmbeddingInfo = {
  provider_id: string;
  model_name: string;
};

export type DirectoryApproval = {
  approval_id: string;
  workspace_id: string;
  name: string;
  root_path: string;
  created_at?: string;
};

export type ProviderOption = {
  provider_id: string;
  display_name: string;
  base_url: string;
  chat_completions_path?: string;
  models_path?: string;
  embedding_model?: string;
};

export async function fetchSettings(): Promise<SettingsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/settings`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取设置失败: ${await readError(response)}`);
  return (await response.json()) as SettingsResponse;
}

export type SettingsUpdatePayload = {
  mode?: string;
  model?: ModelRef;
  workspace_id?: string;
  directory_prompt_enabled?: boolean;
  show_risk_level?: boolean;
  revoke_directory_root?: string;
  add_directory_name?: string;
  add_directory_path?: string;
  embedding_provider_id?: string;
  add_mcp_name?: string;
  add_mcp_url?: string;
  remove_mcp_id?: string;
  mcp_policy_server_id?: string;
  mcp_policy_tool_name?: string;
  mcp_policy_allowed?: boolean;
  mcp_policy_risk_level?: string;
  mcp_policy_requires_confirmation?: boolean;
};

export async function updateSettings(patch: SettingsUpdatePayload): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/settings`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify(patch),
  });
  if (!response.ok) throw new Error(`更新设置失败: ${await readError(response)}`);
}

export async function fetchMCPTools(): Promise<MCPTool[]> {
  const response = await fetch(`${API_BASE}/api/v1/mcp/tools`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取 MCP 工具失败: ${await readError(response)}`);
  const data = await response.json() as { tools: MCPTool[] };
  return data.tools;
}

export async function callMCPTool(serverId: string, name: string, arguments_: Record<string, unknown>): Promise<unknown> {
  const response = await fetch(`${API_BASE}/api/v1/mcp/call`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify({ server_id: serverId, name, arguments: arguments_ }),
  });
  if (!response.ok) throw new Error(`调用 MCP 工具失败: ${await readError(response)}`);
  return response.json();
}

export type ProviderSettingsResponse = {
  active_provider_id?: string;
  providers: ProviderSettingsItem[];
};

export type ProviderCredentialStatus = {
  has_credential: boolean;
  api_key_masked?: string;
  updated_at?: string;
  last_test_status?: string;
  last_test_message?: string;
  last_test_at?: string;
  apply_status: string;
  applied_at?: string;
  pending_reload: boolean;
};

export type ProviderSettingsItem = ProviderOption & {
  credential_kind?: string;
  supports_test?: boolean;
  editable?: boolean;
  credential_status: ProviderCredentialStatus;
};

export async function fetchProviderSettings(): Promise<ProviderSettingsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取服务商设置失败: ${await readError(response)}`);
  return (await response.json()) as ProviderSettingsResponse;
}

export type ProviderCredentialPayload = ProviderOption & {
  api_key: string;
};

export async function testProvider(payload: ProviderCredentialPayload): Promise<{ ok: boolean; message: string }> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers/test`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error(`测试服务商失败: ${await readError(response)}`);
  return (await response.json()) as { ok: boolean; message: string };
}

export async function saveProvider(payload: ProviderCredentialPayload): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers/save`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error(`保存服务商失败: ${await readError(response)}`);
}

export async function applyProvider(providerId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers/apply`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify({ provider_id: providerId }),
  });
  if (!response.ok) throw new Error(`应用服务商失败: ${await readError(response)}`);
}

export async function removeProviderCredential(providerId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/settings/providers/remove`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify({ provider_id: providerId }),
  });
  if (!response.ok) throw new Error(`移除服务商失败: ${await readError(response)}`);
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

type KnowledgeItemWire = Omit<KnowledgeItem, "citationCount" | "createdAt" | "updatedAt"> & {
  citation_count: number;
  created_at: string;
  updated_at: string;
};

export type KnowledgeItemsResponse = {
  items: KnowledgeItem[];
  categories: string[];
  tags: string[];
};

function normalizeKnowledgeItem(item: KnowledgeItemWire): KnowledgeItem {
  return {
    ...item,
    citationCount: item.citation_count,
    createdAt: item.created_at,
    updatedAt: item.updated_at,
  };
}

export async function fetchKnowledgeItems(): Promise<KnowledgeItemsResponse> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取知识库失败: ${await readError(response)}`);
  const data = await response.json() as { items: KnowledgeItemWire[]; categories: string[]; tags: string[] };
  return { ...data, items: data.items.map(normalizeKnowledgeItem) };
}

export async function createKnowledgeItem(item: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">): Promise<KnowledgeItem> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify(item),
  });
  if (!response.ok) throw new Error(`创建知识库条目失败: ${await readError(response)}`);
  return normalizeKnowledgeItem((await response.json()) as KnowledgeItemWire);
}

export async function updateKnowledgeItem(id: string, patch: Partial<KnowledgeItem>): Promise<KnowledgeItem> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items/${id}`, {
    method: "PUT",
    headers: jsonHeaders(),
    body: JSON.stringify(toKnowledgePatch(patch)),
  });
  if (!response.ok) throw new Error(`更新知识库条目失败: ${await readError(response)}`);
  return normalizeKnowledgeItem((await response.json()) as KnowledgeItemWire);
}

function toKnowledgePatch(patch: Partial<KnowledgeItem>): Record<string, unknown> {
  const { citationCount, createdAt, updatedAt, ...rest } = patch;
  return rest;
}

export async function deleteKnowledgeItem(id: string): Promise<void> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/items/${id}`, {
    method: "DELETE",
    headers: authHeaders(),
  });
  if (!response.ok) throw new Error(`删除知识库条目失败: ${await readError(response)}`);
}

export async function uploadKnowledgeFile(file: File): Promise<KnowledgeItem> {
  const formData = new FormData();
  formData.append("file", file);
  const response = await fetch(`${API_BASE}/api/v1/knowledge/upload`, {
    method: "POST",
    headers: authHeaders(),
    body: formData,
  });
  if (!response.ok) throw new Error(`上传文件失败: ${await readError(response)}`);
  return normalizeKnowledgeItem((await response.json()) as KnowledgeItemWire);
}

export async function askKnowledgeBase(question: string, workspaceId?: string): Promise<{ answer: string; sources: string[] }> {
  const response = await fetch(`${API_BASE}/api/v1/knowledge/ask`, {
    method: "POST",
    headers: jsonHeaders(),
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
  detail?: string;
  tool_name?: string;
  tool_display_name?: string;
  tool_category?: string;
  risk_level?: string;
  result_summary?: string;
  final_answer?: string;
  metadata?: Record<string, string>;
  event_type?: string;
  task_title?: string;
  completion_status?: string;
  updated_at?: string;
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

export async function fetchLogs(view: "runs" | "events", params?: { session_id?: string; run_id?: string; limit?: number }): Promise<{ items: LogEntry[]; runs?: LogRun[] }> {
  const search = new URLSearchParams({ view });
  if (params?.session_id) search.set("session_id", params.session_id);
  if (params?.run_id) search.set("run_id", params.run_id);
  if (params?.limit) search.set("limit", String(params.limit));
  const response = await fetch(`${API_BASE}/api/v1/logs?${search.toString()}`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取日志失败: ${await readError(response)}`);
  const data = (await response.json()) as { items: LogEntry[] };
  if (view === "runs") {
    const runs: LogRun[] = data.items.map((item) => ({
      run_id: item.run_id,
      session_id: item.session_id,
      title: item.task_title || item.summary || "未命名任务",
      status: item.completion_status === "failed" || item.level === "error" || item.event_type === "run_failed"
        ? "failed"
        : item.completion_status === "completed" || item.event_type === "run_finished"
          ? "completed"
          : "running",
      started_at: /^\d{13,}$/.test(item.timestamp)
        ? new Date(Number(item.timestamp)).toISOString()
        : item.timestamp,
      completed_at: item.updated_at && /^\d{13,}$/.test(item.updated_at)
        ? new Date(Number(item.updated_at)).toISOString()
        : item.updated_at,
      duration_ms: 0,
      event_count: 0,
    }));
    return { items: data.items, runs };
  }
  return { items: data.items };
}

// ========== System APIs ==========

export type SystemInfo = {
  repo_root: string;
  runtime_status: string;
  version: string;
};

export async function fetchSystemInfo(): Promise<SystemInfo> {
  const response = await fetch(`${API_BASE}/api/v1/system/info`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取系统信息失败: ${await readError(response)}`);
  return (await response.json()) as SystemInfo;
}

// ========== Diagnostics APIs ==========

export type DiagnosticsCheckResponse = {
  checked_at: string;
  overall_ok: boolean;
  diagnostics: {
    checked_at: string;
    repo_root: string;
    repo_root_exists: boolean;
    storage_root: string;
    storage_root_exists: boolean;
    runtime_reachable: boolean;
    runtime_version: string;
    provider_count: number;
    model_count: number;
    workspace_count: number;
  };
  services: ServiceStatus[];
  warnings: string[];
  errors: string[];
};

export type ServiceStatus = {
  id: string;
  label: string;
  status: "ok" | "warning" | "error" | string;
  severity: "info" | "warning" | "error" | string;
  detail: string;
  hint: string;
};

export async function runDiagnosticsCheck(): Promise<DiagnosticsCheckResponse> {
  const response = await fetch(`${API_BASE}/api/v1/settings/diagnostics/check`, {
    method: "POST",
    headers: authHeaders(),
  });
  if (!response.ok) throw new Error(`健康检查失败: ${await readError(response)}`);
  return (await response.json()) as DiagnosticsCheckResponse;
}

// ========== Memory APIs ==========

export type MemoryEntry = {
  id: string;
  kind: string;
  title: string;
  summary: string;
  content: string;
  created_at: string;
  source_run_id?: string;
};

export type MemoryListResponse = {
  items: MemoryEntry[];
};

export async function fetchMemories(): Promise<MemoryListResponse> {
  const response = await fetch(`${API_BASE}/api/v1/memories`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取记忆失败: ${await readError(response)}`);
  return (await response.json()) as MemoryListResponse;
}

export async function deleteMemory(memoryId: string): Promise<MemoryListResponse> {
  const response = await fetch(`${API_BASE}/api/v1/memories/delete`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify({ memory_id: memoryId }),
  });
  if (!response.ok) throw new Error(`删除记忆失败: ${await readError(response)}`);
  return (await response.json()) as MemoryListResponse;
}

// ========== Session APIs ==========

export type SessionItem = {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
};

export type SessionListResponse = {
  items: SessionItem[];
};

export type ChatMessageItem = {
  id: string;
  session_id: string;
  role: string;
  content: string;
  timestamp: string;
};

export type MessageListResponse = {
  items: ChatMessageItem[];
};

export async function fetchSessions(): Promise<SessionListResponse> {
  const response = await fetch(`${API_BASE}/api/v1/sessions`, { headers: authHeaders() });
  if (!response.ok) throw new Error(`获取会话列表失败: ${await readError(response)}`);
  return (await response.json()) as SessionListResponse;
}

export async function fetchSessionMessages(sessionId: string): Promise<MessageListResponse> {
  const response = await fetch(`${API_BASE}/api/v1/sessions/${encodeURIComponent(sessionId)}/messages`, {
    headers: authHeaders(),
  });
  if (!response.ok) throw new Error(`获取会话消息失败: ${await readError(response)}`);
  return (await response.json()) as MessageListResponse;
}

export async function addSessionMessage(
  sessionId: string,
  payload: { role: string; content: string; timestamp?: string },
): Promise<ChatMessageItem> {
  const response = await fetch(`${API_BASE}/api/v1/sessions/${encodeURIComponent(sessionId)}/messages`, {
    method: "POST",
    headers: jsonHeaders(),
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error(`保存消息失败: ${await readError(response)}`);
  return (await response.json()) as ChatMessageItem;
}
