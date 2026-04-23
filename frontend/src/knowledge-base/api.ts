import { KnowledgeItem } from "./types";

export type CreateKnowledgeItemRequest = {
  title: string;
  summary: string;
  content: string;
  category: string;
  tags: string[];
  source?: string;
};

export type UpdateKnowledgeItemRequest = Partial<CreateKnowledgeItemRequest>;

export type ListResponse = {
  items: KnowledgeItem[];
  categories: string[];
  tags: string[];
};

async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || `HTTP ${response.status}`;
}

export async function fetchKnowledgeItems(): Promise<ListResponse> {
  const response = await fetch("/api/v1/knowledge/items");
  if (!response.ok) {
    throw new Error(`获取知识库失败: ${await readErrorText(response)}`);
  }
  return (await response.json()) as ListResponse;
}

export async function createKnowledgeItem(
  payload: CreateKnowledgeItemRequest,
): Promise<KnowledgeItem> {
  const response = await fetch("/api/v1/knowledge/items", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!response.ok) {
    throw new Error(`创建知识条目失败: ${await readErrorText(response)}`);
  }
  return (await response.json()) as KnowledgeItem;
}

export async function updateKnowledgeItem(
  id: string,
  payload: UpdateKnowledgeItemRequest,
): Promise<KnowledgeItem> {
  const response = await fetch(`/api/v1/knowledge/items/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!response.ok) {
    throw new Error(`更新知识条目失败: ${await readErrorText(response)}`);
  }
  return (await response.json()) as KnowledgeItem;
}

export async function deleteKnowledgeItem(id: string): Promise<void> {
  const response = await fetch(`/api/v1/knowledge/items/${id}`, {
    method: "DELETE",
  });
  if (!response.ok) {
    throw new Error(`删除知识条目失败: ${await readErrorText(response)}`);
  }
}

export async function searchKnowledgeItems(query: string): Promise<ListResponse> {
  const response = await fetch(`/api/v1/knowledge/search?q=${encodeURIComponent(query)}`);
  if (!response.ok) {
    throw new Error(`搜索知识库失败: ${await readErrorText(response)}`);
  }
  return (await response.json()) as ListResponse;
}
