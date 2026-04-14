export async function fetchArtifactContent(path: string, signal?: AbortSignal) {
  const query = new URLSearchParams({ path }).toString();
  const response = await fetch(`/api/v1/artifacts/content?${query}`, { signal });
  if (!response.ok) {
    throw new Error(`读取产物失败: ${response.status}`);
  }
  const payload = (await response.json()) as { path: string; content: string };
  return payload.content || "";
}

