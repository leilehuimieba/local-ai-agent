import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { MetaGrid } from "./MetaGrid";

describe("MetaGrid", () => {
  it("渲染默认元信息网格", () => {
    const { container } = render(<MetaGrid items={[{ label: "模型", value: "GPT-5.4" }, { label: "模式", value: "standard" }]} />);
    expect(container.firstElementChild).toHaveClass("detail-meta-grid");
    expect(screen.getByText("模型")).toBeInTheDocument();
    expect(screen.getByText("GPT-5.4")).toBeInTheDocument();
    expect(screen.getByText("模式")).toBeInTheDocument();
  });

  it("合并自定义类名并保留值内容", () => {
    const { container } = render(<MetaGrid className="compact" items={[{ label: "工作区", value: <span>默认工作区</span> }]} />);
    expect(container.firstElementChild).toHaveClass("detail-meta-grid", "compact");
    expect(screen.getByText("默认工作区")).toBeInTheDocument();
  });
});
