import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent, waitFor } from "@testing-library/react"
import { SourcesTab, AskTab } from "../knowledge-view"
import { useKnowledgeStore } from "@/lib/local-agent/store"

Element.prototype.scrollIntoView = vi.fn()

vi.mock("@/lib/local-agent/api", async () => {
  const actual = await vi.importActual<typeof import("@/lib/local-agent/api")>("@/lib/local-agent/api")
  return {
    ...actual,
    createKnowledgeItem: vi.fn(),
    uploadKnowledgeFile: vi.fn(),
    askKnowledgeBase: vi.fn().mockResolvedValue({ answer: "Test answer", sources: ["s1"] }),
  }
})

const mockStore = {
  items: [
    { id: "k1", title: "Item A", content: "content a", category: "Architecture", tags: ["tag1"], source: "manual", citationCount: 0, createdAt: "2024-01-01", updatedAt: "2024-01-01" },
    { id: "k2", title: "Item B", content: "content b", category: "API", tags: ["tag2"], source: "manual", citationCount: 0, createdAt: "2024-01-01", updatedAt: "2024-01-01" },
  ],
  categories: ["Architecture", "API"],
  allTags: ["tag1", "tag2"],
  selectedItem: null,
  searchQuery: "",
  selectedCategory: null,
  selectedTags: [] as string[],
  setItems: vi.fn(),
  setSelectedItem: vi.fn(),
  setSearchQuery: vi.fn(),
  setSelectedCategory: vi.fn(),
  toggleTag: vi.fn(),
  addItem: vi.fn(),
  updateItem: vi.fn(),
  removeItem: vi.fn(),
  loadItems: vi.fn(),
}

vi.mock("@/lib/local-agent/store", () => ({
  useKnowledgeStore: vi.fn((selector?: (s: unknown) => unknown) => {
    return selector ? selector(mockStore) : mockStore
  }),
}))

describe("SourcesTab", () => {
  it("renders source grid with items", () => {
    render(<SourcesTab />)
    expect(screen.getByText("Item A")).toBeInTheDocument()
    expect(screen.getByText("Item B")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("搜索...")).toBeInTheDocument()
  })

  it("renders sidebar filters", () => {
    render(<SourcesTab />)
    expect(screen.getByPlaceholderText("搜索...")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("搜索标签...")).toBeInTheDocument()
  })
})

describe("AskTab", () => {
  it("renders empty ask state", () => {
    render(<AskTab />)
    expect(screen.getByText("向知识库提问")).toBeInTheDocument()
    expect(screen.getByPlaceholderText("输入问题...")).toBeInTheDocument()
  })

  it("sends question and shows answer", async () => {
    const { container } = render(<AskTab />)
    const input = screen.getByPlaceholderText("输入问题...")
    fireEvent.change(input, { target: { value: "What is AI?" } })
    const sendButton = container.querySelector("button[class*='bg-primary']") as HTMLButtonElement
    fireEvent.click(sendButton)
    await waitFor(() => {
      expect(screen.getByText("Test answer")).toBeInTheDocument()
    })
  })

  it("sets question from suggested questions", () => {
    render(<AskTab />)
    fireEvent.click(screen.getByText("认证如何工作？"))
    expect(screen.getByPlaceholderText("输入问题...")).toHaveValue("认证如何工作？")
  })

  it("sends question on Enter key", async () => {
    render(<AskTab />)
    const input = screen.getByPlaceholderText("输入问题...")
    fireEvent.change(input, { target: { value: "Hello" } })
    fireEvent.keyDown(input, { key: "Enter" })
    await waitFor(() => {
      expect(screen.getByText("Test answer")).toBeInTheDocument()
    })
  })

  it("shows error when askKnowledgeBase fails", async () => {
    const { askKnowledgeBase } = await import("@/lib/local-agent/api")
    vi.mocked(askKnowledgeBase).mockRejectedValueOnce(new Error("fail"))
    render(<AskTab />)
    const input = screen.getByPlaceholderText("输入问题...")
    fireEvent.change(input, { target: { value: "Hello" } })
    fireEvent.keyDown(input, { key: "Enter" })
    await waitFor(() => {
      expect(screen.getByText("抱歉，知识库问答服务暂时不可用。请稍后重试。")).toBeInTheDocument()
    })
  })
})
