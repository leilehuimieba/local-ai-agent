export type KnowledgeItem = {
  id: string;
  title: string;
  summary: string;
  content: string;
  category: string;
  tags: string[];
  source?: string;
  citationCount: number;
  createdAt: string;
  updatedAt: string;
};

export type KnowledgeFilter = {
  category: string;
  tag: string;
  search: string;
  sortBy: "updated" | "created" | "cited";
};
