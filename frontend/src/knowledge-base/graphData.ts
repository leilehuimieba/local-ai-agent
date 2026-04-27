import { KnowledgeItem } from "./types";

export type GraphNode = {
  id: string;
  label: string;
  radius: number;
  color: string;
};

export type GraphEdge = {
  source: string;
  target: string;
  strength: number;
};

export type GraphData = {
  nodes: GraphNode[];
  edges: GraphEdge[];
};

const TAG_COLORS = [
  "#ff6b35",
  "#4ecdc4",
  "#45b7d1",
  "#96ceb4",
  "#feca57",
  "#ff9ff3",
  "#54a0ff",
  "#5f27cd",
];

function pickColor(index: number) {
  return TAG_COLORS[index % TAG_COLORS.length];
}

function readPrimaryTag(item: KnowledgeItem): string {
  return item.tags[0] || "default";
}

function extractWikiLinks(content: string): string[] {
  const links: string[] = [];
  const regex = /\[\[([^\]]+)\]\]/g;
  let match;
  while ((match = regex.exec(content)) !== null) {
    links.push(match[1].trim());
  }
  return links;
}

export function buildGraphData(items: KnowledgeItem[]): GraphData {
  const titleMap = new Map<string, string>();
  items.forEach((item) => titleMap.set(item.title.trim().toLowerCase(), item.id));

  const tagIndex = new Map<string, number>();
  let tagCounter = 0;

  const nodes: GraphNode[] = items.map((item) => {
    const tag = readPrimaryTag(item);
    if (!tagIndex.has(tag)) {
      tagIndex.set(tag, tagCounter++);
    }
    return {
      id: item.id,
      label: item.title,
      radius: Math.max(6, Math.min(20, 8 + item.citationCount * 2)),
      color: pickColor(tagIndex.get(tag) || 0),
    };
  });

  const edgeMap = new Map<string, GraphEdge>();

  items.forEach((item) => {
    const links = extractWikiLinks(item.content);
    links.forEach((linkTitle) => {
      const targetId = titleMap.get(linkTitle.toLowerCase());
      if (targetId && targetId !== item.id) {
        const key = [item.id, targetId].sort().join("-");
        const existing = edgeMap.get(key);
        if (existing) {
          existing.strength += 1;
        } else {
          edgeMap.set(key, { source: item.id, target: targetId, strength: 1 });
        }
      }
    });

    items.forEach((other) => {
      if (other.id === item.id) return;
      const sharedTags = item.tags.filter((t) => other.tags.includes(t));
      if (sharedTags.length > 0) {
        const key = [item.id, other.id].sort().join("-");
        const existing = edgeMap.get(key);
        if (existing) {
          existing.strength += sharedTags.length * 0.5;
        } else {
          edgeMap.set(key, { source: item.id, target: other.id, strength: sharedTags.length * 0.5 });
        }
      }
    });
  });

  return { nodes, edges: Array.from(edgeMap.values()) };
}
