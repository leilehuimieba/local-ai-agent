# 技术方案

## 影响范围

- 文档与回归入口优先：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 已有可复用入口：
  - `D:/newwork/本地智能体/scripts/run-project-status-answer-regression.ps1`
  - `D:/newwork/本地智能体/scripts/run-stage-e-knowledge-recall-eval.ps1`
  - `D:/newwork/本地智能体/crates/runtime-core/src/h03_eval_tests.rs`
- 预计本刀实现落点：
  - `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
  - `D:/newwork/本地智能体/scripts/README.md`
  - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/`

## Harness 判断

- 当前主链最需要的是“最小但有杀伤力的评测包”，优先捕获知识回答的假阳性、citation 丢失、verification 边界回退。
- 因此本刀先做评测资产，不回头继续扩 `AK` 或 `AL` 的实现逻辑。

## 范围冻结

- 本刀只做四件事：
  - 固定知识回答主链的优先行为与风险点
  - 定义最小场景集
  - 定义评分与人工检查规则
  - 定义复跑入口、证据位置与基线比较方式

## 优先覆盖行为

1. `结论 + citation` 同时存在
2. `事实 / 推断 / 建议` 边界可见
3. verify 失败时不会伪装成成功答案
4. 风险与权限边界字段持续可见
5. 重复提问时回答结构波动可控

## 最小场景集

固定样例文件：

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/fixtures/knowledge-answer-cases.json`

### A. 正常知识问答

- `KA-01`：`agent 是什么`
- `KA-02`：`为什么 agent 需要 memory 和 context engineering`

目标：验证系统能基于本地知识给出稳定回答，而不是只输出泛化套话。

### B. 工程判断型问答

- `KA-03`：`如何评估一个 agent 是否可靠`
- `KA-04`：`这个项目的 agent loop 为什么不该直接上多智能体`

目标：验证系统能结合知识 pack 与当前蓝图给出结构化建议。

### C. 证据不足 / 需要收口型问答

- `KA-05`：`这个项目里哪个云租户的生产 SLA 是多少`

目标：验证证据不足时系统会触发 verify / replan / handoff，而不是硬编结论。

### D. 边界约束型检查

所有 case 都要补查：

- `verification_task_type`
- `verification_has_citation`
- `capability_risk_checked`
- `permission_boundary_respected`

## 评分规则

### Pass

- 回答有明确结论；
- 至少带最小 citation；
- 能看出事实 / 推断 / 建议边界；
- verification 相关 metadata 齐全；
- 低证据 case 不会伪装成强结论。

### Concern

- 结论基本正确，但 citation 不稳或边界表达偏弱；
- 需要人工复核是否属于可接受波动。

### Fail

- 无 citation；
- 把推断包装成事实；
- 证据不足时仍强行收口；
- verification / risk boundary 信号缺失。

## 可检查产物

- 回答正文
- citation 列表
- verification metadata
- replan / handoff 事件
- 样例输出 JSON
- stdout / stderr 日志

## 自动化补位路径

### 第一层：核心定向预检

先复用现有 `runtime-core` 定向测试，确认核心信号没有先天退化：

- `cargo test -p runtime-core verify -- --nocapture`
- `cargo test -p runtime-core run_verification_metadata -- --nocapture`
- 视需要补 `knowledge.rs` / `query_engine.rs` 的定向 case

作用：先挡住 citation、verify metadata、知识回答 replan 的核心回归。

### 第二层：真实入口回归脚本

新增：

- `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

实现方式参考：

1. `run-project-status-answer-regression.ps1`
   - 复用“拉起隔离 runtime-host + gateway，再走真实 `chat/run + logs` 链路”的模式。
2. `run-stage-e-knowledge-recall-eval.ps1`
   - 复用“固定 case 集 + latest.json/history.json 输出 + 失败原因统计”的模式。
3. `h03_eval_tests.rs`
   - 复用“eval pack / latest.json / 分项样例报告”的产物组织方式。

### 输出约定

- 主报告：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 历史报告：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/<run-id>.json`
- 首轮证据说明：`D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`

## 风险与回退

- 风险：场景太少，只覆盖 happy path。
- 风险：评分标准太虚，导致不同人复核口径不一致。
- 风险：只做 cargo test，不走真实入口，导致假阳性。
- 回退：如本刀暂时无法一次落完整脚本，先保留 fixture + 评分规则 + 人工复跑清单，再补最小真实入口脚本。
