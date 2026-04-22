import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { BottomPanel } from "./BottomPanel";

describe("BottomPanel", () => {
  it("在关闭时显示调查轨道摘要", () => {
    render(<BottomPanel currentRunId="" isOpen={false} events={[]} currentTaskTitle="" runState="idle" submitError={null} onOpenChange={vi.fn()} />);
    expect(screen.getByText("调查与执行轨迹")).toBeInTheDocument();
    expect(screen.getByText("展开调查")).toBeInTheDocument();
  });

  it("在运行中且无事件时显示等待态", () => {
    render(<BottomPanel currentRunId="run-1" isOpen events={[]} currentTaskTitle="核对侧栏状态" runState="streaming" submitError={null} onOpenChange={vi.fn()} />);
    expect(screen.getByText("等待首个事件")).toBeInTheDocument();
    expect(screen.getAllByText("事件流").length).toBeGreaterThan(0);
    expect(screen.getByText("系统正在建立本轮任务的事件流。")).toBeInTheDocument();
  });
});
