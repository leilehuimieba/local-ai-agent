# H-learning-mode-browser-20260415（verify）

更新时间：2026-04-16
状态：部分已验证（协议冻结 + H04-02 达标）

## 验证方式

1. 集成验证：
   - 页面抽取 -> 解释翻译 -> 价值判断 -> 个性化建议 -> 记忆写入链路
2. 样本验证：
   - 至少 20 个学习页面样本（中英文混合）
   - 至少 1 个 BestBlogs 动态文章样本，验证 provider 主路径
3. 人工验证：
   - 建议可执行性、相关性、可理解性评分
4. 回退验证：
   - 关闭学习模式后系统行为可恢复普通路径
   - provider API 失败后可切换浏览器回退路径

## 验收矩阵

| 维度 | 指标 | 阈值 | 证据 |
|---|---|---|---|
| 解析能力 | 页面解析成功率 | >=95% | `tmp/stage-h-learning/extract.json` |
| 协议冻结 | 学习模式最小接口字段固定 | =100% | `design.md` |
| Provider 方案冻结 | BestBlogs 输入输出与回退路径固定 | =100% | `design.md` |
| Provider 主路径 | BestBlogs 文章读取成功率 | =100%（样本） | `tmp/stage-h-learning/bestblogs-provider.json` |
| Markdown 标准化 | `displayDocument` 转换成功 | =100%（样本） | `tmp/stage-h-learning/bestblogs-markdown.json` |
| Provider 回退 | API 失败后浏览器回退可用 | =100%（样本） | `tmp/stage-h-learning/bestblogs-fallback.json` |
| 价值判断 | 输出 `score/reason/next_action` | =100%（样本） | `tmp/stage-h-learning/value-score.json` |
| 解释翻译 | 准确率 | >=90% | `tmp/stage-h-learning/explain-translate.json` |
| 建议质量 | 相关性评分 | >=85% | `tmp/stage-h-learning/recommend.json` |
| 记忆可用 | 有效命中率 | >=80% | `tmp/stage-h-learning/memory-routing.json` |
| 回退能力 | 关闭学习模式后恢复正常 | =100% | `tmp/stage-h-learning/rollback.json` |
| 审计追踪 | trace_id 全链路贯通 | =100% | `tmp/stage-h-learning/audit-trace.json` |

## Gate 映射

- 对应阶段 Gate：Gate-H（子项 H-04）
- 当前覆盖：BestBlogs provider 主路径、学习模式最小采集入口与 Markdown 标准化已补批量样本证据；browser fallback 仅保留占位说明
- 当前结论：
  - `h04.contract_ready=true`
  - `h04.bestblogs_contract_ready=true`
  - `h04.extract_entry_ready=true`
  - `h04.extract_parse_ready=true`
  - `h04.value_score_ready=true`
  - `h04.explain_translate_auto_ready=true`
  - `h04.explain_translate_ready=true`
  - `h04.recommend_ready=true`
  - `h04.memory_routing_ready=true`
  - `h04.audit_trace_ready=true`
  - `h04.rollback_ready=true`
- 通过条件：验收矩阵达标且证据可复跑

## 本轮最小证据

1. 主路径样本：
   - `tmp/stage-h-learning/extract.json`
   - `tmp/stage-h-learning/bestblogs-provider.json`
   - `extract.json` 覆盖 21 个 BestBlogs 文章样本，成功 21 个，成功率 `100%`
   - 覆盖 `article_id=42acaf7d`、标题、summary、html、markdown、images
   - API handler 最小边界测试已补齐：`/api/v1/providers/bestblogs/article/read` 与 `/api/v1/learning/extract` 已覆盖成功路径、`method not allowed`、缺少 `article_url`、`BESTBLOGS_INVALID_INPUT` 透传格式
2. Markdown 样本：
   - `tmp/stage-h-learning/bestblogs-markdown.json`
   - 覆盖关键短语：
     - `为什么我们需要浏览器自动化`
     - `未来软件竞争维度`
3. fallback 说明：
   - `tmp/stage-h-learning/bestblogs-fallback.json`
   - 说明本轮未启用 browser fallback，仅保留主路径外的占位口径
4. value-score 样本：
   - `tmp/stage-h-learning/value-score.json`
   - 覆盖 `article_id=42acaf7d` 的 `score/level/reason/next_action/signals`
5. explain / translate 样本：
   - `tmp/stage-h-learning/explain-translate.json`
   - 覆盖 21 个 BestBlogs 样本的 explain / `reader_bridge` translate 自动结构验证
   - 人工回看 5 个样本，`manual_pass_rate=100%`
6. recommend 样本：
   - `tmp/stage-h-learning/recommend.json`
   - 覆盖 21 个 BestBlogs 样本的规则版学习建议自动结构验证
   - 人工回看 5 个样本，`manual_pass_rate=100%`
7. memory-routing 样本：
   - `tmp/stage-h-learning/memory-routing.json`
   - 覆盖 21 个 BestBlogs 样本的长期记忆写入、读回命中与注入预览
   - `effective_hit_rate=100%`
8. audit-trace 样本：
   - `tmp/stage-h-learning/audit-trace.json`
   - 覆盖 21 个 BestBlogs 样本的 `extract -> explain -> translate -> value_score -> recommend -> memory_write` 聚合审计链
   - `trace_link_rate=100%`
9. rollback 样本：
   - `tmp/stage-h-learning/rollback.json`
   - 覆盖 21 个 BestBlogs 样本在 `learning_mode_enabled=false` 时的受控降级结果
   - `rollback_pass_rate=100%`
