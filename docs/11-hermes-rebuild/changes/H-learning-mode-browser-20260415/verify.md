# H-learning-mode-browser-20260415（verify）

更新时间：2026-04-15
状态：草案

## 验证方式

1. 集成验证：
   - 页面抽取 -> 解释翻译 -> 价值判断 -> 个性化建议 -> 记忆写入链路
2. 样本验证：
   - 至少 20 个学习页面样本（中英文混合）
3. 人工验证：
   - 建议可执行性、相关性、可理解性评分
4. 回退验证：
   - 关闭学习模式后系统行为可恢复普通路径

## 验收矩阵

| 维度 | 指标 | 阈值 | 证据 |
|---|---|---|---|
| 解析能力 | 页面解析成功率 | >=95% | `tmp/stage-h-learning/extract.json` |
| 解释翻译 | 准确率 | >=90% | `tmp/stage-h-learning/explain-translate.json` |
| 建议质量 | 相关性评分 | >=85% | `tmp/stage-h-learning/recommend-eval.json` |
| 记忆可用 | 有效命中率 | >=80% | `tmp/stage-h-learning/memory-routing.json` |
| 回退能力 | 关闭学习模式后恢复正常 | =100% | `tmp/stage-h-learning/rollback.json` |
| 审计追踪 | trace_id 全链路贯通 | =100% | `tmp/stage-h-learning/audit-trace.json` |

## Gate 映射

- 对应阶段 Gate：Gate-H（子项 H-04）
- 当前覆盖：文档草案层
- 通过条件：验收矩阵达标且证据可复跑
