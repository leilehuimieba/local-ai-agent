# 技术方案

## 影响范围

- 主要代码：
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
- 预计拆分后涉及：
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/policy.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/evidence.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/knowledge_answer.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/browser_interaction.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/file_command_memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify/tests/`
- 可能需要最小接线改动的调用方：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/run_verification_metadata.rs`
  - 以及引用 `verify_tool_execution` / `VerificationReport` 的最小接线点

## 方案

### 1. 拆分原则

- 保持外部入口稳定：
  - `verify_tool_execution`
  - `VerificationOutcome`
  - `VerificationReport`
- 优先做“文件内模块职责重排”，而不是借机改行为。
- 每次拆分都以“现有测试继续通过”为前提。

### 2. 建议模块边界

1. `verify/mod.rs`
   - 保留统一入口与公共结构体。
   - 负责调度各类 verifier。
2. `verify/policy.rs`
   - 放 `verification_task_type`
   - 放 `verification_policy`
   - 放 `is_browser_tool_name`
3. `verify/evidence.rs`
   - 放 evidence 提取
   - 放 `skill_hit_reason`
   - 放 `guard_*` 相关辅助
4. `verify/knowledge_answer.rs`
   - 放 knowledge answer 的 citation / fact-inference 校验逻辑
5. `verify/browser_interaction.rs`
   - 放 browser verify、failure type、page/selector、recovery summary
6. `verify/file_command_memory.rs`
   - 放 file change、command execution、memory write 三类较轻逻辑
7. `verify/tests/`
   - 按 `knowledge`、`browser`、`file_command_memory` 至少拆成三个测试文件

### 2.1 当前冻结的函数映射

1. `verify.rs`
   - 保留 `VerificationOutcome`、`VerificationReport`
   - 保留 `verify_tool_execution`
   - 保留通用 outcome 构造与 `used_recovery`
2. `verify/policy.rs`
   - 承接 `verification_task_type`
   - 承接 `verification_policy`
   - 承接 `is_browser_tool_name`
3. `verify/evidence.rs`
   - 承接 `verification_evidence`
   - 承接 `skill_hit_reason`
   - 承接 `guard_downgraded`
   - 承接 `guard_decision_ref`
4. `verify/browser_interaction.rs`
   - 承接 `verify_browser_interaction`
   - 承接 browser failure type / summary / next step
   - 承接 page / selector / effect / risk / output 相关辅助
5. `verify/knowledge_answer.rs`
   - 后续承接 `verify_knowledge_answer`
   - 后续承接 citation / fact-inference split 辅助
6. `verify/file_command_memory.rs`
   - 后续承接 `verify_file_change`
   - 后续承接 `verify_command_execution`
   - 后续承接 `verify_memory_write`
   - 后续承接对应效果判断辅助
7. `verify/tests/`
   - 后续承接当前 `#[cfg(test)] mod tests`
   - 先按 `knowledge / browser / file_command_memory` 三组拆

### 3. 实施顺序

1. 先保住 `mod.rs` 的对外入口和类型定义。
2. 再抽 `policy.rs` 与 `evidence.rs`。
3. 再抽 `knowledge_answer.rs`、`browser_interaction.rs`、`file_command_memory.rs`。
4. 最后拆测试模块并补最小接线修正。

### 4. 风险与回退

- 主要风险：拆分过程中出现引用循环或私有可见性问题。
- 主要风险：测试拆分后遗漏辅助函数，导致行为未变但测试失效。
- 主要风险：顺手改动行为阈值，破坏“只做结构治理”的范围冻结。
- 回退方式：
  - 优先按模块搬移的粒度回退；
  - 不回退 `AR` 已签收的状态口径；
  - 如结构拆分导致行为异常，回退到单文件版本后重新按更小步搬移。
