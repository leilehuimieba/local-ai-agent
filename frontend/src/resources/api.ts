import { MemoryEntry, MemoryListResponse } from "../shared/contracts";

export async function fetchMemories(signal?: AbortSignal): Promise<MemoryEntry[]> {
  const response = await fetch("/api/v1/memories", { signal });
  if (!response.ok) {
    throw new Error(`加载记忆失败: ${await readErrorText(response)}`);
  }
  const payload = (await response.json()) as MemoryListResponse;
  return payload.items;
}

export async function deleteMemory(memoryId: string): Promise<MemoryEntry[]> {
  const response = await fetch("/api/v1/memories/delete", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ memory_id: memoryId }),
  });
  if (!response.ok) {
    throw new Error(`删除记忆失败: ${await readErrorText(response)}`);
  }
  const payload = (await response.json()) as MemoryListResponse;
  return payload.items;
}

async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || String(response.status);
}
