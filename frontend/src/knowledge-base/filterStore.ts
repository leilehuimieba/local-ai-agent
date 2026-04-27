import { create } from "zustand";

type KnowledgeFilterState = {
  category: string;
  tag: string;
  search: string;
  sortBy: "updated" | "created" | "cited";
  setCategory: (c: string) => void;
  setTag: (t: string) => void;
  setSearch: (s: string) => void;
  setSortBy: (s: "updated" | "created" | "cited") => void;
};

export const useKnowledgeFilterStore = create<KnowledgeFilterState>((set) => ({
  category: "全部",
  tag: "",
  search: "",
  sortBy: "updated",
  setCategory: (category) => set({ category, tag: "" }),
  setTag: (tag) => set({ tag }),
  setSearch: (search) => set({ search }),
  setSortBy: (sortBy) => set({ sortBy }),
}));
