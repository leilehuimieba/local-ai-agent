import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { testSettings } from "../test/fixtures";
import { TopBar } from "./TopBar";

describe("TopBar", () => {
  it("渲染品牌、导航与上下文指标", () => {
    render(<TopBar settings={testSettings} currentView="task" statusLine="进行中" runState="streaming" connectionLabel="连接正常" sessionId="session-123456" currentRunId="run-123456" homeStateHint="可直接开始" onOpenHomeStart={vi.fn()} onViewChange={vi.fn()} />);
    expect(screen.getByText("Local Agent")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "首页" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "任务" })).toBeInTheDocument();
    expect(screen.getByText("连接正常")).toBeInTheDocument();
  });

  it("支持导航切换与快捷动作", () => {
    const onOpenHomeStart = vi.fn();
    const onViewChange = vi.fn();
    render(<TopBar settings={testSettings} currentView="task" statusLine="进行中" runState="streaming" connectionLabel="连接正常" sessionId="session-123456" currentRunId="run-123456" homeStateHint="可直接开始" onOpenHomeStart={onOpenHomeStart} onViewChange={onViewChange} />);
    fireEvent.click(screen.getByText("记录"));
    fireEvent.click(screen.getByText("新建任务"));
    fireEvent.click(screen.getByText("前往设置调整"));
    expect(onViewChange).toHaveBeenCalledWith("logs");
    expect(onOpenHomeStart).toHaveBeenCalled();
    expect(onViewChange).toHaveBeenCalledWith("settings");
  });
});
