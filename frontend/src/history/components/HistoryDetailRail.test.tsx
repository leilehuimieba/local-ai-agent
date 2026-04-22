import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testLogEntry } from "../../test/fixtures";
import { HistoryDetailRail } from "./HistoryDetailRail";

describe("HistoryDetailRail", () => {
  it("在无焦点记录时显示等待状态", () => {
    render(<HistoryDetailRail focusLog={null} />);
    expect(screen.getByText("等待选中记录")).toBeInTheDocument();
    expect(screen.getByText("复盘详情栏")).toBeInTheDocument();
  });

  it("在有焦点记录时渲染详情区块", () => {
    render(<HistoryDetailRail focusLog={testLogEntry} />);
    expect(screen.getByText("基本信息")).toBeInTheDocument();
    expect(screen.getByText("结果摘要")).toBeInTheDocument();
    expect(screen.getByText("复盘拆解")).toBeInTheDocument();
    expect(screen.getByText("关键 Metadata")).toBeInTheDocument();
    expect(screen.getByText("验证与后续")).toBeInTheDocument();
  });
});
