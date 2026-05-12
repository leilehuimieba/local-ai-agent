import { describe, it, expect, vi, beforeEach } from "vitest"
import { render, screen, fireEvent, waitFor } from "@testing-library/react"
import { TaskView } from "../task-view"
import { useRuntimeStore } from "@/lib/local-agent/store"

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

vi.mock("@/lib/local-agent/api", () => ({
  submitChatRun: vi.fn().mockResolvedValue({ session_id: "s1", run_id: "r1" }),
  submitChatRetry: vi.fn().mockResolvedValue({ session_id: "s1", run_id: "r1" }),
  submitChatCancel: vi.fn().mockResolvedValue(undefined),
  submitConfirmationDecision: vi.fn().mockResolvedValue(undefined),
  uploadKnowledgeFile: vi.fn().mockResolvedValue({}),
}))

const mockStore = {
  messages: [],
  events: [],
  runState: "idle",
  connectionState: "disconnected",
  confirmation: null,
  composeValue: "",
  criticalError: null,
  submitError: null,
  sessionId: "sess-default",
  currentRunId: null,
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
  useRuntimeStore: Object.assign(vi.fn(() => mockStore), {
    getState: vi.fn(() => mockStore),
  }),
  useSettingsStore: vi.fn(() => ({ model: { model_id: "m1" } })),
  useUIStore: vi.fn(() => ({ searchQuery: "", setSearchQuery: vi.fn() })),
}))

describe("TaskView", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("renders welcome screen when idle", () => {
    render(<TaskView />)
    expect(screen.getByText("需要我做什么？")).toBeInTheDocument()
  })

  it("sends message via quick prompt", () => {
    render(<TaskView />)
    fireEvent.click(screen.getByText("查询数据库"))
    expect(mockStore.addMessage).toHaveBeenCalled()
    expect(mockStore.startNewRun).toHaveBeenCalled()
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

  it("calls clearSession when clicking new task button", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      messages: [{ id: "m1", role: "user", content: "Hello", timestamp: Date.now() }],
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    fireEvent.click(screen.getByTitle("新任务"))
    expect(mockStore.clearSession).toHaveBeenCalled()
  })

  it("cancels running task", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      runState: "running",
      events: [{ stage: "thinking", payload: { content: "Working..." } }],
      currentRunId: "r1",
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    fireEvent.click(screen.getByText("取消"))
    expect(mockStore.cancelRun).toHaveBeenCalled()
  })

  it("clicks edit on user message", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      messages: [{ id: "m1", role: "user", content: "Hello", timestamp: new Date().toISOString() }],
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    fireEvent.click(screen.getByText("编辑"))
    expect(mockStore.editAndResend).toHaveBeenCalledWith("m1")
  })

  it("triggers export when clicking export button", () => {
    const createObjectURL = vi.fn().mockReturnValue("blob:url")
    const revokeObjectURL = vi.fn()
    const originalCreateObjectURL = URL.createObjectURL
    const originalRevokeObjectURL = URL.revokeObjectURL
    URL.createObjectURL = createObjectURL
    URL.revokeObjectURL = revokeObjectURL

    const anchorClick = vi.fn()
    const originalCreateElement = document.createElement
    document.createElement = vi.fn((tagName: string) => {
      const el = originalCreateElement.call(document, tagName)
      if (tagName === "a") {
        el.click = anchorClick
      }
      return el
    }) as unknown as typeof document.createElement

    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      messages: [{ id: "m1", role: "user", content: "Hello", timestamp: new Date().toISOString() }],
      sessionId: "sess-123",
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    fireEvent.click(screen.getByTitle("导出会话"))
    expect(createObjectURL).toHaveBeenCalled()

    URL.createObjectURL = originalCreateObjectURL
    URL.revokeObjectURL = originalRevokeObjectURL
    document.createElement = originalCreateElement
  })

  it("renders file attach button", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      composeValue: "test",
      messages: [{ id: "m1", role: "user", content: "Hello", timestamp: new Date().toISOString() }],
    } as ReturnType<typeof useRuntimeStore>)
    render(<TaskView />)
    expect(screen.getByTitle("附加文件")).toBeInTheDocument()
  })

  it("sends message on Enter key", () => {
    vi.mocked(useRuntimeStore).mockReturnValueOnce({
      ...mockStore,
      composeValue: "hello",
    } as ReturnType<typeof useRuntimeStore>)
    const { container } = render(<TaskView />)
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement
    fireEvent.keyDown(textarea, { key: "Enter", shiftKey: false })
    expect(mockStore.addMessage).toHaveBeenCalled()
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
