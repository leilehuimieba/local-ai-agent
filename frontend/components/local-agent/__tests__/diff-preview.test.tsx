import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { DiffPreview, inferDiffPreviewReport, parseDiffFailure, parseDiffPreview } from "../diff-preview"

describe("DiffPreview", () => {
  it("parses runtime dry-run text output", () => {
    const report = parseDiffPreview("patch dry-run 完成：\n- modify: src/app.ts (10 -> 12 chars)")
    expect(report?.changes[0]).toMatchObject({ kind: "modify", path: "src/app.ts", before_chars: 10, after_chars: 12 })
  })

  it("parses structured dry-run json output", () => {
    const report = parseDiffPreview('{"dry_run":true,"changes":[{"path":"a.ts","kind":"create","before_chars":0,"after_chars":4}]}')
    expect(report?.changes[0]).toMatchObject({ kind: "create", path: "a.ts" })
  })

  it("does not render apply results", () => {
    const { container } = render(<DiffPreview content="patch apply 完成：\n- modify: a.ts (1 -> 2 chars)" />)
    expect(container.textContent).toBe("")
  })

  it("renders change categories", () => {
    const content = "patch dry-run 完成：\n- delete: old.ts (12 -> 0 chars)\n- rename: new.ts (4 -> 4 chars)"
    expect(parseDiffPreview(content)?.changes).toHaveLength(2)
    render(<DiffPreview content={content} />)
    expect(screen.getByText("Patch 预览")).toBeInTheDocument()
    expect(screen.getByText("删除")).toBeInTheDocument()
    expect(screen.getByText("重命名")).toBeInTheDocument()
  })

  it("parses structured failure report", () => {
    const report = parseDiffFailure('{"success":false,"stage":"conflict","path":"src/app.ts","reason":"上下文不匹配","rollback_attempted":true}')
    expect(report).toMatchObject({ stage: "conflict", path: "src/app.ts", reason: "上下文不匹配", rollback_attempted: true })
  })

  it("renders failure report card", () => {
    render(<DiffPreview content='{"success":false,"stage":"write","path":null,"reason":"access denied","rollback_attempted":false}' />)
    expect(screen.getByText("Patch 失败报告")).toBeInTheDocument()
    expect(screen.getByText("未定位到具体文件")).toBeInTheDocument()
    expect(screen.getByText("未触发回滚")).toBeInTheDocument()
  })

  it("infers modify preview from unified diff arguments", () => {
    const report = inferDiffPreviewReport("--- a/src/app.ts\n+++ b/src/app.ts\n@@ -1 +1 @@\n-old\n+new")
    expect(report?.changes[0]).toMatchObject({ kind: "modify", path: "src/app.ts" })
  })

  it("infers create and rename preview from unified diff arguments", () => {
    const report = inferDiffPreviewReport([
      "diff --git a/old.ts b/new.ts",
      "rename from old.ts",
      "rename to new.ts",
      "--- /dev/null",
      "+++ b/created.ts",
      "@@ -0,0 +1 @@",
      "+hello",
    ].join("\n"))
    expect(report?.changes).toMatchObject([
      { kind: "rename", path: "new.ts" },
      { kind: "create", path: "created.ts" },
    ])
  })
})
