"use client"

import { useEffect, useState } from "react"
import { cn } from "@/lib/utils"
import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import {
  Target,
  ListTodo,
  AlertTriangle,
  Brain,
  X,
  Loader2,
  CheckCircle,
  Clock,
} from "lucide-react"
import { useUIStore, useRuntimeStore, useMemoryStore } from "@/lib/local-agent/store"
import type { RunState } from "@/lib/local-agent/types"

const runStateConfig: Record<RunState, { label: string; color: string; icon: React.ReactNode }> = {
  idle: { label: "空闲", color: "bg-muted text-muted-foreground", icon: <Clock className="h-3 w-3" /> },
  running: { label: "运行中", color: "bg-success/10 text-success", icon: <Loader2 className="h-3 w-3 animate-spin" /> },
  awaiting_confirmation: { label: "等待中", color: "bg-warning/10 text-warning", icon: <AlertTriangle className="h-3 w-3" /> },
  completed: { label: "已完成", color: "bg-success/10 text-success", icon: <CheckCircle className="h-3 w-3" /> },
  failed: { label: "失败", color: "bg-destructive/10 text-destructive", icon: <X className="h-3 w-3" /> },
}

export function RightDrawer() {
  const { rightDrawerOpen, setRightDrawerOpen, activeView } = useUIStore()
  const { runState, currentTaskTitle, events, confirmation, messages } = useRuntimeStore()
  const { memories, loadMemories } = useMemoryStore()
  const [elapsedTime, setElapsedTime] = useState(0)

  // Initialize memories
  useEffect(() => {
    loadMemories()
  }, [loadMemories])

  // Elapsed time counter
  useEffect(() => {
    let interval: ReturnType<typeof setInterval>
    if (runState === "running") {
      interval = setInterval(() => {
        setElapsedTime((prev) => prev + 1)
      }, 1000)
    } else {
      setElapsedTime(0)
    }
    return () => clearInterval(interval)
  }, [runState])

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}m ${secs.toString().padStart(2, "0")}s`
  }

  const stateConfig = runStateConfig[runState]

  // Only show in Task view
  if (activeView !== "task") return null

  return (
    <aside
      className={cn(
        "shrink-0 border-l border-border transition-all duration-300 ease-in-out overflow-hidden",
        "backdrop-blur-[18px] bg-card/80 saturate-[1.3]",
        "hidden lg:block",
        rightDrawerOpen ? "w-[280px]" : "w-0 border-l-0"
      )}
    >
      <ScrollArea className="h-full">
        <div className="p-4 space-y-6">
          {/* Header */}
          <div className="flex items-center justify-between">
            <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">
              Task Context
            </span>
            <Button
              variant="ghost"
              size="icon"
              className="h-6 w-6"
              onClick={() => setRightDrawerOpen(false)}
            >
              <X className="h-3.5 w-3.5" />
            </Button>
          </div>

          {/* Task Overview */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <Target className="h-4 w-4 text-primary" />
              <h3 className="text-sm font-semibold text-foreground">Task Overview</h3>
            </div>
            <div className="space-y-2">
              <div className="rounded-lg border border-border bg-card p-3">
                <p className="text-xs text-muted-foreground mb-1">Current Task</p>
                <p className="text-sm font-medium text-foreground line-clamp-2">
                  {currentTaskTitle || "无活跃任务"}
                </p>
                {runState !== "idle" && (
                  <div className="mt-2 flex items-center gap-2">
                    <Badge variant="secondary" className={cn("text-xs border-0 gap-1", stateConfig.color)}>
                      {stateConfig.icon}
                      {stateConfig.label}
                    </Badge>
                    {runState === "running" && (
                      <span className="text-xs text-muted-foreground">{formatTime(elapsedTime)}</span>
                    )}
                  </div>
                )}
              </div>
            </div>
          </section>

          {/* Next Actions / Events */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <ListTodo className="h-4 w-4 text-primary" />
              <h3 className="text-sm font-semibold text-foreground">Recent Activity</h3>
            </div>
            {events.length > 0 ? (
              <ul className="space-y-2">
                {events.slice(-5).reverse().map((event) => (
                  <li
                    key={event.event_id}
                    className="flex items-start gap-2 text-sm text-muted-foreground"
                  >
                    <div className={cn(
                      "mt-1.5 h-1.5 w-1.5 rounded-full shrink-0",
                      event.stage === "end" ? "bg-success" : "bg-primary animate-pulse"
                    )} />
                    <span className="text-xs">{event.summary}</span>
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-xs text-muted-foreground">No recent activity</p>
            )}
          </section>

          {/* Risks */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <AlertTriangle className="h-4 w-4 text-warning" />
              <h3 className="text-sm font-semibold text-foreground">Risks</h3>
            </div>
            {confirmation ? (
              <div className="rounded-lg border border-warning/30 bg-warning/5 p-3">
                <p className="text-xs text-warning font-medium capitalize">
                  {confirmation.risk_level} Risk
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                  {confirmation.action_summary}
                </p>
                {confirmation.hazards.length > 0 && (
                  <ul className="mt-2 space-y-1">
                    {confirmation.hazards.map((hazard, i) => (
                      <li key={i} className="text-xs text-destructive flex items-center gap-1">
                        <span className="h-1 w-1 rounded-full bg-destructive" />
                        {hazard}
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            ) : (
              <p className="text-xs text-muted-foreground">No active risks</p>
            )}
          </section>

          {/* Memories */}
          <section>
            <div className="flex items-center gap-2 mb-3">
              <Brain className="h-4 w-4 text-primary" />
              <h3 className="text-sm font-semibold text-foreground">Memories</h3>
              <Badge variant="secondary" className="text-xs ml-auto">
                {memories.length}
              </Badge>
            </div>
            {memories.length > 0 ? (
              <div className="space-y-2">
                {memories.slice(0, 4).map((memory) => (
                  <div
                    key={memory.id}
                    className="rounded-lg border border-border bg-card/50 p-2"
                  >
                    <div className="flex items-center gap-2 mb-1">
                      <Badge variant="outline" className="text-xs capitalize">
                        {memory.kind}
                      </Badge>
                    </div>
                    <p className="text-xs text-foreground line-clamp-2">{memory.title || memory.content}</p>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-xs text-muted-foreground">暂无记忆记录</p>
            )}
          </section>

          {/* Message Stats */}
          {messages.length > 0 && (
            <section className="pt-4 border-t border-border">
              <div className="flex items-center justify-between text-xs text-muted-foreground">
                <span>Messages</span>
                <span className="font-medium text-foreground">{messages.length}</span>
              </div>
            </section>
          )}
        </div>
      </ScrollArea>
    </aside>
  )
}
