import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { StatusPill } from "./StatusPill";

describe("StatusPill", () => {
  it("渲染默认状态徽标", () => {
    render(<StatusPill label="运行中" />);
    const pill = screen.getByText("运行中");
    expect(pill).toHaveClass("status-badge");
    expect(pill).not.toHaveAttribute("title");
  });

  it("合并自定义类名并透传 title", () => {
    render(<StatusPill label="已暂停" className="warn" title="等待人工恢复" />);
    const pill = screen.getByText("已暂停");
    expect(pill).toHaveClass("status-badge", "warn");
    expect(pill).toHaveAttribute("title", "等待人工恢复");
  });
});
