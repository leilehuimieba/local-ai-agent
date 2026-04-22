import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { InfoCard } from "./InfoCard";

describe("InfoCard", () => {
  it("渲染默认容器类名与内容", () => {
    render(<InfoCard aria-label="信息卡片">卡片正文</InfoCard>);
    const card = screen.getByLabelText("信息卡片");
    expect(card.tagName).toBe("SECTION");
    expect(card).toHaveClass("info-card");
    expect(screen.getByText("卡片正文")).toBeInTheDocument();
  });

  it("合并自定义类名并透传属性", () => {
    render(<InfoCard aria-label="扩展卡片" className="compact" data-state="ready">扩展内容</InfoCard>);
    const card = screen.getByLabelText("扩展卡片");
    expect(card).toHaveClass("info-card", "compact");
    expect(card).toHaveAttribute("data-state", "ready");
  });
});
