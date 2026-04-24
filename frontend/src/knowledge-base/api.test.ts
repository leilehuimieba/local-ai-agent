import { describe, expect, it, vi } from "vitest";
import {
  fetchKnowledgeItems,
  createKnowledgeItem,
  updateKnowledgeItem,
  deleteKnowledgeItem,
  searchKnowledgeItems,
} from "./api";

function mockFetch(response: unknown, ok = true) {
  return vi.fn().mockResolvedValue({
    ok,
    status: ok ? 200 : 400,
    text: vi.fn().mockResolvedValue(""),
    json: vi.fn().mockResolvedValue(response),
  } as unknown as Response);
}

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

describe("api", () => {
  it("fetchKnowledgeItems 返回列表", async () => {
    globalThis.fetch = mockFetch({ items: [sampleItem], categories: ["cat1"], tags: ["a"] });
    const result = await fetchKnowledgeItems();
    expect(result.items).toHaveLength(1);
    expect(result.items[0].title).toBe("t1");
    expect(globalThis.fetch).toHaveBeenCalledWith("/api/v1/knowledge/items");
  });

  it("createKnowledgeItem 创建条目", async () => {
    globalThis.fetch = mockFetch(sampleItem);
    const result = await createKnowledgeItem({
      title: "t1", summary: "s1", content: "c1", category: "cat1", tags: ["a"],
    });
    expect(result.id).toBe("kb-1");
    expect(globalThis.fetch).toHaveBeenCalledWith(
      "/api/v1/knowledge/items",
      expect.objectContaining({ method: "POST" }),
    );
  });

  it("updateKnowledgeItem 更新条目", async () => {
    globalThis.fetch = mockFetch({ ...sampleItem, title: "t2" });
    const result = await updateKnowledgeItem("kb-1", { title: "t2" });
    expect(result.title).toBe("t2");
    expect(globalThis.fetch).toHaveBeenCalledWith(
      "/api/v1/knowledge/items/kb-1",
      expect.objectContaining({ method: "PUT" }),
    );
  });

  it("deleteKnowledgeItem 删除条目", async () => {
    globalThis.fetch = mockFetch(undefined);
    await deleteKnowledgeItem("kb-1");
    expect(globalThis.fetch).toHaveBeenCalledWith(
      "/api/v1/knowledge/items/kb-1",
      expect.objectContaining({ method: "DELETE" }),
    );
  });

  it("searchKnowledgeItems 搜索条目", async () => {
    globalThis.fetch = mockFetch({ items: [sampleItem], categories: [], tags: [] });
    const result = await searchKnowledgeItems("q");
    expect(result.items).toHaveLength(1);
    expect(globalThis.fetch).toHaveBeenCalledWith("/api/v1/knowledge/search?q=q");
  });

  it("fetchKnowledgeItems 失败时抛出异常", async () => {
    globalThis.fetch = mockFetch({}, false);
    await expect(fetchKnowledgeItems()).rejects.toThrow("获取知识库失败");
  });
});
