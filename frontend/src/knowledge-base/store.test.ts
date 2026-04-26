import { describe, expect, it, vi } from "vitest";
import { knowledgeStore } from "./store";

vi.mock("./api", () => ({
  fetchKnowledgeItems: vi.fn(),
  createKnowledgeItem: vi.fn(),
  updateKnowledgeItem: vi.fn(),
  deleteKnowledgeItem: vi.fn(),
  searchKnowledgeItems: vi.fn(),
}));

import {
  fetchKnowledgeItems,
  createKnowledgeItem,
  updateKnowledgeItem,
  deleteKnowledgeItem,
  searchKnowledgeItems,
} from "./api";

const sampleItem = {
  id: "kb-1",
  title: "t1",
  summary: "s1",
  content: "c1",
  category: "cat1",
  tags: ["a"],
  source: "src",
  citationCount: 0,
  createdAt: "2026-01-01T00:00:00Z",
  updatedAt: "2026-01-01T00:00:00Z",
};

const sampleItem2 = {
  ...sampleItem,
  id: "kb-2",
  title: "t2",
  category: "cat2",
  tags: ["b"],
};

describe("knowledgeStore", () => {
  it("getAll 返回条目列表", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [sampleItem],
      categories: ["cat1"],
      tags: ["a"],
    });
    const items = await knowledgeStore.getAll();
    expect(items).toHaveLength(1);
    expect(items[0].id).toBe("kb-1");
  });

  it("getById 返回对应条目", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [sampleItem, sampleItem2],
      categories: ["cat1", "cat2"],
      tags: ["a", "b"],
    });
    const item = await knowledgeStore.getById("kb-2");
    expect(item?.title).toBe("t2");
  });

  it("add 创建后返回新条目", async () => {
    vi.mocked(createKnowledgeItem).mockResolvedValue(sampleItem);
    const item = await knowledgeStore.add({
      title: "t1", summary: "s1", content: "c1", category: "cat1", tags: ["a"],
    });
    expect(item.id).toBe("kb-1");
  });

  it("update 调用 api 并返回结果", async () => {
    vi.mocked(updateKnowledgeItem).mockResolvedValue({ ...sampleItem, title: "t2" });
    const item = await knowledgeStore.update("kb-1", { title: "t2" });
    expect(item?.title).toBe("t2");
  });

  it("update 失败时返回 null", async () => {
    vi.mocked(updateKnowledgeItem).mockRejectedValue(new Error("fail"));
    const item = await knowledgeStore.update("kb-1", { title: "t2" });
    expect(item).toBeNull();
  });

  it("remove 成功返回 true", async () => {
    vi.mocked(deleteKnowledgeItem).mockResolvedValue(undefined);
    const ok = await knowledgeStore.remove("kb-1");
    expect(ok).toBe(true);
  });

  it("remove 失败返回 false", async () => {
    vi.mocked(deleteKnowledgeItem).mockRejectedValue(new Error("fail"));
    const ok = await knowledgeStore.remove("kb-1");
    expect(ok).toBe(false);
  });

  it("search 返回匹配结果", async () => {
    vi.mocked(searchKnowledgeItems).mockResolvedValue({
      items: [sampleItem],
      categories: [],
      tags: [],
    });
    const items = await knowledgeStore.search("q");
    expect(items).toHaveLength(1);
  });

  it("search 空查询返回全部", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [sampleItem, sampleItem2],
      categories: [],
      tags: [],
    });
    const items = await knowledgeStore.search("  ");
    expect(items).toHaveLength(2);
  });

  it("filterByCategory 按分类筛选", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [sampleItem, sampleItem2],
      categories: ["cat1", "cat2"],
      tags: ["a", "b"],
    });
    const items = await knowledgeStore.filterByCategory("cat2");
    expect(items).toHaveLength(1);
    expect(items[0].category).toBe("cat2");
  });

  it("filterByTag 按标签筛选", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [sampleItem, sampleItem2],
      categories: ["cat1", "cat2"],
      tags: ["a", "b"],
    });
    const items = await knowledgeStore.filterByTag("b");
    expect(items).toHaveLength(1);
    expect(items[0].tags).toContain("b");
  });

  it("getCategories 返回含全部前缀的分类", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [],
      categories: ["cat1"],
      tags: [],
    });
    const cats = await knowledgeStore.getCategories();
    expect(cats).toEqual(["全部", "cat1"]);
  });

  it("getTags 返回标签列表", async () => {
    vi.mocked(fetchKnowledgeItems).mockResolvedValue({
      items: [],
      categories: [],
      tags: ["a", "b"],
    });
    const tags = await knowledgeStore.getTags();
    expect(tags).toEqual(["a", "b"]);
  });
});
