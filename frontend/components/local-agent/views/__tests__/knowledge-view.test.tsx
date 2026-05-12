import { describe, it, expect, vi, beforeEach } from "vitest"
import { render, screen, waitFor, fireEvent } from "@testing-library/react"
import { KnowledgeView, KnowledgeEmpty, isKnowledgeVisible, SourcesTab } from "../knowledge-view"
import { useKnowledgeStore } from "@/lib/local-agent/store"
import type { KnowledgeItem } from "@/lib/local-agent/types"

vi.mock("@/lib/local-agent/api", async () => {
  const actual = await vi.importActual<typeof import("@/lib/local-agent/api")>("@/lib/local-agent/api")
  return {
    ...actual,
    fetchKnowledgeItems: vi.fn().mockResolvedValue({ items: [], categories: [], tags: [] }),
    createKnowledgeItem: vi.fn().mockResolvedValue(buildItem()),
  }
})

function buildItem(overrides: Partial<KnowledgeItem> = {}): KnowledgeItem {
  return {
    id: "k1",
    title: "Test Item",
    content: "content",
    category: "Architecture",
    tags: ["tag1"],
    source: "manual",
    citationCount: 0,
    createdAt: "2024-01-01",
    updatedAt: "2024-01-01",
    ...overrides,
  }
}

describe("isKnowledgeVisible", () => {
  const baseStore = {
    items: [],
    categories: [],
    allTags: [],
    selectedItem: null,
    searchQuery: "",
    selectedCategory: null,
    selectedTags: [],
    setItems: vi.fn(),
    setSelectedItem: vi.fn(),
    setSearchQuery: vi.fn(),
    setSelectedCategory: vi.fn(),
    toggleTag: vi.fn(),
    addItem: vi.fn(),
    updateItem: vi.fn(),
    removeItem: vi.fn(),
    loadItems: vi.fn(),
  } as unknown as ReturnType<typeof useKnowledgeStore.getState>

  it("returns true when no filters active", () => {
    expect(isKnowledgeVisible(buildItem(), baseStore)).toBe(true)
  })

  it("filters by category", () => {
    const store = { ...baseStore, selectedCategory: "API" }
    expect(isKnowledgeVisible(buildItem({ category: "Architecture" }), store)).toBe(false)
    expect(isKnowledgeVisible(buildItem({ category: "API" }), store)).toBe(true)
  })

  it("filters by search query", () => {
    const store = { ...baseStore, searchQuery: "alpha" }
    expect(isKnowledgeVisible(buildItem({ title: "Beta" }), store)).toBe(false)
    expect(isKnowledgeVisible(buildItem({ title: "Alpha" }), store)).toBe(true)
  })

  it("filters by tags", () => {
    const store = { ...baseStore, selectedTags: ["tag2"] }
    expect(isKnowledgeVisible(buildItem({ tags: ["tag1"] }), store)).toBe(false)
    expect(isKnowledgeVisible(buildItem({ tags: ["tag1", "tag2"] }), store)).toBe(true)
  })

  it("combines multiple filters", () => {
    const store = { ...baseStore, selectedCategory: "API", searchQuery: "test" }
    expect(isKnowledgeVisible(buildItem({ category: "API", title: "other" }), store)).toBe(false)
    expect(isKnowledgeVisible(buildItem({ category: "Architecture", title: "test" }), store)).toBe(false)
    expect(isKnowledgeVisible(buildItem({ category: "API", title: "test" }), store)).toBe(true)
  })
})

describe("KnowledgeEmpty", () => {
  it("renders empty state", () => {
    render(<KnowledgeEmpty />)
    expect(screen.getByText("未找到资料")).toBeInTheDocument()
    expect(screen.getByText("尝试调整筛选条件或添加新资料。")).toBeInTheDocument()
  })
})

describe("KnowledgeView", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    useKnowledgeStore.setState({
      items: [],
      categories: [],
      allTags: [],
      selectedItem: null,
      searchQuery: "",
      selectedCategory: null,
      selectedTags: [],
    })
  })

  it("renders with empty sources", async () => {
    render(<KnowledgeView />)
    await waitFor(() => {
      expect(screen.getByText("未找到资料")).toBeInTheDocument()
    })
  })

  it("renders source items", async () => {
    useKnowledgeStore.setState({
      items: [buildItem({ id: "k1", title: "Item A" }), buildItem({ id: "k2", title: "Item B" })],
    })
    render(<KnowledgeView />)
    await waitFor(() => {
      expect(screen.getByText("Item A")).toBeInTheDocument()
      expect(screen.getByText("Item B")).toBeInTheDocument()
    })
  })

  it("opens create dialog and submits", async () => {
    const { createKnowledgeItem } = await import("@/lib/local-agent/api")
    render(<SourcesTab />)
    fireEvent.click(screen.getByRole("button", { name: /添加资料/ }))
    await waitFor(() => {
      expect(screen.getByRole("heading", { name: "添加资料" })).toBeInTheDocument()
    })
    const titleInput = screen.getByPlaceholderText("标题")
    fireEvent.change(titleInput, { target: { value: "New Item" } })
    fireEvent.click(screen.getByText("保存"))
    await waitFor(() => {
      expect(createKnowledgeItem).toHaveBeenCalled()
    })
  })
})
