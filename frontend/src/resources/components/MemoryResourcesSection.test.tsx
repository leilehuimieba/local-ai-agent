import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { testMemory, testSettings } from "../../test/fixtures";
import { MemoryResourcesSection } from "./MemoryResourcesSection";

describe("MemoryResourcesSection", () => {
  it("渲染资源工作区与记忆入口", () => {
    render(
      <MemoryResourcesSection
        actionState={null}
        deletingId=""
        error={null}
        isRefreshing={false}
        isRunning={false}
        memories={[testMemory]}
        settings={testSettings}
        onDeleteMemory={vi.fn()}
        onRefresh={vi.fn()}
      />,
    );
    expect(screen.getByText("Memory / Resources Workspace")).toBeInTheDocument();
    expect(screen.getByText("记忆入口")).toBeInTheDocument();
    expect(screen.getByText("保持中文输出")).toBeInTheDocument();
  });

  it("可以展开单条记忆详情", () => {
    render(
      <MemoryResourcesSection
        actionState={null}
        deletingId=""
        error={null}
        isRefreshing={false}
        isRunning={false}
        memories={[testMemory]}
        settings={testSettings}
        onDeleteMemory={vi.fn()}
        onRefresh={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByText("查看详情"));
    expect(screen.getByText("治理依据：人工确认")).toBeInTheDocument();
    expect(screen.getByText("原始内容：默认用中文输出")).toBeInTheDocument();
  });
});
