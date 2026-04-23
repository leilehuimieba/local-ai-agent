import { KnowledgeItem } from "./types";
import {
  fetchKnowledgeItems,
  createKnowledgeItem,
  updateKnowledgeItem,
  deleteKnowledgeItem,
  searchKnowledgeItems,
} from "./api";

export const knowledgeStore = {
  async getAll(): Promise<KnowledgeItem[]> {
    const response = await fetchKnowledgeItems();
    return response.items;
  },

  async getById(id: string): Promise<KnowledgeItem | undefined> {
    const response = await fetchKnowledgeItems();
    return response.items.find((i) => i.id === id);
  },

  async add(
    item: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">,
  ): Promise<KnowledgeItem> {
    return createKnowledgeItem({
      title: item.title,
      summary: item.summary,
      content: item.content,
      category: item.category,
      tags: item.tags,
      source: item.source,
    });
  },

  async update(id: string, patch: Partial<Omit<KnowledgeItem, "id" | "createdAt">>): Promise<KnowledgeItem | null> {
    try {
      return await updateKnowledgeItem(id, {
        title: patch.title,
        summary: patch.summary,
        content: patch.content,
        category: patch.category,
        tags: patch.tags,
        source: patch.source,
      });
    } catch {
      return null;
    }
  },

  async remove(id: string): Promise<boolean> {
    try {
      await deleteKnowledgeItem(id);
      return true;
    } catch {
      return false;
    }
  },

  async search(query: string): Promise<KnowledgeItem[]> {
    if (!query.trim()) {
      const response = await fetchKnowledgeItems();
      return response.items;
    }
    const response = await searchKnowledgeItems(query.trim());
    return response.items;
  },

  async filterByCategory(category: string): Promise<KnowledgeItem[]> {
    const response = await fetchKnowledgeItems();
    if (!category || category === "全部") return response.items;
    return response.items.filter((i) => i.category === category);
  },

  async filterByTag(tag: string): Promise<KnowledgeItem[]> {
    const response = await fetchKnowledgeItems();
    if (!tag) return response.items;
    return response.items.filter((i) => i.tags.includes(tag));
  },

  async getCategories(): Promise<string[]> {
    const response = await fetchKnowledgeItems();
    return ["全部", ...response.categories];
  },

  async getTags(): Promise<string[]> {
    const response = await fetchKnowledgeItems();
    return response.tags;
  },
};
