import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent, waitFor } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { TaskView } from "../task-view"
import { useRuntimeStore, useUIStore } from "@/lib/local-agent/store"

class MockEventSource {
  onmessage: ((event: MessageEvent) => void) | null = null
  onerror: (() => void) | null = null
  onopen: (() => void) | null = null
  close = vi.fn()
  addEventListener = vi.fn()
  removeEventListener = vi.fn()
}

Object.defineProperty(global, "EventSource", { value: MockEventSource })

Element.prototype.scrollIntoView = vi.fn()

const mockStore = {
  messages: [],
  events: [],
  runState: "idle",
  connectionState: "disconnected",
  confirmation: null,
  composeValue: "",
  criticalError: null,
  submitError: null,
  addMessage: vi.fn(),
  setComposeValue: vi.fn(),
  setConfirmation: vi.fn(),
  startNewRun: vi.fn(),
  acceptRun: vi.fn(),
  applyEvent: vi.fn(),
  failRun: vi.fn(),
  cancelRun: vi.fn(),
  setRunState: vi.fn(),
  setConnectionState: vi.fn(),
  setCriticalError: vi.fn(),
  setSubmitError: vi.fn(),
  editAndResend: vi.fn(),
  clearSession: vi.fn(),
  resumeSession: vi.fn(),
}

vi.mock("@/lib/local-agent/store", () => ({
  useRuntimeStore: vi.fn(() => mockStore),
  useSettingsStore: vi.fn(() => ({ model: { model_id: "m1" } })),
  useUIStore: vi.fn(() => ({ searchQuery: "", setSearchQuery: vi.fn() })),
}))

describe("TaskView", () => {
  it("renders welcome screen when idle", () => {
    render(<TaskView />)
    expect(screen.getByText("需要我做什么？")).toBeInTheDocument()
  })

  it("renders confirmation card when confirming", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      runState: "awaiting_confirmation",
      confirmation: {
        confirmation_id: "c1",
        action_summary: "Delete",
        reason: "Cleanup",
        risk_level: "high",
        target_paths: [],
        hazards: [],
        tool_name: "bash",
        tool_arguments_json: "",
      },
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByText("Delete")).toBeInTheDocument()
  })

  it("renders error card when critical error present", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      runState: "failed",
      criticalError: "Something went wrong",
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByText("Something went wrong")).toBeInTheDocument()
  })

  it("renders running card when running", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      runState: "running",
      events: [{ stage: "thinking", payload: { content: "Working..." } }],
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByText("运行中...")).toBeInTheDocument()
  })

  it("renders messages and search bar when has messages", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      messages: [
        { id: "m1", role: "user", content: "Hello", timestamp: Date.now() },
        { id: "m2", role: "assistant", content: "Hi there", timestamp: Date.now() },
      ],
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByPlaceholderText("搜索消息...")).toBeInTheDocument()
    expect(screen.getByText("Hello")).toBeInTheDocument()
    expect(screen.getByText("Hi there")).toBeInTheDocument()
  })



  it("renders disconnected banner", () => {
    render(<TaskView />)
    expect(screen.getByText("事件流连接已断开")).toBeInTheDocument()
    expect(screen.getByText("重连")).toBeInTheDocument()
  })

  it("renders submit error", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      submitError: "Submit failed",
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByText("Submit failed")).toBeInTheDocument()
  })
})
