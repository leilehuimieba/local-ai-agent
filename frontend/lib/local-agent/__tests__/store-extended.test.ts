import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
  useRuntimeStore,
  useUIStore,
  useSettingsStore,
  useKnowledgeStore,
  useLogsStore,
  useMemoryStore,
} from "../store";

vi.mock("../api", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../api")>();
  return {
    ...actual,
    addSessionMessage: vi.fn().mockResolvedValue({}),
    fetchSessionMessages: vi.fn().mockResolvedValue({ items: [] }),
    fetchKnowledgeItems: vi.fn().mockResolvedValue({ items: [], categories: [], tags: [] }),
    fetchLogs: vi.fn().mockResolvedValue({ items: [] }),
    fetchMemories: vi.fn().mockResolvedValue({ items: [] }),
    deleteMemory: vi.fn().mockResolvedValue({ items: [] }),
    fetchSettings: vi.fn().mockResolvedValue({
      app_name: "Test",
      mode: "standard",
      model: { model_id: "", display_name: "", provider_id: "" },
      workspace: { workspace_id: "default", name: "默认工作区", root_path: "/" },
      available_models: [],
      available_workspaces: [],
      approved_directories: [],
      directory_prompt_enabled: true,
      show_risk_level: true,
      ports: {},
      runtime_status: { ok: true, name: "", version: "" },
      embedding: { provider_id: "", model_name: "" },
      providers: [],
      mcp: { servers: [], tools: [] },
    }),
    fetchProviderSettings: vi.fn().mockResolvedValue({ providers: [] }),
    updateSettings: vi.fn().mockResolvedValue(undefined),
  };
});

function resetRuntimeStore() {
  useRuntimeStore.setState({
    sessionId: "test-session",
    currentRunId: "",
    currentTaskTitle: "",
    runState: "idle",
    connectionState: "connected",
    messages: [],
    events: [],
    confirmation: null,
    composeValue: "",
    criticalError: null,
    submitError: null,
  });
}

describe("useRuntimeStore extended", () => {
  beforeEach(() => {
    resetRuntimeStore();
    vi.clearAllMocks();
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("startNewRun resets state and sets runState to running", () => {
    useRuntimeStore.getState().startNewRun("Test Task");
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("running");
    expect(state.currentTaskTitle).toBe("Test Task");
    expect(state.currentRunId).not.toBe("");
    expect(state.events).toEqual([]);
    expect(state.confirmation).toBeNull();
  });

  it("acceptRun sets sessionId and runId", () => {
    useRuntimeStore.getState().acceptRun("s1", "r1");
    const state = useRuntimeStore.getState();
    expect(state.sessionId).toBe("s1");
    expect(state.currentRunId).toBe("r1");
    expect(state.runState).toBe("running");
  });

  it("completeRun sets runState to completed", () => {
    useRuntimeStore.getState().startNewRun("T");
    useRuntimeStore.getState().completeRun();
    expect(useRuntimeStore.getState().runState).toBe("completed");
    expect(useRuntimeStore.getState().currentRunId).toBe("");
  });

  it("failRun sets runState to failed and criticalError", () => {
    useRuntimeStore.getState().failRun("something broke");
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("failed");
    expect(state.criticalError).toBe("something broke");
  });

  it("clearSession resets everything", () => {
    useRuntimeStore.getState().startNewRun("T");
    useRuntimeStore.getState().clearSession();
    const state = useRuntimeStore.getState();
    expect(state.messages).toEqual([]);
    expect(state.events).toEqual([]);
    expect(state.runState).toBe("idle");
    expect(state.currentRunId).toBe("");
    expect(state.currentTaskTitle).toBe("");
  });

  it("addEvent appends event with generated id", () => {
    useRuntimeStore.getState().addEvent({
      event_type: "test",
      run_id: "r1",
      stage: "test",
      summary: "summary",
    });
    const state = useRuntimeStore.getState();
    expect(state.events).toHaveLength(1);
    expect(state.events[0].event_id).toBeDefined();
    expect(state.events[0].timestamp).toBeDefined();
  });

  it("setConnectionState updates connection state", () => {
    useRuntimeStore.getState().setConnectionState("disconnected");
    expect(useRuntimeStore.getState().connectionState).toBe("disconnected");
  });

  it("editAndResend truncates messages and sets composeValue for user message", () => {
    useRuntimeStore.getState().addMessage({ role: "user", content: "hello" });
    useRuntimeStore.getState().addMessage({ role: "assistant", content: "hi" });
    const msgId = useRuntimeStore.getState().messages[0].id;
    useRuntimeStore.getState().editAndResend(msgId);
    const state = useRuntimeStore.getState();
    expect(state.messages).toHaveLength(0);
    expect(state.composeValue).toBe("hello");
    expect(state.runState).toBe("idle");
  });

  it("editAndResend does nothing for non-user message", () => {
    useRuntimeStore.getState().addMessage({ role: "assistant", content: "hi" });
    const msgId = useRuntimeStore.getState().messages[0].id;
    useRuntimeStore.getState().editAndResend(msgId);
    expect(useRuntimeStore.getState().messages).toHaveLength(1);
  });

  it("applyEvent handles confirmation_required", () => {
    useRuntimeStore.getState().applyEvent({
      event_type: "confirmation_required",
      run_id: "r1",
      stage: "action",
      summary: "confirm?",
      metadata: {
        confirmation_id: "c1",
        risk_level: "high",
        action_summary: "delete file",
        reason: "dangerous",
        target_paths: "/a\n/b",
        hazards: "h1\nh2",
        alternatives: "alt1",
        tool_name: "rm",
        tool_arguments_json: '{"path":"/a"}',
        patch_preview_report_json: "",
      },
    });
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("awaiting_confirmation");
    expect(state.confirmation?.confirmation_id).toBe("c1");
    expect(state.confirmation?.risk_level).toBe("high");
    expect(state.confirmation?.target_paths).toEqual(["/a", "/b"]);
  });

  it("applyEvent handles action_completed with final_answer", () => {
    useRuntimeStore.getState().applyEvent({
      event_type: "action_completed",
      run_id: "r1",
      stage: "action",
      summary: "done",
      metadata: { final_answer: "answer" },
    });
    const state = useRuntimeStore.getState();
    expect(state.messages).toHaveLength(1);
    expect(state.messages[0].content).toBe("answer");
    expect(state.messages[0].isStreaming).toBe(true);
  });

  it("applyEvent handles run_finished", () => {
    useRuntimeStore.getState().startNewRun("T");
    useRuntimeStore.getState().applyEvent({
      event_type: "run_finished",
      run_id: "r1",
      stage: "completion",
      summary: "finished",
      metadata: { final_answer: "final" },
    });
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("completed");
    expect(state.messages[state.messages.length - 1].content).toBe("final");
  });

  it("applyEvent handles run_failed", () => {
    useRuntimeStore.getState().startNewRun("T");
    useRuntimeStore.getState().applyEvent({
      event_type: "run_failed",
      run_id: "r1",
      stage: "completion",
      summary: "broke",
      metadata: {},
    });
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("failed");
    expect(state.criticalError).toBe("broke");
  });

  it("applyEvent ignores events when not running", () => {
    useRuntimeStore.getState().applyEvent({
      event_type: "run_finished",
      run_id: "r1",
      stage: "completion",
      summary: "finished",
      metadata: {},
    });
    expect(useRuntimeStore.getState().runState).toBe("idle");
  });

  it("cancelRun stops streaming and resets state", () => {
    useRuntimeStore.getState().startNewRun("T");
    useRuntimeStore.getState().applyEvent({
      event_type: "action_completed",
      run_id: "r1",
      stage: "action",
      summary: "partial",
      metadata: { final_answer: "partial" },
    });
    useRuntimeStore.getState().cancelRun();
    const state = useRuntimeStore.getState();
    expect(state.runState).toBe("idle");
    expect(state.events).toEqual([]);
  });

  it("resumeSession restores from backend messages", async () => {
    const { fetchSessionMessages } = await import("../api");
    vi.mocked(fetchSessionMessages).mockResolvedValueOnce({
      items: [{ id: "m1", session_id: "s1", role: "user", content: "hello", timestamp: "2024-01-01" }],
    });
    useRuntimeStore.getState().resumeSession("s1");
    await new Promise((r) => setTimeout(r, 50));
    const state = useRuntimeStore.getState();
    expect(state.sessionId).toBe("s1");
    expect(state.messages).toHaveLength(1);
    expect(state.messages[0].content).toBe("hello");
  });

  it("resumeSession falls back to localStorage", async () => {
    const { fetchSessionMessages } = await import("../api");
    vi.mocked(fetchSessionMessages).mockRejectedValueOnce(new Error("fail"));
    localStorage.setItem("la:session:s2", JSON.stringify({ sessionId: "s2", messages: [{ id: "m2", role: "assistant", content: "cached", timestamp: "2024-01-01" }], runState: "idle", currentTaskTitle: "Cached" }));
    useRuntimeStore.getState().resumeSession("s2");
    await new Promise((r) => setTimeout(r, 50));
    const state = useRuntimeStore.getState();
    expect(state.sessionId).toBe("s2");
    expect(state.messages).toHaveLength(1);
    expect(state.messages[0].content).toBe("cached");
  });
});

describe("useUIStore", () => {
  beforeEach(() => {
    useUIStore.setState({
      activeView: "task",
      leftSidebarExpanded: false,
      rightDrawerOpen: true,
      mobileMenuOpen: false,
      mobileDrawerOpen: false,
    });
  });

  it("toggleLeftSidebar flips state", () => {
    useUIStore.getState().toggleLeftSidebar();
    expect(useUIStore.getState().leftSidebarExpanded).toBe(true);
  });

  it("toggleRightDrawer flips state", () => {
    useUIStore.getState().toggleRightDrawer();
    expect(useUIStore.getState().rightDrawerOpen).toBe(false);
  });

  it("setActiveView updates view", () => {
    useUIStore.getState().setActiveView("settings");
    expect(useUIStore.getState().activeView).toBe("settings");
  });
});

describe("useSettingsStore", () => {
  beforeEach(() => {
    useSettingsStore.setState({
      mode: "standard",
      model: { model_id: "", display_name: "", provider_id: "" },
      workspace: { workspace_id: "default", name: "默认工作区", root_path: "/" },
      embedding_provider_id: "",
      providers: [],
      available_models: [],
      available_workspaces: [],
      approved_directories: [],
      directory_prompt_enabled: true,
      show_risk_level: true,
      ports: {},
    });
    vi.clearAllMocks();
  });

  it("setMode updates mode", () => {
    useSettingsStore.getState().setMode("agent");
    expect(useSettingsStore.getState().mode).toBe("agent");
  });

  it("addProvider appends provider", () => {
    useSettingsStore.getState().addProvider({ provider_id: "p1", display_name: "P1", base_url: "http://x" });
    expect(useSettingsStore.getState().providers).toHaveLength(1);
  });

  it("removeProvider filters by id", () => {
    useSettingsStore.getState().addProvider({ provider_id: "p1", display_name: "P1", base_url: "http://x" });
    useSettingsStore.getState().removeProvider("p1");
    expect(useSettingsStore.getState().providers).toHaveLength(0);
  });

  it("updateProvider patches fields", () => {
    useSettingsStore.getState().addProvider({ provider_id: "p1", display_name: "P1", base_url: "http://x" });
    useSettingsStore.getState().updateProvider("p1", { display_name: "P1-new" });
    expect(useSettingsStore.getState().providers[0].display_name).toBe("P1-new");
  });

  it("loadSettings fetches and populates state", async () => {
    await useSettingsStore.getState().loadSettings();
    await new Promise((r) => setTimeout(r, 50));
    expect(useSettingsStore.getState().mode).toBe("standard");
  });

  it("loadSettings falls back to mapped providers when provider settings fail", async () => {
    const { fetchSettings, fetchProviderSettings } = await import("../api");
    vi.mocked(fetchSettings).mockResolvedValueOnce({
      app_name: "Test",
      mode: "standard",
      model: { model_id: "", display_name: "", provider_id: "" },
      workspace: { workspace_id: "default", name: "默认工作区", root_path: "/" },
      available_models: [],
      available_workspaces: [],
      approved_directories: [],
      directory_prompt_enabled: true,
      show_risk_level: true,
      ports: {},
      runtime_status: { ok: true, name: "", version: "" },
      embedding: { provider_id: "", model_name: "" },
      providers: [{ provider_id: "p1", display_name: "P1", base_url: "http://x" }],
      mcp: { servers: [], tools: [] },
    });
    vi.mocked(fetchProviderSettings).mockRejectedValueOnce(new Error("fail"));
    await useSettingsStore.getState().loadSettings();
    await new Promise((r) => setTimeout(r, 50));
    expect(useSettingsStore.getState().providers).toHaveLength(1);
    expect(useSettingsStore.getState().providers[0].provider_id).toBe("p1");
  });

  it("loadSettings maps provider status to active when credential is valid", async () => {
    const { fetchProviderSettings } = await import("../api");
    vi.mocked(fetchProviderSettings).mockResolvedValueOnce({
      providers: [{
        provider_id: "p1",
        display_name: "P1",
        base_url: "http://x",
        credential_status: { has_credential: true, last_test_status: "success", apply_status: "applied", pending_reload: false },
      }],
    });
    await useSettingsStore.getState().loadSettings();
    await new Promise((r) => setTimeout(r, 50));
    expect(useSettingsStore.getState().providers[0].status).toBe("active");
  });

  it("loadSettings maps provider status to error when last test failed", async () => {
    const { fetchProviderSettings } = await import("../api");
    vi.mocked(fetchProviderSettings).mockResolvedValueOnce({
      providers: [{
        provider_id: "p1",
        display_name: "P1",
        base_url: "http://x",
        credential_status: { has_credential: true, last_test_status: "failed", apply_status: "applied", pending_reload: false },
      }],
    });
    await useSettingsStore.getState().loadSettings();
    await new Promise((r) => setTimeout(r, 50));
    expect(useSettingsStore.getState().providers[0].status).toBe("error");
  });

  it("addDirectory fetches settings after update", async () => {
    await useSettingsStore.getState().addDirectory("dir1", "/path1");
    await new Promise((r) => setTimeout(r, 50));
    expect(useSettingsStore.getState().approved_directories).toEqual([]);
  });

  it("removeDirectory filters by rootPath", async () => {
    useSettingsStore.setState({ approved_directories: [{ approval_id: "a1", workspace_id: "w1", name: "dir1", root_path: "/path1" }] });
    await useSettingsStore.getState().removeDirectory("/path1");
    expect(useSettingsStore.getState().approved_directories).toHaveLength(0);
  });
});

describe("useKnowledgeStore", () => {
  beforeEach(() => {
    useKnowledgeStore.setState({
      items: [],
      categories: [],
      allTags: [],
      selectedItem: null,
      searchQuery: "",
      selectedCategory: null,
      selectedTags: [],
    });
  });

  it("toggleTag adds and removes tags", () => {
    useKnowledgeStore.getState().toggleTag("t1");
    expect(useKnowledgeStore.getState().selectedTags).toEqual(["t1"]);
    useKnowledgeStore.getState().toggleTag("t1");
    expect(useKnowledgeStore.getState().selectedTags).toEqual([]);
  });

  it("addItem appends item", () => {
    const item = { id: "1", title: "T", summary: "", content: "", category: "", tags: [], citationCount: 0, source: "", createdAt: "", updatedAt: "" };
    useKnowledgeStore.getState().addItem(item);
    expect(useKnowledgeStore.getState().items).toHaveLength(1);
  });

  it("updateItem patches by id", () => {
    const item = { id: "1", title: "T", summary: "", content: "", category: "", tags: [], citationCount: 0, source: "", createdAt: "", updatedAt: "" };
    useKnowledgeStore.getState().addItem(item);
    useKnowledgeStore.getState().updateItem("1", { title: "T2" });
    expect(useKnowledgeStore.getState().items[0].title).toBe("T2");
  });

  it("removeItem filters by id and clears selectedItem", () => {
    const item = { id: "1", title: "T", summary: "", content: "", category: "", tags: [], citationCount: 0, source: "", createdAt: "", updatedAt: "" };
    useKnowledgeStore.getState().addItem(item);
    useKnowledgeStore.getState().setSelectedItem(item);
    useKnowledgeStore.getState().removeItem("1");
    expect(useKnowledgeStore.getState().items).toHaveLength(0);
    expect(useKnowledgeStore.getState().selectedItem).toBeNull();
  });

  it("loadItems fetches and updates items", async () => {
    const { fetchKnowledgeItems } = await import("../api");
    vi.mocked(fetchKnowledgeItems).mockResolvedValueOnce({ items: [], categories: [], tags: [] });
    await useKnowledgeStore.getState().loadItems();
    expect(useKnowledgeStore.getState().items).toEqual([]);
  });

  it("loadItems ignores fetch errors", async () => {
    const { fetchKnowledgeItems } = await import("../api");
    vi.mocked(fetchKnowledgeItems).mockRejectedValueOnce(new Error("fail"));
    await useKnowledgeStore.getState().loadItems();
    expect(useKnowledgeStore.getState().items).toHaveLength(0);
  });
});

describe("useLogsStore", () => {
  beforeEach(() => {
    useLogsStore.setState({
      runs: [],
      selectedRun: null,
      statusFilter: "all",
      timeFilter: "7days",
      searchQuery: "",
    });
  });

  it("setStatusFilter updates filter", () => {
    useLogsStore.getState().setStatusFilter("failed");
    expect(useLogsStore.getState().statusFilter).toBe("failed");
  });

  it("setTimeFilter updates filter", () => {
    useLogsStore.getState().setTimeFilter("today");
    expect(useLogsStore.getState().timeFilter).toBe("today");
  });

  it("setRuns updates runs", () => {
    useLogsStore.getState().setRuns([{ run_id: "r1", session_id: "s1", title: "T", status: "completed", started_at: "", duration_ms: 0, event_count: 0 }]);
    expect(useLogsStore.getState().runs).toHaveLength(1);
  });

  it("setSelectedRun updates selected run", () => {
    const run = { run_id: "r1", session_id: "s1", title: "T", status: "completed", started_at: "", duration_ms: 0, event_count: 0 };
    useLogsStore.getState().setSelectedRun(run);
    expect(useLogsStore.getState().selectedRun).toEqual(run);
  });

  it("setSearchQuery updates query", () => {
    useLogsStore.getState().setSearchQuery("query");
    expect(useLogsStore.getState().searchQuery).toBe("query");
  });

  it("loadLogs fetches and updates runs", async () => {
    const { fetchLogs } = await import("../api");
    vi.mocked(fetchLogs).mockResolvedValueOnce({ items: [], runs: [] });
    useLogsStore.getState().loadLogs();
    await new Promise((r) => setTimeout(r, 50));
    expect(useLogsStore.getState().runs).toEqual([]);
  });
});

describe("useMemoryStore", () => {
  beforeEach(() => {
    useMemoryStore.setState({ memories: [] });
    vi.clearAllMocks();
  });

  it("addMemory appends memory", () => {
    useMemoryStore.getState().addMemory({ id: "m1", kind: "note", title: "T", summary: "S", content: "C", createdAt: "2024-01-01" });
    expect(useMemoryStore.getState().memories).toHaveLength(1);
  });

  it("setMemories replaces memories", () => {
    useMemoryStore.getState().setMemories([{ id: "m1", kind: "note", title: "T", summary: "S", content: "C", createdAt: "2024-01-01" }]);
    expect(useMemoryStore.getState().memories).toHaveLength(1);
  });

  it("removeMemory updates memories from API", async () => {
    const { deleteMemory } = await import("../api");
    vi.mocked(deleteMemory).mockResolvedValueOnce({
      items: [{ id: "m1", kind: "note", title: "T", summary: "S", content: "C", created_at: "2024-01-01" }],
    });
    useMemoryStore.getState().removeMemory("m1");
    await new Promise((r) => setTimeout(r, 50));
    expect(useMemoryStore.getState().memories).toHaveLength(1);
  });

  it("loadMemories fetches from API", async () => {
    const { fetchMemories } = await import("../api");
    vi.mocked(fetchMemories).mockResolvedValueOnce({
      items: [{ id: "m1", kind: "note", title: "T", summary: "S", content: "C", created_at: "2024-01-01" }],
    });
    useMemoryStore.getState().loadMemories();
    await new Promise((r) => setTimeout(r, 50));
    expect(useMemoryStore.getState().memories).toHaveLength(1);
  });
});
