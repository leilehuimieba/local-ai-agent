import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent } from "@testing-library/react"
import {
  formatMessageTime,
  deriveStage,
  isPatchConfirmation,
  parsePatchToolArguments,
  isRecord,
  ErrorCard,
  RunningCard,
  HighlightedText,
  PatchApplyHint,
  ResultBlockRenderer,
  ConfirmationHeader,
  ConfirmationSummary,
  ConfirmationPathList,
  ConfirmationHazards,
  PatchConfirmationPreview,
  PatchArgumentsPreview,
  ConfirmationActions,
  ConfirmationCard,
  MessageBubble,
} from "../task-view"
import type { RuntimeEvent, ResultBlock, Confirmation } from "@/lib/local-agent/types"

Object.defineProperty(global, "navigator", {
  value: {
    clipboard: { writeText: vi.fn() },
  },
  writable: true,
})

describe("formatMessageTime", () => {
  it("returns 刚刚 for < 1 min", () => {
    expect(formatMessageTime(new Date().toISOString())).toBe("刚刚")
  })

  it("returns minutes ago", () => {
    const d = new Date(Date.now() - 5 * 60 * 1000)
    expect(formatMessageTime(d.toISOString())).toBe("5分钟前")
  })

  it("returns hours ago", () => {
    const d = new Date(Date.now() - 3 * 60 * 60 * 1000)
    expect(formatMessageTime(d.toISOString())).toBe("3小时前")
  })

  it("returns 昨天", () => {
    const d = new Date(Date.now() - 24 * 60 * 60 * 1000)
    expect(formatMessageTime(d.toISOString())).toBe("昨天")
  })

  it("returns formatted date for older", () => {
    const d = new Date("2023-01-01T12:00:00Z")
    expect(formatMessageTime(d.toISOString())).toBe(d.toLocaleDateString("zh-CN", { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" }))
  })
})

describe("deriveStage", () => {
  it("returns 运行中 for empty events", () => {
    expect(deriveStage([])).toBe("运行中...")
  })

  it("maps known event types", () => {
    const e = (type: string): RuntimeEvent => ({ event_id: "e1", event_type: type, stage: "done", summary: "S", timestamp: "" })
    expect(deriveStage([e("run_started")])).toBe("已收到")
    expect(deriveStage([e("action_completed")])).toBe("正在执行")
    expect(deriveStage([e("verification_completed")])).toBe("正在验证")
    expect(deriveStage([e("memory_written")])).toBe("正在写入记忆")
    expect(deriveStage([e("knowledge_written")])).toBe("正在写入知识库")
    expect(deriveStage([e("checkpoint_written")])).toBe("正在保存检查点")
    expect(deriveStage([e("run_finished")])).toBe("已完成")
    expect(deriveStage([e("run_failed")])).toBe("运行失败")
  })

  it("falls back to summary for unknown event type", () => {
    const e: RuntimeEvent = { event_id: "e1", event_type: "unknown", stage: "done", summary: "Custom summary", timestamp: "" }
    expect(deriveStage([e])).toBe("Custom summary")
  })

  it("falls back to 运行中 when no summary", () => {
    const e: RuntimeEvent = { event_id: "e1", event_type: "unknown", stage: "done", summary: "", timestamp: "" }
    expect(deriveStage([e])).toBe("运行中...")
  })
})

describe("isPatchConfirmation", () => {
  it("returns true for workspace_apply_patch", () => {
    expect(isPatchConfirmation({ tool_name: "workspace_apply_patch" } as Confirmation)).toBe(true)
  })

  it("returns false for other tools", () => {
    expect(isPatchConfirmation({ tool_name: "workspace_write" } as Confirmation)).toBe(false)
  })
})

describe("parsePatchToolArguments", () => {
  it("returns null for undefined", () => {
    expect(parsePatchToolArguments(undefined)).toBeNull()
  })

  it("returns null for empty string", () => {
    expect(parsePatchToolArguments("")).toBeNull()
  })

  it("returns null for invalid JSON", () => {
    expect(parsePatchToolArguments("not json")).toBeNull()
  })

  it("returns null for non-object", () => {
    expect(parsePatchToolArguments("[]")).toBeNull()
  })

  it("returns null when diff is missing", () => {
    expect(parsePatchToolArguments(JSON.stringify({}))).toBeNull()
  })

  it("returns null when diff is empty", () => {
    expect(parsePatchToolArguments(JSON.stringify({ diff: "" }))).toBeNull()
  })

  it("parses valid arguments with dry_run false", () => {
    expect(parsePatchToolArguments(JSON.stringify({ diff: "--- a\n+++ b", dry_run: false }))).toEqual({ diff: "--- a\n+++ b", dry_run: false })
  })

  it("parses valid arguments with dry_run true", () => {
    expect(parsePatchToolArguments(JSON.stringify({ diff: "--- a\n+++ b", dry_run: true }))).toEqual({ diff: "--- a\n+++ b", dry_run: true })
  })
})

describe("isRecord", () => {
  it("returns false for null", () => {
    expect(isRecord(null)).toBe(false)
  })

  it("returns false for array", () => {
    expect(isRecord([])).toBe(false)
  })

  it("returns false for string", () => {
    expect(isRecord("str")).toBe(false)
  })

  it("returns true for plain object", () => {
    expect(isRecord({})).toBe(true)
  })
})

describe("ErrorCard", () => {
  it("renders error message and retry button", () => {
    const onRetry = vi.fn()
    render(<ErrorCard error="Something went wrong" onRetry={onRetry} />)
    expect(screen.getByText("错误")).toBeInTheDocument()
    expect(screen.getByText("Something went wrong")).toBeInTheDocument()
    fireEvent.click(screen.getByText("重试"))
    expect(onRetry).toHaveBeenCalled()
  })
})

describe("RunningCard", () => {
  it("renders stage and recent events", () => {
    const events: RuntimeEvent[] = [
      { event_id: "e1", event_type: "run_started", stage: "progress", summary: "Started", timestamp: "" },
      { event_id: "e2", event_type: "action_completed", stage: "done", summary: "Action done", timestamp: "" },
    ]
    render(<RunningCard events={events} />)
    expect(screen.getByText("正在执行")).toBeInTheDocument()
    expect(screen.getByText("Started")).toBeInTheDocument()
    expect(screen.getByText("Action done")).toBeInTheDocument()
  })
})

describe("HighlightedText", () => {
  it("renders plain text without highlight", () => {
    render(<HighlightedText content="Hello world" />)
    expect(screen.getByText("Hello world")).toBeInTheDocument()
  })

  it("highlights matching text", () => {
    render(<HighlightedText content="Hello world" highlight="world" />)
    expect(screen.getByText("world")).toBeInTheDocument()
    expect(screen.getByText((content) => content.includes("Hello"))).toBeInTheDocument()
  })

  it("renders plain text when no match", () => {
    render(<HighlightedText content="Hello world" highlight="xyz" />)
    expect(screen.getByText("Hello world")).toBeInTheDocument()
  })
})

describe("PatchApplyHint", () => {
  it("returns null when patchMode is false", () => {
    const { container } = render(<PatchApplyHint patchMode={false} />)
    expect(container.firstChild).toBeNull()
  })

  it("renders hint when patchMode is true", () => {
    render(<PatchApplyHint patchMode={true} />)
    expect(screen.getByText(/确认应用后才会写入文件/)).toBeInTheDocument()
  })
})

describe("ResultBlockRenderer", () => {
  it("renders text block", () => {
    const block: ResultBlock = { type: "text", content: "## Title\nSome text" }
    render(<ResultBlockRenderer block={block} />)
    expect(screen.getByText("Title")).toBeInTheDocument()
  })

  it("renders code block with language", () => {
    const block: ResultBlock = { type: "code", language: "ts", content: "const x = 1" }
    render(<ResultBlockRenderer block={block} />)
    expect(screen.getByText("ts")).toBeInTheDocument()
    expect(screen.getByText("const x = 1")).toBeInTheDocument()
  })

  it("copies code block content on copy button click", () => {
    const block: ResultBlock = { type: "code", language: "ts", content: "const x = 1" }
    render(<ResultBlockRenderer block={block} />)
    const copyBtn = screen.getByRole("button")
    fireEvent.click(copyBtn)
    expect(global.navigator.clipboard.writeText).toHaveBeenCalledWith("const x = 1")
  })

  it("renders list block", () => {
    const block: ResultBlock = { type: "list", items: ["Item 1", "Item 2"] }
    render(<ResultBlockRenderer block={block} />)
    expect(screen.getByText("Item 1")).toBeInTheDocument()
    expect(screen.getByText("Item 2")).toBeInTheDocument()
  })

  it("renders data_grid block", () => {
    const block: ResultBlock = { type: "data_grid", headers: ["A", "B"], rows: [["1", "2"]] }
    render(<ResultBlockRenderer block={block} />)
    expect(screen.getByText("A")).toBeInTheDocument()
    expect(screen.getByText("1")).toBeInTheDocument()
  })

  it("returns null for unknown block type", () => {
    const block = { type: "unknown" } as unknown as ResultBlock
    const { container } = render(<ResultBlockRenderer block={block} />)
    expect(container.firstChild).toBeNull()
  })
})

describe("ConfirmationHeader", () => {
  it("renders header with risk level", () => {
    render(<ConfirmationHeader riskLevel="high" />)
    expect(screen.getByText("需要确认")).toBeInTheDocument()
    expect(screen.getByText("高风险")).toBeInTheDocument()
  })

  it("renders unknown risk level as-is", () => {
    render(<ConfirmationHeader riskLevel="unknown" />)
    expect(screen.getByText("unknown")).toBeInTheDocument()
  })
})

describe("ConfirmationPathList", () => {
  it("returns null for empty paths", () => {
    const { container } = render(<ConfirmationPathList paths={[]} />)
    expect(container.firstChild).toBeNull()
  })

  it("renders path tags", () => {
    render(<ConfirmationPathList paths={["/a", "/b"]} />)
    expect(screen.getByText("/a")).toBeInTheDocument()
    expect(screen.getByText("/b")).toBeInTheDocument()
  })
})

describe("ConfirmationHazards", () => {
  it("returns null for empty hazards", () => {
    const { container } = render(<ConfirmationHazards hazards={[]} />)
    expect(container.firstChild).toBeNull()
  })

  it("renders hazard items", () => {
    render(<ConfirmationHazards hazards={["数据丢失", "系统崩溃"]} />)
    expect(screen.getByText("数据丢失")).toBeInTheDocument()
    expect(screen.getByText("系统崩溃")).toBeInTheDocument()
  })
})

describe("ConfirmationSummary", () => {
  it("renders summary with paths and hazards", () => {
    const confirmation: Confirmation = {
      confirmation_id: "c1",
      action_summary: "Delete files",
      reason: "Cleanup",
      risk_level: "high",
      target_paths: ["/tmp"],
      hazards: ["irreversible"],
      tool_name: "rm",
      tool_arguments_json: "",
    }
    render(<ConfirmationSummary confirmation={confirmation} />)
    expect(screen.getByText("Delete files")).toBeInTheDocument()
    expect(screen.getByText("Cleanup")).toBeInTheDocument()
    expect(screen.getByText("/tmp")).toBeInTheDocument()
    expect(screen.getByText("irreversible")).toBeInTheDocument()
  })
})

describe("PatchArgumentsPreview", () => {
  it("renders diff preview when infer succeeds", () => {
    render(<PatchArgumentsPreview diff="--- a\n+++ b\n@@ -1 +1 @@\n-old\n+new" dry_run={true} />)
    expect(screen.getByText("Patch 参数预览")).toBeInTheDocument()
  })

  it("renders raw diff when infer fails", () => {
    render(<PatchArgumentsPreview diff="some plain text" dry_run={false} />)
    expect(screen.getByText("Patch 参数预览")).toBeInTheDocument()
    expect(screen.getByText("some plain text")).toBeInTheDocument()
  })
})

describe("PatchConfirmationPreview", () => {
  it("returns null for non-patch confirmation", () => {
    const confirmation: Confirmation = {
      confirmation_id: "c1",
      action_summary: "Run command",
      reason: "Test",
      risk_level: "low",
      target_paths: [],
      hazards: [],
      tool_name: "bash",
      tool_arguments_json: "",
    }
    const { container } = render(<PatchConfirmationPreview confirmation={confirmation} />)
    expect(container.firstChild).toBeNull()
  })

  it("renders patch preview for patch confirmation", () => {
    const confirmation: Confirmation = {
      confirmation_id: "c1",
      action_summary: "Apply patch",
      reason: "Fix bug",
      risk_level: "medium",
      target_paths: [],
      hazards: [],
      tool_name: "workspace_apply_patch",
      tool_arguments_json: JSON.stringify({ diff: "--- a\n+++ b\n@@ -1 +1 @@\n-old\n+new", dry_run: true }),
    }
    render(<PatchConfirmationPreview confirmation={confirmation} />)
    expect(screen.getByText("Patch 参数预览")).toBeInTheDocument()
  })
})

describe("MessageBubble", () => {
  it("renders user message", () => {
    const message = { id: "m1", role: "user" as const, content: "hello", timestamp: new Date().toISOString() }
    render(<MessageBubble message={message} />)
    expect(screen.getByText("hello")).toBeInTheDocument()
  })

  it("renders assistant message with copy button", () => {
    const message = { id: "m2", role: "assistant" as const, content: "world", timestamp: new Date().toISOString() }
    render(<MessageBubble message={message} />)
    expect(screen.getByText("world")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: /复制/i })).toBeInTheDocument()
  })

  it("copies message content on copy button click", () => {
    const message = { id: "m2", role: "assistant" as const, content: "world", timestamp: new Date().toISOString() }
    render(<MessageBubble message={message} />)
    fireEvent.click(screen.getByRole("button", { name: /复制/i }))
    expect(global.navigator.clipboard.writeText).toHaveBeenCalledWith("world")
  })

  it("renders assistant message with blocks", () => {
    const message = {
      id: "m3",
      role: "assistant" as const,
      content: "result",
      timestamp: new Date().toISOString(),
      blocks: [{ type: "text" as const, content: "block text" }],
    }
    render(<MessageBubble message={message} />)
    expect(screen.getByText("block text")).toBeInTheDocument()
  })
})

describe("ConfirmationCard", () => {
  it("renders full confirmation and calls onDecision on approve", () => {
    const onDecision = vi.fn()
    const confirmation: Confirmation = {
      confirmation_id: "c1",
      action_summary: "Delete",
      reason: "Cleanup",
      risk_level: "high",
      target_paths: ["/tmp"],
      hazards: ["data loss"],
      tool_name: "bash",
      tool_arguments_json: "",
    }
    render(<ConfirmationCard confirmation={confirmation} onDecision={onDecision} />)
    expect(screen.getByText("Delete")).toBeInTheDocument()
    fireEvent.click(screen.getByRole("button", { name: /批准/i }))
    expect(onDecision).toHaveBeenCalledWith("approve", false)
  })

  it("calls onDecision on deny", () => {
    const onDecision = vi.fn()
    const confirmation: Confirmation = {
      confirmation_id: "c2",
      action_summary: "Write",
      reason: "Update",
      risk_level: "medium",
      target_paths: [],
      hazards: [],
      tool_name: "workspace_apply_patch",
      tool_arguments_json: JSON.stringify({ diff: "--- a\n+++ b\n@@ -1 +1 @@\n-old\n+new", dry_run: false }),
    }
    render(<ConfirmationCard confirmation={confirmation} onDecision={onDecision} />)
    fireEvent.click(screen.getByRole("button", { name: /取消应用/i }))
    expect(onDecision).toHaveBeenCalledWith("deny", false)
  })

  it("toggles remember checkbox", () => {
    const onDecision = vi.fn()
    const confirmation: Confirmation = {
      confirmation_id: "c3",
      action_summary: "Test",
      reason: "test",
      risk_level: "low",
      target_paths: [],
      hazards: [],
      tool_name: "bash",
      tool_arguments_json: "",
    }
    render(<ConfirmationCard confirmation={confirmation} onDecision={onDecision} />)
    const checkbox = screen.getByRole("checkbox")
    fireEvent.click(checkbox)
    expect(checkbox).toBeChecked()
  })
})

describe("ConfirmationActions", () => {
  it("renders approve and deny buttons, calls handlers", () => {
    const onApprove = vi.fn()
    const onDeny = vi.fn()
    const onRememberChange = vi.fn()
    render(
      <ConfirmationActions
        remember={false}
        patchMode={false}
        onApprove={onApprove}
        onDeny={onDeny}
        onRememberChange={onRememberChange}
      />
    )
    fireEvent.click(screen.getByRole("button", { name: /批准/i }))
    expect(onApprove).toHaveBeenCalled()
    fireEvent.click(screen.getByRole("button", { name: /拒绝/i }))
    expect(onDeny).toHaveBeenCalled()
  })

  it("renders patch labels when patchMode true", () => {
    render(
      <ConfirmationActions
        remember={false}
        patchMode={true}
        onApprove={() => {}}
        onDeny={() => {}}
        onRememberChange={() => {}}
      />
    )
    expect(screen.getByRole("button", { name: /确认应用/i })).toBeInTheDocument()
    expect(screen.getByRole("button", { name: /取消应用/i })).toBeInTheDocument()
  })
})
