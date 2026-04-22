import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AppShell } from "./AppShell";

describe("AppShell", () => {
  it("渲染跳转链接与主内容骨架", () => {
    render(<AppShell topbar={<div>顶部导航</div>} overlays={null} content={<div>主工作区</div>} bottomPanel={null} />);
    expect(screen.getByText("跳到主内容")).toBeInTheDocument();
    expect(screen.getByText("顶部导航")).toBeInTheDocument();
    expect(screen.getByText("主工作区")).toBeInTheDocument();
  });

  it("在提供内容时渲染覆盖层与底部抽屉", () => {
    render(<AppShell topbar={<div>顶部导航</div>} overlays={<div>覆盖层内容</div>} content={<div>主工作区</div>} bottomPanel={<div>底部调查层</div>} />);
    expect(screen.getByText("覆盖层内容")).toBeInTheDocument();
    expect(screen.getByText("底部调查层")).toBeInTheDocument();
  });
});
