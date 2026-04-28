"use client"

import { useState, useCallback, useEffect, useRef, useMemo } from "react"
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
  Search,
  Send,
  ZoomIn,
  ZoomOut,
  Maximize,
  FileText,
  Code,
  BookOpen,
  Database,
  Plus,
  Upload,
  X,
  Loader2,
} from "lucide-react"
import { useKnowledgeStore } from "@/lib/local-agent/store"
import { mockKnowledgeItems, categoryColors } from "@/lib/local-agent/mock-data"
import type { KnowledgeItem } from "@/lib/local-agent/types"
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
            <TabsTrigger value="graph" className="data-[state=active]:bg-muted">
              图谱
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent value="sources" className="flex-1 m-0 overflow-hidden">
          <SourcesTab />
        </TabsContent>

        <TabsContent value="ask" className="flex-1 m-0 overflow-hidden">
          <AskTab />
        </TabsContent>

        <TabsContent value="graph" className="flex-1 m-0 overflow-hidden">
          <GraphTab />
        </TabsContent>
      </Tabs>
    </div>
  )
}

function SourcesTab() {
  const {
    items,
    allTags,
    searchQuery,
    selectedCategory,
    selectedTags,
    selectedItem,
    setSearchQuery,
    setSelectedCategory,
    toggleTag,
    setSelectedItem,
    updateItem,
  } = useKnowledgeStore()

  const [isEditing, setIsEditing] = useState(false)
  const [editContent, setEditContent] = useState("")

  const filteredSources = useMemo(() => {
    return items.filter((source) => {
      if (selectedCategory && selectedCategory !== "all" && source.category !== selectedCategory) {
        return false
      }
      if (searchQuery && !source.title.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false
      }
      if (selectedTags.length > 0 && !selectedTags.some((tag) => source.tags.includes(tag))) {
        return false
      }
      return true
    })
  }, [items, selectedCategory, searchQuery, selectedTags])

  const handleEditStart = () => {
    if (selectedItem) {
      setEditContent(selectedItem.content)
      setIsEditing(true)
    }
  }

  const handleEditSave = () => {
    if (selectedItem) {
      updateItem(selectedItem.id, { content: editContent, updatedAt: new Date().toISOString() })
      setIsEditing(false)
    }
  }

  const handleSelectItem = (item: KnowledgeItem) => {
    setSelectedItem(item)
    setIsEditing(false)
  }

  return (
    <div className="flex h-full">
      {/* Left Panel - Filters */}
      <div className="w-56 shrink-0 border-r border-border p-4 hidden md:flex md:flex-col">
        <div className="space-y-4 flex-1">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="搜索..."
              className="pl-9"
            />
          </div>

          {/* Category Filter */}
          <div>
            <p className="text-xs font-medium text-muted-foreground mb-2 uppercase tracking-wider">
              Categories
            </p>
            <div className="flex flex-wrap gap-1.5">
              {categories.map((cat) => (
                <button
                  key={cat.id}
                  onClick={() => setSelectedCategory(cat.id === "all" ? null : cat.id)}
                  className={cn(
                    "px-2.5 py-1 text-xs font-medium rounded-md transition-colors duration-200",
                    (selectedCategory === cat.id || (cat.id === "all" && !selectedCategory))
                      ? "bg-primary text-primary-foreground"
                      : "bg-muted text-muted-foreground hover:text-foreground"
                  )}
                >
                  {cat.label}
                </button>
              ))}
            </div>
          </div>

          {/* Tag Cloud */}
          <div>
            <p className="text-xs font-medium text-muted-foreground mb-2 uppercase tracking-wider">
              Tags
            </p>
            <div className="flex flex-wrap gap-1.5">
              {allTags.map((tag) => (
                <Badge
                  key={tag}
                  variant={selectedTags.includes(tag) ? "default" : "outline"}
                  className={cn(
                    "text-xs cursor-pointer transition-colors duration-200",
                    selectedTags.includes(tag)
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-muted"
                  )}
                  onClick={() => toggleTag(tag)}
                >
                  {tag}
                </Badge>
              ))}
            </div>
          </div>
        </div>

        {/* Add Button */}
        <div className="pt-4 border-t border-border mt-4 space-y-2">
          <Button variant="outline" className="w-full justify-start gap-2" size="sm">
            <Plus className="h-4 w-4" />
            添加资料
          </Button>
          <Button variant="outline" className="w-full justify-start gap-2" size="sm">
            <Upload className="h-4 w-4" />
            上传文件
          </Button>
        </div>
      </div>

      {/* Center Panel - Card Grid */}
      <ScrollArea className="flex-1">
        <div className="p-4">
          {filteredSources.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center">
              <div className="h-12 w-12 rounded-xl bg-muted flex items-center justify-center mb-4">
                <FileText className="h-6 w-6 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-medium text-foreground mb-2">未找到资料</h3>
              <p className="text-sm text-muted-foreground max-w-sm">
                尝试调整筛选条件或添加新资料。
              </p>
            </div>
          ) : (
            <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
              {filteredSources.map((source) => (
                <SourceCard
                  key={source.id}
                  source={source}
                  onSelect={() => handleSelectItem(source)}
                />
              ))}
            </div>
          )}
        </div>
      </ScrollArea>

      {/* Right Panel - Detail Sheet */}
      <Sheet open={!!selectedItem} onOpenChange={() => setSelectedItem(null)}>
        <SheetContent className="w-[400px] sm:w-[540px] overflow-y-auto">
          {selectedItem && (
            <>
              <SheetHeader className="pb-4">
                <div className="flex items-start justify-between">
                  <div>
                    <SheetTitle className="text-lg">{selectedItem.title}</SheetTitle>
                    <Badge
                      className="mt-2 border-0"
                      style={{
                        backgroundColor: `${categoryColors[selectedItem.category]}20`,
                        color: categoryColors[selectedItem.category],
                      }}
                    >
                      {selectedItem.category}
                    </Badge>
                  </div>
                  <div className="flex items-center gap-2">
                    <Switch
                      id="edit-mode"
                      checked={isEditing}
                      onCheckedChange={(checked) => {
                        if (checked) handleEditStart()
                        else setIsEditing(false)
                      }}
                    />
                    <Label htmlFor="edit-mode" className="text-xs">
                      Edit
                    </Label>
                  </div>
                </div>

                {/* Meta Row */}
                <div className="flex items-center gap-4 text-xs text-muted-foreground mt-3">
                  <span>{selectedItem.citationCount} 引用</span>
                  <span>Source: {selectedItem.source}</span>
                  <span>更新于: {new Date(selectedItem.updatedAt).toLocaleDateString("zh-CN")}</span>
                </div>
              </SheetHeader>

              <div className="prose prose-sm dark:prose-invert max-w-none">
                {isEditing ? (
                  <div className="space-y-3">
                    <Textarea
                      value={editContent}
                      onChange={(e) => setEditContent(e.target.value)}
                      className="min-h-[300px] font-mono text-sm resize-none"
                    />
                    <div className="flex gap-2">
                      <Button onClick={handleEditSave} size="sm">
                        Save Changes
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => setIsEditing(false)}
                      >
                        Cancel
                      </Button>
                    </div>
                  </div>
                ) : (
                  <pre className="whitespace-pre-wrap text-sm text-foreground bg-muted/50 p-4 rounded-lg overflow-x-auto">
                    {selectedItem.content}
                  </pre>
                )}
              </div>

              <div className="mt-4 pt-4 border-t border-border">
                <p className="text-xs font-medium text-muted-foreground mb-2">Tags</p>
                <div className="flex flex-wrap gap-1.5">
                  {selectedItem.tags.map((tag) => (
                    <Badge key={tag} variant="secondary" className="text-xs">
                      {tag}
                    </Badge>
                  ))}
                </div>
              </div>
            </>
          )}
        </SheetContent>
      </Sheet>
    </div>
  )
}

function SourceCard({
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

function AskTab() {
  const { items } = useKnowledgeStore()
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

    // Simulate RAG response
    setTimeout(() => {
      const relevantSources = items
        .filter((item) =>
          question.toLowerCase().split(" ").some((word) =>
            item.title.toLowerCase().includes(word) ||
            item.content.toLowerCase().includes(word)
          )
        )
        .slice(0, 2)

      const assistantMessage: AskMessage = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: relevantSources.length > 0
          ? `Based on your knowledge base, I found relevant information about "${question}". ${relevantSources[0]?.summary || "The documentation covers this topic in detail."}`
          : "I couldn't find specific information about this in your knowledge base. Try rephrasing your question or adding relevant sources.",
        sources: relevantSources.map((s) => s.title),
      }

      setMessages((prev) => [...prev, assistantMessage])
      setIsLoading(false)
    }, 1500)
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
                    <span className="text-sm text-muted-foreground">Searching knowledge base...</span>
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

interface GraphNode {
  id: string
  title: string
  category: string
  x: number
  y: number
  vx: number
  vy: number
}

function GraphTab() {
  const { items } = useKnowledgeStore()
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [zoom, setZoom] = useState(1)
  const [hoveredNode, setHoveredNode] = useState<GraphNode | null>(null)
  const [pan, setPan] = useState({ x: 0, y: 0 })
  const nodesRef = useRef<GraphNode[]>([])
  const animationRef = useRef<number>()

  // Initialize nodes
  useEffect(() => {
    const centerX = 250
    const centerY = 200
    nodesRef.current = items.map((item, i) => ({
      id: item.id,
      title: item.title,
      category: item.category,
      x: centerX + Math.cos((i * 2 * Math.PI) / items.length) * 120,
      y: centerY + Math.sin((i * 2 * Math.PI) / items.length) * 120,
      vx: 0,
      vy: 0,
    }))
  }, [items])

  const drawGraph = useCallback(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    const dpr = window.devicePixelRatio || 1
    const rect = canvas.getBoundingClientRect()
    canvas.width = rect.width * dpr
    canvas.height = rect.height * dpr
    ctx.scale(dpr, dpr)

    ctx.clearRect(0, 0, rect.width, rect.height)

    ctx.save()
    ctx.translate(pan.x, pan.y)
    ctx.scale(zoom, zoom)

    const nodes = nodesRef.current

    // Draw edges
    ctx.strokeStyle = "rgba(128, 128, 128, 0.15)"
    ctx.lineWidth = 1
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        // Only draw edges between nodes in same or related categories
        if (
          nodes[i].category === nodes[j].category ||
          Math.random() > 0.7
        ) {
          ctx.beginPath()
          ctx.moveTo(nodes[i].x, nodes[i].y)
          ctx.lineTo(nodes[j].x, nodes[j].y)
          ctx.stroke()
        }
      }
    }

    // Draw nodes
    nodes.forEach((node) => {
      const color = categoryColors[node.category] || "#ff6b35"
      const isHovered = hoveredNode?.id === node.id
      const radius = isHovered ? 24 : 18

      // Glow effect for hovered node
      if (isHovered) {
        ctx.beginPath()
        ctx.arc(node.x, node.y, radius + 8, 0, 2 * Math.PI)
        ctx.fillStyle = `${color}30`
        ctx.fill()
      }

      // Main node
      ctx.beginPath()
      ctx.arc(node.x, node.y, radius, 0, 2 * Math.PI)
      ctx.fillStyle = color
      ctx.fill()

      // Border
      ctx.strokeStyle = isHovered ? "#ffffff" : `${color}80`
      ctx.lineWidth = isHovered ? 3 : 2
      ctx.stroke()
    })

    ctx.restore()
  }, [zoom, hoveredNode, pan])

  useEffect(() => {
    const animate = () => {
      drawGraph()
      animationRef.current = requestAnimationFrame(animate)
    }
    animate()

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current)
      }
    }
  }, [drawGraph])

  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const x = (e.clientX - rect.left - pan.x) / zoom
    const y = (e.clientY - rect.top - pan.y) / zoom

    const nodes = nodesRef.current
    const hovered = nodes.find(
      (node) => Math.sqrt((node.x - x) ** 2 + (node.y - y) ** 2) < 20
    )
    setHoveredNode(hovered || null)
  }

  return (
    <div className="relative h-full w-full bg-muted/30">
      <canvas
        ref={canvasRef}
        className="h-full w-full cursor-crosshair"
        onMouseMove={handleCanvasMouseMove}
      />

      {/* Hover Tooltip */}
      {hoveredNode && (
        <div
          className="absolute pointer-events-none bg-card border border-border rounded-lg px-3 py-2 shadow-lg z-10"
          style={{
            left: hoveredNode.x * zoom + pan.x + 30,
            top: hoveredNode.y * zoom + pan.y - 10,
          }}
        >
          <p className="text-sm font-medium text-foreground">{hoveredNode.title}</p>
          <Badge
            className="mt-1 text-xs border-0"
            style={{
              backgroundColor: `${categoryColors[hoveredNode.category]}20`,
              color: categoryColors[hoveredNode.category],
            }}
          >
            {hoveredNode.category}
          </Badge>
        </div>
      )}

      {/* Legend */}
      <div className="absolute top-4 left-4 bg-card/80 backdrop-blur-sm border border-border rounded-lg p-3 space-y-2">
        <p className="text-xs font-medium text-muted-foreground uppercase tracking-wider">Categories</p>
        <div className="space-y-1.5">
          {Object.entries(categoryColors).map(([category, color]) => (
            <div key={category} className="flex items-center gap-2">
              <div
                className="h-3 w-3 rounded-full"
                style={{ backgroundColor: color }}
              />
              <span className="text-xs text-foreground">{category}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Toolbar */}
      <div className="absolute bottom-4 right-4 flex items-center gap-1 rounded-lg border border-border bg-card p-1 shadow-sm">
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={() => {
            setZoom(1)
            setPan({ x: 0, y: 0 })
          }}
        >
          <Maximize className="h-4 w-4" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={() => setZoom(Math.max(0.5, zoom - 0.1))}
        >
          <ZoomOut className="h-4 w-4" />
        </Button>
        <span className="text-xs text-muted-foreground w-12 text-center">
          {Math.round(zoom * 100)}%
        </span>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={() => setZoom(Math.min(2, zoom + 0.1))}
        >
          <ZoomIn className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}
