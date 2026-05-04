"use client"

import { useEffect, useState } from "react"
import { cn } from "@/lib/utils"
import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet"
import {
  Target,
  ListTodo,
  AlertTriangle,
  Brain,
  X,
  Loader2,
  CheckCircle,
  Clock,
  History,
  XCircle,
} from "lucide-react"
import { useUIStore, useRuntimeStore, useMemoryStore, useLogsStore } from "@/lib/local-agent/store"
import type { RunState, LogStatus } from "@/lib/local-agent/types"
import { useIsMobile } from "@/hooks/use-mobile"

const runStateConfig: Record<RunState, { label: string; color: string; icon: React.ReactNode }> = {
  idle: { label: "空闲", color: "bg-muted text-muted-foreground", icon: <Clock className="h-3 w-3" /> },
  running: { label: "运行中", color: "bg-success/10 text-success", icon: <Loader2 className="h-3 w-3 animate-spin" /> },
  awaiting_confirmation: { label: "等待中", color: "bg-warning/10 text-warning", icon: <AlertTriangle className="h-3 w-3" /> },
  completed: { label: "已完成", color: "bg-success/10 text-success", icon: <CheckCircle className="h-3 w-3" /> },
  failed: { label: "失败", color: "bg-destructive/10 text-destructive", icon: <X className="h-3 w-3" /> },
}

const logStatusConfig: Record<LogStatus, { icon: React.ReactNode; color: string }> = {
  completed: { icon: <CheckCircle className="h-3 w-3" />, color: "text-success" },
  failed: { icon: <XCircle className="h-3 w-3" />, color: "text-destructive" },
  running: { icon: <Loader2 className="h-3 w-3 animate-spin" />, color: "text-chart-3" },
}

function formatRelativeTime(timestamp: string): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)
  if (diffMins < 1) return "刚刚"
  if (diffMins < 60) return `${diffMins}分钟前`
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays === 1) return "昨天"
  return date.toLocaleDateString("zh-CN", { month: "short", day: "numeric" })
}

function DrawerContent() {
  const { runState, currentTaskTitle, events, confirmation, messages, sessionId, resumeSession } = useRuntimeStore()
  const { memories, loadMemories } = useMemoryStore()
  const { runs, loadLogs } = useLogsStore()
  const [elapsedTime, setElapsedTime] = useState(0)

  useEffect(() => {
    loadMemories()
    loadLogs()
  }, [loadMemories, loadLogs])

  useEffect(() => {
    let interval: ReturnType<typeof setInterval>
    if (runState === "running") {
      interval = setInterval(() => { setElapsedTime((prev) => prev + 1) }, 1000)
    } else { setElapsedTime(0) }
    return () => clearInterval(interval)
  }, [runState])

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}m ${secs.toString().padStart(2, "0")}s`
  }
  const stateConfig = runStateConfig[runState]

  return (
    <div className="p-4 space-y-6 overflow-hidden break-words">
      <section>
        <div className="flex items-center gap-2 mb-3">
          <Target className="h-4 w-4 text-primary" />
          <h3 className="text-sm font-semibold text-foreground">Task Overview</h3>
        </div>
        <div className="space-y-2">
          <div className="rounded-lg border border-border bg-card p-3">
            <p className="text-xs text-muted-foreground mb-1">Current Task</p>
            <p className="text-sm font-medium text-foreground line-clamp-2">{currentTaskTitle || "无活跃任务"}</p>
            {runState !== "idle" && (
              <div className="mt-2 flex items-center gap-2">
                <Badge variant="secondary" className={cn("text-xs border-0 gap-1", stateConfig.color)}>
                  {stateConfig.icon}{stateConfig.label}
                </Badge>
                {runState === "running" && <span className="text-xs text-muted-foreground">{formatTime(elapsedTime)}</span>}
              </div>
            )}
          </div>
        </div>
      </section>
      <section>
        <div className="flex items-center gap-2 mb-3">
          <ListTodo className="h-4 w-4 text-primary" />
          <h3 className="text-sm font-semibold text-foreground">Recent Activity</h3>
        </div>
        {events.length > 0 ? (
          <ul className="space-y-2">
            {events.slice(-5).reverse().map((event) => (
              <li key={event.event_id} className="flex items-start gap-2 text-sm text-muted-foreground">
                <div className={cn("mt-1.5 h-1.5 w-1.5 rounded-full shrink-0", event.stage === "end" ? "bg-success" : "bg-primary animate-pulse")} />
                <span className="text-xs">{event.summary}</span>
              </li>
            ))}
          </ul>
        ) : <p className="text-xs text-muted-foreground">No recent activity</p>}
      </section>
      <section>
        <div className="flex items-center gap-2 mb-3">
          <AlertTriangle className="h-4 w-4 text-warning" />
          <h3 className="text-sm font-semibold text-foreground">Risks</h3>
        </div>
        {confirmation ? (
          <div className="rounded-lg border border-warning/30 bg-warning/5 p-3">
            <p className="text-xs text-warning font-medium capitalize">{confirmation.risk_level} Risk</p>
            <p className="text-xs text-muted-foreground mt-1">{confirmation.action_summary}</p>
            {confirmation.hazards.length > 0 && (
              <ul className="mt-2 space-y-1">
                {confirmation.hazards.map((hazard, i) => (
                  <li key={i} className="text-xs text-destructive flex items-center gap-1">
                    <span className="h-1 w-1 rounded-full bg-destructive" />{hazard}
                  </li>
                ))}
              </ul>
            )}
          </div>
        ) : <p className="text-xs text-muted-foreground">No active risks</p>}
      </section>
      <section>
        <div className="flex items-center gap-2 mb-3">
          <Brain className="h-4 w-4 text-primary" />
          <h3 className="text-sm font-semibold text-foreground">Memories</h3>
          <Badge variant="secondary" className="text-xs ml-auto">{memories.length}</Badge>
        </div>
        {memories.length > 0 ? (
          <div className="space-y-2">
            {memories.slice(0, 4).map((memory) => (
              <div key={memory.id} className="rounded-lg border border-border bg-card/50 p-2 overflow-hidden">
                <div className="flex items-center gap-2 mb-1 overflow-hidden">
                  <Badge variant="outline" className="text-xs capitalize truncate max-w-full">{memory.kind}</Badge>
                </div>
                <p className="text-xs text-foreground line-clamp-2 break-all">{memory.title || memory.content}</p>
              </div>
            ))}
          </div>
        ) : <p className="text-xs text-muted-foreground">暂无记忆记录</p>}
      </section>
      <section>
        <div className="flex items-center gap-2 mb-3">
          <History className="h-4 w-4 text-primary" />
          <h3 className="text-sm font-semibold text-foreground">Recent Runs</h3>
        </div>
        {runs.length > 0 ? (
          <div className="space-y-1.5">
            {runs.filter((run) => run.session_id !== sessionId).slice(-8).reverse().map((run) => {
              const config = logStatusConfig[run.status]
              return (
                <button key={run.run_id} onClick={() => resumeSession(run.session_id)}
                  className="w-full flex items-center gap-2 rounded-lg border border-border bg-card p-2 text-left hover:bg-muted/50 transition-colors overflow-hidden">
                  <span className={cn("shrink-0", config.color)}>{config.icon}</span>
                  <div className="flex-1 min-w-0 overflow-hidden">
                    <p className="text-xs font-medium text-foreground truncate">{run.title}</p>
                    <p className="text-[10px] text-muted-foreground">{formatRelativeTime(run.started_at)}</p>
                  </div>
                </button>
              )
            })}
          </div>
        ) : <p className="text-xs text-muted-foreground">暂无运行记录</p>}
      </section>
      {messages.length > 0 && (
        <section className="pt-4 border-t border-border">
          <div className="flex items-center justify-between text-xs text-muted-foreground">
            <span>Messages</span><span className="font-medium text-foreground">{messages.length}</span>
          </div>
        </section>
      )}
    </div>
  )
}

export function RightDrawer() {
  const { rightDrawerOpen, setRightDrawerOpen, activeView, mobileDrawerOpen, setMobileDrawerOpen } = useUIStore()
  const isMobile = useIsMobile()

  if (activeView !== "task") return null

  if (isMobile) {
    return (
      <Sheet open={mobileDrawerOpen} onOpenChange={setMobileDrawerOpen}>
        <SheetContent side="right" className="w-[300px] p-0">
          <ScrollArea className="h-full">
            <DrawerContent />
          </ScrollArea>
        </SheetContent>
      </Sheet>
    )
  }

  return (
    <aside className={cn(
      "shrink-0 border-l border-border transition-all duration-300 ease-in-out overflow-hidden",
      "backdrop-blur-[18px] bg-card/80 saturate-[1.3]",
      "hidden lg:block",
      rightDrawerOpen ? "w-[240px]" : "w-0 border-l-0"
    )}>
      <div className="h-full overflow-y-auto">
        <DrawerContent />
      </div>
    </aside>
  )
}
