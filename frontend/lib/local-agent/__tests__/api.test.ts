process.env.NEXT_PUBLIC_API_BASE = "http://localhost:8080";

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
  submitChatRun,
  submitChatRetry,
  submitChatCancel,
  submitConfirmationDecision,
  fetchSettings,
  updateSettings,
  fetchMCPTools,
  fetchMCPAudits,
  callMCPTool,
  fetchProviderSettings,
  testProvider,
  saveProvider,
  applyProvider,
  removeProviderCredential,
  fetchKnowledgeItems,
  createKnowledgeItem,
  updateKnowledgeItem,
  deleteKnowledgeItem,
  uploadKnowledgeFile,
  askKnowledgeBase,
  fetchLogs,
  fetchSystemInfo,
  runDiagnosticsCheck,
  fetchMemories,
  deleteMemory,
  fetchSessions,
  fetchSessionMessages,
  addSessionMessage,
  type SubmitChatRunPayload,
  type ChatRetryRequest,
  type SubmitConfirmationPayload,
  type SettingsUpdatePayload,
  type ProviderCredentialPayload,
} from "../api";



function mockResponse(body: unknown, ok = true, status = 200) {
  return {
    ok,
    status,
    text: vi.fn().mockResolvedValue(JSON.stringify(body)),
    json: vi.fn().mockResolvedValue(body),
  } as unknown as Response;
}

function mockErrorResponse(status: number, text = "error") {
  return {
    ok: false,
    status,
    text: vi.fn().mockResolvedValue(text),
    json: vi.fn().mockRejectedValue(new Error("invalid json")),
  } as unknown as Response;
}

describe("api.ts", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
    global.fetch = fetchMock;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe("Chat APIs", () => {
    const payload: SubmitChatRunPayload = {
      sessionId: "s1",
      userInput: "hello",
      mode: "standard",
      model: { model_id: "m1", display_name: "M1", provider_id: "p1" },
      workspace: { workspace_id: "w1", name: "W1", root_path: "/" },
      knowledgeBaseId: "kb1",
    };

    it("submitChatRun returns parsed JSON on success", async () => {
      const accepted = { accepted: true, session_id: "s1", run_id: "r1", initial_status: "running" };
      fetchMock.mockResolvedValue(mockResponse(accepted));
      const result = await submitChatRun(payload);
      expect(result).toEqual(accepted);
      expect(global.fetch).toHaveBeenCalledWith(
        "/api/v1/chat/run",
        expect.objectContaining({ method: "POST" }),
      );
    });

    it("submitChatRun throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500, "server error"));
      await expect(submitChatRun(payload)).rejects.toThrow("提交任务失败");
    });

    it("submitChatRetry returns parsed JSON on success", async () => {
      const req: ChatRetryRequest = { session_id: "s1", run_id: "r1" };
      const accepted = { accepted: true, session_id: "s1", run_id: "r1", initial_status: "running" };
      fetchMock.mockResolvedValue(mockResponse(accepted));
      const result = await submitChatRetry(req);
      expect(result).toEqual(accepted);
    });

    it("submitChatRetry throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(submitChatRetry({ session_id: "s1", run_id: "r1" })).rejects.toThrow("提交重试失败");
    });

    it("submitChatCancel resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      await expect(submitChatCancel("s1", "r1")).resolves.toBeUndefined();
    });

    it("submitChatCancel throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(submitChatCancel("s1", "r1")).rejects.toThrow("取消任务失败");
    });
  });

  describe("Confirmation APIs", () => {
    it("submitConfirmationDecision resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      const payload: SubmitConfirmationPayload = {
        confirmationId: "c1",
        runId: "r1",
        decision: "approve",
        remember: true,
      };
      await expect(submitConfirmationDecision(payload)).resolves.toBeUndefined();
    });

    it("submitConfirmationDecision throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(
        submitConfirmationDecision({ confirmationId: "c1", runId: "r1", decision: "reject", remember: false }),
      ).rejects.toThrow("提交确认失败");
    });
  });

  describe("Settings APIs", () => {
    it("fetchSettings returns parsed JSON", async () => {
      const settings = { app_name: "Test", mode: "standard", model: { model_id: "m1", display_name: "M1", provider_id: "p1" }, workspace: { workspace_id: "w1", name: "W1", root_path: "/" }, available_models: [], available_workspaces: [], approved_directories: [], directory_prompt_enabled: true, show_risk_level: true, ports: {}, runtime_status: { ok: true, name: "rt", version: "1" }, embedding: { provider_id: "", model_name: "" }, providers: [], mcp: { servers: [], tools: [] } };
      fetchMock.mockResolvedValue(mockResponse(settings));
      const result = await fetchSettings();
      expect(result).toEqual(settings);
    });

    it("fetchSettings throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(fetchSettings()).rejects.toThrow("获取设置失败");
    });

    it("updateSettings resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      const patch: SettingsUpdatePayload = { mode: "agent" };
      await expect(updateSettings(patch)).resolves.toBeUndefined();
    });

    it("updateSettings throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(updateSettings({})).rejects.toThrow("更新设置失败");
    });

    it("fetchMCPTools returns tools array", async () => {
      const tools = [{ name: "t1", description: "d1", allowed: true, risk_level: "low", requires_confirmation: false, audit_enabled: false, policy_source: "" }];
      fetchMock.mockResolvedValue(mockResponse({ tools }));
      const result = await fetchMCPTools();
      expect(result).toEqual(tools);
    });

    it("fetchMCPTools throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(fetchMCPTools()).rejects.toThrow("获取 MCP 工具失败");
    });

    it("fetchMCPAudits returns items array with query params", async () => {
      const items = [{ audit_id: "a1", timestamp: "2024-01-01", server_id: "s1", tool_name: "t1", allowed: true, risk_level: "low", requires_confirmation: false, audit_enabled: false, policy_source: "", arguments_hash: "", outcome: "success", elapsed_ms: 100 }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchMCPAudits({ limit: 10, server_id: "s1", tool_name: "t1" });
      expect(result).toEqual(items);
      const url = fetchMock.mock.calls[0][0] as string;
      expect(url).toContain("limit=10");
      expect(url).toContain("server_id=s1");
      expect(url).toContain("tool_name=t1");
    });

    it("callMCPTool returns parsed JSON", async () => {
      fetchMock.mockResolvedValue(mockResponse({ result: "ok" }));
      const result = await callMCPTool("s1", "t1", { a: 1 });
      expect(result).toEqual({ result: "ok" });
    });

    it("callMCPTool throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(callMCPTool("s1", "t1", { a: 1 })).rejects.toThrow("调用 MCP 工具失败");
    });

    it("fetchProviderSettings returns parsed JSON", async () => {
      const data = { providers: [] };
      fetchMock.mockResolvedValue(mockResponse(data));
      const result = await fetchProviderSettings();
      expect(result).toEqual(data);
    });

    it("fetchProviderSettings throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(fetchProviderSettings()).rejects.toThrow("获取服务商设置失败");
    });

    it("testProvider returns parsed JSON", async () => {
      const payload: ProviderCredentialPayload = { provider_id: "p1", display_name: "P1", base_url: "http://x", api_key: "key" };
      fetchMock.mockResolvedValue(mockResponse({ ok: true, message: "ok" }));
      const result = await testProvider(payload);
      expect(result).toEqual({ ok: true, message: "ok" });
    });

    it("saveProvider resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      await expect(saveProvider({ provider_id: "p1", display_name: "P1", base_url: "http://x", api_key: "key" })).resolves.toBeUndefined();
    });

    it("applyProvider resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      await expect(applyProvider("p1")).resolves.toBeUndefined();
    });

    it("removeProviderCredential resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      await expect(removeProviderCredential("p1")).resolves.toBeUndefined();
    });
  });

  describe("Knowledge APIs", () => {
    const wireItem = {
      id: "1",
      title: "T",
      summary: "S",
      content: "C",
      category: "cat",
      tags: ["t"],
      citation_count: 5,
      source: "src",
      created_at: "2024-01-01",
      updated_at: "2024-01-02",
    };

    it("fetchKnowledgeItems normalizes wire items", async () => {
      fetchMock.mockResolvedValue(mockResponse({ items: [wireItem], categories: ["cat"], tags: ["t"] }));
      const result = await fetchKnowledgeItems();
      expect(result.items[0]).toMatchObject({
        citationCount: 5,
        createdAt: "2024-01-01",
        updatedAt: "2024-01-02",
      });
    });

    it("fetchKnowledgeItems throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(fetchKnowledgeItems()).rejects.toThrow("获取知识库失败");
    });

    it("createKnowledgeItem normalizes response", async () => {
      fetchMock.mockResolvedValue(mockResponse(wireItem));
      const result = await createKnowledgeItem({ title: "T", summary: "S", content: "C", category: "cat", tags: ["t"], source: "src" });
      expect(result.citationCount).toBe(5);
    });

    it("updateKnowledgeItem normalizes response", async () => {
      fetchMock.mockResolvedValue(mockResponse(wireItem));
      const result = await updateKnowledgeItem("1", { title: "T2" });
      expect(result.citationCount).toBe(5);
    });

    it("deleteKnowledgeItem resolves on success", async () => {
      fetchMock.mockResolvedValue(mockResponse({}));
      await expect(deleteKnowledgeItem("1")).resolves.toBeUndefined();
    });

    it("uploadKnowledgeFile normalizes response", async () => {
      fetchMock.mockResolvedValue(mockResponse(wireItem));
      const file = new File(["content"], "test.txt", { type: "text/plain" });
      const result = await uploadKnowledgeFile(file);
      expect(result.citationCount).toBe(5);
    });

    it("askKnowledgeBase returns answer and sources", async () => {
      fetchMock.mockResolvedValue(mockResponse({ answer: "A", sources: ["S1"] }));
      const result = await askKnowledgeBase("Q?", "w1");
      expect(result).toEqual({ answer: "A", sources: ["S1"] });
    });

    it("askKnowledgeBase throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(askKnowledgeBase("Q?", "w1")).rejects.toThrow("知识库问答失败");
    });
  });

  describe("Logs APIs", () => {
    it("fetchLogs returns items for events view", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("events");
      expect(result.items).toEqual(items);
      expect(result.runs).toBeUndefined();
    });

    it("fetchLogs returns runs for runs view", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum", completion_status: "completed", task_title: "Task" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs).toHaveLength(1);
      expect(result.runs?.[0].status).toBe("completed");
    });

    it("fetchLogs handles 13-digit timestamps", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "1704067200000", level: "info", category: "cat", source: "src", summary: "sum", completion_status: "completed" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].started_at).toMatch(/^\d{4}-/);
    });

    it("fetchLogs returns failed runs via completion_status", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum", completion_status: "failed" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].status).toBe("failed");
    });

    it("fetchLogs returns failed runs via event_type", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum", event_type: "run_failed" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].status).toBe("failed");
    });

    it("fetchLogs returns running runs by default", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].status).toBe("running");
    });

    it("fetchLogs handles updated_at 13-digit timestamps", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "sum", completion_status: "completed", updated_at: "1704067200000" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].completed_at).toMatch(/^\d{4}-/);
    });

    it("fetchLogs derives title from summary when task_title missing", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "Summary", completion_status: "completed" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].title).toBe("Summary");
    });

    it("fetchLogs falls back to unnamed task title", async () => {
      const items = [{ log_id: "l1", session_id: "s1", run_id: "r1", timestamp: "2024-01-01", level: "info", category: "cat", source: "src", summary: "", completion_status: "completed" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchLogs("runs");
      expect(result.runs?.[0].title).toBe("未命名任务");
    });
  });

  describe("System APIs", () => {
    it("fetchSystemInfo returns parsed JSON", async () => {
      fetchMock.mockResolvedValue(mockResponse({ repo_root: "/", runtime_status: "ok", version: "1" }));
      const result = await fetchSystemInfo();
      expect(result.version).toBe("1");
    });

    it("fetchSystemInfo throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(fetchSystemInfo()).rejects.toThrow("获取系统信息失败");
    });

    it("runDiagnosticsCheck returns parsed JSON", async () => {
      const data = { checked_at: "2024-01-01", overall_ok: true, diagnostics: { checked_at: "", repo_root: "/", repo_root_exists: true, storage_root: "/tmp", storage_root_exists: true, runtime_reachable: true, runtime_version: "1", provider_count: 1, model_count: 1, workspace_count: 1 }, services: [], warnings: [], errors: [] };
      fetchMock.mockResolvedValue(mockResponse(data));
      const result = await runDiagnosticsCheck();
      expect(result.overall_ok).toBe(true);
    });

    it("runDiagnosticsCheck throws on HTTP error", async () => {
      fetchMock.mockResolvedValue(mockErrorResponse(500));
      await expect(runDiagnosticsCheck()).rejects.toThrow("健康检查失败");
    });
  });

  describe("Memory APIs", () => {
    it("fetchMemories returns items", async () => {
      const items = [{ id: "m1", kind: "note", title: "T", summary: "S", content: "C", created_at: "2024-01-01" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchMemories();
      expect(result.items).toHaveLength(1);
    });

    it("deleteMemory returns updated items", async () => {
      const items = [{ id: "m1", kind: "note", title: "T", summary: "S", content: "C", created_at: "2024-01-01" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await deleteMemory("m1");
      expect(result.items).toHaveLength(1);
    });
  });

  describe("Session APIs", () => {
    it("fetchSessions returns items", async () => {
      const items = [{ id: "s1", title: "T", created_at: "2024-01-01", updated_at: "2024-01-02" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchSessions();
      expect(result.items).toHaveLength(1);
    });

    it("fetchSessionMessages returns items", async () => {
      const items = [{ id: "m1", session_id: "s1", role: "user", content: "hello", timestamp: "2024-01-01" }];
      fetchMock.mockResolvedValue(mockResponse({ items }));
      const result = await fetchSessionMessages("s1");
      expect(result.items).toHaveLength(1);
    });

    it("addSessionMessage returns created item", async () => {
      const item = { id: "m1", session_id: "s1", role: "user", content: "hello", timestamp: "2024-01-01" };
      fetchMock.mockResolvedValue(mockResponse(item));
      const result = await addSessionMessage("s1", { role: "user", content: "hello" });
      expect(result).toEqual(item);
    });
  });
});
