import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testLogEntry } from "../../test/fixtures";
import { HistoryReviewSpotlight } from "./HistorySpotlight";

describe("HistoryReviewSpotlight", () => {
  it("在无焦点记录时显示空复盘卡", () => {
    render(<HistoryReviewSpotlight focusLog={null} />);
    expect(screen.getByText("焦点复盘卡")).toBeInTheDocument();
    expect(screen.getByText("从左侧选择一条记录后，这里会显示压缩后的复盘摘要。")).toBeInTheDocument();
  });

  it("在有焦点记录时显示复盘卡片", () => {
    render(<HistoryReviewSpotlight focusLog={testLogEntry} />);
    expect(screen.getByText("当前结论")).toBeInTheDocument();
    expect(screen.getByText("下一步")).toBeInTheDocument();
    expect(screen.getByText("执行依据")).toBeInTheDocument();
    expect(screen.getByText("验证结果")).toBeInTheDocument();
    expect(screen.getAllByText("History / Review 挂在 Logs 工作区内。").length).toBeGreaterThan(0);
  });
});
