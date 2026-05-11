import { describe, it, expect, vi, beforeEach } from "vitest"
import { render, screen, fireEvent, waitFor } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { LogsView, LogCard, formatDuration, formatTimestamp } from "../logs-view"
import { useLogsStore } from "@/lib/local-agent/store"
import type { LogRun } from "@/lib/local-agent/types"
import type { LogEntry } from "@/lib/local-agent/api"

vi.mock("@/lib/local-agent/api", async () => {
  const actual = await vi.importActual<typeof import("@/lib/local-agent/api")>("@/lib/local-agent/api")
  return {
    ...actual,
    fetchLogs: vi.fn().mockResolvedValue({ items: [], runs: [] }),
    fetchSessions: vi.fn().mockResolvedValue({ items: [] }),
  }
})

const { fetchLogs } = await import("@/lib/local-agent/api")

function buildRun(overrides: Partial<LogRun> = {}): LogRun {
  return {
    run_id: "r1",
    session_id: "s1",
    title: "Test Task",
    status: "completed",
    started_at: new Date().toISOString(),
    duration_ms: 5000,
    event_count: 3,
    ...overrides,
  }
}

function buildLogEvent(overrides: Partial<LogEntry> = {}): LogEntry {
  return {
    log_id: "l1",
    session_id: "s1",
    run_id: "r1",
    timestamp: new Date().toISOString(),
    level: "info",
    category: "tool",
    source: "runtime",
    summary: "summary",
    ...overrides,
  }
}

describe("formatDuration", () => {
  it("returns 进行中 for 0ms", () => {
    expect(formatDuration(0)).toBe("进行中...")
  })

  it("returns seconds only", () => {
    expect(formatDuration(45000)).toBe("45s")
  })

  it("returns minutes and seconds", () => {
    expect(formatDuration(125000)).toBe("2m 5s")
  })
})

describe("formatTimestamp", () => {
  it("returns 刚刚 for < 1 min", () => {
    const now = new Date()
    expect(formatTimestamp(now.toISOString())).toBe("刚刚")
  })

  it("returns minutes ago", () => {
    const d = new Date(Date.now() - 5 * 60 * 1000)
    expect(formatTimestamp(d.toISOString())).toBe("5分钟前")
  })

  it("returns hours ago", () => {
    const d = new Date(Date.now() - 3 * 60 * 60 * 1000)
    expect(formatTimestamp(d.toISOString())).toBe("3小时前")
  })

  it("returns 昨天", () => {
    const d = new Date(Date.now() - 24 * 60 * 60 * 1000)
    expect(formatTimestamp(d.toISOString())).toBe("昨天")
  })

  it("returns date string for older", () => {
    const d = new Date("2023-01-01T00:00:00Z")
    expect(formatTimestamp(d.toISOString())).toBe(d.toLocaleDateString())
  })
})

describe("LogsView", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    useLogsStore.setState({
      runs: [],
      selectedRun: null,
      statusFilter: "all",
      timeFilter: "7days",
      searchQuery: "",
    })
  })

  it("renders empty runs state", async () => {
    render(<LogsView />)
    await waitFor(() => {
      expect(screen.getByText("暂无运行记录")).toBeInTheDocument()
    })
  })

  it("renders run list and filters by status", async () => {
    const runs = [
      buildRun({ run_id: "r1", status: "completed", title: "Task A" }),
      buildRun({ run_id: "r2", status: "failed", title: "Task B" }),
    ]
    useLogsStore.setState({ runs })
    render(<LogsView />)
    await waitFor(() => {
      expect(screen.getByText("Task A")).toBeInTheDocument()
      expect(screen.getByText("Task B")).toBeInTheDocument()
    })

    fireEvent.click(screen.getByText("成功"))
    await waitFor(() => {
      expect(screen.queryByText("Task B")).not.toBeInTheDocument()
    })
  })

  it("filters runs by search query", async () => {
    const runs = [buildRun({ run_id: "r1", title: "Alpha" }), buildRun({ run_id: "r2", title: "Beta" })]
    useLogsStore.setState({ runs, searchQuery: "alp" })
    render(<LogsView />)
    await waitFor(() => {
      expect(screen.getByText("Alpha")).toBeInTheDocument()
      expect(screen.queryByText("Beta")).not.toBeInTheDocument()
    })
  })

  it("filters runs by time range", async () => {
    const oldRun = buildRun({ run_id: "r1", title: "Old", started_at: "2020-01-01T00:00:00Z" })
    const newRun = buildRun({ run_id: "r2", title: "New", started_at: new Date().toISOString() })
    useLogsStore.setState({ runs: [oldRun, newRun], timeFilter: "today" })
    render(<LogsView />)
    await waitFor(() => {
      expect(screen.queryByText("Old")).not.toBeInTheDocument()
      expect(screen.getByText("New")).toBeInTheDocument()
    })
  })
})

describe("LogCard", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(fetchLogs).mockResolvedValue({ items: [], runs: [] })
  })

  it("renders collapsed log card", () => {
    render(<LogCard log={buildRun({ title: "Collapsed" })} expanded={false} onToggle={vi.fn()} />)
    expect(screen.getByText("Collapsed")).toBeInTheDocument()
    expect(screen.getByText("5s")).toBeInTheDocument()
  })

  it("calls onToggle when clicked", () => {
    const onToggle = vi.fn()
    render(<LogCard log={buildRun()} expanded={false} onToggle={onToggle} />)
    fireEvent.click(screen.getByText("Test Task"))
    expect(onToggle).toHaveBeenCalled()
  })

  it("shows loading state when expanded without details", async () => {
    vi.mocked(fetchLogs).mockImplementation(() => new Promise(() => {}))
    render(<LogCard log={buildRun()} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByRole("status")).toBeInTheDocument()
    })
  })

  it("shows summary tab with final answer", async () => {
    vi.mocked(fetchLogs).mockResolvedValue({
      items: [buildLogEvent({ final_answer: "The answer is 42" })],
      runs: [],
    })
    render(<LogCard log={buildRun()} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("The answer is 42")).toBeInTheDocument()
    })
  })

  it("shows summary fallback to title when no final answer", async () => {
    vi.mocked(fetchLogs).mockResolvedValue({ items: [], runs: [] })
    render(<LogCard log={buildRun({ title: "Fallback Title" })} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("Fallback Title")).toBeInTheDocument()
    })
  })

  it("renders running status icon", () => {
    render(<LogCard log={buildRun({ status: "running", duration_ms: 0 })} expanded={false} onToggle={vi.fn()} />)
    expect(screen.getByText("进行中...")).toBeInTheDocument()
  })

  it("renders failed status", () => {
    render(<LogCard log={buildRun({ status: "failed" })} expanded={false} onToggle={vi.fn()} />)
    expect(screen.getByText("Test Task")).toBeInTheDocument()
  })

  it("shows tools, validation, risks, metadata tabs", async () => {
    const user = userEvent.setup()
    vi.mocked(fetchLogs).mockResolvedValue({
      items: [
        buildLogEvent({ tool_name: "bash", tool_display_name: "Bash", risk_level: "medium", summary: "Risky", result_summary: "OK", metadata: { key: "val" } }),
      ],
      runs: [],
    })
    render(<LogCard log={buildRun()} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("摘要")).toBeInTheDocument()
    })
    await user.click(screen.getByText("工具调用"))
    await waitFor(() => {
      expect(screen.getByText("Bash")).toBeInTheDocument()
    })
    await user.click(screen.getByText("验证"))
    await waitFor(() => {
      expect(screen.getByText("OK")).toBeInTheDocument()
    })
    await user.click(screen.getByText("风险"))
    await waitFor(() => {
      expect(screen.getByText("Risky")).toBeInTheDocument()
    })
    await user.click(screen.getByText("元数据"))
    await waitFor(() => {
      expect(screen.getByText("key")).toBeInTheDocument()
    })
  })

  it("switches to tools tab and shows empty state", async () => {
    const user = userEvent.setup()
    render(<LogCard log={buildRun()} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("摘要")).toBeInTheDocument()
    })
    await user.click(screen.getByText("工具调用"))
    await waitFor(() => {
      expect(screen.getByText("无工具调用记录")).toBeInTheDocument()
    })
  })

  it("shows continue chat button for non-running logs", async () => {
    render(<LogCard log={buildRun({ status: "completed" })} expanded={false} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("继续聊天")).toBeInTheDocument()
    })
  })

  it("handles fetchLogs error gracefully", async () => {
    vi.mocked(fetchLogs).mockRejectedValue(new Error("fail"))
    render(<LogCard log={buildRun({ title: "Error Run" })} expanded={true} onToggle={vi.fn()} />)
    await waitFor(() => {
      expect(screen.getByText("Error Run")).toBeInTheDocument()
    })
  })
})
