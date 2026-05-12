import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent, waitFor } from "@testing-library/react"
import {
  RuntimeSection,
  ModelSection,
  EmbeddingSection,
  RiskSection,
  MemorySection,
  DiagnosticsSection,
  ProvidersSection,
  WorkspacesSection,
  MCPServerCard,
  MCPAddForm,
} from "../settings-view"

vi.mock("@/lib/local-agent/api", () => ({
  runDiagnosticsCheck: vi.fn().mockResolvedValue({
    services: [{ label: "Gateway", status: "ok", detail: "running", hint: "" }],
  }),
  updateSettings: vi.fn().mockResolvedValue(undefined),
}))

vi.mock("@/lib/local-agent/store", () => ({
  useSettingsStore: vi.fn((selector?: (s: unknown) => unknown) => {
    const store = {
      ports: { gateway: 8080, runtime: 50051 },
      runtime_status: { ok: true, name: "Runtime", version: "v1.0" },
      mode: "standard",
      model: { model_id: "m1" },
      available_models: [{ model_id: "m1", display_name: "Model One", provider_id: "p1" }],
      setMode: vi.fn(),
      setModel: vi.fn(),
      embedding_provider_id: "p1",
      providers: [
        { provider_id: "p1", display_name: "Provider One", status: "active", credential_status: { has_credential: true, api_key_masked: "sk-***" }, base_url: "http://localhost", chat_completions_path: "/v1/chat/completions", models_path: "/v1/models" },
      ],
      active_provider_id: "p1",
      embedding: { model_name: "embed-1" },
      setEmbeddingProvider: vi.fn(),
      directory_prompt_enabled: true,
      show_risk_level: false,
      setDirectoryPromptEnabled: vi.fn(),
      setShowRiskLevel: vi.fn(),
      workspace: { workspace_id: "w1", name: "Default" },
      available_workspaces: [{ workspace_id: "w1", name: "Default" }],
      approved_directories: [{ name: "src", path: "/src", workspace_id: "w1" }],
      setWorkspace: vi.fn(),
      addDirectory: vi.fn(),
      removeDirectory: vi.fn(),
      loadSettings: vi.fn(),
    }
    return selector ? selector(store) : store
  }),
  useMemoryStore: vi.fn((selector?: (s: unknown) => unknown) => {
    const store = {
      memories: [{ id: "mem1", kind: "note", title: "Note 1", content: "Content 1", createdAt: "2024-01-01" }],
      loadMemories: vi.fn(),
      removeMemory: vi.fn(),
    }
    return selector ? selector(store) : store
  }),
  useUIStore: vi.fn((selector?: (s: unknown) => unknown) => {
    const store = { searchQuery: "", setSearchQuery: vi.fn() }
    return selector ? selector(store) : store
    return selector ? selector(store) : store
  }),
}))

describe("Settings Sections", () => {
  it("RuntimeSection renders connection cards", () => {
    render(<RuntimeSection />)
    expect(screen.getByText("网关服务")).toBeInTheDocument()
    expect(screen.getByText("Runtime")).toBeInTheDocument()
    expect(screen.getByText("localhost:8080")).toBeInTheDocument()
    expect(screen.getByText("localhost:50051")).toBeInTheDocument()
  })

  it("ModelSection renders model and mode selects", () => {
    render(<ModelSection />)
    expect(screen.getByRole("heading", { name: "模型" })).toBeInTheDocument()
    expect(screen.getByText("访问模式")).toBeInTheDocument()
  })

  it("EmbeddingSection renders provider select", () => {
    render(<EmbeddingSection />)
    expect(screen.getByText("嵌入")).toBeInTheDocument()
    expect(screen.getByText("Provider One")).toBeInTheDocument()
    expect(screen.getByText("当前模型：embed-1")).toBeInTheDocument()
  })

  it("RiskSection renders toggles", () => {
    render(<RiskSection />)
    expect(screen.getByText("风险")).toBeInTheDocument()
    expect(screen.getByText("新目录访问前确认")).toBeInTheDocument()
    expect(screen.getByText("显示风险等级")).toBeInTheDocument()
  })

  it("MemorySection renders memories", () => {
    render(<MemorySection />)
    expect(screen.getByText("记忆")).toBeInTheDocument()
    expect(screen.getByText("Note 1")).toBeInTheDocument()
  })



  it("DiagnosticsSection renders health check button", () => {
    render(<DiagnosticsSection />)
    expect(screen.getByText("诊断")).toBeInTheDocument()
    expect(screen.getByText("健康检查")).toBeInTheDocument()
  })

  it("runs health check and shows results", async () => {
    render(<DiagnosticsSection />)
    fireEvent.click(screen.getByText("健康检查"))
    await waitFor(() => {
      expect(screen.getByText("Gateway")).toBeInTheDocument()
      expect(screen.getByText("running")).toBeInTheDocument()
    })
  })

  it("shows error when health check fails", async () => {
    const { runDiagnosticsCheck } = await import("@/lib/local-agent/api")
    vi.mocked(runDiagnosticsCheck).mockRejectedValueOnce(new Error("fail"))
    render(<DiagnosticsSection />)
    fireEvent.click(screen.getByText("健康检查"))
    await waitFor(() => {
      expect(screen.getByText("健康检查请求失败")).toBeInTheDocument()
    })
  })

  it("ProvidersSection renders provider rows", () => {
    render(<ProvidersSection />)
    expect(screen.getByRole("heading", { name: "服务商" })).toBeInTheDocument()
    expect(screen.getByText("sk-***")).toBeInTheDocument()
    expect(screen.getByText("当前")).toBeInTheDocument()
  })

  it("WorkspacesSection renders workspace and directories", () => {
    render(<WorkspacesSection />)
    expect(screen.getByText("工作区")).toBeInTheDocument()
    expect(screen.getByText("Default")).toBeInTheDocument()
    expect(screen.getByText("src")).toBeInTheDocument()
  })

  it("MCPServerCard shows 未就绪 status", () => {
    render(
      <MCPServerCard
        server={{ id: "s1", name: "Test", type: "http", url: "http://localhost", enabled: true, ready: false, tool_count: 0, allowed_tool_count: 0, blocked_tool_count: 0, requires_policy: false }}
        tools={[]}
        onReload={vi.fn()}
      />
    )
    expect(screen.getByText("未就绪")).toBeInTheDocument()
  })

  it("MCPAddForm updates inputs and submits for HTTP", async () => {
    render(<MCPAddForm onReload={vi.fn().mockResolvedValue(undefined)} />)
    const nameInput = screen.getByPlaceholderText("名称")
    const urlInput = screen.getByPlaceholderText("HTTP 地址 (如 http://127.0.0.1:3344/mcp)")
    fireEvent.change(nameInput, { target: { value: "MyMCP" } })
    fireEvent.change(urlInput, { target: { value: "http://mcp.local" } })
    expect(nameInput).toHaveValue("MyMCP")
    expect(urlInput).toHaveValue("http://mcp.local")
    fireEvent.click(screen.getByText("添加"))
    await waitFor(() => {
      expect(urlInput).toHaveValue("")
    })
  })
})
