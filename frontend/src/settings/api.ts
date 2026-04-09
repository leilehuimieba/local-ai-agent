import {
  DiagnosticsCheckResponse,
  ExternalConnectionActionRequest,
  ExternalConnectionActionResponse,
  ModelRef,
  ProviderApplyRequest,
  ProviderApplyResponse,
  ProviderCredentialStatus,
  ProviderRemoveRequest,
  ProviderRemoveResponse,
  ProviderSaveRequest,
  ProviderSaveResponse,
  ProviderSettingsItem,
  ProviderSettingsResponse,
  ProviderTestRequest,
  ProviderTestResponse,
  SettingsResponse,
} from "../shared/contracts";

export async function fetchSettings(signal?: AbortSignal): Promise<SettingsResponse> {
  const response = await fetch("/api/v1/settings", { signal });
  if (!response.ok) {
    throw new Error(`加载设置失败: ${await readErrorText(response)}`);
  }
  return normalizeSettingsResponse(await response.json());
}

type UpdateSettingsPayload = {
  mode?: string;
  model?: ModelRef;
  workspace_id?: string;
  directory_prompt_enabled?: boolean;
  show_risk_level?: boolean;
  revoke_directory_root?: string;
};

export async function updateSettings(payload: UpdateSettingsPayload): Promise<SettingsResponse> {
  const response = await fetch("/api/v1/settings", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error(await readErrorText(response));
  }

  return normalizeSettingsResponse(await response.json());
}

export async function fetchProviderSettings(signal?: AbortSignal): Promise<ProviderSettingsResponse> {
  const response = await fetch("/api/v1/settings/providers", { signal });
  if (!response.ok) {
    throw new Error(`加载模型服务设置失败: ${await readErrorText(response)}`);
  }
  return normalizeProviderSettings(await response.json());
}

export function testProviderConnection(payload: ProviderTestRequest) {
  return postProviderAction<ProviderTestResponse>("/api/v1/settings/providers/test", payload);
}

export function saveProviderCredential(payload: ProviderSaveRequest) {
  return postProviderAction<ProviderSaveResponse>("/api/v1/settings/providers/save", payload);
}

export function applyProviderCredential(payload: ProviderApplyRequest) {
  return postProviderAction<ProviderApplyResponse>("/api/v1/settings/providers/apply", payload);
}

export function removeProviderCredential(payload: ProviderRemoveRequest) {
  return postProviderAction<ProviderRemoveResponse>("/api/v1/settings/providers/remove", payload);
}

export async function exportSettingsSnapshot(settings: SettingsResponse) {
  return downloadJSON("settings-snapshot.json", settings);
}

export async function openDiagnosticsSnapshot(settings: SettingsResponse) {
  const blob = new Blob([JSON.stringify(settings.diagnostics, null, 2)], { type: "application/json;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const opened = window.open(url, "_blank", "noopener,noreferrer");
  window.setTimeout(() => URL.revokeObjectURL(url), 1000);
  if (!opened) throw new Error("浏览器阻止了诊断窗口，请允许弹窗后重试。");
  return "诊断摘要已在新窗口打开。";
}

export async function exportRunLogs() {
  const response = await fetch("/api/v1/logs");
  if (!response.ok) {
    throw new Error(`导出日志失败: ${await readErrorText(response)}`);
  }
  const payload = await response.json();
  return downloadJSON("run-logs.json", payload);
}

export async function runExternalConnectionAction(
  payload: ExternalConnectionActionRequest,
): Promise<ExternalConnectionActionResponse> {
  const response = await fetch("/api/v1/settings/external-connections/action", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });
  if (!response.ok) {
    throw new Error(`外部连接动作失败: ${await readErrorText(response)}`);
  }
  return response.json() as Promise<ExternalConnectionActionResponse>;
}

export async function checkDiagnostics(): Promise<DiagnosticsCheckResponse> {
  const response = await fetch("/api/v1/settings/diagnostics/check", {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`重新检测失败: ${await readErrorText(response)}`);
  }
  return response.json() as Promise<DiagnosticsCheckResponse>;
}

function downloadJSON(fileName: string, payload: unknown) {
  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: "application/json;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = fileName;
  anchor.click();
  URL.revokeObjectURL(url);
  return `${fileName} 已开始导出。`;
}

async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || String(response.status);
}

async function postProviderAction<T>(url: string, payload: unknown): Promise<T> {
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!response.ok) {
    throw new Error(await readErrorText(response));
  }
  return response.json() as Promise<T>;
}

function normalizeSettingsResponse(payload: unknown): SettingsResponse {
  const data = payload as SettingsResponse;
  return {
    ...data,
    approved_directories: data.approved_directories ?? [],
    available_models: data.available_models ?? [],
    available_workspaces: data.available_workspaces ?? [],
    external_connections: data.external_connections ?? [],
    providers: data.providers ?? [],
  };
}

function normalizeProviderSettings(payload: unknown): ProviderSettingsResponse {
  const data = payload as ProviderSettingsResponse;
  return {
    active_provider_id: data.active_provider_id,
    providers: (data.providers ?? []).map(normalizeProviderItem),
  };
}

function normalizeProviderItem(item: ProviderSettingsItem): ProviderSettingsItem {
  return {
    ...item,
    credential_status: normalizeCredentialStatus(item.credential_status),
  };
}

function normalizeCredentialStatus(status: ProviderCredentialStatus): ProviderCredentialStatus {
  return {
    ...status,
    last_test_status: status.last_test_status ?? "idle",
  };
}
