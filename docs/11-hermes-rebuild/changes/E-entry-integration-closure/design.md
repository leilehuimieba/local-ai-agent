# 技术方案

## 影响范围

- 验收脚本：`scripts/run-stage-e-entry-failure-acceptance.ps1`
- 收口文档：`docs/11-hermes-rebuild/changes/E-entry-integration-closure/*.md`
- 索引与总表：`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. 失败样本脚本（E-05 新增）

- 新增 `run-stage-e-entry-failure-acceptance.ps1`，只启动 gateway，不启动 runtime。
- 通过 `chat/run` 发起任务，验证网关失败收口链：
  1. 请求被受理（`accepted=true`）。
  2. 日志出现 `run_failed` 且 `error_code=runtime_unavailable`。
  3. 日志出现终态 `run_finished`，并与同一 `run_id/session_id` 对齐。
- 输出 `tmp/stage-e-entry-failure/latest.json`，供联调报告直接引用。

### 2. 入口联调收口包（E-05 主交付）

- 在 `E-entry-integration-closure/review.md` 汇总三类样本：
  1. `tmp/stage-e-entry1/latest.json`（入口协议成功链）。
  2. `tmp/stage-e-consistency/latest.json`（跨入口一致性链）。
  3. `tmp/stage-e-entry-failure/latest.json`（失败收口链）。
- 在同一文档中给出回退路径：
  - 运行时不可达的运维回退。
  - 多入口联调异常时的单入口降级回退。
  - 外部身份锚点注入异常时的兼容回退（回退到网关自生成 ID）。

## 风险与回退

- 风险：失败样本仅覆盖 runtime 不可达，不等同所有失败类型。
- 缓解：将该样本定义为 E-05 最小必过失败链，其他失败类型留给 E-G1 批量阶段补齐。
- 回退：若多入口联调在 E-G1 不稳定，先冻结到 E-02 单入口协议路径，保留统一协议层与日志过滤能力。
