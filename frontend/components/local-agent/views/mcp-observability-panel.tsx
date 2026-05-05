"use client"

import { useMemo, useState } from "react"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import type { MCPAuditRecord, MCPServerInfo, MCPTool } from "@/lib/local-agent/types"

type MCPToolViewFilter = "all" | "allowed" | "blocked" | "confirm" | "high-risk" | "audit-off"

const viewFilters: Array<{ value: MCPToolViewFilter; label: string }> = [
  { value: "all", label: "全部工具" },
  { value: "allowed", label: "仅已允许" },
  { value: "blocked", label: "待放行" },
  { value: "confirm", label: "需确认" },
  { value: "high-risk", label: "高风险" },
  { value: "audit-off", label: "未审计" },
]

const policyLabels: Record<string, string> = {
  config: "配置策略",
  default_deny: "默认拒绝",
}

const riskLabels: Record<string, string> = {
  low: "低风险",
  medium: "中风险",
  high: "高风险",
  critical: "关键风险",
}

export function MCPObservabilityPanel(props: { servers: MCPServerInfo[]; tools: MCPTool[]; audits?: MCPAuditRecord[]; auditError?: string; emptyMessage?: string }) {
  const [query, setQuery] = useState("")
  const [serverFilter, setServerFilter] = useState("all")
  const [viewFilter, setViewFilter] = useState<MCPToolViewFilter>("all")
  const stats = useMemo(() => summarizeMCP(props.servers, props.tools), [props.servers, props.tools])
  const filtered = useMemo(
    () => filterMCPTools(props.tools, query, serverFilter, viewFilter),
    [props.tools, query, serverFilter, viewFilter],
  )
  const filteredAudits = useMemo(
    () => filterMCPAudits(props.audits, serverFilter),
    [props.audits, serverFilter],
  )
  return (
    <div className="space-y-3">
      <MCPOverview stats={stats} />
      <div className="rounded-lg border border-border p-4 space-y-3">
        <MCPObservationHeader count={filtered.length} />
        <MCPObservationFilters
          query={query}
          serverFilter={serverFilter}
          viewFilter={viewFilter}
          servers={props.servers}
          onQueryChange={setQuery}
          onServerFilterChange={setServerFilter}
          onViewFilterChange={setViewFilter}
        />
        <MCPObservationList tools={filtered} servers={props.servers} emptyMessage={props.emptyMessage} />
        <MCPAuditSection audits={filteredAudits} auditError={props.auditError} servers={props.servers} serverFilter={serverFilter} />
      </div>
    </div>
  )
}

function summarizeMCP(servers: MCPServerInfo[], tools: MCPTool[]) {
  return {
    serverCount: servers.length,
    toolCount: tools.length,
    blockedCount: tools.filter((tool) => !tool.allowed).length,
    highRiskCount: tools.filter((tool) => tool.risk_level === "high" || tool.risk_level === "critical").length,
    confirmCount: tools.filter((tool) => tool.requires_confirmation).length,
    auditOffCount: tools.filter((tool) => !tool.audit_enabled).length,
  }
}

function filterMCPTools(tools: MCPTool[], query: string, serverFilter: string, viewFilter: MCPToolViewFilter) {
  return tools.filter((tool) => {
    const text = `${tool.name} ${tool.description} ${tool.server_id || ""}`.toLowerCase()
    if (query && !text.includes(query.toLowerCase())) return false
    if (serverFilter !== "all" && tool.server_id !== serverFilter) return false
    return matchesViewFilter(tool, viewFilter)
  })
}

function matchesViewFilter(tool: MCPTool, viewFilter: MCPToolViewFilter) {
  if (viewFilter === "allowed") return tool.allowed
  if (viewFilter === "blocked") return !tool.allowed
  if (viewFilter === "confirm") return tool.requires_confirmation
  if (viewFilter === "high-risk") return tool.risk_level === "high" || tool.risk_level === "critical"
  if (viewFilter === "audit-off") return !tool.audit_enabled
  return true
}

function filterMCPAudits(audits: MCPAuditRecord[] | undefined, serverFilter: string) {
  if (!audits) return audits
  if (serverFilter === "all") return audits
  return audits.filter((audit) => audit.server_id === serverFilter)
}

function MCPOverview({ stats }: { stats: ReturnType<typeof summarizeMCP> }) {
  const items = [
    { label: "服务器", value: stats.serverCount, hint: "已配置入口" },
    { label: "工具", value: stats.toolCount, hint: `${stats.confirmCount} 个需确认` },
    { label: "待放行", value: stats.blockedCount, hint: "默认拒绝或未配置" },
    { label: "高风险", value: stats.highRiskCount, hint: `${stats.auditOffCount} 个未审计` },
  ]
  return <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">{items.map((item) => <MCPMetricCard key={item.label} {...item} />)}</div>
}

function MCPMetricCard({ label, value, hint }: { label: string; value: number; hint: string }) {
  return (
    <Card>
      <CardContent className="space-y-1 pt-5">
        <p className="text-xs font-medium uppercase tracking-wider text-muted-foreground">{label}</p>
        <p className="text-2xl font-semibold text-foreground">{value}</p>
        <p className="text-xs text-muted-foreground">{hint}</p>
      </CardContent>
    </Card>
  )
}

function MCPObservationHeader({ count }: { count: number }) {
  return (
    <div className="space-y-1">
      <div className="flex items-center justify-between gap-2">
        <p className="text-sm font-medium text-foreground">运营视图</p>
        <Badge variant="outline" className="text-[10px]">{count} 个命中</Badge>
      </div>
      <p className="text-xs text-muted-foreground">先看风险、allowlist、确认和审计覆盖，再结合最近动作与错误聚合判断当前阻塞点。</p>
    </div>
  )
}

function MCPObservationFilters(props: {
  query: string
  serverFilter: string
  viewFilter: MCPToolViewFilter
  servers: MCPServerInfo[]
  onQueryChange: (value: string) => void
  onServerFilterChange: (value: string) => void
  onViewFilterChange: (value: MCPToolViewFilter) => void
}) {
  return (
    <div className="flex flex-col gap-2 lg:flex-row">
      <Input value={props.query} onChange={(e) => props.onQueryChange(e.target.value)} placeholder="搜索工具名、描述或 server id" className="lg:flex-1" />
      <MCPServerFilter value={props.serverFilter} servers={props.servers} onChange={props.onServerFilterChange} />
      <MCPViewFilter value={props.viewFilter} onChange={props.onViewFilterChange} />
    </div>
  )
}

function MCPServerFilter({ value, servers, onChange }: { value: string; servers: MCPServerInfo[]; onChange: (value: string) => void }) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="h-10 lg:w-44"><SelectValue placeholder="全部服务器" /></SelectTrigger>
      <SelectContent>
        <SelectItem value="all">全部服务器</SelectItem>
        {servers.map((server) => <SelectItem key={server.id} value={server.id}>{server.name}</SelectItem>)}
      </SelectContent>
    </Select>
  )
}

function MCPViewFilter({ value, onChange }: { value: MCPToolViewFilter; onChange: (value: MCPToolViewFilter) => void }) {
  return (
    <Select value={value} onValueChange={(next) => onChange(next as MCPToolViewFilter)}>
      <SelectTrigger className="h-10 lg:w-40"><SelectValue /></SelectTrigger>
      <SelectContent>{viewFilters.map((item) => <SelectItem key={item.value} value={item.value}>{item.label}</SelectItem>)}</SelectContent>
    </Select>
  )
}

function MCPObservationList({ tools, servers, emptyMessage }: { tools: MCPTool[]; servers: MCPServerInfo[]; emptyMessage?: string }) {
  if (!tools.length) return <p className="rounded-lg bg-muted/50 px-3 py-4 text-sm text-muted-foreground">{emptyMessage || "当前筛选条件下没有工具命中。"}</p>
  return <div className="space-y-2">{tools.map((tool) => <MCPObservationCard key={`${tool.server_id}:${tool.name}`} tool={tool} servers={servers} />)}</div>
}

function MCPObservationCard({ tool, servers }: { tool: MCPTool; servers: MCPServerInfo[] }) {
  const serverName = servers.find((server) => server.id === tool.server_id)?.name || tool.server_id || "未归属服务器"
  return (
    <div className="rounded-lg bg-muted/50 px-3 py-3 space-y-2">
      <div className="flex flex-wrap items-start justify-between gap-2">
        <div className="min-w-0"><p className="text-sm font-medium text-foreground">{tool.name}</p><p className="text-xs text-muted-foreground">{serverName}</p></div>
        <div className="flex flex-wrap items-center gap-1"><MCPToolBadges tool={tool} /><Badge variant="outline" className="text-[10px]">{mcpPolicyLabel(tool.policy_source)}</Badge></div>
      </div>
      <p className="text-xs text-muted-foreground">{tool.description || "该工具未提供额外描述。"}</p>
      <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span>确认: <span className="font-medium text-foreground">{tool.requires_confirmation ? "需要" : "直连"}</span></span>
        <span>风险: <span className="font-medium text-foreground">{mcpRiskLabel(tool.risk_level)}</span></span>
        <span>审计: <span className="font-medium text-foreground">{tool.audit_enabled ? "已启用" : "未启用"}</span></span>
      </div>
    </div>
  )
}

export function MCPToolBadges({ tool }: { tool: MCPTool }) {
  return <div className="flex items-center gap-1"><Badge variant={tool.allowed ? "default" : "secondary"} className="text-[10px]">{tool.allowed ? "已允许" : "已拦截"}</Badge><Badge variant="outline" className="text-[10px]">{mcpRiskLabel(tool.risk_level || "medium")}</Badge>{tool.audit_enabled && <Badge variant="outline" className="text-[10px]">审计</Badge>}</div>
}

function MCPAuditSection(props: { audits?: MCPAuditRecord[]; auditError?: string; servers: MCPServerInfo[]; serverFilter: string }) {
  const errorStats = useMemo(() => summarizeAuditErrors(props.audits || []), [props.audits])
  if (!props.audits && !props.auditError) return null
  const serverName = selectedServerName(props.serverFilter, props.servers)
  return (
    <div className="space-y-2 border-t border-border pt-3">
      <div className="space-y-1">
        <p className="text-sm font-medium text-foreground">最近动作</p>
        <p className="text-xs text-muted-foreground">基于只读审计记录展示最近的 MCP 调用结果，不直接暴露参数原文。</p>
      </div>
      <MCPAuditSummary stats={errorStats} serverName={serverName} total={props.audits?.length || 0} />
      {props.auditError ? <p className="rounded-lg bg-muted/50 px-3 py-3 text-sm text-muted-foreground">{props.auditError}</p> : <MCPAuditList audits={props.audits || []} servers={props.servers} />}
    </div>
  )
}

function summarizeAuditErrors(audits: MCPAuditRecord[]) {
  const counts = new Map<string, number>()
  for (const audit of audits) {
    if (!audit.error_code) continue
    counts.set(audit.error_code, (counts.get(audit.error_code) || 0) + 1)
  }
  return Array.from(counts.entries())
    .sort((a, b) => b[1] - a[1])
    .slice(0, 3)
}

function selectedServerName(serverFilter: string, servers: MCPServerInfo[]) {
  if (serverFilter === "all") return "全部服务器"
  return servers.find((server) => server.id === serverFilter)?.name || serverFilter || "当前服务器"
}

function MCPAuditSummary(props: { stats: Array<[string, number]>; serverName: string; total: number }) {
  if (!props.total) return <p className="text-xs text-muted-foreground">{props.serverName}下暂无最近动作，后续调用 MCP 工具后会出现在这里。</p>
  if (!props.stats.length) return <p className="text-xs text-muted-foreground">{props.serverName}下暂无错误码聚合，最近动作以成功或无错误记录为主。</p>
  return (
    <div className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
      <span>{props.serverName}错误聚合</span>
      {props.stats.map(([code, count]) => <Badge key={code} variant="outline" className="text-[10px]">{code} x {count}</Badge>)}
    </div>
  )
}

function MCPAuditList({ audits, servers }: { audits: MCPAuditRecord[]; servers: MCPServerInfo[] }) {
  if (!audits.length) return <p className="rounded-lg bg-muted/50 px-3 py-3 text-sm text-muted-foreground">暂无审计记录，后续调用 MCP 工具后会出现在这里。</p>
  return <div className="space-y-2">{audits.map((audit) => <MCPAuditCard key={audit.audit_id} audit={audit} servers={servers} />)}</div>
}

function MCPAuditCard({ audit, servers }: { audit: MCPAuditRecord; servers: MCPServerInfo[] }) {
  const serverName = servers.find((server) => server.id === audit.server_id)?.name || audit.server_id || "未知服务器"
  return (
    <div className="rounded-lg bg-muted/50 px-3 py-3 space-y-2">
      <div className="flex flex-wrap items-start justify-between gap-2">
        <div className="min-w-0"><p className="text-sm font-medium text-foreground">{audit.tool_name}</p><p className="text-xs text-muted-foreground">{serverName}</p></div>
        <div className="flex flex-wrap items-center gap-1">
          <Badge variant={audit.outcome === "success" ? "default" : "secondary"} className="text-[10px]">{mcpOutcomeLabel(audit.outcome)}</Badge>
          <Badge variant="outline" className="text-[10px]">{mcpRiskLabel(audit.risk_level)}</Badge>
          {audit.error_code && <Badge variant="outline" className="text-[10px]">{audit.error_code}</Badge>}
        </div>
      </div>
      <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span>耗时: <span className="font-medium text-foreground">{audit.elapsed_ms} ms</span></span>
        <span>确认: <span className="font-medium text-foreground">{audit.requires_confirmation ? "需要" : "直连"}</span></span>
        <span>时间: <span className="font-medium text-foreground">{formatAuditTime(audit.timestamp)}</span></span>
      </div>
      {audit.error_message && <p className="text-xs text-muted-foreground">{audit.error_message}</p>}
    </div>
  )
}

export function mcpPolicyLabel(value: string) {
  return policyLabels[value] || value || "未知策略"
}

export function mcpRiskLabel(value: string) {
  return riskLabels[value] || value || "未知风险"
}

function mcpOutcomeLabel(value: string) {
  if (value === "success") return "成功"
  if (value === "denied") return "已拒绝"
  if (value === "failed") return "失败"
  return value || "未知结果"
}

function formatAuditTime(value: string) {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value || "-"
  return date.toLocaleString("zh-CN", { hour12: false })
}
