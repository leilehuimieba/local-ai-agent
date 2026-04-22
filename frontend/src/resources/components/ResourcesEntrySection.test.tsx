import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { testMemory, testSettings } from "../../test/fixtures";
import { ResourcesEntrySection } from "./ResourcesEntrySection";

describe("ResourcesEntrySection", () => {
  it("渲染资源工作区说明和资源模块", () => {
    render(
      <ResourcesEntrySection
        actionState={null}
        deletingId=""
        error={null}
        isRefreshing={false}
        isRunning={false}
        memories={[testMemory]}
        settings={testSettings}
        onDeleteMemory={vi.fn()}
        onRefreshMemories={vi.fn()}
      />,
    );
    expect(screen.getByText("资源工作区")).toBeInTheDocument();
    expect(screen.getByText("先看记忆策略与治理摘要，再进入列表筛选和单条详情，避免把资源区读成普通设置表单。")).toBeInTheDocument();
    expect(screen.getAllByText("Memory / Resources Workspace").length).toBeGreaterThan(0);
  });

  it("承接记忆资源列表内容", () => {
    render(
      <ResourcesEntrySection
        actionState={null}
        deletingId=""
        error={null}
        isRefreshing={false}
        isRunning={false}
        memories={[testMemory]}
        settings={testSettings}
        onDeleteMemory={vi.fn()}
        onRefreshMemories={vi.fn()}
      />,
    );
    expect(screen.getByText("记忆入口")).toBeInTheDocument();
    expect(screen.getByText("保持中文输出")).toBeInTheDocument();
  });
});
