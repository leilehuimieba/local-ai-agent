# 验证方案

## 文档级验收

本 change 当前第一阶段以“评测包设计是否可执行”为验收口径。

必须满足：

1. 已明确最优先覆盖的知识回答行为与失败模式。
2. 已定义最小场景集，而不是只有原则没有题目。
3. 已定义可复用的评分标准与 inspect 信号。
4. 已定义复跑入口、证据位置和基线比较方法。
5. 已决定自动化补位路径，而不是停留在纯人工口径。

## 后续实现验证要求

### 场景覆盖

至少覆盖：

1. 正常知识问答
2. 工程判断型问答
3. 证据不足型问答
4. 边界约束型检查

### 检查信号

至少检查：

1. citation 是否存在
2. `事实 / 推断 / 建议` 边界是否可见
3. `verification_task_type` 是否为知识回答主链预期值
4. `verification_has_citation`、`capability_risk_checked`、`permission_boundary_respected` 是否存在
5. verify 失败时是否进入 replan / handoff，而不是伪装成功

## 固定问题集

样例源：

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/fixtures/knowledge-answer-cases.json`

当前固定问题：

1. `agent 是什么`
2. `为什么 agent 需要 memory 和 context engineering`
3. `如何评估一个 agent 是否可靠`
4. `这个项目的 agent loop 为什么不该直接上多智能体`
5. `这个项目里哪个云租户的生产 SLA 是多少`

## 复跑入口

### 预检

- `cargo test -p runtime-core verify -- --nocapture`
- `cargo test -p runtime-core run_verification_metadata -- --nocapture`

### 真实入口

- `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

## 证据位置

- 文档证据：本 change 目录下五个入口文件与固定 case fixture
- 运行证据：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 首轮执行说明：`D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`

## 本轮执行结果

### 预检结果

- `cargo test -p runtime-core verify -- --nocapture`：通过
- `cargo test -p runtime-core run_verification_metadata -- --nocapture`：通过

### 首轮真实入口结果

- 执行时间：2026-05-06 12:37 ~ 12:40
- 结果文件：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 总体结论：失败
- 通过情况：`5` 题中 `1` 题通过，仅 `KA-05` 低证据收口场景通过

### 首轮失败模式

1. `KA-01 ~ KA-04` 全部落为：
   - `terminal_event_type = run_finished`
   - `verification_task_type = generic`
   - `verification_has_citation = false`
2. 最终输出仍是“本地知识检索结果”列表，而不是带引证的知识回答收口。
3. `knowledge_digest = 当前阶段未注入知识摘要`，说明 ask/knowledge 主链在真实入口下没有按 AP 预期稳定收口。
4. `KA-05` 之所以通过，是因为命中了 `replan_requested = true` 的低证据收口路径，而不是知识回答主链本身已达标。

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期
- 当前覆盖情况：
  - 已完成独立 change 建档
  - 已完成最小场景集、评分规则、复跑入口、固定 fixture、真实入口脚本与 README
  - 已完成预检与首轮真实入口执行
  - 已确认下一步问题位于知识问答主链行为，而不是 AP 回归资产本身
