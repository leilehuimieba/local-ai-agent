"use client"

import { useState, useEffect, useMemo } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Input } from "@/components/ui/input"
import { Badge } from "@/components/ui/badge"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import {
  Search,
  CheckCircle,
  XCircle,
  Loader2,
  ChevronDown,
  ChevronRight,
  Clock,
  FileText,
  AlertTriangle,
} from "lucide-react"
import { useLogsStore } from "@/lib/local-agent/store"
import type { LogRun } from "@/lib/local-agent/types"
import { fetchLogs } from "@/lib/local-agent/api"
import { cn } from "@/lib/utils"

type TimeFilter = "today" | "7days" | "30days"

interface LogRunWithDetails extends LogRun {
  summary?: string
  toolCalls?: string[]
  validation?: { passed: boolean; message: string }[]
  risks?: { level: string; description: string }[]
  metadata?: Record<string, string>
  context?: Record<string, string>
}

const statusFilters = [
  { id: "all", label: "全部" },
  { id: "completed", label: "成功" },
  { id: "failed", label: "失败" },
  { id: "running", label: "运行中" },
] as const

const timeFilters: { id: TimeFilter; label: string }[] = [
  { id: "today", label: "今天" },
  { id: "7days", label: "7天" },
  { id: "30days", label: "30天" },
]

function formatDuration(ms: number): string {
  if (ms === 0) return "进行中..."
  const seconds = Math.floor(ms / 1000)
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return minutes > 0 ? `${minutes}m ${remainingSeconds}s` : `${remainingSeconds}s`
}

function formatTimestamp(isoString: string): string {
  const date = new Date(isoString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMins / 60)
  const diffDays = Math.floor(diffHours / 24)

  if (diffMins < 1) return "刚刚"
  if (diffMins < 60) return `${diffMins}分钟前`
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays === 1) return "昨天"
  return date.toLocaleDateString()
}

export function LogsView() {
  const {
    runs,
    statusFilter,
    timeFilter,
    searchQuery,
    setStatusFilter,
    setTimeFilter,
    setSearchQuery,
    loadLogs,
  } = useLogsStore()

  const [expandedId, setExpandedId] = useState<string | null>(null)

  useEffect(() => {
    loadLogs()
  }, [loadLogs])

  const filteredLogs = useMemo(() => {
    return runs.filter((log) => {
      if (statusFilter !== "all" && log.status !== statusFilter) return false
      if (searchQuery && !log.title.toLowerCase().includes(searchQuery.toLowerCase())) return false
      const logDate = new Date(log.started_at)
      const now = new Date()
      const diffDays = Math.floor((now.getTime() - logDate.getTime()) / (1000 * 60 * 60 * 24))
      if (timeFilter === "today" && diffDays > 0) return false
      if (timeFilter === "7days" && diffDays > 7) return false
      if (timeFilter === "30days" && diffDays > 30) return false
      return true
    })
  }, [runs, statusFilter, timeFilter, searchQuery])

  const isEmpty = filteredLogs.length === 0

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="shrink-0 border-b border-border bg-card p-4">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <h1 className="text-lg font-semibold text-foreground">历史记录</h1>
          
          <div className="flex flex-wrap items-center gap-3">
            {/* Status Filter Pills */}
            <div className="flex items-center gap-1 rounded-lg bg-muted p-1">
              {statusFilters.map((filter) => (
                <button
                  key={filter.id}
                  onClick={() => setStatusFilter(filter.id as typeof statusFilter)}
                  className={cn(
                    "px-3 py-1 text-xs font-medium rounded-md transition-colors duration-200",
                    statusFilter === filter.id
                      ? "bg-card text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground"
                  )}
                >
                  {filter.label}
                </button>
              ))}
            </div>

            {/* Time Filter Pills */}
            <div className="flex items-center gap-1 rounded-lg bg-muted p-1">
              {timeFilters.map((filter) => (
                <button
                  key={filter.id}
                  onClick={() => setTimeFilter(filter.id)}
                  className={cn(
                    "px-3 py-1 text-xs font-medium rounded-md transition-colors duration-200",
                    timeFilter === filter.id
                      ? "bg-card text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground"
                  )}
                >
                  {filter.label}
                </button>
              ))}
            </div>

            {/* Search Input */}
            <div className="relative">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="搜索..."
                className="h-8 w-48 pl-9"
              />
            </div>
          </div>
        </div>
      </div>

      {/* Timeline */}
      <ScrollArea className="flex-1 p-4">
        {isEmpty ? (
          /* Empty State */
          <div className="flex h-full flex-col items-center justify-center text-center">
            <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-muted mb-4">
              <FileText className="h-8 w-8 text-muted-foreground" />
            </div>
            <h3 className="text-lg font-medium text-foreground mb-2">暂无运行记录</h3>
            <p className="text-sm text-muted-foreground max-w-sm">
              开始运行任务后，历史记录将显示在这里。
            </p>
          </div>
        ) : (
          <div className="relative ml-4">
            {/* Timeline Line */}
            <div className="absolute left-0 top-0 bottom-0 w-px bg-border" />

            {/* Log Entries */}
            <div className="space-y-4">
              {filteredLogs.map((log) => (
                <LogCard
                  key={log.run_id}
                  log={log}
                  expanded={expandedId === log.run_id}
                  onToggle={() => setExpandedId(expandedId === log.run_id ? null : log.run_id)}
                />
              ))}
            </div>
          </div>
        )}
      </ScrollArea>
    </div>
  )
}

function LogCard({
  log,
  expanded,
  onToggle,
}: {
  log: LogRunWithDetails
  expanded: boolean
  onToggle: () => void
}) {
  const [details, setDetails] = useState<LogRunWithDetails | null>(null)
  const [loadingDetails, setLoadingDetails] = useState(false)

  useEffect(() => {
    if (!expanded || details || loadingDetails) return
    setLoadingDetails(true)
    fetchLogs("events", { run_id: log.run_id, limit: 100 })
      .then((data) => {
        const items = data.items
        const toolCalls: string[] = []
        const validation: { passed: boolean; message: string }[] = []
        const risks: { level: string; description: string }[] = []
        const metadata: Record<string, string> = {}
        const context: Record<string, string> = {}
        let summary = ""

        for (const item of items) {
          if (item.tool_name && !toolCalls.includes(item.tool_name)) {
            toolCalls.push(item.tool_display_name || item.tool_name)
          }
          if (item.risk_level && item.summary) {
            risks.push({ level: item.risk_level, description: item.summary })
          }
          if (item.result_summary) {
            validation.push({ passed: item.level !== "error", message: item.result_summary })
          }
          if (item.metadata) {
            Object.assign(metadata, item.metadata)
          }
          if (item.final_answer) {
            summary = item.final_answer
          }
        }

        setDetails({
          ...log,
          summary: summary || log.title,
          toolCalls,
          validation,
          risks,
          metadata: Object.keys(metadata).length ? metadata : undefined,
          context: Object.keys(context).length ? context : undefined,
        })
      })
      .catch(() => {
        setDetails({ ...log, summary: log.title })
      })
      .finally(() => {
        setLoadingDetails(false)
      })
  }, [expanded, log, details, loadingDetails])

  const display = details || log
  const statusConfig = {
    completed: {
      icon: CheckCircle,
      color: "text-success",
      borderColor: "border-l-success",
      bgColor: "bg-success",
    },
    failed: {
      icon: XCircle,
      color: "text-destructive",
      borderColor: "border-l-destructive",
      bgColor: "bg-destructive",
    },
    running: {
      icon: Loader2,
      color: "text-chart-3",
      borderColor: "border-l-chart-3",
      bgColor: "bg-chart-3",
    },
  }

  const config = statusConfig[display.status]
  const Icon = config.icon

  return (
    <div className="relative pl-6 animate-in fade-in slide-in-from-left-2 duration-200">
      {/* Timeline Node */}
      <div
        className={cn(
          "absolute left-0 top-4 -translate-x-1/2 h-2.5 w-2.5 rounded-full",
          config.bgColor
        )}
      />

      {/* Card */}
      <div
        className={cn(
          "rounded-xl border border-l-4 bg-card overflow-hidden transition-all duration-200",
          config.borderColor
        )}
      >
        {/* Header */}
        <button
          onClick={onToggle}
          className="w-full flex items-center gap-3 p-4 text-left hover:bg-muted/50 transition-colors duration-200"
        >
          <Icon
            className={cn(
              "h-5 w-5 shrink-0",
              config.color,
              display.status === "running" && "animate-spin"
            )}
          />
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium text-foreground truncate">{display.title}</p>
            <p className="text-xs text-muted-foreground">{formatTimestamp(display.started_at)}</p>
          </div>
          <Badge variant="secondary" className="shrink-0 text-xs gap-1">
            <Clock className="h-3 w-3" />
            {formatDuration(display.duration_ms)}
          </Badge>
          {expanded ? (
            <ChevronDown className="h-4 w-4 text-muted-foreground transition-transform duration-200" />
          ) : (
            <ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200" />
          )}
        </button>

        {/* Expanded Content */}
        {expanded && (
          <div className="border-t border-border p-4 animate-in fade-in slide-in-from-top-2 duration-200">
            {loadingDetails ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
              </div>
            ) : (
              <Tabs defaultValue="summary" className="w-full">
                <TabsList className="mb-4 h-auto flex-wrap">
                  <TabsTrigger value="summary" className="text-xs">摘要</TabsTrigger>
                  <TabsTrigger value="tools" className="text-xs">工具调用</TabsTrigger>
                  <TabsTrigger value="validation" className="text-xs">验证</TabsTrigger>
                  <TabsTrigger value="risks" className="text-xs">风险</TabsTrigger>
                  <TabsTrigger value="metadata" className="text-xs">元数据</TabsTrigger>
                </TabsList>

                <TabsContent value="summary" className="mt-0">
                  <p className="text-sm text-muted-foreground">
                    {display.summary || "无摘要"}
                  </p>
                </TabsContent>

                <TabsContent value="tools" className="mt-0">
                  {display.toolCalls?.length ? (
                    <div className="flex flex-wrap gap-2">
                      {display.toolCalls.map((tool, i) => (
                        <Badge key={i} variant="outline" className="font-mono text-xs">
                          {tool}
                        </Badge>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-muted-foreground">无工具调用记录</p>
                  )}
                </TabsContent>

                <TabsContent value="validation" className="mt-0">
                  {display.validation?.length ? (
                    <ul className="space-y-2">
                      {display.validation.map((v, i) => (
                        <li key={i} className="flex items-center gap-2 text-sm">
                          {v.passed ? (
                            <CheckCircle className="h-4 w-4 text-success shrink-0" />
                          ) : (
                            <XCircle className="h-4 w-4 text-destructive shrink-0" />
                          )}
                          <span className={v.passed ? "text-muted-foreground" : "text-destructive"}>
                            {v.message}
                          </span>
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-sm text-muted-foreground">无验证步骤</p>
                  )}
                </TabsContent>

                <TabsContent value="risks" className="mt-0">
                  {display.risks?.length ? (
                    <ul className="space-y-2">
                      {display.risks.map((r, i) => (
                        <li key={i} className="flex items-start gap-2 text-sm">
                          <AlertTriangle className={cn(
                            "h-4 w-4 shrink-0 mt-0.5",
                            r.level === "high" ? "text-destructive" : "text-warning"
                          )} />
                          <span className="text-muted-foreground">{r.description}</span>
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-sm text-muted-foreground">无风险记录</p>
                  )}
                </TabsContent>

                <TabsContent value="metadata" className="mt-0">
                  {display.metadata && Object.keys(display.metadata).length ? (
                    <div className="space-y-2">
                      {Object.entries(display.metadata).map(([key, value]) => (
                        <div key={key} className="flex justify-between text-sm">
                          <span className="text-muted-foreground">{key}</span>
                          <span className="font-medium text-foreground font-mono">{value}</span>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-muted-foreground">无元数据</p>
                  )}
                </TabsContent>
              </Tabs>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
