"use client"

import { forwardRef, useEffect, useRef, useState } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { Brain, CheckCircle, ChevronDown, ChevronRight, Cpu, Database, Eye, EyeOff, FolderOpen, Key, Loader2, Play, Save, Server, Shield, Trash2, Wrench } from "lucide-react"
import { MCPObservabilityPanel, MCPToolBadges } from "@/components/local-agent/views/mcp-observability-panel"
import { useMemoryStore, useSettingsStore } from "@/lib/local-agent/store"
import type { AgentMode, DirectoryApproval, MCPAuditRecord, Model, Provider, Workspace } from "@/lib/local-agent/types"
import { applyProvider, fetchMCPAudits, removeProviderCredential, runDiagnosticsCheck, saveProvider, testProvider, updateSettings } from "@/lib/local-agent/api"
import { cn } from "@/lib/utils"

const settingsModules = [
  { id: "runtime", label: "运行环境", icon: Server },
  { id: "model", label: "模型", icon: Cpu },
  { id: "providers", label: "服务商", icon: Key },
  { id: "embedding", label: "嵌入", icon: Database },
  { id: "workspaces", label: "工作区", icon: FolderOpen },
  { id: "mcp", label: "MCP", icon: Wrench },
  { id: "risk", label: "风险", icon: Shield },
  { id: "memory", label: "记忆", icon: Brain },
  { id: "diagnostics", label: "诊断", icon: Wrench },
]

export function SettingsView() {
  const [activeSection, setActiveSection] = useState("runtime")
  const sectionRefs = useRef<Record<string, HTMLDivElement | null>>({})
  const { loadSettings } = useSettingsStore()

  useEffect(() => { loadSettings() }, [loadSettings])
  useSettingsScrollSpy(sectionRefs, setActiveSection)

  return (
    <div className="flex h-full">
      <SettingsNav activeSection={activeSection} refs={sectionRefs} />
      <ScrollArea id="settings-scroll" className="flex-1">
        <div className="max-w-3xl mx-auto p-6 space-y-6">
          <RuntimeSection ref={(el) => { sectionRefs.current.runtime = el }} />
          <ModelSection ref={(el) => { sectionRefs.current.model = el }} />
          <ProvidersSection ref={(el) => { sectionRefs.current.providers = el }} />
          <EmbeddingSection ref={(el) => { sectionRefs.current.embedding = el }} />
          <WorkspacesSection ref={(el) => { sectionRefs.current.workspaces = el }} />
          <MCPSection ref={(el) => { sectionRefs.current.mcp = el }} />
          <RiskSection ref={(el) => { sectionRefs.current.risk = el }} />
          <MemorySection ref={(el) => { sectionRefs.current.memory = el }} />
          <DiagnosticsSection ref={(el) => { sectionRefs.current.diagnostics = el }} />
        </div>
      </ScrollArea>
    </div>
  )
}

function useSettingsScrollSpy(refs: React.MutableRefObject<Record<string, HTMLDivElement | null>>, setActive: (id: string) => void) {
  useEffect(() => {
    const handleScroll = (e: Event) => {
      const top = (e.target as HTMLElement).scrollTop
      settingsModules.forEach((m) => {
        if ((refs.current[m.id]?.offsetTop || 0) - 100 <= top) setActive(m.id)
      })
    }
    const el = document.getElementById("settings-scroll")
    el?.addEventListener("scroll", handleScroll)
    return () => el?.removeEventListener("scroll", handleScroll)
  }, [refs, setActive])
}

function SettingsNav({ activeSection, refs }: { activeSection: string; refs: React.MutableRefObject<Record<string, HTMLDivElement | null>> }) {
  return (
    <nav className="w-48 shrink-0 border-r border-border p-4 hidden lg:block">
      <ul className="space-y-1">
        {settingsModules.map((module) => <SettingsNavItem key={module.id} active={activeSection === module.id} module={module} refs={refs} />)}
      </ul>
    </nav>
  )
}

function SettingsNavItem({ active, module, refs }: { active: boolean; module: (typeof settingsModules)[number]; refs: React.MutableRefObject<Record<string, HTMLDivElement | null>> }) {
  const Icon = module.icon
  return (
    <li>
      <button onClick={() => refs.current[module.id]?.scrollIntoView({ behavior: "smooth", block: "start" })} className={cn("w-full flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors", active ? "bg-primary/10 text-primary" : "text-muted-foreground hover:bg-muted hover:text-foreground")}>
        <Icon className="h-4 w-4" />
        {module.label}
      </button>
    </li>
  )
}

const RuntimeSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { ports, runtime_status } = useSettingsStore()
  const runtimeState = runtime_status?.ok ? "connected" : "disconnected"
  return (
    <SettingsSection ref={ref} id="runtime" title="运行环境" description="连接状态和服务器配置" icon={Server}>
      <div className="grid gap-4 sm:grid-cols-2">
        <ConnectionCard name="网关服务" port={ports.gateway} status="connected" />
        <ConnectionCard name={runtime_status?.name || "Runtime"} port={ports.runtime} status={runtimeState} detail={runtime_status?.version} />
      </div>
    </SettingsSection>
  )
})
RuntimeSection.displayName = "RuntimeSection"

function ConnectionCard({ name, port, status, detail }: { name: string; port?: number; status: string; detail?: string }) {
  return (
    <Card><CardContent className="pt-6"><div className="flex items-center gap-3">
      <div className={cn("h-3 w-3 rounded-full", status === "connected" ? "bg-success animate-pulse" : "bg-destructive")} />
      <div><p className="text-sm font-medium text-foreground">{name}</p>
        <p className="text-xs text-muted-foreground font-mono">localhost:{port || "-"}</p>
        {detail && <p className="text-xs text-muted-foreground">{detail}</p>}
      </div>
    </div></CardContent></Card>
  )
}

const ModelSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { mode, model, available_models, setMode, setModel } = useSettingsStore()
  return (
    <SettingsSection ref={ref} id="model" title="模型" description="AI 模型选择和访问模式" icon={Cpu}>
      <div className="space-y-6">
        <ModelSelect modelId={model.model_id} models={available_models} onSelect={setModel} />
        <ModeSelect mode={mode} onMode={setMode} />
      </div>
    </SettingsSection>
  )
})
ModelSection.displayName = "ModelSection"

function ModelSelect({ modelId, models, onSelect }: { modelId: string; models: Model[]; onSelect: (model: Model) => void }) {
  return (
    <div className="space-y-2"><Label>模型</Label><Select value={modelId} onValueChange={(v) => selectModel(v, models, onSelect)}>
      <SelectTrigger><SelectValue placeholder="选择模型" /></SelectTrigger>
      <SelectContent>{models.map((m) => <SelectItem key={m.model_id} value={m.model_id}>{m.display_name}</SelectItem>)}</SelectContent>
    </Select></div>
  )
}

function selectModel(value: string, models: Model[], onSelect: (model: Model) => void) {
  const selected = models.find((m) => m.model_id === value)
  if (selected) onSelect(selected)
}

function ModeSelect({ mode, onMode }: { mode: AgentMode; onMode: (mode: AgentMode) => void }) {
  const descriptions = { observe: "只读访问，不允许修改", standard: "平衡访问，风险操作需确认", full_access: "完全访问所有系统功能" }
  return (
    <div className="space-y-3"><Label>访问模式</Label><RadioGroup value={mode} onValueChange={(v) => onMode(v as AgentMode)} className="space-y-2">
      {(["observe", "standard", "full_access"] as AgentMode[]).map((m) => <ModeOption key={m} active={mode === m} mode={m} description={descriptions[m]} />)}
    </RadioGroup></div>
  )
}

function ModeOption({ active, mode, description }: { active: boolean; mode: AgentMode; description: string }) {
  return (
    <div className={cn("flex items-center space-x-3 rounded-lg border p-3 transition-colors cursor-pointer", active ? "border-primary bg-primary/5" : "border-border hover:bg-muted/50")}>
      <RadioGroupItem value={mode} id={mode} />
      <div className="flex-1"><Label htmlFor={mode} className="font-medium cursor-pointer capitalize">{mode.replace("_", " ")}</Label><p className="text-xs text-muted-foreground">{description}</p></div>
    </div>
  )
}

const ProvidersSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { providers, active_provider_id, loadSettings } = useSettingsStore()
  return (
    <SettingsSection ref={ref} id="providers" title="服务商" description="API 密钥和服务商" icon={Key}>
      <CurrentRoutingInfo />
      <div className="space-y-3">{providers.map((provider) => <ProviderRow key={provider.provider_id} provider={provider} active={active_provider_id === provider.provider_id} reload={loadSettings} />)}</div>
    </SettingsSection>
  )
})
ProvidersSection.displayName = "ProvidersSection"

function ProviderRow({ provider, active, reload }: { provider: Provider; active: boolean; reload: () => Promise<void> }) {
  const [apiKey, setApiKey] = useState("")
  const [busy, setBusy] = useState("")
  const [message, setMessage] = useState<ProviderActionMessage | null>(null)
  const [showKey, setShowKey] = useState(false)
  const masked = provider.credential_status?.api_key_masked || (provider.credential_status?.has_credential ? "已保存密钥" : "未配置密钥")
  return (
    <div className="rounded-lg border border-border p-4 space-y-3">
      <ProviderHeader provider={provider} active={active} masked={masked} />
      <ProviderStatusBar provider={provider} />
      <div className="flex flex-col gap-2 sm:flex-row"><Input type={showKey ? "text" : "password"} value={apiKey} onChange={(e) => setApiKey(e.target.value)} placeholder="输入 API Key 后测试或保存" autoComplete="off" /><Button variant="ghost" size="icon" onClick={() => setShowKey(!showKey)}>{showKey ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}</Button></div>
      <ProviderActions provider={provider} apiKey={apiKey} busy={busy} setBusy={setBusy} setMessage={setMessage} reload={reload} />
      {message && <ProviderActionNotice message={message} />}
    </div>
  )
}

type ProviderActionMessage = {
  ok: boolean
  text: string
}

function ProviderHeader({ provider, active, masked }: { provider: Provider; active: boolean; masked: string }) {
  return (
    <div className="flex items-center justify-between gap-3"><div className="flex items-center gap-3">
      <div className={cn("h-2.5 w-2.5 rounded-full", provider.status === "active" ? "bg-success" : provider.status === "error" ? "bg-destructive" : "bg-muted-foreground")} />
      <div><p className="text-sm font-medium text-foreground">{provider.display_name}</p><p className="text-xs text-muted-foreground font-mono">{masked}</p></div>
    </div>{active && <Badge variant="secondary" className="text-xs">当前</Badge>}</div>
  )
}

function ProviderActions({ provider, apiKey, busy, setBusy, setMessage, reload }: { provider: Provider; apiKey: string; busy: string; setBusy: (value: string) => void; setMessage: (message: ProviderActionMessage) => void; reload: () => Promise<void> }) {
  const payload = { provider_id: provider.provider_id, display_name: provider.display_name, base_url: provider.base_url, chat_completions_path: provider.chat_completions_path, models_path: provider.models_path, api_key: apiKey }
  return (
    <div className="flex flex-wrap items-center gap-2">
      <ProviderButton label="测试" icon={Play} busy={busy === "test"} disabled={!apiKey} onClick={() => runProviderAction("test", setBusy, setMessage, reload, () => testProvider(payload))} />
      <ProviderButton label="保存" icon={Save} busy={busy === "save"} disabled={!apiKey} onClick={() => runProviderAction("save", setBusy, setMessage, reload, () => saveProvider(payload))} />
      <AlertDialog>
        <AlertDialogTrigger asChild>
          <Button variant="outline" size="sm" className="gap-1" disabled={busy === "apply"}>
            {busy === "apply" ? <Loader2 className="h-3 w-3 animate-spin" /> : <CheckCircle className="h-3 w-3" />}
            应用
          </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>确认应用服务商</AlertDialogTitle>
            <AlertDialogDescription>
              应用后将使用 {provider.display_name} 作为当前模型服务商，可能需要重启 Runtime 才能生效。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction onClick={() => runProviderAction("apply", setBusy, setMessage, reload, () => applyProvider(provider.provider_id))}>
              应用
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog>
        <AlertDialogTrigger asChild>
          <Button variant="ghost" size="sm" className="gap-1" disabled={busy === "remove"}>
            {busy === "remove" ? <Loader2 className="h-3 w-3 animate-spin" /> : <Trash2 className="h-3 w-3" />}
            移除
          </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>确认移除服务商凭据</AlertDialogTitle>
            <AlertDialogDescription>
              此操作将移除 {provider.display_name} 的 API Key 配置，移除后该服务商将无法使用。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              onClick={() => runProviderAction("remove", setBusy, setMessage, reload, () => removeProviderCredential(provider.provider_id))}
            >
              移除
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}

function ProviderButton({ label, icon: Icon, busy, disabled, variant = "outline", onClick }: { label: string; icon: React.ElementType; busy: boolean; disabled?: boolean; variant?: "outline" | "ghost"; onClick: () => void }) {
  return <Button variant={variant} size="sm" className="gap-1" onClick={onClick} disabled={busy || disabled}>{busy ? <Loader2 className="h-3 w-3 animate-spin" /> : <Icon className="h-3 w-3" />}{label}</Button>
}

function ProviderActionNotice({ message }: { message: ProviderActionMessage }) {
  return <p className={cn("text-xs", message.ok ? "text-success" : "text-destructive")}>{message.text}</p>
}

function ProviderStatusBar({ provider }: { provider: Provider }) {
  const status = provider.credential_status
  if (!status) return null
  const items: { label: string; value: string; color?: string }[] = []
  if (!status.has_credential) {
    items.push({ label: "凭据", value: "未配置" })
  } else {
    items.push({ label: "凭据", value: "已保存" })
  }
  if (status.last_test_status) {
    const ok = status.last_test_status === "success" || status.last_test_status === "passed"
    items.push({ label: "测试", value: ok ? "已通过" : "失败", color: ok ? "text-success" : "text-destructive" })
  }
  if (status.apply_status) {
    items.push({ label: "应用", value: status.apply_status === "applied" ? "已应用" : status.apply_status })
  }
  if (status.pending_reload) {
    items.push({ label: "状态", value: "待重启", color: "text-warning" })
  }
  if (status.last_test_at) {
    items.push({ label: "测试时间", value: new Date(status.last_test_at).toLocaleString("zh-CN") })
  }
  if (status.last_test_message && status.last_test_status !== "success") {
    items.push({ label: "错误", value: status.last_test_message, color: "text-destructive" })
  }
  return (
    <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs">
      {items.map((item) => (
        <span key={item.label} className="text-muted-foreground">
          {item.label}: <span className={cn("font-medium text-foreground", item.color)}>{item.value}</span>
        </span>
      ))}
    </div>
  )
}

function CurrentRoutingInfo() {
  const { model, active_provider_id, providers } = useSettingsStore()
  const activeProvider = providers.find((p) => p.provider_id === active_provider_id)
  return (
    <div className="rounded-lg bg-muted p-3 mb-4">
      <p className="text-xs font-medium text-muted-foreground mb-1">当前聊天路由</p>
      <div className="flex flex-wrap gap-x-4 gap-y-1 text-sm">
        <span>模型: <span className="font-medium">{model.display_name || model.model_id || "-"}</span></span>
        <span>服务商: <span className="font-medium">{model.provider_id || "-"}</span></span>
        <span>激活服务商: <span className="font-medium">{activeProvider?.display_name || active_provider_id || "-"}</span></span>
      </div>
      {model.provider_id && active_provider_id && model.provider_id !== active_provider_id && (
        <p className="text-xs text-warning mt-1">
          注意：当前模型所属服务商与激活服务商不一致，可能通过兼容网关路由。
        </p>
      )}
    </div>
  )
}

async function runProviderAction(name: string, setBusy: (value: string) => void, setMessage: (message: ProviderActionMessage) => void, reload: () => Promise<void>, action: () => Promise<unknown>) {
  setBusy(name)
  try { const result = await action(); setMessage(successMessage(result)); await reload() }
  catch (error) { setMessage({ ok: false, text: errorMessage(error) }) }
  finally { setBusy("") }
}

function successMessage(result: unknown): ProviderActionMessage {
  const message = typeof result === "object" && result && "message" in result ? String(result.message) : "操作成功"
  const ok = typeof result === "object" && result && "ok" in result ? Boolean(result.ok) : true
  return { ok, text: message }
}

function errorMessage(error: unknown) {
  return error instanceof Error ? error.message : "操作失败"
}

const EmbeddingSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { embedding_provider_id, providers, embedding, setEmbeddingProvider } = useSettingsStore()
  return (
    <SettingsSection ref={ref} id="embedding" title="嵌入" description="向量嵌入模型配置" icon={Database}>
      <div className="space-y-2"><Label>Embedding Provider</Label><Select value={embedding_provider_id} onValueChange={setEmbeddingProvider}>
        <SelectTrigger><SelectValue placeholder="选择嵌入服务商" /></SelectTrigger>
        <SelectContent>{providers.map((p) => <SelectItem key={p.provider_id} value={p.provider_id}>{p.display_name}</SelectItem>)}</SelectContent>
      </Select>{embedding?.model_name && <p className="text-xs text-muted-foreground">当前模型：{embedding.model_name}</p>}</div>
    </SettingsSection>
  )
})
EmbeddingSection.displayName = "EmbeddingSection"

const WorkspacesSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { workspace, available_workspaces, approved_directories, setWorkspace, addDirectory, removeDirectory } = useSettingsStore()
  return (
    <SettingsSection ref={ref} id="workspaces" title="工作区" description="授权目录和工作区路径" icon={FolderOpen}>
      <div className="space-y-4"><WorkspaceSelect workspaceId={workspace.workspace_id} items={available_workspaces} onSelect={setWorkspace} /><DirectoryList items={approved_directories} onAdd={addDirectory} onRemove={removeDirectory} /></div>
    </SettingsSection>
  )
})
WorkspacesSection.displayName = "WorkspacesSection"

function WorkspaceSelect({ workspaceId, items, onSelect }: { workspaceId: string; items: Workspace[]; onSelect: (item: Workspace) => void }) {
  return <div className="space-y-2"><Label>当前工作区</Label><Select value={workspaceId} onValueChange={(v) => selectWorkspace(v, items, onSelect)}><SelectTrigger><SelectValue /></SelectTrigger><SelectContent>{items.map((item) => <SelectItem key={item.workspace_id} value={item.workspace_id}>{item.name}</SelectItem>)}</SelectContent></Select></div>
}

function selectWorkspace(value: string, items: Workspace[], onSelect: (item: Workspace) => void) {
  const selected = items.find((i) => i.workspace_id === value)
  if (selected) onSelect(selected)
}

function DirectoryList({ items, onAdd, onRemove }: { items: DirectoryApproval[]; onAdd: (name: string, path: string) => Promise<void>; onRemove: (path: string) => Promise<void> }) {
  const [name, setName] = useState("")
  const [path, setPath] = useState("")
  const [busy, setBusy] = useState(false)
  const handleAdd = async () => { if (!path) return; setBusy(true); try { await onAdd(name || path, path); setName(""); setPath("") } finally { setBusy(false) } }
  return (
    <div className="space-y-2">
      <p className="text-xs font-medium text-muted-foreground uppercase tracking-wider">授权目录</p>
      {items.map((dir) => <DirectoryItem key={dir.approval_id} dir={dir} onRemove={onRemove} />)}
      <div className="flex gap-2">
        <Input placeholder="名称（可选）" value={name} onChange={(e) => setName(e.target.value)} className="flex-1" />
        <Input placeholder="绝对路径" value={path} onChange={(e) => setPath(e.target.value)} className="flex-[2]" />
        <Button size="sm" onClick={handleAdd} disabled={busy || !path}>{busy ? <Loader2 className="h-3 w-3 animate-spin" /> : "添加"}</Button>
      </div>
    </div>
  )
}

function DirectoryItem({ dir, onRemove }: { dir: DirectoryApproval; onRemove: (path: string) => Promise<void> }) {
  const [busy, setBusy] = useState(false)
  const handleRemove = async () => { setBusy(true); try { await onRemove(dir.root_path) } finally { setBusy(false) } }
  return (
    <div className="flex items-start gap-2 rounded-lg bg-muted px-3 py-2">
      <div className="flex-1 min-w-0">
        <p className="text-sm text-foreground">{dir.name}</p>
        <p className="text-xs font-mono text-muted-foreground break-all">{dir.root_path}</p>
      </div>
      <AlertDialog>
        <AlertDialogTrigger asChild>
          <Button variant="ghost" size="icon" className="h-6 w-6 text-destructive shrink-0" disabled={busy}>
            {busy ? <Loader2 className="h-3 w-3 animate-spin" /> : <Trash2 className="h-3 w-3" />}
          </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>确认移除授权目录</AlertDialogTitle>
            <AlertDialogDescription>确定要移除「{dir.name}」的授权吗？移除后智能体将无法访问该目录。</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction onClick={handleRemove} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">移除</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}

const MCPSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { mcp, loadSettings } = useSettingsStore()
  const servers = mcp?.servers || []
  const tools = mcp?.tools || []
  const audits = useMCPAuditFeed(servers.length > 0)
  return (
    <SettingsSection ref={ref} id="mcp" title="MCP" description="模型上下文协议工具" icon={Wrench}>
      <div className="space-y-3">
        <MCPObservabilityPanel servers={servers} tools={tools} audits={audits.items} auditError={audits.error} />
        {servers.map((s) => <MCPServerCard key={s.id} server={s} tools={tools.filter((t) => t.server_id === s.id)} onReload={loadSettings} />)}
        {servers.length === 0 && <p className="text-sm text-muted-foreground">暂无配置的 MCP 服务器</p>}
        <MCPAddForm onReload={loadSettings} />
      </div>
    </SettingsSection>
  )
})
MCPSection.displayName = "MCPSection"

function useMCPAuditFeed(enabled: boolean) {
  const [items, setItems] = useState<MCPAuditRecord[] | undefined>(undefined)
  const [error, setError] = useState<string | undefined>(undefined)
  useEffect(() => {
    if (!enabled) return
    let cancelled = false
    fetchMCPAudits({ limit: 20 })
      .then((data) => { if (!cancelled) { setItems(data); setError(undefined) } })
      .catch(() => { if (!cancelled) { setItems([]); setError("审计 API 已预留，当前环境尚未返回记录。") } })
    return () => { cancelled = true }
  }, [enabled])
  return { items, error }
}

function MCPServerCard({ server, tools, onReload }: { server: import("@/lib/local-agent/types").MCPServerInfo; tools: import("@/lib/local-agent/types").MCPTool[]; onReload: () => Promise<void> }) {
  const [open, setOpen] = useState(false)
  const statusText = server.ready ? "已连接" : server.enabled ? "未就绪" : "未启用"
  const statusVariant = server.ready ? "default" : "secondary"
  return (
    <div className="rounded-lg border border-border">
      <button onClick={() => setOpen(!open)} className="w-full flex items-center justify-between p-3 text-left">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium truncate">{server.name}</span>
            <Badge variant={statusVariant} className="text-[10px] px-1.5 py-0 shrink-0">{statusText}</Badge>
          </div>
          <p className="text-xs text-muted-foreground font-mono truncate">{server.url}</p>
        </div>
        <div className="flex items-center gap-2 shrink-0 ml-2">
          <span className="text-xs text-muted-foreground">{server.allowed_tool_count || 0}/{tools.length} 已允许</span>
          <ChevronDown className={cn("h-4 w-4 text-muted-foreground transition-transform", open && "rotate-180")} />
        </div>
      </button>
      {open && (
        <div className="px-3 pb-3 space-y-3">
          <MCPToolList tools={tools} onReload={onReload} />
          <MCPRemoveButton server={server} onReload={onReload} />
        </div>
      )}
    </div>
  )
}

function MCPRemoveButton({ server, onReload }: { server: import("@/lib/local-agent/types").MCPServerInfo; onReload: () => Promise<void> }) {
  const [busy, setBusy] = useState(false)
  const handleRemove = async () => { setBusy(true); try { await updateSettings({ remove_mcp_id: server.id }); await onReload() } finally { setBusy(false) } }
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild><Button variant="ghost" size="sm" className="gap-1 text-destructive" disabled={busy}>{busy ? <Loader2 className="h-3 w-3 animate-spin" /> : <Trash2 className="h-3 w-3" />}移除服务器</Button></AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader><AlertDialogTitle>确认移除 MCP 服务器</AlertDialogTitle><AlertDialogDescription>移除「{server.name}」后，智能体将不再加载该服务器提供的工具。</AlertDialogDescription></AlertDialogHeader>
        <AlertDialogFooter><AlertDialogCancel>取消</AlertDialogCancel><AlertDialogAction className="bg-destructive text-destructive-foreground hover:bg-destructive/90" onClick={handleRemove}>移除</AlertDialogAction></AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}

function MCPToolList({ tools, onReload }: { tools: import("@/lib/local-agent/types").MCPTool[]; onReload: () => Promise<void> }) {
  if (tools.length === 0) return <p className="text-xs text-muted-foreground py-1">暂无可用工具</p>
  return (
    <div className="space-y-1.5 pt-2">
      <p className="text-xs font-medium text-muted-foreground uppercase tracking-wider">可用工具</p>
      {tools.map((tool) => <MCPToolRow key={`${tool.server_id}:${tool.name}`} tool={tool} onReload={onReload} />)}
    </div>
  )
}

function MCPToolRow({ tool, onReload }: { tool: import("@/lib/local-agent/types").MCPTool; onReload: () => Promise<void> }) {
  const [busy, setBusy] = useState(false)
  const update = async (patch: Partial<Pick<typeof tool, "allowed" | "risk_level" | "requires_confirmation">>) => {
    if (!tool.server_id) return
    setBusy(true)
    try { await updateMCPToolPolicy(tool, patch); await onReload() } finally { setBusy(false) }
  }
  return (
    <div className="rounded bg-muted px-3 py-2 space-y-2">
      <MCPToolHeader tool={tool} />
      <p className="text-xs text-muted-foreground">{tool.description}</p>
      <div className="flex flex-wrap items-center gap-3 text-xs">
        <PolicySwitch label="允许" checked={tool.allowed} disabled={busy} onChange={(v) => void update({ allowed: v })} />
        <PolicySwitch label="确认" checked={tool.requires_confirmation} disabled={busy} onChange={(v) => void update({ requires_confirmation: v })} />
        <MCPRiskSelect value={tool.risk_level || "medium"} disabled={busy} onChange={(v) => void update({ risk_level: v })} />
      </div>
    </div>
  )
}

function MCPToolHeader({ tool }: { tool: import("@/lib/local-agent/types").MCPTool }) {
  return <div className="flex items-center justify-between gap-2"><p className="text-sm font-medium text-foreground">{tool.name}</p><MCPToolBadges tool={tool} /></div>
}

function PolicySwitch({ label, checked, disabled, onChange }: { label: string; checked: boolean; disabled: boolean; onChange: (value: boolean) => void }) {
  return <label className="flex items-center gap-1.5 text-muted-foreground"><Switch checked={checked} disabled={disabled} onCheckedChange={onChange} />{label}</label>
}

function MCPRiskSelect({ value, disabled, onChange }: { value: "low" | "medium" | "high" | "critical"; disabled: boolean; onChange: (value: "low" | "medium" | "high" | "critical") => void }) {
  return <Select value={value} onValueChange={(v) => onChange(v as typeof value)} disabled={disabled}><SelectTrigger className="h-8 w-28"><SelectValue /></SelectTrigger><SelectContent>{["low", "medium", "high", "critical"].map((risk) => <SelectItem key={risk} value={risk}>{risk}</SelectItem>)}</SelectContent></Select>
}

async function updateMCPToolPolicy(tool: import("@/lib/local-agent/types").MCPTool, patch: Partial<Pick<typeof tool, "allowed" | "risk_level" | "requires_confirmation">>) {
  await updateSettings({ mcp_policy_server_id: tool.server_id, mcp_policy_tool_name: tool.name, mcp_policy_allowed: patch.allowed, mcp_policy_risk_level: patch.risk_level, mcp_policy_requires_confirmation: patch.requires_confirmation })
}

function MCPAddForm({ onReload }: { onReload: () => Promise<void> }) {
  const [name, setName] = useState("")
  const [url, setUrl] = useState("")
  const [busy, setBusy] = useState(false)
  const handleAdd = async () => {
    if (!url) return
    setBusy(true)
    try {
      await updateSettings({ add_mcp_name: name || url, add_mcp_url: url })
      await onReload()
      setName("")
      setUrl("")
    } finally {
      setBusy(false)
    }
  }
  return (
    <div className="rounded-lg border border-dashed border-border p-3 space-y-2">
      <p className="text-xs font-medium text-muted-foreground uppercase tracking-wider">添加服务器</p>
      <div className="flex gap-2">
        <Input placeholder="名称（如：本地MCP）" value={name} onChange={(e) => setName(e.target.value)} className="flex-1" />
        <Input placeholder="HTTP 地址" value={url} onChange={(e) => setUrl(e.target.value)} className="flex-[2]" />
        <Button size="sm" onClick={handleAdd} disabled={busy || !url}>{busy ? <Loader2 className="h-3 w-3 animate-spin" /> : "添加"}</Button>
      </div>
      <p className="text-[10px] text-muted-foreground">当前仅支持 HTTP transport。stdio 类型的 MCP Server 需通过 mcp-proxy 等工具转换。</p>
    </div>
  )
}

const RiskSection = forwardRef<HTMLDivElement>((_, ref) => {
  const { directory_prompt_enabled, show_risk_level, setDirectoryPromptEnabled, setShowRiskLevel } = useSettingsStore()
  return (
    <SettingsSection ref={ref} id="risk" title="风险" description="安全和风险管理设置" icon={Shield}>
      <div className="space-y-4">
        <RiskToggle id="directory-prompt" label="新目录访问前确认" checked={directory_prompt_enabled} onChange={setDirectoryPromptEnabled} />
        <RiskToggle id="show-risk" label="显示风险等级" checked={show_risk_level} onChange={setShowRiskLevel} />
      </div>
    </SettingsSection>
  )
})
RiskSection.displayName = "RiskSection"

function RiskToggle({ id, label, checked, onChange }: { id: string; label: string; checked: boolean; onChange: (checked: boolean) => void }) {
  return <div className="flex items-center justify-between rounded-lg border border-border p-4"><Label htmlFor={id} className="cursor-pointer">{label}</Label><Switch id={id} checked={checked} onCheckedChange={onChange} /></div>
}

const MemorySection = forwardRef<HTMLDivElement>((_, ref) => {
  const { memories, loadMemories } = useMemoryStore()
  useEffect(() => { loadMemories() }, [loadMemories])
  return <SettingsSection ref={ref} id="memory" title="记忆" description="智能体记忆和上下文存储" icon={Brain}><div className="space-y-2">{memories.map((memory) => <MemoryItem key={memory.id} memory={memory} />)}</div></SettingsSection>
})
MemorySection.displayName = "MemorySection"

function MemoryItem({ memory }: { memory: { id: string; kind: string; title: string; content: string; createdAt: string } }) {
  const { removeMemory } = useMemoryStore()
  return <Collapsible><CollapsibleTrigger className="flex w-full items-center justify-between rounded-lg border border-border p-3 hover:bg-muted"><span className="text-sm font-medium text-foreground line-clamp-1">{memory.title || memory.content}</span><ChevronRight className="h-4 w-4 text-muted-foreground" /></CollapsibleTrigger><CollapsibleContent className="px-3 py-2"><div className="flex items-start justify-between gap-4"><p className="text-sm text-muted-foreground">{memory.content}</p><AlertDialog>
              <AlertDialogTrigger asChild>
                <Button variant="ghost" size="icon" className="h-6 w-6 text-destructive">
                  <Trash2 className="h-3 w-3" />
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>确认删除记忆</AlertDialogTitle>
                  <AlertDialogDescription>
                    此操作将永久删除这条记忆，删除后无法恢复。
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>取消</AlertDialogCancel>
                  <AlertDialogAction
                    className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                    onClick={() => void removeMemory(memory.id)}
                  >
                    删除
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog></div></CollapsibleContent></Collapsible>
}

const DiagnosticsSection = forwardRef<HTMLDivElement>((_, ref) => {
  const [isRunning, setIsRunning] = useState(false)
  const [results, setResults] = useState<DiagnosticRow[]>([])
  return (
    <SettingsSection ref={ref} id="diagnostics" title="诊断" description="服务状态和启动自检" icon={Wrench}>
      <Button variant="outline" className="gap-2" onClick={() => void runHealthCheck(setIsRunning, setResults)} disabled={isRunning}>{isRunning ? <Loader2 className="h-4 w-4 animate-spin" /> : <CheckCircle className="h-4 w-4" />}健康检查</Button>
      <DiagnosticResults results={results} />
    </SettingsSection>
  )
})
DiagnosticsSection.displayName = "DiagnosticsSection"

type DiagnosticRow = {
  category: string
  status: "ok" | "warning" | "error"
  message: string
  hint?: string
}

async function runHealthCheck(setRunning: (value: boolean) => void, setResults: (value: DiagnosticRow[]) => void) {
  setRunning(true)
  try { const data = await runDiagnosticsCheck(); setResults(diagnosticRows(data)) }
  catch { setResults([{ category: "检查", status: "error", message: "健康检查请求失败" }]) }
  finally { setRunning(false) }
}

function diagnosticRows(data: Awaited<ReturnType<typeof runDiagnosticsCheck>>) {
  if (data.services?.length) {
    return data.services.map((item) => ({
      category: item.label,
      status: normalizeDiagnosticStatus(item.status),
      message: item.detail,
      hint: item.hint,
    }))
  }
  const d = data.diagnostics
  return [
    { category: "仓库", status: d.repo_root_exists ? "ok" as const : "error" as const, message: d.repo_root_exists ? "可访问" : "不可访问" },
    { category: "Runtime", status: d.runtime_reachable ? "ok" as const : "error" as const, message: d.runtime_reachable ? `可达 (${d.runtime_version})` : "不可达" },
    { category: "服务商", status: d.provider_count > 0 ? "ok" as const : "error" as const, message: `${d.provider_count} 个已配置` },
  ]
}

function normalizeDiagnosticStatus(status: string): DiagnosticRow["status"] {
  if (status === "ok" || status === "warning" || status === "error") return status
  return "warning"
}

function DiagnosticResults({ results }: { results: DiagnosticRow[] }) {
  if (!results.length) return null
  return <div className="mt-4 rounded-lg border border-border p-4 space-y-3">{results.map((r) => <DiagnosticResultRow key={r.category} row={r} />)}</div>
}

function DiagnosticResultRow({ row }: { row: DiagnosticRow }) {
  return (
    <div className="flex items-start gap-3">
      <CheckCircle className={cn("mt-0.5 h-4 w-4", diagnosticColor(row.status))} />
      <div className="min-w-0 flex-1">
        <div className="flex flex-wrap items-center gap-2">
          <span className="text-sm font-medium text-foreground">{row.category}</span>
          <Badge variant="secondary" className="text-xs">{diagnosticLabel(row.status)}</Badge>
        </div>
        <p className="text-sm text-muted-foreground">{row.message}</p>
        {row.hint && <p className="text-xs text-muted-foreground">{row.hint}</p>}
      </div>
    </div>
  )
}

function diagnosticColor(status: DiagnosticRow["status"]) {
  if (status === "ok") return "text-success"
  if (status === "warning") return "text-warning"
  return "text-destructive"
}

function diagnosticLabel(status: DiagnosticRow["status"]) {
  if (status === "ok") return "正常"
  if (status === "warning") return "提醒"
  return "异常"
}

interface SettingsSectionProps {
  id: string
  title: string
  description: string
  icon: React.ElementType
  children: React.ReactNode
}

const SettingsSection = forwardRef<HTMLDivElement, SettingsSectionProps>(({ id, title, description, icon: Icon, children }, ref) => {
  const [isOpen, setIsOpen] = useState(true)
  return (
    <div ref={ref} id={id} className="scroll-mt-6"><Collapsible open={isOpen} onOpenChange={setIsOpen}>
      <CollapsibleTrigger className="flex w-full items-center justify-between rounded-xl border border-border bg-card p-4 hover:bg-muted/50 transition-colors">
        <div className="flex items-center gap-3"><div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10"><Icon className="h-5 w-5 text-primary" /></div><div className="text-left"><h3 className="text-sm font-semibold text-foreground">{title}</h3><p className="text-xs text-muted-foreground">{description}</p></div></div>
        {isOpen ? <ChevronDown className="h-5 w-5 text-muted-foreground" /> : <ChevronRight className="h-5 w-5 text-muted-foreground" />}
      </CollapsibleTrigger><CollapsibleContent className="pt-4 animate-in fade-in slide-in-from-top-2 duration-200">{children}</CollapsibleContent>
    </Collapsible></div>
  )
})
SettingsSection.displayName = "SettingsSection"
