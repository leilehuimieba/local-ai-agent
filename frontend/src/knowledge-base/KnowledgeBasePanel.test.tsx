import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { KnowledgeBasePanel } from "./KnowledgeBasePanel";

const sampleItems = [
  {
    id: "kb-1",
    title: "项目架构",
    summary: "系统分层设计",
    content: "详细内容",
    category: "技术",
    tags: ["架构"],
    source: "docs",
    citationCount: 3,
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-02T00:00:00Z",
  },
  {
    id: "kb-2",
    title: "部署手册",
    summary: "生产环境部署步骤",
    content: "详细步骤",
    category: "运维",
    tags: ["部署"],
    source: "wiki",
    citationCount: 1,
    createdAt: "2026-01-03T00:00:00Z",
    updatedAt: "2026-01-03T00:00:00Z",
  },
];

vi.mock("./store", () => ({
  knowledgeStore: {
    getAll: vi.fn(),
    getCategories: vi.fn(),
    getTags: vi.fn(),
    add: vi.fn(),
    update: vi.fn(),
    remove: vi.fn(),
  },
}));

import { knowledgeStore } from "./store";

describe("KnowledgeBasePanel", () => {
  it("加载完成后渲染知识库标题和添加按钮", async () => {
    vi.mocked(knowledgeStore.getAll).mockResolvedValue(sampleItems);
    vi.mocked(knowledgeStore.getCategories).mockResolvedValue(["全部", "技术", "运维"]);
    vi.mocked(knowledgeStore.getTags).mockResolvedValue(["架构", "部署"]);

    render(<KnowledgeBasePanel />);

    await waitFor(() => {
      expect(screen.getByText("知识库")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /添加条目/ })).toBeInTheDocument();
    });
  });

  it("空状态时显示提示和添加按钮", async () => {
    vi.mocked(knowledgeStore.getAll).mockResolvedValue([]);
    vi.mocked(knowledgeStore.getCategories).mockResolvedValue(["全部"]);
    vi.mocked(knowledgeStore.getTags).mockResolvedValue([]);

    render(<KnowledgeBasePanel />);

    await waitFor(() => {
      expect(screen.getByText("还没有资料")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /添加第一篇知识/ })).toBeInTheDocument();
    });
  });

  it("渲染条目卡片列表", async () => {
    vi.mocked(knowledgeStore.getAll).mockResolvedValue(sampleItems);
    vi.mocked(knowledgeStore.getCategories).mockResolvedValue(["全部", "技术", "运维"]);
    vi.mocked(knowledgeStore.getTags).mockResolvedValue(["架构", "部署"]);

    render(<KnowledgeBasePanel />);

    await waitFor(() => {
      expect(screen.getByText("项目架构")).toBeInTheDocument();
      expect(screen.getByText("部署手册")).toBeInTheDocument();
    });
  });

  it("搜索筛选只显示匹配条目", async () => {
    vi.mocked(knowledgeStore.getAll).mockResolvedValue(sampleItems);
    vi.mocked(knowledgeStore.getCategories).mockResolvedValue(["全部", "技术", "运维"]);
    vi.mocked(knowledgeStore.getTags).mockResolvedValue(["架构", "部署"]);

    render(<KnowledgeBasePanel />);

    await waitFor(() => expect(screen.getByText("项目架构")).toBeInTheDocument());

    const searchInput = screen.getByPlaceholderText("搜索知识...") as HTMLInputElement;
    fireEvent.change(searchInput, { target: { value: "部署" } });

    await waitFor(() => {
      expect(screen.queryByText("部署手册")).toBeInTheDocument();
      expect(screen.queryByText("项目架构")).not.toBeInTheDocument();
    });
  });

  it("点击卡片打开详情面板", async () => {
    vi.mocked(knowledgeStore.getAll).mockResolvedValue(sampleItems);
    vi.mocked(knowledgeStore.getCategories).mockResolvedValue(["全部", "技术", "运维"]);
    vi.mocked(knowledgeStore.getTags).mockResolvedValue(["架构", "部署"]);

    render(<KnowledgeBasePanel />);

    await waitFor(() => expect(screen.getByText("项目架构")).toBeInTheDocument());

    const card = screen.getByRole("button", { name: /项目架构/ });
    fireEvent.click(card);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: /返回/ })).toBeInTheDocument();
    });
  });
});
