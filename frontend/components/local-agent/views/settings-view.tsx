"use client"

import { useState, useEffect, useRef, forwardRef } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"

import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import {
  ChevronDown,
  ChevronRight,
  Server,
  Cpu,
  Key,
  Database,
  FolderOpen,
  Shield,
  Brain,
  Wrench,
  Trash2,
  Play,
  Save,
  Download,
  CheckCircle,
  X,
  Plus,
  Loader2,
  Eye,
  EyeOff,
} from "lucide-react"
import { useSettingsStore, useRuntimeStore, useMemoryStore } from "@/lib/local-agent/store"
import type { AgentMode } from "@/lib/local-agent/types"
import { runDiagnosticsCheck } from "@/lib/local-agent/api"
import { cn } from "@/lib/utils"

const settingsModules = [
  { id: "runtime", label: "运行环境", icon: Server },
  { id: "model", label: "模型", icon: Cpu },
  { id: "providers", label: "服务商", icon: Key },
  { id: "embedding", label: "嵌入", icon: Database },
  { id: "workspaces", label: "工作区", icon: FolderOpen },
  { id: "risk", label: "风险", icon: Shield },
  { id: "memory", label: "记忆", icon: Brain },
  { id: "diagnostics", label: "诊断", icon: Wrench },
]

export function SettingsView() {
  const [activeSection, setActiveSection] = useState("runtime")
  const sectionRefs = useRef<Record<string, HTMLDivElement | null>>({})
  const { loadSettings } = useSettingsStore()

  useEffect(() => {
    loadSettings()
  }, [loadSettings])

  useEffect(() => {
    const handleScroll = (e: Event) => {
      const container = e.target as HTMLElement
      const scrollTop = container.scrollTop

      for (const module of settingsModules) {
        const ref = sectionRefs.current[module.id]
        if (ref && ref.offsetTop - 100 <= scrollTop) {
          setActiveSection(module.id)
        }
      }
    }

    const scrollContainer = document.getElementById("settings-scroll")
    scrollContainer?.addEventListener("scroll", handleScroll)
    return () => scrollContainer?.removeEventListener("scroll", handleScroll)
  }, [])

  const scrollToSection = (id: string) => {
    const ref = sectionRefs.current[id]
    if (ref) {
      ref.scrollIntoView({ behavior: "smooth", block: "start" })
    }
  }

  return (
    <div className="flex h-full">
      {/* Left Sticky Nav */}
      <nav className="w-48 shrink-0 border-r border-border p-4 hidden lg:block">
        <ul className="space-y-1">
          {settingsModules.map((module) => {
            const Icon = module.icon
            return (
              <li key={module.id}>
                <button
                  onClick={() => scrollToSection(module.id)}
                  className={cn(
                    "w-full flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors duration-200",
                    activeSection === module.id
                      ? "bg-primary/10 text-primary"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground"
                  )}
                >
                  <Icon className="h-4 w-4" />
                  {module.label}
                </button>
              </li>
            )
          })}
        </ul>
      </nav>

      {/* Right Scrollable Content */}
      <ScrollArea id="settings-scroll" className="flex-1">
        <div className="max-w-3xl mx-auto p-6 space-y-6">
          <RuntimeSection ref={(el) => { sectionRefs.current["runtime"] = el }} />
          <ModelSection ref={(el) => { sectionRefs.current["model"] = el }} />
          <ProvidersSection ref={(el) => { sectionRefs.current["providers"] = el }} />
          <EmbeddingSection ref={(el) => { sectionRefs.current["embedding"] = el }} />
          <WorkspacesSection ref={(el) => { sectionRefs.current["workspaces"] = el }} />
          <RiskSection ref={(el) => { sectionRefs.current["risk"] = el }} />
          <MemorySection ref={(el) => { sectionRefs.current["memory"] = el }} />
          <DiagnosticsSection ref={(el) => { sectionRefs.current["diagnostics"] = el }} />
        </div>
      </ScrollArea>
    </div>
  )
}

// Runtime Section
const RuntimeSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { connectionState } = useRuntimeStore()

  const connections = [
    { name: "智能体服务", port: "8080", status: connectionState },
    { name: "数据库", port: "5432", status: "connected" as const },
  ]

  return (
    <SettingsSection
      ref={ref}
      id="runtime"
      title="运行环境"
      description="连接状态和服务器配置"
      icon={Server}
    >
      <div className="grid gap-4 sm:grid-cols-2">
        {connections.map((conn) => (
          <Card key={conn.name}>
            <CardContent className="pt-6">
              <div className="flex items-center gap-3">
                <div className={cn(
                  "h-3 w-3 rounded-full",
                  conn.status === "connected" ? "bg-success animate-pulse" : "bg-destructive"
                )} />
                <div>
                  <p className="text-sm font-medium text-foreground">{conn.name}</p>
                  <p className="text-xs text-muted-foreground font-mono">localhost:{conn.port}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </SettingsSection>
  )
})
RuntimeSection.displayName = "RuntimeSection"

// Model Section
const ModelSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { mode, model, setMode, setModel } = useSettingsStore()

  const models = [
    { id: "gpt-4o", name: "GPT-4o", provider: "openai" },
    { id: "gpt-4-turbo", name: "GPT-4 Turbo", provider: "openai" },
    { id: "claude-3-opus", name: "Claude 3 Opus", provider: "anthropic" },
    { id: "llama-3-70b", name: "Llama 3 70B", provider: "meta" },
  ]

  const modeDescriptions: Record<AgentMode, string> = {
    observer: "只读访问，不允许修改",
    standard: "平衡访问，风险操作需确认",
    full_access: "完全访问所有系统功能",
  }

  return (
    <SettingsSection
      ref={ref}
      id="model"
      title="模型"
      description="AI 模型选择和访问模式"
      icon={Cpu}
    >
      <div className="space-y-6">
        <div className="space-y-2">
          <Label>模型</Label>
          <Select
            value={model.model_id}
            onValueChange={(value) => {
              const selected = models.find((m) => m.id === value)
              if (selected) {
                setModel({
                  model_id: selected.id,
                  display_name: selected.name,
                  provider_id: selected.provider,
                })
              }
            }}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {models.map((m) => (
                <SelectItem key={m.id} value={m.id}>
                  {m.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-3">
          <Label>访问模式</Label>
          <RadioGroup
            value={mode}
            onValueChange={(value) => setMode(value as AgentMode)}
            className="space-y-2"
          >
            {(["observer", "standard", "full_access"] as AgentMode[]).map((m) => (
              <div
                key={m}
                className={cn(
                  "flex items-center space-x-3 rounded-lg border p-3 transition-colors cursor-pointer",
                  mode === m ? "border-primary bg-primary/5" : "border-border hover:bg-muted/50"
                )}
              >
                <RadioGroupItem value={m} id={m} />
                <div className="flex-1">
                  <Label htmlFor={m} className="font-medium cursor-pointer capitalize">
                    {m.replace("_", " ")}
                  </Label>
                  <p className="text-xs text-muted-foreground">{modeDescriptions[m]}</p>
                </div>
              </div>
            ))}
          </RadioGroup>
        </div>
      </div>
    </SettingsSection>
  )
})
ModelSection.displayName = "ModelSection"

// Providers Section
const ProvidersSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { providers, updateProvider, removeProvider } = useSettingsStore()
  const [testingId, setTestingId] = useState<string | null>(null)
  const [showKeyId, setShowKeyId] = useState<string | null>(null)

  const handleTest = (providerId: string) => {
    setTestingId(providerId)
    setTimeout(() => {
      updateProvider(providerId, { status: "active" })
      setTestingId(null)
    }, 1500)
  }

  return (
    <SettingsSection
      ref={ref}
      id="providers"
      title="服务商"
      description="API 密钥和服务商"
      icon={Key}
    >
      <div className="space-y-3">
        {providers.map((provider) => (
          <div
            key={provider.provider_id}
            className="flex items-center justify-between rounded-lg border border-border p-4"
          >
            <div className="flex items-center gap-3">
              <div
                className={cn(
                  "h-2.5 w-2.5 rounded-full",
                  provider.status === "active" ? "bg-success" : "bg-muted-foreground"
                )}
              />
              <div>
                <p className="text-sm font-medium text-foreground">{provider.display_name}</p>
                <div className="flex items-center gap-2">
                  <p className="text-xs text-muted-foreground font-mono">
                    {showKeyId === provider.provider_id && provider.api_key
                      ? provider.api_key
                      : "••••••••••••••••"}
                  </p>
                  {provider.api_key && (
                    <button
                      onClick={() => setShowKeyId(
                        showKeyId === provider.provider_id ? null : provider.provider_id
                      )}
                      className="text-muted-foreground hover:text-foreground"
                    >
                      {showKeyId === provider.provider_id ? (
                        <EyeOff className="h-3 w-3" />
                      ) : (
                        <Eye className="h-3 w-3" />
                      )}
                    </button>
                  )}
                </div>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                className="gap-1"
                onClick={() => handleTest(provider.provider_id)}
                disabled={testingId === provider.provider_id}
              >
                {testingId === provider.provider_id ? (
                  <Loader2 className="h-3 w-3 animate-spin" />
                ) : (
                  <Play className="h-3 w-3" />
                )}
                测试
              </Button>
              <Button variant="outline" size="sm" className="gap-1">
                <Save className="h-3 w-3" />
                保存
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 text-destructive hover:text-destructive"
                onClick={() => removeProvider(provider.provider_id)}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        ))}

        <Button variant="outline" className="w-full gap-2 mt-2">
          <Plus className="h-4 w-4" />
          添加服务商
        </Button>
      </div>
    </SettingsSection>
  )
})
ProvidersSection.displayName = "ProvidersSection"

// Embedding Section
const EmbeddingSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { embedding_provider_id, setEmbeddingProvider } = useSettingsStore()

  return (
    <SettingsSection
      ref={ref}
      id="embedding"
      title="嵌入"
      description="向量嵌入模型配置"
      icon={Database}
    >
      <div className="space-y-2">
        <Label>Embedding Provider</Label>
        <Select value={embedding_provider_id} onValueChange={setEmbeddingProvider}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="openai">OpenAI Embeddings</SelectItem>
            <SelectItem value="cohere">Cohere Embed</SelectItem>
            <SelectItem value="local">Local (sentence-transformers)</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </SettingsSection>
  )
})
EmbeddingSection.displayName = "EmbeddingSection"

// Workspaces Section
const WorkspacesSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { workspace } = useSettingsStore()

  const authorizedDirs = ["/tmp", "/var/data", "/home/user/downloads"]

  return (
    <SettingsSection
      ref={ref}
      id="workspaces"
      title="工作区"
      description="授权目录和工作区路径"
      icon={FolderOpen}
    >
      <div className="space-y-4">
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-foreground">{workspace.name}</p>
                <p className="text-xs text-muted-foreground font-mono">{workspace.root_path}</p>
              </div>
              <Badge variant="secondary" className="text-xs">当前</Badge>
            </div>
          </CardContent>
        </Card>

        <div className="pt-2">
          <p className="text-xs font-medium text-muted-foreground mb-2 uppercase tracking-wider">
            授权目录
          </p>
          <div className="space-y-2">
            {authorizedDirs.map((dir) => (
              <div
                key={dir}
                className="flex items-center justify-between rounded-lg bg-muted px-3 py-2"
              >
                <span className="text-sm font-mono text-foreground">{dir}</span>
                <Button variant="ghost" size="icon" className="h-6 w-6 text-destructive hover:text-destructive">
                  <X className="h-3 w-3" />
                </Button>
              </div>
            ))}
          </div>
          <Button variant="outline" className="w-full gap-2 mt-3" size="sm">
            <Plus className="h-4 w-4" />
            添加目录
          </Button>
        </div>
      </div>
    </SettingsSection>
  )
})
WorkspacesSection.displayName = "WorkspacesSection"

// Risk Section
const RiskSection = forwardRef<HTMLDivElement>((_, ref) => {
  const [settings, setSettings] = useState({
    confirmDelete: true,
    networkAccess: true,
    codeExecution: false,
    sandbox: true,
  })

  const riskSettings = [
    {
      id: "confirmDelete",
      label: "删除文件前确认",
      description: "删除文件前需要审批",
    },
    {
      id: "networkAccess",
      label: "允许网络访问",
      description: "启用外部 API 调用和网络请求",
    },
    {
      id: "codeExecution",
      label: "允许代码执行",
      description: "启用脚本和代码片段运行",
    },
    {
      id: "sandbox",
      label: "沙盒模式",
      description: "在隔离环境中运行所有操作",
    },
  ] as const

  return (
    <SettingsSection
      ref={ref}
      id="risk"
      title="风险"
      description="安全和风险管理设置"
      icon={Shield}
    >
      <div className="space-y-4">
        {riskSettings.map((setting) => (
          <div
            key={setting.id}
            className="flex items-center justify-between rounded-lg border border-border p-4"
          >
            <div>
              <Label htmlFor={setting.id} className="cursor-pointer">
                {setting.label}
              </Label>
              <p className="text-xs text-muted-foreground">{setting.description}</p>
            </div>
            <Switch
              id={setting.id}
              checked={settings[setting.id]}
              onCheckedChange={(checked) =>
                setSettings((prev) => ({ ...prev, [setting.id]: checked }))
              }
            />
          </div>
        ))}
      </div>
    </SettingsSection>
  )
})
RiskSection.displayName = "RiskSection"

// Memory Section
const MemorySection = forwardRef<HTMLDivElement>((_, ref) => {
  const { memories, loadMemories, removeMemory } = useMemoryStore()

  useEffect(() => {
    loadMemories()
  }, [loadMemories])

  return (
    <SettingsSection
      ref={ref}
      id="memory"
      title="记忆"
      description="智能体记忆和上下文存储"
      icon={Brain}
    >
      <div className="space-y-2">
        {memories.map((memory) => (
          <Collapsible key={memory.id}>
            <CollapsibleTrigger className="flex w-full items-center justify-between rounded-lg border border-border p-3 hover:bg-muted transition-colors [&[data-state=open]>svg]:rotate-90">
              <div className="flex items-center gap-2">
                <Badge variant="outline" className="text-xs capitalize">
                  {memory.kind}
                </Badge>
                <span className="text-sm font-medium text-foreground line-clamp-1">
                  {memory.title.slice(0, 40) || memory.content.slice(0, 40)}...
                </span>
              </div>
              <ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200" />
            </CollapsibleTrigger>
            <CollapsibleContent className="px-3 py-2">
              <div className="flex items-start justify-between gap-4">
                <div>
                  <p className="text-sm text-muted-foreground">{memory.content}</p>
                  <p className="text-xs text-muted-foreground mt-1">
                    创建于: {new Date(memory.createdAt).toLocaleDateString("zh-CN")}
                  </p>
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-6 w-6 shrink-0 text-destructive hover:text-destructive"
                  onClick={() => removeMemory(memory.id)}
                >
                  <Trash2 className="h-3 w-3" />
                </Button>
              </div>
            </CollapsibleContent>
          </Collapsible>
        ))}
      </div>
    </SettingsSection>
  )
})
MemorySection.displayName = "MemorySection"

// Diagnostics Section
const DiagnosticsSection = forwardRef<HTMLDivElement>((_, ref) => {
  const [isRunning, setIsRunning] = useState(false)
  const [results, setResults] = useState<{ category: string; status: "ok" | "error"; message: string }[]>([])

  const runHealthCheck = async () => {
    setIsRunning(true)
    setResults([])

    try {
      const data = await runDiagnosticsCheck()
      const checks = [
        { category: "仓库", status: data.diagnostics.repo_root_exists ? "ok" : "error", message: data.diagnostics.repo_root_exists ? "可访问" : "不可访问" },
        { category: "存储", status: data.diagnostics.storage_root_exists ? "ok" : "error", message: data.diagnostics.storage_root_exists ? "可访问" : "不可访问" },
        { category: "Runtime", status: data.diagnostics.runtime_reachable ? "ok" : "error", message: data.diagnostics.runtime_reachable ? `可达 (${data.diagnostics.runtime_version})` : "不可达" },
        { category: "服务商", status: data.diagnostics.provider_count > 0 ? "ok" : "error", message: `${data.diagnostics.provider_count} 个已配置` },
        { category: "模型", status: data.diagnostics.model_count > 0 ? "ok" : "error", message: `${data.diagnostics.model_count} 个可用` },
        { category: "工作区", status: data.diagnostics.workspace_count > 0 ? "ok" : "error", message: `${data.diagnostics.workspace_count} 个已配置` },
      ]
      if (data.warnings.length > 0) {
        checks.push({ category: "警告", status: "error", message: data.warnings.join("；") })
      }
      if (data.errors.length > 0) {
        checks.push({ category: "错误", status: "error", message: data.errors.join("；") })
      }
      setResults(checks)
    } catch {
      setResults([{ category: "检查", status: "error", message: "健康检查请求失败" }])
    } finally {
      setIsRunning(false)
    }
  }

  return (
    <SettingsSection
      ref={ref}
      id="diagnostics"
      title="诊断"
      description="系统诊断和数据导出"
      icon={Wrench}
    >
      <div className="space-y-4">
        <div className="flex flex-wrap gap-3">
          <Button variant="outline" className="gap-2">
            <Download className="h-4 w-4" />
            导出日志
          </Button>
          <Button variant="outline" className="gap-2">
            <Download className="h-4 w-4" />
            导出配置
          </Button>
          <Button
            variant="outline"
            className="gap-2"
            onClick={runHealthCheck}
            disabled={isRunning}
          >
            {isRunning ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <CheckCircle className="h-4 w-4" />
            )}
            健康检查
          </Button>
        </div>

        {results.length > 0 && (
          <div className="rounded-lg border border-border p-4 space-y-2">
            {results.map((result) => (
              <div key={result.category} className="flex items-center gap-2">
                <CheckCircle className={cn(
                  "h-4 w-4",
                  result.status === "ok" ? "text-success" : "text-destructive"
                )} />
                <span className="text-sm font-medium text-foreground">{result.category}:</span>
                <span className="text-sm text-muted-foreground">{result.message}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </SettingsSection>
  )
})
DiagnosticsSection.displayName = "DiagnosticsSection"

// Settings Section Component
interface SettingsSectionProps {
  id: string
  title: string
  description: string
  icon: React.ElementType
  children: React.ReactNode
}

const SettingsSection = forwardRef<HTMLDivElement, SettingsSectionProps>(
  ({ id, title, description, icon: Icon, children }, ref) => {
    const [isOpen, setIsOpen] = useState(true)

    return (
      <div ref={ref} id={id} className="scroll-mt-6">
        <Collapsible open={isOpen} onOpenChange={setIsOpen}>
          <CollapsibleTrigger className="flex w-full items-center justify-between rounded-xl border border-border bg-card p-4 hover:bg-muted/50 transition-colors duration-200">
            <div className="flex items-center gap-3">
              <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
                <Icon className="h-5 w-5 text-primary" />
              </div>
              <div className="text-left">
                <h3 className="text-sm font-semibold text-foreground">{title}</h3>
                <p className="text-xs text-muted-foreground">{description}</p>
              </div>
            </div>
            {isOpen ? (
              <ChevronDown className="h-5 w-5 text-muted-foreground" />
            ) : (
              <ChevronRight className="h-5 w-5 text-muted-foreground" />
            )}
          </CollapsibleTrigger>
          <CollapsibleContent className="pt-4 animate-in fade-in slide-in-from-top-2 duration-200">
            {children}
          </CollapsibleContent>
        </Collapsible>
      </div>
    )
  }
)
SettingsSection.displayName = "SettingsSection"
