import type { ReactNode } from "react"
import { MCPObservabilityPanel } from "@/components/local-agent/views/mcp-observability-panel"
import type { MCPAuditRecord, MCPServerInfo, MCPTool } from "@/lib/local-agent/types"

const populatedServers: MCPServerInfo[] = [
  {
    id: "docs",
    name: "文档检索",
    type: "http",
    url: "http://127.0.0.1:3456/mcp",
    enabled: true,
    ready: true,
    tool_count: 3,
    allowed_tool_count: 2,
    blocked_tool_count: 1,
    requires_policy: true,
  },
  {
    id: "browser",
    name: "浏览器操作",
    type: "http",
    url: "http://127.0.0.1:4567/mcp",
    enabled: true,
    ready: false,
    tool_count: 2,
    allowed_tool_count: 0,
    blocked_tool_count: 2,
    requires_policy: true,
  },
]

const populatedTools: MCPTool[] = [
  {
    name: "search",
    description: "检索项目文档与知识条目。",
    server_id: "docs",
    allowed: true,
    risk_level: "low",
    requires_confirmation: false,
    audit_enabled: true,
    policy_source: "config",
  },
  {
    name: "read_note",
    description: "读取指定笔记正文。",
    server_id: "docs",
    allowed: true,
    risk_level: "medium",
    requires_confirmation: true,
    audit_enabled: true,
    policy_source: "config",
  },
  {
    name: "open_page",
    description: "打开目标页面并执行浏览器动作。",
    server_id: "browser",
    allowed: false,
    risk_level: "high",
    requires_confirmation: true,
    audit_enabled: true,
    policy_source: "default_deny",
  },
]

const missingFieldTools: MCPTool[] = [
  {
    name: "write_cache",
    description: "",
    server_id: "cache",
    allowed: false,
    risk_level: "critical",
    requires_confirmation: true,
    audit_enabled: false,
    policy_source: "",
  },
]

const populatedAudits: MCPAuditRecord[] = [
  {
    audit_id: "audit-4",
    timestamp: "2026-05-05T14:32:00+08:00",
    server_id: "docs",
    tool_name: "read_note",
    allowed: true,
    risk_level: "medium",
    requires_confirmation: true,
    audit_enabled: true,
    policy_source: "config",
    arguments_hash: "hash-read-2",
    outcome: "failed",
    error_code: "mcp_call_failed",
    error_message: "mcp tool error 500: downstream note shard timeout",
    elapsed_ms: 119,
  },
  {
    audit_id: "audit-3",
    timestamp: "2026-05-05T14:31:00+08:00",
    server_id: "docs",
    tool_name: "read_note",
    allowed: true,
    risk_level: "medium",
    requires_confirmation: true,
    audit_enabled: true,
    policy_source: "config",
    arguments_hash: "hash-read",
    outcome: "failed",
    error_code: "mcp_call_failed",
    error_message: "mcp tool error 500: note service unavailable",
    elapsed_ms: 83,
  },
  {
    audit_id: "audit-2",
    timestamp: "2026-05-05T14:30:00+08:00",
    server_id: "browser",
    tool_name: "open_page",
    allowed: false,
    risk_level: "high",
    requires_confirmation: true,
    audit_enabled: true,
    policy_source: "default_deny",
    arguments_hash: "hash-open",
    outcome: "denied",
    error_code: "mcp_tool_not_allowlisted",
    error_message: "mcp tool \"open_page\" is not allowlisted",
    elapsed_ms: 14,
  },
  {
    audit_id: "audit-1",
    timestamp: "2026-05-05T14:28:00+08:00",
    server_id: "docs",
    tool_name: "search",
    allowed: true,
    risk_level: "low",
    requires_confirmation: false,
    audit_enabled: true,
    policy_source: "config",
    arguments_hash: "hash-search",
    outcome: "success",
    elapsed_ms: 42,
  },
]

export default function MCPObservabilityAcceptancePage() {
  return (
    <main className="min-h-dvh bg-background p-6 text-foreground">
      <div className="mx-auto max-w-6xl space-y-6">
        <section className="space-y-2">
          <h1 className="text-xl font-semibold">MCP 观测验收</h1>
          <p className="text-sm text-muted-foreground">用于验收设置页中的 MCP 可观测视图，重点检查概览卡、筛选、只读运营列表、服务器联动审计筛选、错误码聚合、空态和缺字段态。</p>
        </section>
        <AcceptanceBlock title="空态" detail="未配置服务器时，仍应显示概览、筛选区和稳定空态。">
          <MCPObservabilityPanel servers={[]} tools={[]} emptyMessage="当前没有可观测的 MCP 工具，请先添加服务器或刷新设置。" />
        </AcceptanceBlock>
        <AcceptanceBlock title="只读观测态" detail="已配置服务器和工具时，应能直接扫描风险、allowlist、确认和审计覆盖。">
          <MCPObservabilityPanel servers={populatedServers} tools={populatedTools} audits={populatedAudits} />
        </AcceptanceBlock>
        <AcceptanceBlock title="缺字段态" detail="工具描述、策略来源或服务器映射不完整时，仍应给出可读占位，不出现布局断裂。">
          <MCPObservabilityPanel servers={[]} tools={missingFieldTools} />
        </AcceptanceBlock>
      </div>
    </main>
  )
}

function AcceptanceBlock({ title, detail, children }: { title: string; detail: string; children: ReactNode }) {
  return (
    <section className="space-y-3">
      <div className="space-y-1">
        <h2 className="text-sm font-medium text-muted-foreground">{title}</h2>
        <p className="text-sm text-muted-foreground">{detail}</p>
      </div>
      {children}
    </section>
  )
}
