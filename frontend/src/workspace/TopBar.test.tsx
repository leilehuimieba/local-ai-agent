import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { testSettings } from "../test/fixtures";
import { TopBar } from "./TopBar";

describe("TopBar", () => {
  it("渲染品牌与状态指示灯", () => {
    render(<TopBar settings={testSettings} runState="idle" statusLine="等待中" rightPanelOpen={false} onToggleRightPanel={vi.fn()} />);
    expect(screen.getByText("Local Agent")).toBeInTheDocument();
    expect(screen.getByLabelText("展开右侧面板")).toBeInTheDocument();
  });

  it("支持切换右侧面板", () => {
    const onToggleRightPanel = vi.fn();
    render(<TopBar settings={testSettings} runState="streaming" statusLine="运行中" rightPanelOpen={false} onToggleRightPanel={onToggleRightPanel} />);
    fireEvent.click(screen.getByLabelText("展开右侧面板"));
    expect(onToggleRightPanel).toHaveBeenCalled();
  });
});
