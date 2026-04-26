# I-sustainable-delivery-20260424 Design

## 1. Warning 清理策略

### Rust unused/dead_code（25 个）

来源文件：
- `crates/runtime-core/src/memory_object_store.rs`：未使用的 struct/fn
- `crates/runtime-core/src/sqlite_store.rs`：未使用的 fn

处理方式：
- 对预留接口（未来可能接线的）添加 `#[allow(dead_code)]` 显式标记，说明保留原因。
- 对确定废弃的内部 helper，直接移除。
- 目标：`cargo test --workspace` 在默认 warning 级别下无输出污染。

### PowerShell stderr 误报

根因：PowerShell 将 native command 的 stderr 包装为 `RemoteException`，导致 exit code 非零。

处理方式：
- 在关键脚本中重定向 stderr 到 stdout 或 `$null`。
- 或在 PowerShell 中设置 `$ErrorActionPreference = 'SilentlyContinue'` 仅用于测试命令。
- 优先方案：脚本侧统一处理，不改动全局 PowerShell 配置。

## 2. 自动化回归流水线

最小流水线定义：

| 步骤 | 命令 | 通过条件 |
|---|---|---|
| Rust 编译检查 | `cargo check --workspace` | exit code 0 |
| Rust 测试 | `cargo test --workspace` | 0 failed |
| Go 编译 | `cd gateway; go build ./...` | exit code 0 |
| Go 测试 | `cd gateway; go test ./internal/service/...` | exit code 0 |
| 前端构建 | `cd frontend; npm run build` | exit code 0 |
| E2E 抽样 | `scripts/run-stage-e-entry1-acceptance.ps1` | `status=passed` |

触发方式：
- 本地：每次 commit 前手动执行 `scripts/run-full-regression.ps1`（新建）。
- 远程：利用 `.github/workflows/` 中现有工作流扩展。

## 3. 行尾符治理

现状：
- 仓库中大量文件为 LF，Windows PowerShell/Git 默认 CRLF，导致 `git diff` 和脚本执行时产生大量 LF→CRLF warning。

方案：
- 在 `.gitattributes` 中显式声明关键文件类型的行尾符策略。
- 对已有混用文件做一次批量标准化（`git add --renormalize .`）。
- 目标：消除 `git diff` 时的 LF→CRLF warning。

## 4. 发布标签与回滚

最小规范：
- 标签格式：`v{主版本}.{次版本}.{补丁}-{日期}`，例如 `v1.0.0-20260424`。
- 每次阶段收口后打一个标签。
- 回滚入口：`git checkout <tag>` + 数据库兼容迁移确认。
- 当前基线标签：待本 change 完成后打 `v1.0.0-20260424`。
