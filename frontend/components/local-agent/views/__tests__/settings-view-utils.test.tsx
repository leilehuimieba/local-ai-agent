import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent } from "@testing-library/react"
import { RadioGroup } from "@/components/ui/radio-group"
import {
  ConnectionCard,
  ModeOption,
  selectModel,
  ProviderButton,
  ProviderActionNotice,
  successMessage,
  errorMessage,
  RiskToggle,
  diagnosticRows,
  normalizeDiagnosticStatus,
  diagnosticColor,
  diagnosticLabel,
  DiagnosticResultRow,
  DiagnosticResults,
  PolicySwitch,
  MCPToolHeader,
  MCPToolList,
  MCPRiskSelect,
  selectWorkspace,
  ProviderHeader,
  SettingsNavItem,
  ProviderStatusBar,
  DirectoryList,
  DirectoryItem,
  MCPServerCard,
  MCPAddForm,
  ModelSelect,
  ModeSelect,
  SettingsSection,
  SettingsNav,
  CurrentRoutingInfo,
  MCPRemoveButton,
} from "../settings-view"

import type { Model, Provider, DiagnosticRow, Workspace } from "@/lib/local-agent/types"

vi.mock("@/components/local-agent/views/mcp-observability-panel", () => ({
  MCPToolBadges: ({ tool }: { tool: { name: string } }) => <span data-testid="badges">{tool.name}</span>,
}))

describe("selectModel", () => {
  const models: Model[] = [
    { model_id: "m1", display_name: "Model One", provider_id: "p1" },
    { model_id: "m2", display_name: "Model Two", provider_id: "p2" },
  ]

  it("calls onSelect with matching model", () => {
    const onSelect = vi.fn()
    selectModel("m2", models, onSelect)
    expect(onSelect).toHaveBeenCalledWith(models[1])
  })

  it("does nothing when value not found", () => {
    const onSelect = vi.fn()
    selectModel("m3", models, onSelect)
    expect(onSelect).not.toHaveBeenCalled()
  })
})

describe("successMessage", () => {
  it("returns default message for null", () => {
    expect(successMessage(null)).toEqual({ ok: true, text: "操作成功" })
  })

  it("returns default message for string", () => {
    expect(successMessage("done")).toEqual({ ok: true, text: "操作成功" })
  })

  it("extracts message and ok from object", () => {
    expect(successMessage({ message: "Saved", ok: true })).toEqual({ ok: true, text: "Saved" })
  })

  it("uses ok false when provided", () => {
    expect(successMessage({ message: "Failed", ok: false })).toEqual({ ok: false, text: "Failed" })
  })
})

describe("errorMessage", () => {
  it("returns message for Error instance", () => {
    expect(errorMessage(new Error("oops"))).toBe("oops")
  })

  it("returns fallback for non-error", () => {
    expect(errorMessage("fail")).toBe("操作失败")
  })
})

describe("normalizeDiagnosticStatus", () => {
  it.each([
    ["ok", "ok"],
    ["warning", "warning"],
    ["error", "error"],
    ["unknown", "warning"],
    ["", "warning"],
  ] as const)("maps %s to %s", (input, expected) => {
    expect(normalizeDiagnosticStatus(input)).toBe(expected)
  })
})

describe("diagnosticColor", () => {
  it.each([
    ["ok", "text-success"],
    ["warning", "text-warning"],
    ["error", "text-destructive"],
  ] as const)("returns %s for %s", (status, expected) => {
    expect(diagnosticColor(status)).toBe(expected)
  })
})

describe("diagnosticLabel", () => {
  it.each([
    ["ok", "正常"],
    ["warning", "提醒"],
    ["error", "异常"],
  ] as const)("returns %s for %s", (status, expected) => {
    expect(diagnosticLabel(status)).toBe(expected)
  })
})

describe("diagnosticRows", () => {
  it("returns services rows when services present", () => {
    const data = {
      services: [
        { label: "Svc1", status: "ok" as const, detail: "good", hint: "hint1" },
        { label: "Svc2", status: "error" as const, detail: "bad" },
      ],
      diagnostics: { repo_root_exists: true, runtime_reachable: true, runtime_version: "v1", provider_count: 2 },
    }
    const rows = diagnosticRows(data)
    expect(rows).toHaveLength(2)
    expect(rows[0]).toEqual({ category: "Svc1", status: "ok", message: "good", hint: "hint1" })
    expect(rows[1]).toEqual({ category: "Svc2", status: "error", message: "bad", hint: undefined })
  })

  it("returns default diagnostics when no services", () => {
    const data = {
      diagnostics: { repo_root_exists: true, runtime_reachable: false, runtime_version: "v2", provider_count: 0 },
    }
    const rows = diagnosticRows(data)
    expect(rows).toHaveLength(3)
    expect(rows[0]).toEqual({ category: "仓库", status: "ok", message: "可访问" })
    expect(rows[1]).toEqual({ category: "Runtime", status: "error", message: "不可达" })
    expect(rows[2]).toEqual({ category: "服务商", status: "error", message: "0 个已配置" })
  })
})

describe("ConnectionCard", () => {
  it("renders connected status with detail", () => {
    render(<ConnectionCard name="Gateway" port={8080} status="connected" detail="v1.0" />)
    expect(screen.getByText("Gateway")).toBeInTheDocument()
    expect(screen.getByText("localhost:8080")).toBeInTheDocument()
    expect(screen.getByText("v1.0")).toBeInTheDocument()
  })

  it("renders disconnected status without port", () => {
    render(<ConnectionCard name="Runtime" status="disconnected" />)
    expect(screen.getByText("localhost:-")).toBeInTheDocument()
  })
})

describe("ModeOption", () => {
  it("renders active mode option", () => {
    render(<RadioGroup value="standard"><ModeOption active={true} mode="standard" description="balanced" /></RadioGroup>)
    expect(screen.getByText("standard")).toBeInTheDocument()
    expect(screen.getByText("balanced")).toBeInTheDocument()
  })

  it("renders inactive mode option", () => {
    render(<RadioGroup value=""><ModeOption active={false} mode="observe" description="read only" /></RadioGroup>)
    expect(screen.getByText("observe")).toBeInTheDocument()
  })
})

describe("ProviderButton", () => {
  it("renders label and icon, calls onClick", () => {
    const Icon = () => <span data-testid="icon">*</span>
    const onClick = vi.fn()
    render(<ProviderButton label="Test" icon={Icon} busy={false} onClick={onClick} />)
    fireEvent.click(screen.getByRole("button", { name: /Test/i }))
    expect(onClick).toHaveBeenCalled()
  })

  it("is disabled when busy", () => {
    const Icon = () => <span data-testid="icon">*</span>
    render(<ProviderButton label="Save" icon={Icon} busy={true} disabled={false} onClick={() => {}} />)
    expect(screen.getByRole("button", { name: /Save/i })).toBeDisabled()
  })

  it("is disabled when disabled prop true", () => {
    const Icon = () => <span data-testid="icon">*</span>
    render(<ProviderButton label="Save" icon={Icon} busy={false} disabled={true} onClick={() => {}} />)
    expect(screen.getByRole("button", { name: /Save/i })).toBeDisabled()
  })
})

describe("ProviderActionNotice", () => {
  it("renders success message in success color", () => {
    render(<ProviderActionNotice message={{ ok: true, text: "Done" }} />)
    expect(screen.getByText("Done")).toBeInTheDocument()
  })

  it("renders error message in error color", () => {
    render(<ProviderActionNotice message={{ ok: false, text: "Failed" }} />)
    expect(screen.getByText("Failed")).toBeInTheDocument()
  })
})

describe("RiskToggle", () => {
  it("renders label and switch", () => {
    const onChange = vi.fn()
    render(<RiskToggle id="rt1" label="Confirm" checked={false} onChange={onChange} />)
    expect(screen.getByText("Confirm")).toBeInTheDocument()
  })
})

describe("DiagnosticResultRow", () => {
  it("renders ok row without hint", () => {
    const row: DiagnosticRow = { category: "Repo", status: "ok", message: "Accessible" }
    render(<DiagnosticResultRow row={row} />)
    expect(screen.getByText("Repo")).toBeInTheDocument()
    expect(screen.getByText("正常")).toBeInTheDocument()
    expect(screen.getByText("Accessible")).toBeInTheDocument()
  })

  it("renders warning row with hint", () => {
    const row: DiagnosticRow = { category: "Net", status: "warning", message: "Slow", hint: "Check DNS" }
    render(<DiagnosticResultRow row={row} />)
    expect(screen.getByText("提醒")).toBeInTheDocument()
    expect(screen.getByText("Check DNS")).toBeInTheDocument()
  })
})

describe("DiagnosticResults", () => {
  it("returns null for empty results", () => {
    const { container } = render(<DiagnosticResults results={[]} />)
    expect(container.firstChild).toBeNull()
  })

  it("renders multiple rows", () => {
    const results: DiagnosticRow[] = [
      { category: "A", status: "ok", message: "ok" },
      { category: "B", status: "error", message: "err" },
    ]
    render(<DiagnosticResults results={results} />)
    expect(screen.getByText("A")).toBeInTheDocument()
    expect(screen.getByText("B")).toBeInTheDocument()
  })
})

describe("PolicySwitch", () => {
  it("renders label", () => {
    render(<PolicySwitch label="Allow" checked={true} disabled={false} onChange={() => {}} />)
    expect(screen.getByText("Allow")).toBeInTheDocument()
  })
})

describe("MCPToolHeader", () => {
  it("renders tool name and badges", () => {
    const tool = { name: "read_file", description: "", server_id: "s1", allowed: true, requires_confirmation: false, risk_level: "low" as const }
    render(<MCPToolHeader tool={tool} />)
    expect(screen.getByTestId("badges")).toBeInTheDocument()
  })
})

describe("MCPToolList", () => {
  it("shows empty message when no tools", () => {
    render(<MCPToolList tools={[]} onReload={async () => {}} />)
    expect(screen.getByText("暂无可用工具")).toBeInTheDocument()
  })

  it("renders tool rows", () => {
    const tools = [
      { name: "t1", description: "d1", server_id: "s1", allowed: true, requires_confirmation: false, risk_level: "low" as const },
    ]
    render(<MCPToolList tools={tools} onReload={async () => {}} />)
    expect(screen.getByText("d1")).toBeInTheDocument()
    expect(screen.getByTestId("badges")).toBeInTheDocument()
  })
})

describe("ProviderStatusBar", () => {
  it("returns null when no credential_status", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "P1",
      status: "active",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
    }
    const { container } = render(<ProviderStatusBar provider={provider} />)
    expect(container.firstChild).toBeNull()
  })

  it("shows unconfigured credential", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "P1",
      status: "active",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
      credential_status: { has_credential: false },
    }
    render(<ProviderStatusBar provider={provider} />)
    expect(screen.getByText(/凭据:/)).toBeInTheDocument()
    expect(screen.getByText("未配置")).toBeInTheDocument()
  })

  it("shows saved credential and last test status", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "P1",
      status: "active",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
      credential_status: {
        has_credential: true,
        last_test_status: "success",
        apply_status: "applied",
        pending_reload: true,
        last_test_at: "2025-01-01T00:00:00Z",
      },
    }
    render(<ProviderStatusBar provider={provider} />)
    expect(screen.getByText(/凭据:/)).toBeInTheDocument()
    expect(screen.getByText(/测试:/)).toBeInTheDocument()
    expect(screen.getByText(/应用:/)).toBeInTheDocument()
    expect(screen.getByText(/状态:/)).toBeInTheDocument()
    expect(screen.getByText(/测试时间:/)).toBeInTheDocument()
  })

  it("shows failed test message", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "P1",
      status: "active",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
      credential_status: {
        has_credential: true,
        last_test_status: "failed",
        last_test_message: "bad key",
      },
    }
    render(<ProviderStatusBar provider={provider} />)
    expect(screen.getByText(/错误:/)).toBeInTheDocument()
    expect(screen.getByText("bad key")).toBeInTheDocument()
  })
})

describe("selectWorkspace", () => {
  const items: Workspace[] = [
    { workspace_id: "w1", name: "W1", path: "/w1" },
    { workspace_id: "w2", name: "W2", path: "/w2" },
  ]

  it("calls onSelect with matching workspace", () => {
    const onSelect = vi.fn()
    selectWorkspace("w2", items, onSelect)
    expect(onSelect).toHaveBeenCalledWith(items[1])
  })

  it("does nothing when value not found", () => {
    const onSelect = vi.fn()
    selectWorkspace("w3", items, onSelect)
    expect(onSelect).not.toHaveBeenCalled()
  })
})

describe("SettingsNavItem", () => {
  it("renders active item", () => {
    const refs = { current: {} } as React.MutableRefObject<Record<string, HTMLDivElement | null>>
    render(<SettingsNavItem active={true} module={{ id: "runtime", label: "运行环境", icon: () => <span>*</span> }} refs={refs} />)
    expect(screen.getByText("运行环境")).toBeInTheDocument()
  })

  it("renders inactive item", () => {
    const refs = { current: {} } as React.MutableRefObject<Record<string, HTMLDivElement | null>>
    render(<SettingsNavItem active={false} module={{ id: "model", label: "模型", icon: () => <span>*</span> }} refs={refs} />)
    expect(screen.getByText("模型")).toBeInTheDocument()
  })
})

describe("ProviderHeader", () => {
  it("renders provider name and active badge", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "OpenAI",
      status: "active",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
    }
    render(<ProviderHeader provider={provider} active={true} masked="sk-***" />)
    expect(screen.getByText("OpenAI")).toBeInTheDocument()
    expect(screen.getByText("sk-***")).toBeInTheDocument()
    expect(screen.getByText("当前")).toBeInTheDocument()
  })

  it("does not show badge when inactive", () => {
    const provider: Provider = {
      provider_id: "p1",
      display_name: "OpenAI",
      status: "error",
      base_url: "",
      chat_completions_path: "",
      models_path: "",
    }
    render(<ProviderHeader provider={provider} active={false} masked="none" />)
    expect(screen.queryByText("当前")).not.toBeInTheDocument()
  })
})

describe("DirectoryItem", () => {
  it("renders directory name and path", () => {
    const dir = { approval_id: "a1", workspace_id: "w1", name: "Proj", root_path: "/proj" }
    render(<DirectoryItem dir={dir} onRemove={async () => {}} />)
    expect(screen.getByText("Proj")).toBeInTheDocument()
    expect(screen.getByText("/proj")).toBeInTheDocument()
  })
})

describe("DirectoryList", () => {
  it("renders items and input fields", () => {
    const items = [{ approval_id: "a1", workspace_id: "w1", name: "Proj", root_path: "/proj" }]
    render(<DirectoryList items={items} onAdd={async () => {}} onRemove={async () => {}} />)
    expect(screen.getByText("Proj")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("名称（可选）")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("绝对路径")).toBeInTheDocument()
  })
})

describe("MCPServerCard", () => {
  it("renders server name and ready status", () => {
    const server = { id: "s1", name: "Local", url: "http://localhost:3001", ready: true, enabled: true, allowed_tool_count: 2 }
    render(<MCPServerCard server={server} tools={[]} onReload={async () => {}} />)
    expect(screen.getByText("Local")).toBeInTheDocument()
    expect(screen.getByText("已连接")).toBeInTheDocument()
  })

  it("renders not ready status", () => {
    const server = { id: "s1", name: "Remote", url: "http://remote", ready: false, enabled: true, allowed_tool_count: 0 }
    render(<MCPServerCard server={server} tools={[]} onReload={async () => {}} />)
    expect(screen.getByText("未就绪")).toBeInTheDocument()
  })
})

describe("MCPAddForm", () => {
  it("renders inputs and add button", () => {
    render(<MCPAddForm onReload={async () => {}} />)
    expect(screen.getByPlaceholderText("名称（如：本地MCP）")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("HTTP 地址")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "添加" })).toBeInTheDocument()
  })
})

describe("ModelSelect", () => {
  it("renders model options", () => {
    const models: Model[] = [
      { model_id: "m1", display_name: "Model 1", provider_id: "p1" },
      { model_id: "m2", display_name: "Model 2", provider_id: "p2" },
    ]
    render(<ModelSelect modelId="m1" models={models} onSelect={() => {}} />)
    expect(screen.getByRole("combobox")).toBeInTheDocument()
  })
})

describe("ModeSelect", () => {
  it("renders mode options", () => {
    render(<ModeSelect mode="standard" onMode={() => {}} />)
    expect(screen.getByText("standard")).toBeInTheDocument()
    expect(screen.getByText("observe")).toBeInTheDocument()
    expect(screen.getByText("full access")).toBeInTheDocument()
  })
})

describe("SettingsSection", () => {
  it("renders title and children", () => {
    render(<SettingsSection id="test" title="Test" description="Desc" icon={() => <span>*</span>}>Content</SettingsSection>)
    expect(screen.getByText("Test")).toBeInTheDocument()
    expect(screen.getByText("Content")).toBeInTheDocument()
  })
})

describe("SettingsNav", () => {
  it("renders nav items", () => {
    const refs = { current: {} } as React.MutableRefObject<Record<string, HTMLDivElement | null>>
    render(<SettingsNav activeSection="runtime" refs={refs} />)
    expect(screen.getByText("运行环境")).toBeInTheDocument()
    expect(screen.getByText("模型")).toBeInTheDocument()
  })
})

describe("CurrentRoutingInfo", () => {
  it("renders routing info", () => {
    render(<CurrentRoutingInfo />)
    expect(screen.getByText(/模型:/)).toBeInTheDocument()
    expect(screen.getByText(/激活服务商:/)).toBeInTheDocument()
  })
})

describe("MCPRemoveButton", () => {
  it("renders remove button", () => {
    const server = { id: "s1", name: "Test", url: "http://test", ready: true, enabled: true, allowed_tool_count: 0 }
    render(<MCPRemoveButton server={server} onReload={async () => {}} />)
    expect(screen.getByText("移除服务器")).toBeInTheDocument()
  })
})

describe("MCPRiskSelect", () => {
  it("renders select with value", () => {
    render(<MCPRiskSelect value="high" disabled={false} onChange={() => {}} />)
    expect(document.querySelector("[data-state='closed']")).toBeInTheDocument()
  })
})
