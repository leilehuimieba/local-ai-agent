import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testSettings } from "../test/fixtures";
import { StatusCard } from "./StatusCard";

describe("StatusCard", () => {
  it("在无设置且无错误时显示读取空态", () => {
    render(<StatusCard settings={null} bootstrapError={null} pendingAction={null} actionError={null} lastSuccess={null} />);
    expect(screen.getByText("设置运行态")).toBeInTheDocument();
    expect(screen.getByText("正在读取基础设置。")).toBeInTheDocument();
  });

  it("在正常设置下显示运行态摘要", () => {
    render(<StatusCard settings={testSettings} bootstrapError={null} pendingAction={null} actionError={null} lastSuccess={null} />);
    expect(screen.getByText("Local Agent")).toBeInTheDocument();
    expect(screen.getByText("GPT-5.4")).toBeInTheDocument();
    expect(screen.getByText("standard")).toBeInTheDocument();
    expect(screen.getByText("默认工作区")).toBeInTheDocument();
  });

  it("在 bootstrapError 存在时显示错误块", () => {
    render(<StatusCard settings={null} bootstrapError="运行时启动失败" pendingAction={null} actionError={null} lastSuccess={null} />);
    expect(screen.getByText("设置加载失败")).toBeInTheDocument();
    expect(screen.getByText("运行时启动失败")).toBeInTheDocument();
  });
});
