# B 阶段执行记录 03：confirmation 恢复入口接入

更新时间：2026-04-10
阶段：B（进行中）

## 1. 本次目标

1. 将 B-02 的 checkpoint 存储真正接入一条恢复使用路径。
2. 优先打通“等待确认 -> 用户批准 -> 带 checkpoint 恢复重入 runtime”最小链路。
3. 保持现有确认流和主循环不重写。

## 2. 本次改动范围

1. Rust：`crates/runtime-core/src/checkpoint.rs`
2. Rust：`crates/runtime-core/src/lib.rs`
3. Go：`gateway/internal/state/confirmation_store.go`
4. Go：`gateway/internal/api/chat.go`

## 3. 关键改动

### 3.1 Runtime 恢复提示事件

1. 新增 `checkpoint_resume_event`。
2. 当请求带有 `resume_from_checkpoint_id` 时，runtime 会先尝试读取 checkpoint。
3. 若 checkpoint 匹配当前 run/session/workspace，则插入 `checkpoint_resumed` 事件。
4. 若 checkpoint 缺失或不匹配，则插入 `checkpoint_resume_skipped` 事件。

### 3.2 Gateway 确认流接入 checkpoint

1. `PendingConfirmation` 新增 `CheckpointID`。
2. 当 runtime 返回 `awaiting_confirmation` 时，gateway 会同时保存 `checkpoint_id`。
3. 用户批准确认后，gateway 会把：
4. `resume_from_checkpoint_id`
5. `resume_strategy=after_confirmation`
6. 一并带回 runtime。

### 3.3 最小恢复策略

1. 当前恢复策略不是“从中间阶段继续执行”。
2. 当前恢复策略是“按确认后的同一 run 重入，并显式读取 checkpoint 作为恢复依据”。
3. 这样先把 checkpoint 从“只存不用”推进到“正式参与恢复入口”。

## 4. 验证结果

### 4.1 定向测试

1. `cargo test -p runtime-core checkpoint -- --nocapture`：通过。
2. 新增覆盖：
3. checkpoint 写入与读取。
4. 匹配确认恢复请求时插入 `checkpoint_resumed` 事件。

### 4.2 构建验证

1. `cargo build`：通过。
2. `go build ./...`：通过。

## 5. 当前结论

1. B-03 已完成“确认后恢复入口”最小闭环。
2. checkpoint 已从静态存储提升为参与运行入口判断的恢复依据。
3. 当前仍未实现真正的阶段级恢复执行，只完成最小恢复接入。

## 6. 下一步（B-04）

1. 设计阶段级恢复状态结构。
2. 区分“确认后恢复”和“失败后恢复”两类策略。
3. 为恢复幂等、事件补写和重试边界增加测试。
