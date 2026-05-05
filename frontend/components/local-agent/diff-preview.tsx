"use client"

import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"

type DiffKind = "create" | "modify" | "delete" | "rename"

export type DiffPreviewChange = {
  path: string
  kind: DiffKind
  before_chars: number
  after_chars: number
}

export type DiffPreviewReport = {
  dry_run: true
  changes: DiffPreviewChange[]
}

export type DiffFailureReport = {
  success: false
  stage: string
  path: string | null
  reason: string
  rollback_attempted: boolean
}

type DiffPreviewCardProps = {
  report: DiffPreviewReport
  title?: string
  subtitle?: string
  badgeLabel?: string
}

type PatchScan = {
  old_path: string | null
  new_path: string | null
  before_chars: number
  after_chars: number
  seen_hunk: boolean
}

const kindLabels: Record<DiffKind, string> = {
  create: "新增",
  modify: "修改",
  delete: "删除",
  rename: "重命名",
}

const kindClasses: Record<DiffKind, string> = {
  create: "border-success/30 bg-success/10 text-success",
  modify: "border-primary/30 bg-primary/10 text-primary",
  delete: "border-destructive/30 bg-destructive/10 text-destructive",
  rename: "border-warning/30 bg-warning/10 text-warning",
}

export function DiffPreview({ content }: { content: string }) {
  const report = parseDiffPreview(content)
  const failure = report ? null : parseDiffFailure(content)
  if (!report) return failure ? <DiffFailureCard report={failure} /> : null
  return <DiffPreviewReportCard report={report} />
}

export function DiffPreviewReportCard({
  report,
  title = "Patch 预览",
  subtitle = "dry-run 结果，尚未写入文件。",
  badgeLabel,
}: DiffPreviewCardProps) {
  return (
    <div className="mt-3 rounded-lg border border-border bg-muted/20 p-3">
      <DiffPreviewHeader count={report.changes.length} title={title} subtitle={subtitle} badgeLabel={badgeLabel} />
      <div className="mt-3 space-y-2">
        {report.changes.map((change) => <DiffPreviewRow key={`${change.kind}:${change.path}`} change={change} />)}
      </div>
    </div>
  )
}

function DiffFailureCard({ report }: { report: DiffFailureReport }) {
  return (
    <div className="mt-3 rounded-lg border border-destructive/30 bg-destructive/5 p-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div>
          <p className="text-sm font-medium text-destructive">Patch 失败报告</p>
          <p className="text-xs text-muted-foreground">Runtime 已拒绝应用此 patch。</p>
        </div>
        <Badge variant="outline" className="border-destructive/30 bg-destructive/10 text-xs text-destructive">{report.stage}</Badge>
      </div>
      <div className="mt-3 space-y-2">
        <FailureRow label="路径" value={report.path || "未定位到具体文件"} />
        <FailureRow label="原因" value={report.reason} />
        <FailureRow label="回滚" value={report.rollback_attempted ? "已尝试回滚已写入文件" : "未触发回滚"} />
      </div>
    </div>
  )
}

function DiffPreviewHeader({
  count,
  title,
  subtitle,
  badgeLabel,
}: {
  count: number
  title: string
  subtitle: string
  badgeLabel?: string
}) {
  return (
    <div className="flex flex-wrap items-center justify-between gap-2">
      <div>
        <p className="text-sm font-medium text-foreground">{title}</p>
        <p className="text-xs text-muted-foreground">{subtitle}</p>
      </div>
      <Badge variant="secondary" className="text-xs">{badgeLabel || `${count} 个文件`}</Badge>
    </div>
  )
}

function DiffPreviewRow({ change }: { change: DiffPreviewChange }) {
  return (
    <div className="rounded-md border border-border bg-card px-3 py-2">
      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="outline" className={cn("text-xs", kindClasses[change.kind])}>{kindLabels[change.kind]}</Badge>
        <code className="min-w-0 break-all text-xs text-foreground">{change.path}</code>
      </div>
      <p className="mt-1 text-xs text-muted-foreground">{change.before_chars} {"->"} {change.after_chars} chars</p>
    </div>
  )
}

export function parseDiffPreview(content: string): DiffPreviewReport | null {
  return parseDiffPreviewJSON(content) ?? parseDiffPreviewLines(content)
}

export function inferDiffPreviewReport(diff: string): DiffPreviewReport | null {
  const changes: DiffPreviewChange[] = []
  let scan = newPatchScan()
  for (const line of diff.split(/\r?\n/)) {
    if (line.startsWith("diff --git ")) {
      pushScannedChange(changes, scan)
      scan = scanFromGitHeader(line)
      continue
    }
    if (shouldStartScannedPatch(scan, line)) {
      pushScannedChange(changes, scan)
      scan = newPatchScan()
    }
    scanPatchLine(scan, line)
  }
  pushScannedChange(changes, scan)
  return changes.length ? { dry_run: true, changes } : null
}

export function parseDiffFailure(content: string): DiffFailureReport | null {
  const payload = parseJSONCandidate(content)
  if (!isRecord(payload) || payload.success !== false) return null
  if (typeof payload.stage !== "string" || typeof payload.reason !== "string") return null
  return {
    success: false,
    stage: payload.stage,
    path: typeof payload.path === "string" ? payload.path : null,
    reason: payload.reason,
    rollback_attempted: payload.rollback_attempted === true,
  }
}

function FailureRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-md border border-border bg-card px-3 py-2">
      <p className="text-xs font-medium text-muted-foreground">{label}</p>
      <p className="mt-1 break-words text-sm text-foreground">{value}</p>
    </div>
  )
}

function parseDiffPreviewJSON(content: string): DiffPreviewReport | null {
  const payload = parseJSONCandidate(content)
  if (!isRecord(payload) || payload.dry_run !== true || !Array.isArray(payload.changes)) return null
  const changes = payload.changes.map(readChange).filter((item): item is DiffPreviewChange => Boolean(item))
  return changes.length ? { dry_run: true, changes } : null
}

function parseJSONCandidate(content: string): unknown {
  const trimmed = content.trim()
  const start = trimmed.indexOf("{")
  const end = trimmed.lastIndexOf("}")
  if (start < 0 || end <= start) return null
  try { return JSON.parse(trimmed.slice(start, end + 1)) } catch { return null }
}

function parseDiffPreviewLines(content: string): DiffPreviewReport | null {
  if (!content.includes("patch dry-run 完成")) return null
  const changes = content.split(/\r?\n/).map(parseChangeLine).filter((item): item is DiffPreviewChange => Boolean(item))
  return changes.length ? { dry_run: true, changes } : null
}

function parseChangeLine(line: string): DiffPreviewChange | null {
  const match = line.match(/^-\s+(create|modify|delete|rename):\s+(.+)\s+\((\d+)\s+->\s+(\d+)\s+chars\)$/)
  if (!match) return null
  return { kind: match[1] as DiffKind, path: match[2], before_chars: Number(match[3]), after_chars: Number(match[4]) }
}

function newPatchScan(): PatchScan {
  return { old_path: null, new_path: null, before_chars: 0, after_chars: 0, seen_hunk: false }
}

function scanFromGitHeader(line: string): PatchScan {
  const match = line.match(/^diff --git a\/(.+?) b\/(.+)$/)
  return {
    old_path: normalizePatchPath(match?.[1]),
    new_path: normalizePatchPath(match?.[2]),
    before_chars: 0,
    after_chars: 0,
    seen_hunk: false,
  }
}

function scanPatchLine(scan: PatchScan, line: string) {
  if (line.startsWith("--- ")) return void (scan.old_path = readHeaderPath(line, "--- "))
  if (line.startsWith("+++ ")) return void (scan.new_path = readHeaderPath(line, "+++ "))
  if (line.startsWith("rename from ")) return void (scan.old_path = normalizePatchPath(line.slice(12).trim()))
  if (line.startsWith("rename to ")) return void (scan.new_path = normalizePatchPath(line.slice(10).trim()))
  if (line.startsWith("@@")) return void (scan.seen_hunk = true)
  if (!scan.seen_hunk || !line || line.startsWith("\\")) return
  if (line.startsWith(" ") || line.startsWith("-")) scan.before_chars += line.slice(1).length
  if (line.startsWith(" ") || line.startsWith("+")) scan.after_chars += line.slice(1).length
}

function shouldStartScannedPatch(scan: PatchScan, line: string): boolean {
  if (!line.startsWith("--- ")) return false
  if (!scan.old_path || !scan.new_path) return scan.seen_hunk
  return readHeaderPath(line, "--- ") !== scan.old_path
}

function pushScannedChange(changes: DiffPreviewChange[], scan: PatchScan) {
  const change = buildScannedChange(scan)
  if (change) changes.push(change)
}

function buildScannedChange(scan: PatchScan): DiffPreviewChange | null {
  const kind = kindFromPaths(scan.old_path, scan.new_path)
  const path = scan.new_path || scan.old_path
  if (!kind || !path) return null
  return {
    path,
    kind,
    before_chars: scan.before_chars,
    after_chars: scan.after_chars,
  }
}

function kindFromPaths(oldPath: string | null, newPath: string | null): DiffKind | null {
  if (!oldPath && newPath) return "create"
  if (oldPath && !newPath) return "delete"
  if (oldPath && newPath && oldPath !== newPath) return "rename"
  if (oldPath && newPath) return "modify"
  return null
}

function readHeaderPath(line: string, prefix: string): string | null {
  const path = line.slice(prefix.length).trim().split(/\s+/)[0]
  return normalizePatchPath(path)
}

function normalizePatchPath(path?: string): string | null {
  if (!path || path === "/dev/null") return null
  if (path.startsWith("a/") || path.startsWith("b/")) return path.slice(2)
  return path
}

function readChange(value: unknown): DiffPreviewChange | null {
  if (!isRecord(value) || !isDiffKind(value.kind) || typeof value.path !== "string") return null
  if (typeof value.before_chars !== "number" || typeof value.after_chars !== "number") return null
  return { path: value.path, kind: value.kind, before_chars: value.before_chars, after_chars: value.after_chars }
}

function isDiffKind(value: unknown): value is DiffKind {
  return value === "create" || value === "modify" || value === "delete" || value === "rename"
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value && typeof value === "object" && !Array.isArray(value))
}
