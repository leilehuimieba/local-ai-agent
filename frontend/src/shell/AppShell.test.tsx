import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AppShell } from "./AppShell";

describe("AppShell", () => {
  it("渲染主内容骨架", () => {
    render(<AppShell topbar={<div>顶部导航</div>} leftNav={<div>左侧导航</div>} overlays={null} content={<div>主工作区</div>} rightPanel={null} bottomPanel={null} drawer={null} />);
    expect(screen.getByText("顶部导航")).toBeInTheDocument();
    expect(screen.getByText("左侧导航")).toBeInTheDocument();
    expect(screen.getByText("主工作区")).toBeInTheDocument();
  });

  it("在提供内容时渲染覆盖层与右侧面板", () => {
    render(<AppShell topbar={<div>顶部导航</div>} leftNav={<div>左侧导航</div>} overlays={<div>覆盖层内容</div>} content={<div>主工作区</div>} rightPanel={<div>右侧面板</div>} bottomPanel={<div>底部输入条</div>} drawer={null} />);
    expect(screen.getByText("覆盖层内容")).toBeInTheDocument();
    expect(screen.getByText("右侧面板")).toBeInTheDocument();
    expect(screen.getByText("底部输入条")).toBeInTheDocument();
  });
});
