"use client"

import { ConfirmationCard } from "@/components/local-agent/views/task-view"
import type { Confirmation } from "@/lib/local-agent/types"

const previewConfirmation = buildConfirmation({
  patch_preview_report_json: JSON.stringify({
    dry_run: true,
    changes: [{ path: "src/runtime.ts", kind: "modify", before_chars: 12, after_chars: 18 }],
  }),
})

const fallbackConfirmation = buildConfirmation({
  patch_preview_report_json: "",
  tool_arguments_json: JSON.stringify({
    diff: [
      "diff --git a/src/old.ts b/src/new.ts",
      "rename from src/old.ts",
      "rename to src/new.ts",
      "--- a/src/app.ts",
      "+++ b/src/app.ts",
      "@@ -1 +1 @@",
      "-old",
      "+new",
    ].join("\n"),
    dry_run: false,
  }),
})

export default function ConfirmationPreviewAcceptancePage() {
  return (
    <main className="min-h-dvh bg-background p-6 text-foreground">
      <div className="mx-auto max-w-5xl space-y-6">
        <section className="space-y-2">
          <h1 className="text-xl font-semibold">Patch Confirmation Acceptance</h1>
          <p className="text-sm text-muted-foreground">用于验收 patch confirmation 的结构化预览、fallback 摘要和 apply 前确认文案。</p>
        </section>
        <PreviewBlock title="Structured Preview" confirmation={previewConfirmation} />
        <PreviewBlock title="Fallback Summary" confirmation={fallbackConfirmation} />
      </div>
    </main>
  )
}

function PreviewBlock({ title, confirmation }: { title: string; confirmation: Confirmation }) {
  return (
    <section className="space-y-3">
      <h2 className="text-sm font-medium text-muted-foreground">{title}</h2>
      <ConfirmationCard confirmation={confirmation} onDecision={() => undefined} />
    </section>
  )
}

function buildConfirmation(overrides: Partial<Confirmation>): Confirmation {
  return {
    confirmation_id: "debug-confirmation",
    run_id: "debug-run",
    risk_level: "high",
    action_summary: "应用 patch：2 个文件",
    reason: "确认后才会执行 apply。",
    target_paths: ["src/app.ts", "src/new.ts"],
    hazards: ["将修改工作区文件"],
    alternatives: [],
    tool_name: "workspace_apply_patch",
    tool_arguments_json: "",
    patch_preview_report_json: "",
    ...overrides,
  }
}
