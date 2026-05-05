import { describe, expect, it, vi } from "vitest"
import { fireEvent, render, screen } from "@testing-library/react"
import { ConfirmationCard } from "../views/task-view"
import type { Confirmation } from "@/lib/local-agent/types"

function buildConfirmation(overrides: Partial<Confirmation> = {}): Confirmation {
  return {
    confirmation_id: "confirm-1",
    run_id: "run-1",
    risk_level: "high",
    action_summary: "应用 patch：1 个文件",
    reason: "需要确认后才写入文件。",
    target_paths: ["src/app.ts"],
    hazards: ["将修改工作区文件"],
    alternatives: [],
    tool_name: "workspace_apply_patch",
    tool_arguments_json: "",
    patch_preview_report_json: "",
    ...overrides,
  }
}

describe("ConfirmationCard", () => {
  it("优先渲染 patch_preview_report_json", () => {
    const confirmation = buildConfirmation({
      tool_arguments_json: JSON.stringify({ diff: "--- a/a.ts", dry_run: false }),
      patch_preview_report_json: JSON.stringify({
        dry_run: true,
        changes: [{ path: "src/app.ts", kind: "modify", before_chars: 10, after_chars: 12 }],
      }),
    })
    render(<ConfirmationCard confirmation={confirmation} onDecision={() => undefined} />)
    expect(screen.getByText("Patch 预览")).toBeInTheDocument()
    expect(screen.queryByText("Patch 参数预览")).not.toBeInTheDocument()
  })

  it("缺少 dry-run 报告时回退渲染 patch 参数", () => {
    const confirmation = buildConfirmation({
      tool_arguments_json: JSON.stringify({ diff: "--- a/a.ts\n+++ b/a.ts", dry_run: false }),
    })
    render(<ConfirmationCard confirmation={confirmation} onDecision={() => undefined} />)
    expect(screen.getByText("Patch 参数预览")).toBeInTheDocument()
    expect(screen.getByText("未收到 dry-run 报告，以下根据 patch 参数生成只读摘要。")).toBeInTheDocument()
    expect(screen.getByText("dry-run: 否")).toBeInTheDocument()
    expect(screen.getByText("a.ts")).toBeInTheDocument()
    expect(screen.getByText("修改")).toBeInTheDocument()
  })

  it("非 patch 确认不展示 diff 预览", () => {
    const confirmation = buildConfirmation({
      tool_name: "workspace_write",
      tool_arguments_json: JSON.stringify({ diff: "--- a/a.ts\n+++ b/a.ts", dry_run: false }),
    })
    render(<ConfirmationCard confirmation={confirmation} onDecision={() => undefined} />)
    expect(screen.queryByText("Patch 预览")).not.toBeInTheDocument()
    expect(screen.queryByText("Patch 参数预览")).not.toBeInTheDocument()
    expect(screen.getByText("批准")).toBeInTheDocument()
    expect(screen.getByText("拒绝")).toBeInTheDocument()
  })

  it("patch 确认使用应用语义并传递决策", () => {
    const onDecision = vi.fn()
    render(<ConfirmationCard confirmation={buildConfirmation()} onDecision={onDecision} />)
    fireEvent.click(screen.getByText("确认应用"))
    fireEvent.click(screen.getByText("取消应用"))
    expect(screen.getByText("确认应用后才会写入文件，取消应用不会修改工作区。")).toBeInTheDocument()
    expect(onDecision).toHaveBeenNthCalledWith(1, "approve", false)
    expect(onDecision).toHaveBeenNthCalledWith(2, "deny", false)
  })
})
