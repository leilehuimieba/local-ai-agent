"use client"

import { useState, useRef, useEffect, useCallback } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Checkbox } from "@/components/ui/checkbox"
import { Kbd } from "@/components/ui/kbd"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  Send,
  RefreshCw,
  CheckCircle,
  XCircle,
  Database,
  FileText,
  Code,
  BarChart3,
  Loader2,
  AlertTriangle,
  Copy,
  Check,
} from "lucide-react"
import { useRuntimeStore, useSettingsStore } from "@/lib/local-agent/store"
import type { Message, RuntimeEvent, ResultBlock, Confirmation, ConnectionState } from "@/lib/local-agent/types"
import { cn } from "@/lib/utils"
import { submitChatRun, submitConfirmationDecision, type SubmitChatRunPayload } from "@/lib/local-agent/api"
import { useSessionEventStream } from "@/hooks/useSessionEventStream"
import type { ConnectionState as StreamConnectionState } from "@/hooks/useSessionEventStream"

const quickPrompts = [
  { icon: Database, label: "查询数据库" },
  { icon: FileText, label: "总结文档" },
  { icon: Code, label: "编写代码" },
  { icon: BarChart3, label: "分析数据" },
]

export function TaskView() {
  const {
    messages,
    events,
    runState,
    confirmation,
    composeValue,
    criticalError,
    submitError,
    addMessage,
    setComposeValue,
    setConfirmation,
    startNewRun,
    acceptRun,
    applyEvent,
    completeRun,
    failRun,
    setRunState,
    setConnectionState,
    setCriticalError,
    setSubmitError,
  } = useRuntimeStore()

  const settings = useSettingsStore()
  const [selectedKb, setSelectedKb] = useState("all")
  const [isUserScrolling, setIsUserScrolling] = useState(false)
  const scrollRef = useRef<HTMLDivElement>(null)
  const bottomRef = useRef<HTMLDivElement>(null)

  // SSE event stream
  const sessionId = useRuntimeStore((s) => s.sessionId)
  useSessionEventStream(sessionId, {
    onEvent: (event) => {
      applyEvent(event)
    },
    onConnectionChange: (state: StreamConnectionState) => {
      setConnectionState(state as ConnectionState)
    },
    onStreamError: (message) => {
      setCriticalError(message)
    },
  })

  // Auto-scroll to bottom when new messages arrive (unless user is scrolling)
  useEffect(() => {
    if (!isUserScrolling && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: "smooth" })
    }
  }, [messages, events, isUserScrolling])

  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    const target = e.currentTarget
    const isAtBottom = target.scrollHeight - target.scrollTop - target.clientHeight < 50
    setIsUserScrolling(!isAtBottom)
  }, [])

  const handleSend = async () => {
    if (!composeValue.trim()) return

    addMessage({ role: "user", content: composeValue })
    const userInput = composeValue
    setComposeValue("")
    startNewRun(userInput)

    try {
      const payload: SubmitChatRunPayload = {
        sessionId,
        userInput,
        mode: settings.mode,
        model: settings.model,
        workspace: settings.workspace,
        knowledgeBaseId: selectedKb === "all" ? "" : selectedKb,
      }
      const result = await submitChatRun(payload)
      acceptRun(result.session_id, result.run_id)
    } catch (err) {
      failRun(err instanceof Error ? err.message : "提交任务失败")
    }
  }

  const handleConfirmation = async (decision: "approve" | "deny", remember: boolean) => {
    if (!confirmation) return
    try {
      await submitConfirmationDecision({
        confirmationId: confirmation.confirmation_id,
        runId: confirmation.run_id,
        decision: decision === "approve" ? "approve" : "reject",
        remember,
      })
      setConfirmation(null)
      setRunState("running")
    } catch (err) {
      setSubmitError(err instanceof Error ? err.message : "提交确认失败")
    }
  }

  const handleRetry = () => {
    setRunState("running")
    startNewRun("重试之前的任务")
  }

  const hasMessages = messages.length > 0
  const showIdle = !hasMessages && runState === "idle"

  return (
    <div className="flex h-full flex-col">
      {showIdle ? (
        /* Idle State */
        <div className="flex flex-1 flex-col items-center justify-center p-8">
          <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/10 mb-4">
            <div className="h-10 w-10 rounded-xl bg-primary flex items-center justify-center">
              <span className="text-xl font-bold text-primary-foreground">LA</span>
            </div>
          </div>
          <h2 className="text-xl font-semibold text-foreground mb-2 text-balance text-center">
            需要我做什么？
          </h2>
          <p className="text-sm text-muted-foreground mb-8 text-center">
            开始对话或选择下方快捷操作
          </p>
          <div className="flex flex-wrap justify-center gap-3 mb-8">
            {quickPrompts.map((prompt) => {
              const Icon = prompt.icon
              return (
                <button
                  key={prompt.label}
                  onClick={() => setComposeValue(prompt.label)}
                  className="flex items-center gap-2 rounded-full border border-border bg-card px-4 py-2 text-sm font-medium text-foreground hover:bg-muted transition-colors duration-200"
                >
                  <Icon className="h-4 w-4 text-primary" />
                  {prompt.label}
                </button>
              )
            })}
          </div>
          

        </div>
      ) : (
        /* Message Thread */
        <ScrollArea 
          className="flex-1"
          onScrollCapture={handleScroll}
        >
          <div className="p-4">
            <div className="mx-auto max-w-3xl space-y-4">
              {messages.map((message) => (
                <MessageBubble key={message.id} message={message} />
              ))}

              {/* Running State */}
              {runState === "running" && (
                <RunningCard events={events.slice(-3)} />
              )}

              {/* Error State */}
              {runState === "failed" && criticalError && (
                <ErrorCard error={criticalError} onRetry={handleRetry} />
              )}

              {/* Confirmation State */}
              {runState === "awaiting_confirmation" && confirmation && (
                <ConfirmationCard
                  confirmation={confirmation}
                  onDecision={handleConfirmation}
                />
              )}

              <div ref={bottomRef} />
            </div>
          </div>
        </ScrollArea>
      )}

      {/* Bottom Composer */}
      <div className="shrink-0 border-t border-border bg-card p-4">
        <div className="mx-auto max-w-3xl">
          {submitError && (
            <div className="mb-3 flex items-center gap-2 text-sm text-destructive">
              <AlertTriangle className="h-4 w-4" />
              {submitError}
            </div>
          )}
          <div className="flex items-center gap-3">
            <Select value={selectedKb} onValueChange={setSelectedKb}>
              <SelectTrigger className="w-[140px] h-10">
                <SelectValue placeholder="知识库" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">全部来源</SelectItem>
                <SelectItem value="docs">文档</SelectItem>
                <SelectItem value="code">代码库</SelectItem>
                <SelectItem value="notes">笔记</SelectItem>
              </SelectContent>
            </Select>

            <div className="relative flex-1">
              <Input
                value={composeValue}
                onChange={(e) => setComposeValue(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault()
                    handleSend()
                  }
                }}
                placeholder="描述一个任务..."
                className="h-10 pr-16"
                disabled={runState === "running" || runState === "awaiting_confirmation"}
              />
              <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2">
                <Kbd className="hidden sm:inline-flex text-xs">⌘K</Kbd>
              </div>
            </div>

            <Button
              onClick={handleSend}
              size="icon"
              className="h-10 w-10 shrink-0 bg-primary hover:bg-primary/90"
              disabled={!composeValue.trim() || runState === "running" || runState === "awaiting_confirmation"}
            >
              {runState === "running" ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <Send className="h-4 w-4" />
              )}
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}

function MessageBubble({ message }: { message: Message }) {
  if (message.role === "user") {
    return (
      <div className="flex justify-end animate-in fade-in slide-in-from-bottom-2 duration-200">
        <div className="max-w-[80%] rounded-2xl rounded-tr-md bg-secondary px-4 py-3">
          <p className="text-sm text-foreground whitespace-pre-wrap">{message.content}</p>
        </div>
      </div>
    )
  }

  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[80%] rounded-2xl rounded-tl-md border border-border bg-card p-4">
        <p className="text-sm text-foreground whitespace-pre-wrap">{message.content}</p>
        
        {message.blocks?.map((block, index) => (
          <ResultBlockRenderer key={index} block={block} />
        ))}
      </div>
    </div>
  )
}

function ResultBlockRenderer({ block }: { block: ResultBlock }) {
  const [copied, setCopied] = useState(false)

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  switch (block.type) {
    case "text":
      return (
        <div className="mt-3 prose prose-sm dark:prose-invert max-w-none">
          <div 
            className="text-sm text-foreground"
            dangerouslySetInnerHTML={{ 
              __html: block.content
                .replace(/^## (.+)$/gm, '<h3 class="text-base font-semibold mt-4 mb-2">$1</h3>')
                .replace(/^### (.+)$/gm, '<h4 class="text-sm font-medium mt-3 mb-1">$1</h4>')
                .replace(/\n/g, '<br />') 
            }} 
          />
        </div>
      )

    case "code":
      return (
        <div className="mt-3 relative group">
          <div className="flex items-center justify-between bg-muted/50 px-3 py-1.5 rounded-t-lg border-b border-border">
            <span className="text-xs font-medium text-muted-foreground uppercase">
              {block.language}
            </span>
            <button
              onClick={() => handleCopy(block.content)}
              className="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-muted rounded"
            >
              {copied ? (
                <Check className="h-3.5 w-3.5 text-success" />
              ) : (
                <Copy className="h-3.5 w-3.5 text-muted-foreground" />
              )}
            </button>
          </div>
          <pre className="rounded-b-lg bg-muted p-3 text-xs font-mono overflow-x-auto">
            <code className="text-foreground">{block.content}</code>
          </pre>
        </div>
      )

    case "list":
      return (
        <ul className="mt-3 space-y-1.5">
          {block.items.map((item, i) => (
            <li key={i} className="flex items-start gap-2 text-sm text-foreground">
              <span className="mt-1.5 h-1.5 w-1.5 rounded-full bg-primary shrink-0" />
              {item}
            </li>
          ))}
        </ul>
      )

    case "data_grid":
      return (
        <div className="mt-3 overflow-x-auto rounded-lg border border-border">
          <table className="w-full text-xs">
            <thead>
              <tr className="border-b border-border bg-muted">
                {block.headers.map((h, i) => (
                  <th key={i} className="px-3 py-2 text-left font-medium text-foreground">
                    {h}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {block.rows.map((row, i) => (
                <tr key={i} className="border-b border-border last:border-0">
                  {row.map((cell, j) => (
                    <td key={j} className="px-3 py-2 text-muted-foreground">
                      {cell}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )

    default:
      return null
  }
}

function RunningCard({ events }: { events: RuntimeEvent[] }) {
  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[80%] rounded-2xl rounded-tl-md border border-primary/20 bg-card p-4">
        <div className="flex items-center gap-2 mb-3">
          <div className="relative">
            <Loader2 className="h-4 w-4 animate-spin text-primary" />
            <div className="absolute inset-0 h-4 w-4 animate-ping rounded-full bg-primary/20" />
          </div>
          <span className="text-sm font-medium text-foreground">运行中...</span>
        </div>
        <div className="space-y-2">
          {events.map((event) => (
            <div
              key={event.event_id}
              className="flex items-center gap-2 text-xs text-muted-foreground"
            >
              <div className={cn(
                "h-1.5 w-1.5 rounded-full shrink-0",
                event.stage === "progress" ? "bg-primary animate-pulse" : "bg-success"
              )} />
              <span>{event.summary}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

function ErrorCard({ error, onRetry }: { error: string; onRetry: () => void }) {
  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[80%] rounded-2xl rounded-tl-md border-l-4 border-l-destructive border border-border bg-card p-4">
        <div className="flex items-center gap-2 mb-2">
          <XCircle className="h-4 w-4 text-destructive" />
          <span className="text-sm font-medium text-destructive">Error</span>
        </div>
        <p className="text-sm text-muted-foreground mb-3">{error}</p>
        <Button variant="outline" size="sm" onClick={onRetry} className="gap-2">
          <RefreshCw className="h-3 w-3" />
          Retry
        </Button>
      </div>
    </div>
  )
}

function ConfirmationCard({ 
  confirmation, 
  onDecision 
}: { 
  confirmation: Confirmation
  onDecision: (decision: "approve" | "deny", remember: boolean) => void 
}) {
  const [remember, setRemember] = useState(false)

  const riskColors = {
    low: "text-success",
    medium: "text-warning",
    high: "text-destructive",
    critical: "text-destructive",
  }

  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[90%] rounded-2xl rounded-tl-md border-l-4 border-l-warning border border-border bg-card p-4">
        <div className="flex items-center gap-2 mb-3">
          <AlertTriangle className="h-4 w-4 text-warning" />
          <span className="text-sm font-medium text-warning">需要确认</span>
          <span className={cn("text-xs font-medium uppercase", riskColors[confirmation.risk_level])}>
            {confirmation.risk_level} risk
          </span>
        </div>
        
        <p className="text-sm font-medium text-foreground mb-2">
          {confirmation.action_summary}
        </p>
        <p className="text-sm text-muted-foreground mb-3">
          {confirmation.reason}
        </p>

        {confirmation.target_paths.length > 0 && (
          <div className="mb-3">
            <p className="text-xs font-medium text-muted-foreground mb-1">Affected paths:</p>
            <div className="flex flex-wrap gap-1">
              {confirmation.target_paths.map((path, i) => (
                <code key={i} className="text-xs bg-muted px-1.5 py-0.5 rounded text-foreground">
                  {path}
                </code>
              ))}
            </div>
          </div>
        )}

        {confirmation.hazards.length > 0 && (
          <div className="mb-3">
            <p className="text-xs font-medium text-muted-foreground mb-1">Potential hazards:</p>
            <ul className="space-y-0.5">
              {confirmation.hazards.map((hazard, i) => (
                <li key={i} className="text-xs text-destructive flex items-center gap-1">
                  <span className="h-1 w-1 rounded-full bg-destructive" />
                  {hazard}
                </li>
              ))}
            </ul>
          </div>
        )}

        <div className="flex items-center gap-3 pt-2">
          <Button 
            size="sm" 
            onClick={() => onDecision("approve", remember)}
            className="bg-success hover:bg-success/90 text-success-foreground"
          >
            <CheckCircle className="h-3.5 w-3.5 mr-1.5" />
            批准
          </Button>
          <Button 
            variant="outline" 
            size="sm"
            onClick={() => onDecision("deny", remember)}
          >
            <XCircle className="h-3.5 w-3.5 mr-1.5" />
            拒绝
          </Button>
          <div className="flex items-center gap-2 ml-auto">
            <Checkbox
              id="remember"
              checked={remember}
              onCheckedChange={(checked) => setRemember(checked as boolean)}
            />
            <label htmlFor="remember" className="text-xs text-muted-foreground cursor-pointer">
              记住此选择
            </label>
          </div>
        </div>
      </div>
    </div>
  )
}
