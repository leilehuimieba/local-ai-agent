"use client"

import { useState, useEffect, useRef, useMemo } from "react"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@/components/ui/sheet"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
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
import {
  Search,
  Send,
  FileText,
  Code,
  BookOpen,
  Database,
  Plus,
  Upload,
  Loader2,
  Trash2,
} from "lucide-react"
import { useKnowledgeStore } from "@/lib/local-agent/store"
import { categoryColors } from "@/lib/local-agent/mock-data"
import type { KnowledgeItem } from "@/lib/local-agent/types"
import { askKnowledgeBase, createKnowledgeItem, deleteKnowledgeItem, updateKnowledgeItem, uploadKnowledgeFile } from "@/lib/local-agent/api"
import { cn } from "@/lib/utils"

const categories = [
  { id: "all", label: "全部", icon: null },
  { id: "Architecture", label: "架构", icon: FileText },
  { id: "API", label: "API", icon: Code },
  { id: "Security", label: "安全", icon: BookOpen },
  { id: "Performance", label: "性能", icon: Database },
  { id: "Testing", label: "测试", icon: Code },
  { id: "Documentation", label: "文档", icon: FileText },
]

export function KnowledgeView() {
  const [activeTab, setActiveTab] = useState("sources")
  const { loadItems } = useKnowledgeStore()

  useEffect(() => {
    loadItems()
  }, [loadItems])

  return (
    <div className="flex h-full flex-col">
      <Tabs value={activeTab} onValueChange={setActiveTab} className="flex h-full flex-col">
        <div className="shrink-0 border-b border-border bg-card px-4">
          <TabsList className="h-12 bg-transparent">
            <TabsTrigger value="sources" className="data-[state=active]:bg-muted">
              资料源
            </TabsTrigger>
            <TabsTrigger value="ask" className="data-[state=active]:bg-muted">
              问答
            </TabsTrigger>

          </TabsList>
        </div>

        <TabsContent value="sources" className="flex-1 m-0 overflow-hidden">
          <SourcesTab />
        </TabsContent>

        <TabsContent value="ask" className="flex-1 m-0 overflow-hidden">
          <AskTab />
        </TabsContent>


      </Tabs>
    </div>
  )
}
export function SourcesTab() {
  const store = useKnowledgeStore()
  const filteredSources = useFilteredSources(store)
  const [dialogOpen, setDialogOpen] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const handleCreate = async (item: KnowledgeDraft) => {
    const created = await createKnowledgeItem(item)
    store.addItem(created)
    setDialogOpen(false)
  }

  const handleUpload = async (file?: File) => {
    if (!file) return
    const uploaded = await uploadKnowledgeFile(file)
    store.addItem(uploaded)
  }

  return (
    <div className="flex h-full">
      <KnowledgeSidebar store={store} fileInputRef={fileInputRef} openDialog={() => setDialogOpen(true)} upload={handleUpload} />
      <KnowledgeGrid items={filteredSources} onSelect={store.setSelectedItem} />
      <KnowledgeDetailSheet selectedItem={store.selectedItem} setSelectedItem={store.setSelectedItem} updateItem={store.updateItem} removeItem={store.removeItem} />
      <KnowledgeCreateDialog open={dialogOpen} onOpenChange={setDialogOpen} onCreate={handleCreate} />
    </div>
  )
}
type KnowledgeDraft = Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">
type KnowledgeStoreSnapshot = ReturnType<typeof useKnowledgeStore.getState>

function useFilteredSources(store: KnowledgeStoreSnapshot) {
  return useMemo(() => {
    return store.items.filter((source) => isKnowledgeVisible(source, store))
  }, [store])
}

export function isKnowledgeVisible(source: KnowledgeItem, store: KnowledgeStoreSnapshot) {
  if (store.selectedCategory && store.selectedCategory !== "all" && source.category !== store.selectedCategory) return false
  if (store.searchQuery && !source.title.toLowerCase().includes(store.searchQuery.toLowerCase())) return false
  return !store.selectedTags.length || store.selectedTags.some((tag) => source.tags.includes(tag))
}

function KnowledgeSidebar({ store, fileInputRef, openDialog, upload }: { store: KnowledgeStoreSnapshot; fileInputRef: React.RefObject<HTMLInputElement | null>; openDialog: () => void; upload: (file?: File) => Promise<void> }) {
  return (
    <div className="w-56 shrink-0 border-r border-border p-4 hidden md:flex md:flex-col">
      <KnowledgeFilters store={store} />
      <div className="pt-4 border-t border-border mt-4 space-y-2">
        <Button variant="outline" className="w-full justify-start gap-2" size="sm" onClick={openDialog}><Plus className="h-4 w-4" />添加资料</Button>
        <input ref={fileInputRef} type="file" className="hidden" onChange={(e) => void upload(e.target.files?.[0])} />
        <Button variant="outline" className="w-full justify-start gap-2" size="sm" onClick={() => fileInputRef.current?.click()}><Upload className="h-4 w-4" />上传文件</Button>
      </div>
    </div>
  )
}

function KnowledgeFilters({ store }: { store: KnowledgeStoreSnapshot }) {
  return (
    <div className="space-y-4 flex-1">
      <div className="relative"><Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" /><Input value={store.searchQuery} onChange={(e) => store.setSearchQuery(e.target.value)} placeholder="搜索..." className="pl-9" /></div>
      <CategoryFilter selected={store.selectedCategory} onSelect={store.setSelectedCategory} />
      <TagFilter tags={store.allTags} selected={store.selectedTags} toggle={store.toggleTag} />
    </div>
  )
}

function CategoryFilter({ selected, onSelect }: { selected: string | null; onSelect: (category: string | null) => void }) {
  return <div><p className="text-xs font-medium text-muted-foreground mb-2 tracking-wider">分类</p><div className="flex flex-wrap gap-1.5">{categories.map((cat) => <CategoryButton key={cat.id} cat={cat} selected={selected} onSelect={onSelect} />)}</div></div>
}

export function CategoryButton({ cat, selected, onSelect }: { cat: (typeof categories)[number]; selected: string | null; onSelect: (category: string | null) => void }) {
  const active = selected === cat.id || (cat.id === "all" && !selected)
  return <button onClick={() => onSelect(cat.id === "all" ? null : cat.id)} className={cn("px-2.5 py-1 text-xs font-medium rounded-md transition-colors", active ? "bg-primary text-primary-foreground" : "bg-muted text-muted-foreground hover:text-foreground")}>{cat.label}</button>
}

export function TagFilter({ tags, selected, toggle }: { tags: string[]; selected: string[]; toggle: (tag: string) => void }) {
  const [query, setQuery] = useState("")
  const [expanded, setExpanded] = useState(false)
  const filtered = query ? tags.filter((t) => t.toLowerCase().includes(query.toLowerCase())) : tags
  const display = expanded ? filtered : filtered.slice(0, 20)
  const hasMore = filtered.length > 20

  return (
    <div>
      <p className="text-xs font-medium text-muted-foreground mb-2 tracking-wider">标签</p>
      <div className="relative mb-2">
        <Search className="absolute left-2 top-1/2 h-3 w-3 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="搜索标签..."
          className="h-7 text-xs pl-7"
        />
      </div>
      <div className="flex flex-wrap gap-1.5">
        {display.map((tag) => (
          <Badge key={tag} variant={selected.includes(tag) ? "default" : "outline"} className="text-xs cursor-pointer" onClick={() => toggle(tag)}>
            {tag}
          </Badge>
        ))}
      </div>
      {hasMore && !expanded && (
        <button onClick={() => setExpanded(true)} className="text-xs text-muted-foreground hover:text-foreground mt-1">
          展开全部 ({filtered.length})
        </button>
      )}
      {expanded && (
        <button onClick={() => setExpanded(false)} className="text-xs text-muted-foreground hover:text-foreground mt-1">
          收起
        </button>
      )}
    </div>
  )
}

function KnowledgeGrid({ items, onSelect }: { items: KnowledgeItem[]; onSelect: (item: KnowledgeItem) => void }) {
  return <ScrollArea className="flex-1"><div className="p-4">{items.length ? <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">{items.map((source) => <SourceCard key={source.id} source={source} onSelect={() => onSelect(source)} />)}</div> : <KnowledgeEmpty />}</div></ScrollArea>
}

export function KnowledgeEmpty() {
  return <div className="flex flex-col items-center justify-center h-64 text-center"><div className="h-12 w-12 rounded-xl bg-muted flex items-center justify-center mb-4"><FileText className="h-6 w-6 text-muted-foreground" /></div><h3 className="text-lg font-medium text-foreground mb-2">未找到资料</h3><p className="text-sm text-muted-foreground max-w-sm">尝试调整筛选条件或添加新资料。</p></div>
}

function KnowledgeDetailSheet({ selectedItem, setSelectedItem, updateItem, removeItem }: { selectedItem: KnowledgeItem | null; setSelectedItem: (item: KnowledgeItem | null) => void; updateItem: (id: string, item: Partial<KnowledgeItem>) => void; removeItem: (id: string) => void }) {
  return <Sheet open={!!selectedItem} onOpenChange={() => setSelectedItem(null)}><SheetContent className="w-[400px] sm:w-[540px] overflow-y-auto">{selectedItem && <KnowledgeDetail item={selectedItem} setSelectedItem={setSelectedItem} updateItem={updateItem} removeItem={removeItem} />}</SheetContent></Sheet>
}

function KnowledgeDetail({ item, setSelectedItem, updateItem, removeItem }: { item: KnowledgeItem; setSelectedItem: (item: KnowledgeItem | null) => void; updateItem: (id: string, item: Partial<KnowledgeItem>) => void; removeItem: (id: string) => void }) {
  const [isEditing, setIsEditing] = useState(false)
  const [editContent, setEditContent] = useState(item.content)
  const [saving, setSaving] = useState(false)
  return <><KnowledgeDetailHeader item={item} isEditing={isEditing} toggle={(v) => { setEditContent(item.content); setIsEditing(v) }} /><KnowledgeEditor item={item} isEditing={isEditing} content={editContent} saving={saving} setContent={setEditContent} save={() => saveKnowledgeEdit(item, editContent, setSaving, updateItem, setSelectedItem, setIsEditing)} cancel={() => setIsEditing(false)} /><KnowledgeTags item={item} /><AlertDialog>
        <AlertDialogTrigger asChild>
          <Button variant="ghost" className="mt-4 gap-2 text-destructive hover:text-destructive">
            <Trash2 className="h-4 w-4" />删除资料
          </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>确认删除资料</AlertDialogTitle>
            <AlertDialogDescription>
              此操作将永久删除「{item.title}」，资料内容不可恢复。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              onClick={() => void deleteSelectedKnowledge(item, removeItem)}
            >
              删除
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog></>
}

function KnowledgeDetailHeader({ item, isEditing, toggle }: { item: KnowledgeItem; isEditing: boolean; toggle: (value: boolean) => void }) {
  return <SheetHeader className="pb-4"><div className="flex items-start justify-between"><div><SheetTitle className="text-lg">{item.title}</SheetTitle><Badge className="mt-2 border-0" style={{ backgroundColor: `${categoryColors[item.category]}20`, color: categoryColors[item.category] }}>{item.category}</Badge></div><div className="flex items-center gap-2"><Switch id="edit-mode" checked={isEditing} onCheckedChange={toggle} /><Label htmlFor="edit-mode" className="text-xs">编辑</Label></div></div><div className="flex items-center gap-4 text-xs text-muted-foreground mt-3"><span>{item.citationCount} 引用</span><span>来源: {item.source}</span><span>更新于: {new Date(item.updatedAt).toLocaleDateString("zh-CN")}</span></div></SheetHeader>
}

function KnowledgeEditor({ item, isEditing, content, saving, setContent, save, cancel }: { item: KnowledgeItem; isEditing: boolean; content: string; saving: boolean; setContent: (value: string) => void; save: () => void; cancel: () => void }) {
  if (!isEditing) return <pre className="whitespace-pre-wrap text-sm text-foreground bg-muted/50 p-4 rounded-lg overflow-x-auto">{item.content}</pre>
  return <div className="space-y-3"><Textarea value={content} onChange={(e) => setContent(e.target.value)} className="min-h-[300px] font-mono text-sm resize-none" /><div className="flex gap-2"><Button onClick={save} size="sm" disabled={saving}>{saving ? "保存中..." : "保存"}</Button><Button variant="outline" size="sm" onClick={cancel}>取消</Button></div></div>
}

export function KnowledgeTags({ item }: { item: KnowledgeItem }) {
  return <div className="mt-4 pt-4 border-t border-border"><p className="text-xs font-medium text-muted-foreground mb-2">标签</p><div className="flex flex-wrap gap-1.5">{item.tags.map((tag) => <Badge key={tag} variant="secondary" className="text-xs">{tag}</Badge>)}</div></div>
}

async function saveKnowledgeEdit(item: KnowledgeItem, content: string, setSaving: (saving: boolean) => void, updateItem: (id: string, item: Partial<KnowledgeItem>) => void, setSelectedItem: (item: KnowledgeItem | null) => void, setEditing: (editing: boolean) => void) {
  setSaving(true)
  try { const updated = await updateKnowledgeItem(item.id, { content }); updateItem(updated.id, updated); setSelectedItem(updated); setEditing(false) }
  finally { setSaving(false) }
}

async function deleteSelectedKnowledge(item: KnowledgeItem, removeItem: (id: string) => void) {
  await deleteKnowledgeItem(item.id)
  removeItem(item.id)
}

function KnowledgeCreateDialog({
  open,
  onOpenChange,
  onCreate,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  onCreate: (item: KnowledgeDraft) => Promise<void>
}) {
  const [draft, setDraft] = useState(emptyKnowledgeDraft())
  const [saving, setSaving] = useState(false)

  const submit = async () => {
    setSaving(true)
    try { await onCreate(normalizeDraft(draft)); setDraft(emptyKnowledgeDraft()) }
    finally { setSaving(false) }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader><DialogTitle>添加资料</DialogTitle></DialogHeader>
        <KnowledgeDraftForm draft={draft} setDraft={setDraft} />
        <DialogFooter><Button onClick={() => void submit()} disabled={saving || !draft.title.trim()}>{saving ? "保存中..." : "保存"}</Button></DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

export function KnowledgeDraftForm({ draft, setDraft }: { draft: KnowledgeDraft; setDraft: (draft: KnowledgeDraft) => void }) {
  return (
    <div className="space-y-3">
      <Input value={draft.title} onChange={(e) => setDraft({ ...draft, title: e.target.value })} placeholder="标题" />
      <Input value={draft.category} onChange={(e) => setDraft({ ...draft, category: e.target.value })} placeholder="分类" />
      <Input value={draft.tags.join(", ")} onChange={(e) => setDraft({ ...draft, tags: splitTags(e.target.value) })} placeholder="标签，用逗号分隔" />
      <Textarea value={draft.summary} onChange={(e) => setDraft({ ...draft, summary: e.target.value })} placeholder="摘要" />
      <Textarea value={draft.content} onChange={(e) => setDraft({ ...draft, content: e.target.value })} placeholder="正文" className="min-h-[180px]" />
    </div>
  )
}

export function emptyKnowledgeDraft(): KnowledgeDraft {
  return { title: "", summary: "", content: "", category: "Documentation", tags: [], source: "" }
}

export function normalizeDraft(draft: KnowledgeDraft): KnowledgeDraft {
  return { ...draft, summary: draft.summary || draft.content.slice(0, 120), source: draft.source || "manual" }
}

export function splitTags(value: string) {
  return value.split(",").map((tag) => tag.trim()).filter(Boolean)
}

export function SourceCard({
  source,
  onSelect,
}: {
  source: KnowledgeItem
  onSelect: () => void
}) {
  const categoryColor = categoryColors[source.category] || "#ff6b35"

  return (
    <button
      onClick={onSelect}
      className="text-left rounded-xl border border-border bg-card p-4 hover:border-primary/50 hover:shadow-md transition-all duration-200 group"
    >
      <p className="text-sm font-medium text-foreground truncate mb-1.5 group-hover:text-primary transition-colors">
        {source.title}
      </p>
      <Badge
        className="mb-2 text-xs border-0"
        style={{
          backgroundColor: `${categoryColor}20`,
          color: categoryColor,
        }}
      >
        {source.category}
      </Badge>
      <p className="text-xs text-muted-foreground line-clamp-2 mb-3">
        {source.summary}
      </p>
      <div className="flex items-center justify-between">
        <div className="flex flex-wrap gap-1">
          {source.tags.slice(0, 3).map((tag) => (
            <Badge key={tag} variant="secondary" className="text-xs">
              {tag}
            </Badge>
          ))}
        </div>
        <span className="text-xs text-muted-foreground">
          {source.citationCount} 引用
        </span>
      </div>
    </button>
  )
}

interface AskMessage {
  id: string
  role: "user" | "assistant"
  content: string
  sources?: string[]
  isLoading?: boolean
}

export function AskTab() {
  useKnowledgeStore()
  const [question, setQuestion] = useState("")
  const [messages, setMessages] = useState<AskMessage[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const scrollRef = useRef<HTMLDivElement>(null)

  const handleAsk = async () => {
    if (!question.trim() || isLoading) return

    const userMessage: AskMessage = {
      id: Date.now().toString(),
      role: "user",
      content: question,
    }

    setMessages((prev) => [...prev, userMessage])
    setQuestion("")
    setIsLoading(true)

    try {
      const result = await askKnowledgeBase(question)
      const assistantMessage: AskMessage = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: result.answer,
        sources: result.sources,
      }
      setMessages((prev) => [...prev, assistantMessage])
    } catch {
      const assistantMessage: AskMessage = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: "抱歉，知识库问答服务暂时不可用。请稍后重试。",
      }
      setMessages((prev) => [...prev, assistantMessage])
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    scrollRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [messages])

  return (
    <div className="flex h-full flex-col max-w-3xl mx-auto">
      <ScrollArea className="flex-1 p-4">
        {messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <div className="h-12 w-12 rounded-xl bg-primary/10 flex items-center justify-center mb-4">
              <Search className="h-6 w-6 text-primary" />
            </div>
            <h3 className="text-lg font-semibold text-foreground mb-2">
              向知识库提问
            </h3>
            <p className="text-sm text-muted-foreground max-w-sm">
              针对文档提问，获取带引用来源的答案。
            </p>

            {/* Suggested Questions */}
            <div className="mt-6 flex flex-wrap justify-center gap-2">
              {["认证如何工作？", "API 速率限制是多少？", "解释数据库结构"].map(
                (q) => (
                  <button
                    key={q}
                    onClick={() => setQuestion(q)}
                    className="px-3 py-1.5 text-xs font-medium rounded-full border border-border bg-card hover:bg-muted transition-colors"
                  >
                    {q}
                  </button>
                )
              )}
            </div>
          </div>
        ) : (
          <div className="space-y-4">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={cn(
                  "flex animate-in fade-in slide-in-from-bottom-2 duration-200",
                  msg.role === "user" ? "justify-end" : "justify-start"
                )}
              >
                <div
                  className={cn(
                    "max-w-[85%] rounded-2xl px-4 py-3",
                    msg.role === "user"
                      ? "bg-secondary rounded-tr-md"
                      : "bg-card border border-border rounded-tl-md"
                  )}
                >
                  {msg.sources && msg.sources.length > 0 && (
                    <div className="flex flex-wrap gap-1.5 mb-2">
                      {msg.sources.map((source) => (
                        <Badge
                          key={source}
                          variant="secondary"
                          className="text-xs bg-primary/10 text-primary border-0"
                        >
                          {source}
                        </Badge>
                      ))}
                    </div>
                  )}
                  <p className="text-sm text-foreground">{msg.content}</p>
                </div>
              </div>
            ))}

            {isLoading && (
              <div className="flex justify-start animate-in fade-in duration-200">
                <div className="rounded-2xl rounded-tl-md bg-card border border-border px-4 py-3">
                  <div className="flex items-center gap-2">
                    <Loader2 className="h-4 w-4 animate-spin text-primary" />
                    <span className="text-sm text-muted-foreground">正在搜索知识库...</span>
                  </div>
                </div>
              </div>
            )}

            <div ref={scrollRef} />
          </div>
        )}
      </ScrollArea>

      <div className="shrink-0 border-t border-border p-4">
        <div className="flex gap-3">
          <Input
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleAsk()}
            placeholder="输入问题..."
            className="flex-1"
            disabled={isLoading}
          />
          <Button
            onClick={handleAsk}
            className="bg-primary hover:bg-primary/90"
            disabled={!question.trim() || isLoading}
          >
            {isLoading ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Send className="h-4 w-4" />
            )}
          </Button>
        </div>
      </div>
    </div>
  )
}
