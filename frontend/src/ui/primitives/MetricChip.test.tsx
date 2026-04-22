import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { MetricChip } from "./MetricChip";

describe("MetricChip", () => {
  it("渲染默认摘要样式与数值标题回退", () => {
    render(<MetricChip label="事件数" value="24" />);
    const value = screen.getByText("24");
    expect(value).toHaveAttribute("title", "24");
    expect(value.parentElement).toHaveClass("summary-chip");
    expect(screen.getByText("事件数")).toBeInTheDocument();
  });

  it("合并自定义类名并使用显式 title", () => {
    render(<MetricChip label="耗时" value="3.2s" title="最近一次运行耗时" className="accent" />);
    const value = screen.getByText("3.2s");
    expect(value).toHaveAttribute("title", "最近一次运行耗时");
    expect(value.parentElement).toHaveClass("summary-chip", "accent");
  });
});
