# B 阶段执行记录 02：checkpoint 存储接入与事件落点

更新时间：2026-04-10
阶段：B（进行中）

## 1. 本次目标

1. 新增独立 checkpoint 存储模块，不重写现有主循环。
2. 打通 `run snapshot -> sqlite -> checkpoint_written event` 最小链路。
3. 保持三端合同兼容，保证现有调用方不被破坏。

## 2. 本次改动范围

1. Rust：`crates/runtime-core/src/checkpoint.rs`
2. Rust：`crates/runtime-core/src/lib.rs`
3. Rust：`crates/runtime-core/src/sqlite_store.rs`
4. Rust：`crates/runtime-core/src/contracts.rs`
5. Rust：`crates/runtime-core/src/events.rs`
6. Go：`gateway/internal/contracts/contracts.go`
7. TS：`frontend/src/shared/contracts.ts`

## 3. 关键改动

### 3.1 新增 checkpoint 模块

1. 新增 `RunCheckpoint` 结构，保存 `request + response + status + final_stage + event_count`。
2. 写入前对 `provider_ref.api_key` 做脱敏清空，不把明文 key 落入 checkpoint。
3. 统一在 `simulate_run_with_runtime_events` 外层接入，不侵入 `simulate_run` 主流程主体。

### 3.2 SQLite 持久化落点

1. 新增 `runtime_checkpoints` 表。
2. 新增 `write_runtime_checkpoint_sqlite`。
3. 新增 `load_runtime_checkpoint_sqlite`，为后续 resume 入口预留读取能力。

### 3.3 事件与结果补齐

1. `RunResult.checkpoint_id` 现在会在外层写入成功后返回。
2. `RunResult.resumable` 继续沿用兼容式可选字段。
3. 新增 `checkpoint_written` 事件。
4. 三端 `RunEvent` 合同补齐 `checkpoint_written?: bool`。

## 4. 设计取舍

1. 本次只做 checkpoint 基础设施，不直接接恢复执行入口。
2. 事件插入点固定在终态事件前，保证事件流可读性。
3. checkpoint 写入失败时回退为原响应，不影响现有运行结果返回。

## 5. 验证结果

### 5.1 定向测试

1. `cargo test -p runtime-core checkpoint -- --nocapture`：通过。
2. 新增测试覆盖：
3. checkpoint 写入成功。
4. `checkpoint_written` 事件插入成功。
5. checkpoint 可从 SQLite 读回。

### 5.2 构建验证

1. `cargo build`：通过。
2. `go build ./...`：通过。
3. `npm run build`：通过。

## 6. 当前结论

1. B-02 已完成“checkpoint 存储基础设施 + 事件落点”最小闭环。
2. 当前仍未启用真正的 resume 执行分支，但格式与读接口已准备好。
3. 下一步可继续推进 checkpoint 恢复入口和状态机恢复策略。

## 7. 下一步（B-03）

1. 设计 resume 入口读取路径。
2. 为 `resume_from_checkpoint_id` 增加最小恢复策略。
3. 明确恢复后事件链补写规则与幂等策略。
