# I-sustainable-delivery-20260424 Proposal

## 背景

总路线阶段 A~H 已全部完成，代码已全量入库，E2E 验收通过。项目从"追赶 Hermes"进入"可持续自主演进"阶段。当前存在以下工程治理缺口：

1. Rust 侧 25 个 `unused`/`dead_code` warning 未清理，主要来自 memory object 预留接口。
2. 缺乏自动化回归流水线，每次变更依赖人工跑验收脚本。
3. Git LF/CRLF 混用导致 PowerShell stderr 误报干扰自动化判断。
4. 无明确的发布标签与版本回滚机制。

## 目标

1. 清理核心技术债务，使 `cargo test --workspace` 在严格模式下零 warning。
2. 建立最小自动化回归流水线（编译 + 测试 + E2E 抽样）。
3. 统一行尾符治理，消除 PowerShell stderr 误报。
4. 建立发布标签规范与最小回滚入口。

## 非目标

1. 不新增产品功能。
2. 不扩大 memory object 接口使用面（只清理 warning，不移除预留接口）。
3. 不替换现有 CI/CD 基础设施（利用现有 GitHub Actions 或本地脚本）。
