# 验证记录

## 验证方式

- 单元测试：
  1. 脚本型改动，本刀以真实样例回归验证为主。
- 集成测试：
  1. 通过 Cortex API 构造 7 条混合样例（高质量/失败/短噪声/无价值/低分/重复），验证删留结果。
- 人工验证：
  1. 核对评分明细与 reason 是否一致。
  2. 核对重复内容仅保留一条高质量记录（`duplicate_shadow`）。

## 证据位置

- 测试记录：
  1. `powershell -NoProfile -File scripts/cortex/cleanup-low-quality-memories.ps1 -AgentId eval-lq-score -DryRun -OutputPath tmp/stage-e-low-quality-scoring/dryrun.json`
  2. `powershell -NoProfile -File scripts/cortex/cleanup-low-quality-memories.ps1 -AgentId eval-lq-score -OutputPath tmp/stage-e-low-quality-scoring/latest.json`
- 日志或截图：
  1. 评分报告：`tmp/stage-e-low-quality-scoring/dryrun.json`
  2. 清洗报告：`tmp/stage-e-low-quality-scoring/latest.json`
  3. 清洗后剩余样例核对（`remaining=2`，保留 1 条手工高质量 + 1 条 runtime 高质量）

## Gate 映射

- 对应阶段 Gate：
  1. Gate-E（执行中）
- 当前覆盖情况：
  1. 仅覆盖低质量识别升级，不做 Gate-E 完成声明。

## T01 验证记录（评分模型接入）

- 执行动作：
  1. 升级 `scripts/cortex/cleanup-low-quality-memories.ps1`，新增评分字段：
     - `confidence_score`
     - `source_trust`
     - `duplication_score`
     - `quality_score`
  2. 新增模型参数：`ScoreThreshold`、`MinLength`、`model_weights`。
- 验证结果：
  1. 报告中已输出上述评分字段和权重配置。
  2. `scripts/README.md` 已更新入口参数说明。
- 结论：
  1. 已满足 `T01` 完成判据。

## T02 验证记录（删留策略落地）

- 执行动作：
  1. 保留硬规则：`failed_result`、`short_noise`、`no_value`。
  2. 新增低分策略：`score_low`（`quality_score < ScoreThreshold`）。
  3. 新增重复清理策略：`duplicate_shadow`（重复内容只保留最优一条）。
- 验证结果：
  1. `dryrun.json` 中 `reason_stats` 出现 `score_low` 与 `duplicate_shadow`。
  2. 清洗后仅保留 2 条高质量记录，低质量与重复记录被识别。
- 结论：
  1. 已满足 `T02` 完成判据。

## T03 验证记录（真实样例回归）

- 执行动作：
  1. 注入 7 条混合样例到 `agent_id=eval-lq-score`。
  2. 先执行 `-DryRun` 复核，再执行真实删除。
  3. 清理后回读 `GET /api/v1/memories?agent_id=eval-lq-score&limit=500` 核对剩余结果。
- 验证结果：
  1. 清理前：`total_memories=7`。
  2. 候选识别：`candidate_count=5`，`kept_count=2`。
  3. 清理后：`deleted_count=5`，剩余 `2` 条高质量记录。
  4. 原因分布：`duplicate_shadow/failed_result/no_value/score_low/short_noise` 各 1 条。
- 结论：
  1. 已满足 `T03` 完成判据。
