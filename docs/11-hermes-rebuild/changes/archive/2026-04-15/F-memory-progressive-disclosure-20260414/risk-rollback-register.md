# 风险与回退登记

## 基线

1. 适用 change：`F-memory-progressive-disclosure-20260414`
2. 适用阶段：并行专项 M0~M5
3. 生效时间：2026-04-14

## 风险清单

### R1 检索命中波动

1. 风险描述：关键词检索与融合排序在样本分布变化时可能出现命中抖动。
2. 触发信号：`tmp/stage-mem-eval/latest.json` 中 `top5_hit_rate < 70`。
3. 回退动作：
   - 保留 `search_observations` 关键词主路径；
   - 关闭排序融合加权（只按基础匹配+时间降序）。
4. 验证动作：复跑 `export_observation_eval` 并比对命中率。

### R2 注入预算失控

1. 风险描述：上下文注入文本超过预算或节省率下降。
2. 触发信号：`saved_percent < 50` 或 `used_chars > budget_total_chars`。
3. 回退动作：
   - 缩减 budget；
   - 关闭 details 层，仅保留 summary + timeline。
4. 验证动作：复跑 `export_observation_injection`，检查 `ab_test` 字段。

### R3 隐私治理漏检

1. 风险描述：敏感字段或 private 片段进入存储层。
2. 触发信号：
   - `privacy-redact.json` 中出现未脱敏敏感样例；
   - `private-skip.json` 中 `private_marker_count > 0` 但 `stored_count` 未下降。
3. 回退动作：
   - 启用 hard-block（命中敏感/私有标记直接拒绝写入）；
   - 仅保留最小审计摘要。
4. 验证动作：复跑 `export_observation_privacy_redact` / `export_observation_private_skip`。

### R4 增强链路影响主流程

1. 风险描述：memory-enhanced 注入影响主链路稳定性。
2. 触发信号：运行异常或注入链导致失败率显著上升。
3. 回退动作：
   - 将 `context_hints.memory_enhanced_enabled=false`；
   - 走 legacy 检索主路径。
4. 验证动作：复跑 `export_observation_rollback`，检查 `disabled.fallback_to_legacy=true`。

## 当前回退演练结果

1. `tmp/stage-mem-m5/rollback.json` 显示：
   - enabled：增强链路可用；
   - disabled：已回退 legacy，`fallback_to_legacy=true`。
