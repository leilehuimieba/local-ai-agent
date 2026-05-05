# 技术方案

## 影响范围

- 涉及模块：
  - `crates/runtime-core/src/verify.rs`
  - `crates/runtime-core/src/run_verification_metadata.rs`
  - `crates/runtime-core/src/query_engine.rs`
  - `crates/runtime-core/src/events.rs`
- 可能新增：
  - `crates/runtime-core/src/verify_knowledge.rs`
  - 其他 `verify_*` 子模块
- 涉及文档或 contract：
  - Runtime verification metadata 口径
  - `docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/design.md`

## Harness 判断

- 前四刀已经把“主循环、上下文、记忆、知识回答”主链打通。
- 第五刀最值得补的是“独立验证”，否则系统仍然容易把“看起来像答案”的输出误判为可交付结果。
- 因此本刀先收紧 verify，不回头继续扩知识质量，也不提前混 UI 或复杂观测。

## 范围冻结

- 本刀只做四件事：
  - 定义最小任务类型 verify 矩阵
  - 先落知识回答类 verify
  - 补最小 verification metadata 扩展
  - 让 verify 结果能驱动 `finish / replan / handoff` 的后续判断

## 任务类型矩阵

### 最小分型

- 本刀先保留 5 类任务类型，但实现顺序固定为：
  - `knowledge_answer`
  - `file_change`
  - `command_execution`
  - `memory_write`
  - `browser_interaction`

### 本刀先实现的 verify

- `knowledge_answer`
  - 至少 2 条相关命中
  - 至少 1 条 citation
  - 能区分事实 / 推断 / 建议
  - 如果 evidence 不足，应明确给出继续检索、replan 或 handoff 建议

## Knowledge Verify 口径

### 输入信号

- 优先使用已有 metadata，不重查大段上下文：
  - `knowledge_pack_question_type`
  - `knowledge_pack_citations`
  - `knowledge_pack_match_reason`
  - `knowledge_digest`
  - `verification_evidence`
  - tool result summary / reasoning summary

### 输出信号

- `verify.rs` 不只返回统一 `policy`，应至少补下面这些字段：
  - `verification_task_type`
  - `verification_evidence_count`
  - `verification_has_citation`
  - `verification_fact_inference_split`
  - `capability_risk_checked`
  - `permission_boundary_respected`

### 通过标准

- `knowledge_answer` 最小通过条件：
  - 已命中知识 pack
  - citation 非空
  - 证据条数达标
  - reasoning / summary 中能看出“事实 / 推断 / 建议”边界

### 失败标准

- 以下任一情况视为 verify 不通过：
  - citation 为空
  - evidence 少于最小阈值
  - reasoning 仅有泛化总结，无法区分事实与推断
  - 明显绕开风险或权限边界

## Metadata / Event 出口

- `run_verification_metadata.rs`
  - 追加知识回答 verify 的最小 metadata 字段
- `events.rs`
  - 让 verification snapshot / event metadata 能看到新的任务类型与边界状态
- `query_engine.rs`
  - 当 verify 失败且属于知识回答类时，优先触发：
    - `replan`
    - 或 `handoff`
  - 不要默认继续硬跑

## 风险与回退

- 主要风险：
  - 规则写得太硬，导致知识问答被误判失败
  - 规则写得太松，仍然挡不住“像答案但无证据”的输出
  - 把 verify 与知识 pack 紧耦合，影响后续其它任务类型接入
- 回退方式：
  - 保留当前统一 verify 作为 fallback
  - 新任务类型判断失败时，退回 `check_result_summary`
  - 先补知识回答 verify 测试，再决定是否把其他类型纳入同轮实现
