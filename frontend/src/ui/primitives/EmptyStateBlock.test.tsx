import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { EmptyStateBlock } from "./EmptyStateBlock";

describe("EmptyStateBlock", () => {
  it("渲染默认空态内容", () => {
    const { container } = render(<EmptyStateBlock title="暂无记录" text="运行后这里会显示结果。" />);
    expect(container.firstElementChild).toHaveClass("empty-state");
    expect(screen.getByRole("heading", { level: 3, name: "暂无记录" })).toBeInTheDocument();
    expect(screen.getByText("运行后这里会显示结果。")).toBeInTheDocument();
  });

  it("compact 模式追加紧凑类名", () => {
    const { container } = render(<EmptyStateBlock compact title="正在加载" text="请稍候。" />);
    expect(container.firstElementChild).toHaveClass("empty-state", "compact");
    expect(screen.getByText("请稍候。")).toBeInTheDocument();
  });
});
