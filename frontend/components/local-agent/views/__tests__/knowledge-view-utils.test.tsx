import { describe, it, expect, vi } from "vitest"
import { render, screen, fireEvent } from "@testing-library/react"
import {
  CategoryButton,
  TagFilter,
  KnowledgeTags,
  KnowledgeDraftForm,
  emptyKnowledgeDraft,
  normalizeDraft,
  splitTags,
  SourceCard,
} from "../knowledge-view"
import type { KnowledgeItem } from "@/lib/local-agent/types"

describe("CategoryButton", () => {
  it("renders active button", () => {
    const onSelect = vi.fn()
    render(<CategoryButton cat={{ id: "doc", label: "文档" }} selected="doc" onSelect={onSelect} />)
    fireEvent.click(screen.getByText("文档"))
    expect(onSelect).toHaveBeenCalledWith("doc")
  })

  it("renders all as active when no selection", () => {
    render(<CategoryButton cat={{ id: "all", label: "全部" }} selected={null} onSelect={() => {}} />)
    expect(screen.getByText("全部")).toBeInTheDocument()
  })
})

describe("TagFilter", () => {
  it("renders tags and calls toggle", () => {
    const toggle = vi.fn()
    render(<TagFilter tags={["t1", "t2"]} selected={[]} toggle={toggle} />)
    fireEvent.click(screen.getByText("t1"))
    expect(toggle).toHaveBeenCalledWith("t1")
  })

  it("shows search input", () => {
    render(<TagFilter tags={["tag1", "tag2"]} selected={[]} toggle={() => {}} />)
    expect(screen.getByPlaceholderText("搜索标签...")).toBeInTheDocument()
  })
})

describe("KnowledgeTags", () => {
  it("renders tags", () => {
    const item: KnowledgeItem = {
      id: "k1",
      title: "Test",
      content: "content",
      category: "doc",
      tags: ["a", "b"],
      source: "",
      createdAt: "",
      updatedAt: "",
    }
    render(<KnowledgeTags item={item} />)
    expect(screen.getByText("a")).toBeInTheDocument()
    expect(screen.getByText("b")).toBeInTheDocument()
  })

  it("renders empty tags section", () => {
    const item: KnowledgeItem = { id: "k1", title: "Test", content: "", category: "doc", tags: [], source: "", createdAt: "", updatedAt: "" }
    render(<KnowledgeTags item={item} />)
    expect(screen.getByText("标签")).toBeInTheDocument()
  })
})

describe("KnowledgeDraftForm", () => {
  it("renders form fields", () => {
    const draft = { title: "Draft", content: "Body", category: "doc", tags: ["tag1"] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    expect(screen.getByDisplayValue("Draft")).toBeInTheDocument()
    expect(screen.getByDisplayValue("Body")).toBeInTheDocument()
  })

  it("updates title on change", () => {
    const draft = { title: "", content: "", category: "", tags: [] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    fireEvent.change(screen.getByPlaceholderText("标题"), { target: { value: "New Title" } })
    expect(setDraft).toHaveBeenCalledWith(expect.objectContaining({ title: "New Title" }))
  })

  it("updates tags on change", () => {
    const draft = { title: "", content: "", category: "", tags: [] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    fireEvent.change(screen.getByPlaceholderText(/标签/), { target: { value: "a, b" } })
    expect(setDraft).toHaveBeenCalledWith(expect.objectContaining({ tags: ["a", "b"] }))
  })

  it("updates content on change", () => {
    const draft = { title: "", content: "", category: "", tags: [] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    fireEvent.change(screen.getByPlaceholderText("正文"), { target: { value: "New Content" } })
    expect(setDraft).toHaveBeenCalledWith(expect.objectContaining({ content: "New Content" }))
  })

  it("updates category on change", () => {
    const draft = { title: "", content: "", category: "", tags: [] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    fireEvent.change(screen.getByPlaceholderText("分类"), { target: { value: "API" } })
    expect(setDraft).toHaveBeenCalledWith(expect.objectContaining({ category: "API" }))
  })

  it("updates summary on change", () => {
    const draft = { title: "", content: "", category: "", tags: [] }
    const setDraft = vi.fn()
    render(<KnowledgeDraftForm draft={draft} setDraft={setDraft} />)
    fireEvent.change(screen.getByPlaceholderText("摘要"), { target: { value: "Summary text" } })
    expect(setDraft).toHaveBeenCalledWith(expect.objectContaining({ summary: "Summary text" }))
  })
})

describe("emptyKnowledgeDraft", () => {
  it("returns empty draft", () => {
    expect(emptyKnowledgeDraft()).toEqual({ title: "", summary: "", content: "", category: "Documentation", tags: [], source: "" })
  })
})

describe("normalizeDraft", () => {
  it("fills summary from content and defaults source", () => {
    expect(normalizeDraft({ title: "T", content: "C", summary: "", category: "doc", tags: [], source: "" })).toEqual({
      title: "T",
      content: "C",
      summary: "C",
      category: "doc",
      tags: [],
      source: "manual",
    })
  })
})

describe("splitTags", () => {
  it("splits comma separated tags", () => {
    expect(splitTags("a, b, c")).toEqual(["a", "b", "c"])
  })

  it("returns empty array for empty string", () => {
    expect(splitTags("")).toEqual([])
  })
})

describe("SourceCard", () => {
  it("renders source card", () => {
    const item: KnowledgeItem = {
      id: "k1",
      title: "Source",
      content: "Content",
      category: "doc",
      tags: ["tag1"],
      source: "http://example.com",
      createdAt: "2025-01-01",
      updatedAt: "2025-01-02",
    }
    render(<SourceCard source={item} onSelect={() => {}} />)
    expect(screen.getByText("Source")).toBeInTheDocument()
    expect(screen.getByText("tag1")).toBeInTheDocument()
  })
})
