"use client"

import { useState, useRef, useEffect, useCallback, useMemo } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { TextareaAuto } from "@/components/ui/textarea-auto"
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
  Plus,
  Download,
  Search,
  ChevronUp,
  ChevronDown,
  X,
  Paperclip,
  Pencil,
} from "lucide-react"
import { useRuntimeStore, useSettingsStore, useUIStore } from "@/lib/local-agent/store"
import type { Message, RuntimeEvent, ResultBlock, Confirmation, ConnectionState } from "@/lib/local-agent/types"
import { cn } from "@/lib/utils"
import { submitChatRun, submitChatRetry, submitChatCancel, submitConfirmationDecision, uploadKnowledgeFile, type SubmitChatRunPayload } from "@/lib/local-agent/api"
import { useSessionEventStream } from "@/hooks/useSessionEventStream"
import type { ConnectionState as StreamConnectionState } from "@/hooks/useSessionEventStream"
import { LightweightMarkdown } from "@/components/local-agent/markdown"
import { DiffPreview, DiffPreviewReportCard, inferDiffPreviewReport } from "@/components/local-agent/diff-preview"
import { toast } from "sonner"

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
    connectionState,
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
    cancelRun,
    editAndResend,
    setRunState,
    setConnectionState,
    setCriticalError,
    setSubmitError,
  } = useRuntimeStore()

  const settings = useSettingsStore()
  const [isUserScrolling, setIsUserScrolling] = useState(false)
  const [searchQuery, setSearchQuery] = useState("")
  const [currentMatchIndex, setCurrentMatchIndex] = useState(0)
  const [attachedFiles, setAttachedFiles] = useState<{ name: string; content: string }[]>([])
  const fileInputRef = useRef<HTMLInputElement>(null)
  const scrollRef = useRef<HTMLDivElement>(null)
  const bottomRef = useRef<HTMLDivElement>(null)
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const messageRefs = useRef<Record<string, HTMLDivElement | null>>({})

  // Auto-focus input when run completes or on initial load
  useEffect(() => {
    if (runState !== "running" && runState !== "awaiting_confirmation") {
      textareaRef.current?.focus()
    }
  }, [runState])

  // Update browser title based on run state
  useEffect(() => {
    const base = "本地智能体"
    if (runState === "running") {
      document.title = `运行中... | ${base}`
    } else if (runState === "awaiting_confirmation") {
      document.title = `需要确认 | ${base}`
    } else if (runState === "failed") {
      document.title = `运行失败 | ${base}`
    } else {
      document.title = base
    }
  }, [runState])

  // Global keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === "k") {
        e.preventDefault()
        textareaRef.current?.focus()
        return
      }
    }
    document.addEventListener("keydown", handler)
    return () => document.removeEventListener("keydown", handler)
  }, [])

  // SSE event stream
  const sessionId = useRuntimeStore((s) => s.sessionId)
  const { reconnect } = useSessionEventStream(sessionId, {
    onEvent: (event) => {
      applyEvent(event)
    },
    onConnectionChange: (state: StreamConnectionState) => {
      setConnectionState(state as ConnectionState)
    },
    onStreamError: (message) => {
      setCriticalError(message)
      toast.error(message)
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

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (!files) return
    const newFiles: { name: string; content: string }[] = []
    for (const file of Array.from(files)) {
      if (file.size > 5 * 1024 * 1024) continue // skip files > 5MB
      const text = await file.text()
      newFiles.push({ name: file.name, content: text })
      uploadKnowledgeFile(file)
        .then(() => toast.success(`「${file.name}」已保存到知识库`))
        .catch(() => toast.error(`「${file.name}」保存到知识库失败`))
    }
    setAttachedFiles((prev) => [...prev, ...newFiles])
    if (fileInputRef.current) fileInputRef.current.value = ""
  }

  const handleRemoveFile = (index: number) => {
    setAttachedFiles((prev) => prev.filter((_, i) => i !== index))
  }

  const handleSend = async (overrideInput?: string) => {
    const userInput = overrideInput?.trim() || composeValue.trim()
    if (!userInput && attachedFiles.length === 0) return

    let fullContent = userInput
    if (attachedFiles.length > 0) {
      const fileSections = attachedFiles.map((f) => `--- ${f.name} ---\n${f.content}`).join("\n\n")
      fullContent = userInput ? `${userInput}\n\n${fileSections}` : fileSections
    }

    addMessage({ role: "user", content: fullContent })
    setComposeValue("")
    setAttachedFiles([])
    startNewRun(userInput)

    try {
      const payload: SubmitChatRunPayload = {
        sessionId,
        userInput,
        mode: settings.mode,
        model: settings.model,
        workspace: settings.workspace,
        knowledgeBaseId: "",
      }
      const result = await submitChatRun(payload)
      acceptRun(result.session_id, result.run_id)
    } catch (err) {
      const msg = err instanceof Error ? err.message : "提交任务失败"
      failRun(msg)
      toast.error(msg)
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

  const handleNewTask = () => {
    useRuntimeStore.getState().clearSession()
  }

  const handleRetry = async () => {
    const currentRunId = useRuntimeStore.getState().currentRunId
    if (!currentRunId) {
      failRun("没有可重试的任务")
      return
    }
    setRunState("running")
    setCriticalError(null)
    try {
      const result = await submitChatRetry({ session_id: sessionId, run_id: currentRunId })
      acceptRun(result.session_id, result.run_id)
    } catch (err) {
      failRun(err instanceof Error ? err.message : "重试失败")
    }
  }

  const handleCancel = async () => {
    const currentRunId = useRuntimeStore.getState().currentRunId
    if (!currentRunId) {
      cancelRun()
      return
    }
    try {
      await submitChatCancel(sessionId, currentRunId)
      cancelRun()
    } catch (err) {
      const msg = err instanceof Error ? err.message : "取消失败"
      setSubmitError(msg)
      toast.error(msg)
      cancelRun()
    }
  }

  const handleExport = () => {
    const state = useRuntimeStore.getState()
    const lines: string[] = []
    lines.push("# 本地智能体会话导出")
    lines.push("")
    lines.push(`**会话 ID**: ${state.sessionId}`)
    lines.push(`**导出时间**: ${new Date().toLocaleString("zh-CN")}`)
    lines.push(`**消息数**: ${state.messages.length}`)
    lines.push("")
    lines.push("---")
    lines.push("")
    for (const msg of state.messages) {
      const role = msg.role === "user" ? "用户" : "助手"
      const time = formatMessageTime(msg.timestamp)
      lines.push(`## ${role} (${time})`)
      lines.push("")
      lines.push(msg.content)
      lines.push("")
      lines.push("---")
      lines.push("")
    }
    const blob = new Blob([lines.join("\n")], { type: "text/markdown;charset=utf-8" })
    const url = URL.createObjectURL(blob)
    const a = document.createElement("a")
    a.href = url
    a.download = `会话-${state.sessionId.slice(0, 8)}-${new Date().toISOString().slice(0, 10)}.md`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  const hasMessages = messages.length > 0
  const showIdle = !hasMessages && runState === "idle"

  const searchMatches = useMemo(() => {
    if (!searchQuery.trim()) return []
    const q = searchQuery.toLowerCase()
    return messages
      .map((m, idx) => ({ idx, message: m }))
      .filter(({ message }) => message.content.toLowerCase().includes(q))
  }, [messages, searchQuery])

  const totalMatches = searchMatches.length

  useEffect(() => {
    if (totalMatches > 0) {
      const match = searchMatches[currentMatchIndex % totalMatches]
      if (match) {
        const el = messageRefs.current[match.message.id]
        el?.scrollIntoView({ behavior: "smooth", block: "center" })
      }
    }
  }, [currentMatchIndex, totalMatches, searchMatches])

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
                  onClick={() => handleSend(prompt.label)}
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
          {hasMessages && (
            <div className="sticky top-0 z-10 bg-background/95 backdrop-blur-sm border-b border-border px-4 py-2">
              <div className="mx-auto max-w-3xl flex items-center gap-2">
                <Search className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => {
                    setSearchQuery(e.target.value)
                    setCurrentMatchIndex(0)
                  }}
                  placeholder="搜索消息..."
                  className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
                />
                {totalMatches > 0 && (
                  <span className="text-xs text-muted-foreground shrink-0">
                    {(currentMatchIndex % totalMatches) + 1} / {totalMatches}
                  </span>
                )}
                {totalMatches > 0 && (
                  <div className="flex items-center gap-0.5 shrink-0">
                    <button
                      onClick={() => setCurrentMatchIndex((v) => (v - 1 + totalMatches) % totalMatches)}
                      className="p-1 rounded hover:bg-muted text-muted-foreground"
                    >
                      <ChevronUp className="h-3.5 w-3.5" />
                    </button>
                    <button
                      onClick={() => setCurrentMatchIndex((v) => (v + 1) % totalMatches)}
                      className="p-1 rounded hover:bg-muted text-muted-foreground"
                    >
                      <ChevronDown className="h-3.5 w-3.5" />
                    </button>
                  </div>
                )}
                {searchQuery && (
                  <button
                    onClick={() => { setSearchQuery(""); setCurrentMatchIndex(0) }}
                    className="p-1 rounded hover:bg-muted text-muted-foreground shrink-0"
                  >
                    <X className="h-3.5 w-3.5" />
                  </button>
                )}
              </div>
            </div>
          )}
          <div className="p-4">
            <div className="mx-auto max-w-3xl space-y-4">
              {messages.map((message) => (
                <MessageBubble
                  key={message.id}
                  message={message}
                  highlightText={searchQuery}
                  isSearchActive={searchMatches.some((m) => m.message.id === message.id)}
                  isCurrentMatch={searchMatches[currentMatchIndex % Math.max(totalMatches, 1)]?.message.id === message.id}
                  refCallback={(el) => { messageRefs.current[message.id] = el }}
                />
              ))}

              {/* Running State */}
              {runState === "running" && (
                <div className="space-y-2">
                  <RunningCard events={events} />
                  <div className="flex justify-start pl-1">
                    <button
                      onClick={handleCancel}
                      className="text-xs text-muted-foreground hover:text-destructive transition-colors flex items-center gap-1 px-2 py-1 rounded hover:bg-destructive/10"
                    >
                      <XCircle className="h-3 w-3" />
                      取消
                    </button>
                  </div>
                </div>
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
          {connectionState === "disconnected" && (
            <div className="mb-3 flex items-center justify-between rounded-lg border border-warning/30 bg-warning/10 px-3 py-2">
              <div className="flex items-center gap-2 text-sm text-warning">
                <AlertTriangle className="h-4 w-4" />
                事件流连接已断开
              </div>
              <Button variant="ghost" size="sm" className="h-7 text-xs" onClick={reconnect}>
                重连
              </Button>
            </div>
          )}
          {submitError && (
            <div className="mb-3 flex items-center gap-2 text-sm text-destructive">
              <AlertTriangle className="h-4 w-4" />
              {submitError}
            </div>
          )}
          {attachedFiles.length > 0 && (
            <div className="mb-2 flex flex-wrap gap-2">
              {attachedFiles.map((file, idx) => (
                <span key={idx} className="inline-flex items-center gap-1 rounded-md bg-muted px-2 py-1 text-xs">
                  <FileText className="h-3 w-3" />
                  {file.name}
                  <button onClick={() => handleRemoveFile(idx)} className="ml-1 hover:text-destructive">
                    <X className="h-3 w-3" />
                  </button>
                </span>
              ))}
            </div>
          )}
          <div className="flex items-center gap-2 sm:gap-3">
            <Select value="all">
              <SelectTrigger className="w-[100px] sm:w-[140px] h-10">
                <SelectValue placeholder="知识库" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">全部来源</SelectItem>
              </SelectContent>
            </Select>

            <input
              ref={fileInputRef}
              type="file"
              accept=".txt,.md,.pdf,.docx"
              multiple
              className="hidden"
              onChange={handleFileSelect}
            />
            <Button
              variant="outline"
              size="icon"
              className="h-10 w-10 shrink-0"
              onClick={() => fileInputRef.current?.click()}
              title="附加文件"
            >
              <Paperclip className="h-4 w-4" />
            </Button>

            <div className="relative flex-1">
              <TextareaAuto
                ref={textareaRef}
                value={composeValue}
                onChange={(e) => setComposeValue(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault()
                    handleSend()
                  }
                  if (e.key === "Escape") {
                    setComposeValue("")
                  }
                  if (e.key === "ArrowUp" && !composeValue.trim()) {
                    e.preventDefault()
                    const lastUserMsg = messages.filter((m) => m.role === "user").pop()
                    if (lastUserMsg) {
                      setComposeValue(lastUserMsg.content)
                    }
                  }
                }}
                placeholder="描述一个任务... (Shift+Enter 换行)"
                className="min-h-[40px] max-h-[160px] pr-16 py-2.5"
                disabled={runState === "running" || runState === "awaiting_confirmation"}
              />
              <div className="absolute right-3 bottom-2.5 flex items-center gap-2">
                <Kbd className="hidden sm:inline-flex text-xs">Enter</Kbd>
              </div>
            </div>

            <Button
              variant="outline"
              size="icon"
              className="hidden sm:flex h-10 w-10 shrink-0"
              onClick={handleExport}
              title="导出会话"
              disabled={messages.length === 0}
            >
              <Download className="h-4 w-4" />
            </Button>

            <Button
              variant="outline"
              size="icon"
              className="hidden sm:flex h-10 w-10 shrink-0"
              onClick={handleNewTask}
              title="新任务"
            >
              <Plus className="h-4 w-4" />
            </Button>

            <Button
              onClick={() => handleSend()}
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

function formatMessageTime(isoString: string): string {
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
  return date.toLocaleDateString("zh-CN", { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" })
}

function MessageBubble({
  message,
  highlightText,
  isSearchActive,
  isCurrentMatch,
  refCallback,
}: {
  message: Message
  highlightText?: string
  isSearchActive?: boolean
  isCurrentMatch?: boolean
  refCallback?: (el: HTMLDivElement | null) => void
}) {
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    navigator.clipboard.writeText(message.content)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const highlightClass = isCurrentMatch
    ? "ring-2 ring-primary/50 rounded-2xl"
    : isSearchActive
      ? "ring-1 ring-primary/20 rounded-2xl"
      : ""

  if (message.role === "user") {
    return (
      <div ref={refCallback} className={highlightClass}>
        <div className="flex justify-end animate-in fade-in slide-in-from-bottom-2 duration-200">
          <div className="max-w-[92%] sm:max-w-[80%]">
            <div className="rounded-2xl rounded-tr-md bg-secondary px-4 py-3">
              <HighlightedText content={message.content} highlight={highlightText} />
            </div>
            <div className="flex justify-end items-center gap-2 mt-1 pr-1">
              <p className="text-[10px] text-muted-foreground">{formatMessageTime(message.timestamp)}</p>
              <button
                onClick={() => useRuntimeStore.getState().editAndResend(message.id)}
                className="text-[10px] text-muted-foreground hover:text-foreground transition-colors"
              >
                编辑
              </button>
            </div>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div ref={refCallback} className={highlightClass}>
      <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
        <div className="max-w-[92%] sm:max-w-[80%]">
          <div className="rounded-2xl rounded-tl-md border border-border bg-card p-4">
            <LightweightMarkdown content={message.content} highlightText={highlightText} />
            <DiffPreview content={message.content} />
            {message.isStreaming && (
              <span className="inline-block w-1.5 h-3 ml-0.5 bg-primary animate-pulse rounded-sm" />
            )}
            {message.blocks?.map((block, index) => (
              <ResultBlockRenderer key={index} block={block} />
            ))}
            <div className="flex justify-end mt-2">
              <button
                onClick={handleCopy}
                className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors px-2 py-1 rounded hover:bg-muted"
              >
                {copied ? (
                  <>
                    <Check className="h-3 w-3 text-success" />
                    已复制
                  </>
                ) : (
                  <>
                    <Copy className="h-3 w-3" />
                    复制
                  </>
                )}
              </button>
            </div>
          </div>
          <p className="text-[10px] text-muted-foreground mt-1 pl-1">{formatMessageTime(message.timestamp)}</p>
        </div>
      </div>
    </div>
  )
}

function HighlightedText({ content, highlight }: { content: string; highlight?: string }) {
  if (!highlight?.trim()) {
    return <p className="text-sm text-foreground whitespace-pre-wrap">{content}</p>
  }
  const q = highlight.toLowerCase()
  const parts: React.ReactNode[] = []
  let remaining = content
  let key = 0
  while (remaining.length > 0) {
    const idx = remaining.toLowerCase().indexOf(q)
    if (idx < 0) {
      parts.push(<span key={key++}>{remaining}</span>)
      break
    }
    if (idx > 0) {
      parts.push(<span key={key++}>{remaining.slice(0, idx)}</span>)
    }
    parts.push(
      <mark key={key++} className="bg-primary/20 text-foreground rounded px-0.5">
        {remaining.slice(idx, idx + highlight.length)}
      </mark>
    )
    remaining = remaining.slice(idx + highlight.length)
  }
  return <p className="text-sm text-foreground whitespace-pre-wrap">{parts}</p>
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
  const stage = deriveStage(events)
  const recent = events.slice(-5)
  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[92%] sm:max-w-[80%] rounded-2xl rounded-tl-md border border-primary/20 bg-card p-4">
        <div className="flex items-center gap-2 mb-3">
          <div className="relative">
            <Loader2 className="h-4 w-4 animate-spin text-primary" />
            <div className="absolute inset-0 h-4 w-4 animate-ping rounded-full bg-primary/20" />
          </div>
          <span className="text-sm font-medium text-foreground">{stage}</span>
        </div>
        <div className="space-y-2">
          {recent.map((event) => (
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

function deriveStage(events: RuntimeEvent[]): string {
  if (events.length === 0) return "运行中..."
  const last = events[events.length - 1]
  const map: Record<string, string> = {
    run_started: "已收到",
    action_completed: "正在执行",
    verification_completed: "正在验证",
    memory_written: "正在写入记忆",
    knowledge_written: "正在写入知识库",
    checkpoint_written: "正在保存检查点",
    run_finished: "已完成",
    run_failed: "运行失败",
  }
  return map[last.event_type] || last.summary || "运行中..."
}

function ErrorCard({ error, onRetry }: { error: string; onRetry: () => void }) {
  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[92%] sm:max-w-[80%] rounded-2xl rounded-tl-md border-l-4 border-l-destructive border border-border bg-card p-4">
        <div className="flex items-center gap-2 mb-2">
          <XCircle className="h-4 w-4 text-destructive" />
          <span className="text-sm font-medium text-destructive">错误</span>
        </div>
        <p className="text-sm text-muted-foreground mb-3">{error}</p>
        <Button variant="outline" size="sm" onClick={onRetry} className="gap-2">
          <RefreshCw className="h-3 w-3" />
          重试
        </Button>
      </div>
    </div>
  )
}

const riskColors = {
  low: "text-success",
  medium: "text-warning",
  high: "text-destructive",
  critical: "text-destructive",
  irreversible: "text-destructive",
}

const riskLabels: Record<string, string> = {
  low: "低风险",
  medium: "中风险",
  high: "高风险",
  critical: "极高风险",
}

type PatchToolArguments = {
  diff: string
  dry_run: boolean
}

export function ConfirmationCard({
  confirmation,
  onDecision,
}: {
  confirmation: Confirmation
  onDecision: (decision: "approve" | "deny", remember: boolean) => void
}) {
  const [remember, setRemember] = useState(false)
  const patchMode = isPatchConfirmation(confirmation)

  return (
    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-2 duration-200">
      <div className="max-w-[90%] rounded-2xl rounded-tl-md border-l-4 border-l-warning border border-border bg-card p-4">
        <ConfirmationHeader riskLevel={confirmation.risk_level} />
        <ConfirmationSummary confirmation={confirmation} />
        <PatchConfirmationPreview confirmation={confirmation} />
        <ConfirmationActions
          remember={remember}
          patchMode={patchMode}
          onApprove={() => onDecision("approve", remember)}
          onDeny={() => onDecision("deny", remember)}
          onRememberChange={(checked) => setRemember(checked)}
        />
      </div>
    </div>
  )
}

function ConfirmationHeader({ riskLevel }: { riskLevel: Confirmation["risk_level"] }) {
  return (
    <div className="mb-3 flex items-center gap-2">
      <AlertTriangle className="h-4 w-4 text-warning" />
      <span className="text-sm font-medium text-warning">需要确认</span>
      <span className={cn("text-xs font-medium uppercase", riskColors[riskLevel])}>
        {riskLabels[riskLevel] || riskLevel}
      </span>
    </div>
  )
}

function ConfirmationSummary({ confirmation }: { confirmation: Confirmation }) {
  return (
    <>
      <p className="mb-2 text-sm font-medium text-foreground">{confirmation.action_summary}</p>
      <p className="mb-3 text-sm text-muted-foreground">{confirmation.reason}</p>
      <ConfirmationPathList paths={confirmation.target_paths} />
      <ConfirmationHazards hazards={confirmation.hazards} />
    </>
  )
}

function ConfirmationPathList({ paths }: { paths: string[] }) {
  if (paths.length === 0) return null
  return (
    <div className="mb-3">
      <p className="mb-1 text-xs font-medium text-muted-foreground">影响路径：</p>
      <div className="flex flex-wrap gap-1">
        {paths.map((path) => <code key={path} className="rounded bg-muted px-1.5 py-0.5 text-xs text-foreground">{path}</code>)}
      </div>
    </div>
  )
}

function ConfirmationHazards({ hazards }: { hazards: string[] }) {
  if (hazards.length === 0) return null
  return (
    <div className="mb-3">
      <p className="mb-1 text-xs font-medium text-muted-foreground">潜在风险：</p>
      <ul className="space-y-0.5">
        {hazards.map((hazard) => <li key={hazard} className="flex items-center gap-1 text-xs text-destructive"><span className="h-1 w-1 rounded-full bg-destructive" />{hazard}</li>)}
      </ul>
    </div>
  )
}

function PatchConfirmationPreview({ confirmation }: { confirmation: Confirmation }) {
  if (!isPatchConfirmation(confirmation)) return null
  const preview = confirmation.patch_preview_report_json?.trim() || ""
  if (preview) return <DiffPreview content={preview} />
  const args = parsePatchToolArguments(confirmation.tool_arguments_json)
  if (!args) return null
  return <PatchArgumentsPreview diff={args.diff} dry_run={args.dry_run} />
}

function PatchArgumentsPreview({ diff, dry_run }: PatchToolArguments) {
  const report = inferDiffPreviewReport(diff)
  if (report) {
    return (
      <DiffPreviewReportCard
        report={report}
        title="Patch 参数预览"
        subtitle="未收到 dry-run 报告，以下根据 patch 参数生成只读摘要。"
        badgeLabel={`dry-run: ${dry_run ? "是" : "否"}`}
      />
    )
  }
  return (
    <div className="mb-3 rounded-lg border border-border bg-muted/20 p-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div>
          <p className="text-sm font-medium text-foreground">Patch 参数预览</p>
          <p className="text-xs text-muted-foreground">未收到 dry-run 报告，以下展示待确认的 patch 参数原文。</p>
        </div>
        <span className="text-xs text-muted-foreground">dry-run: {dry_run ? "是" : "否"}</span>
      </div>
      <pre className="mt-3 max-h-56 overflow-auto rounded-md border border-border bg-background px-3 py-2 text-xs text-foreground">{diff}</pre>
    </div>
  )
}

function ConfirmationActions({
  remember,
  patchMode,
  onApprove,
  onDeny,
  onRememberChange,
}: {
  remember: boolean
  patchMode: boolean
  onApprove: () => void
  onDeny: () => void
  onRememberChange: (checked: boolean) => void
}) {
  const primaryLabel = patchMode ? "确认应用" : "批准"
  const secondaryLabel = patchMode ? "取消应用" : "拒绝"
  return (
    <div className="pt-2">
      <PatchApplyHint patchMode={patchMode} />
      <div className="flex items-center gap-3">
        <Button size="sm" onClick={onApprove} className="bg-success text-success-foreground hover:bg-success/90"><CheckCircle className="mr-1.5 h-3.5 w-3.5" />{primaryLabel}</Button>
        <Button variant="outline" size="sm" onClick={onDeny}><XCircle className="mr-1.5 h-3.5 w-3.5" />{secondaryLabel}</Button>
        <div className="ml-auto flex items-center gap-2">
          <Checkbox id="remember" checked={remember} onCheckedChange={(checked) => onRememberChange(checked as boolean)} />
          <label htmlFor="remember" className="cursor-pointer text-xs text-muted-foreground">记住此选择</label>
        </div>
      </div>
    </div>
  )
}

function PatchApplyHint({ patchMode }: { patchMode: boolean }) {
  if (!patchMode) return null
  return <p className="mb-3 text-xs text-muted-foreground">确认应用后才会写入文件，取消应用不会修改工作区。</p>
}

function isPatchConfirmation(confirmation: Confirmation): boolean {
  return confirmation.tool_name === "workspace_apply_patch"
}

function parsePatchToolArguments(raw?: string): PatchToolArguments | null {
  if (!raw?.trim()) return null
  try {
    const value = JSON.parse(raw)
    if (!isRecord(value) || typeof value.diff !== "string" || !value.diff.trim()) return null
    return { diff: value.diff, dry_run: value.dry_run === true }
  } catch {
    return null
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value && typeof value === "object" && !Array.isArray(value))
}
