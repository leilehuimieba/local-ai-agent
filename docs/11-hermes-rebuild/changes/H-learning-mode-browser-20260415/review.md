# 阶段性提审包（H-learning-mode-browser-20260415）

更新时间：2026-04-16  
提审类型：阶段 H 子项提审（H-04 学习模式与浏览器辅助）  
评审状态：已收口（建议签收）

## 1. 提审范围

本次提审覆盖 H-04 学习模式主线，不包含透明执行与运行时状态可视化实现。

覆盖范围：

1. 学习模式开关与接口合同（extract/explain/translate/value-score/recommend）。
2. 页面结构化抽取（标题/段落/选区/术语）。
3. 解释与翻译输出。
4. 价值判断与个性化学习建议。
5. 记忆路由联动（写入/读取/注入）与审计回放。
6. 回退路径（学习模式关闭降级）。

## 2. 对标口径（固定）

参考来源（仅作为对标，不做直接架构替代）：

1. `https://github.com/mutuyihao/yilan`
2. `docs/TECHNICAL_ARCHITECTURE.md`
3. `PRIVACY_POLICY.md`

对标边界：

1. 插件端负责采集与展示；
2. 核心策略与记忆治理由本地网关/运行时负责；
3. 禁止插件端承担长期记忆决策与核心编排。

## 3. 核心证据（回填区）

1. 聚合报告：
   - `tmp/stage-h-learning/latest.json`
2. 子证据：
   - `tmp/stage-h-learning/extract.json`
   - `tmp/stage-h-learning/explain-translate.json`
   - `tmp/stage-h-learning/value-score.json`
   - `tmp/stage-h-learning/recommend.json`
   - `tmp/stage-h-learning/memory-routing.json`
   - `tmp/stage-h-learning/audit-trace.json`
   - `tmp/stage-h-learning/rollback.json`
3. 样本集与评测：
   - `tmp/stage-h-learning/page-cases/*.json`
   - `tmp/stage-h-learning/recommend-eval.json`

## 4. 指标判定（回填区）

| 指标 | 阈值 | 实测 | 结论 |
|---|---|---|---|
| 页面解析成功率 | >= 95% | 100%（21/21） | passed |
| 价值判断输出完整性 | = 100% | 100%（主样本） | passed |
| 解释/翻译准确率 | >= 90% | 自动结构验证 100%（21/21），人工回看 100%（5/5） | passed |
| 学习建议相关性 | >= 85% | 自动结构验证 100%（21/21），人工回看 100%（5/5） | passed |
| 记忆命中有效率 | >= 80% | 100%（21/21） | passed |
| trace 全链路贯通率 | = 100% | 100%（21/21） | passed |
| 学习模式回退可用率 | = 100% | 100%（21/21） | passed |

当前已冻结子项：

1. `H04-01` 学习模式协议 v1：已冻结。
2. `H04-02a` BestBlogs provider 接入方案：已冻结。

当前已落地子项：

1. `H04-02` 最小采集入口：已提供 `POST /api/v1/learning/extract`，并完成 21 个 BestBlogs 样本批量实证。
2. `H04-04` 最小价值判断入口：已提供 `POST /api/v1/learning/value-score`，并完成主样本规则评分实证。
3. `H04-03` 最小解释/翻译入口：已提供 `POST /api/v1/learning/explain` 与 `POST /api/v1/learning/translate`，并完成 21 个样本自动结构实证与 5 个样本人工回看通过。
4. `H04-05` 最小学习建议入口：已提供 `POST /api/v1/learning/recommend`，并完成 21 个样本自动结构实证与 5 个样本人工回看通过。
5. `H04-06` 最小记忆写入入口：已提供 `POST /api/v1/learning/memory/write`，并完成 21 个样本的写入/读回/注入预览实证。
6. `H04-07` 最小审计入口：已提供 `POST /api/v1/learning/audit-trace`，并完成 21 个样本的 trace 串联与回放顺序实证。
7. `H04-08` 最小回退校验入口：已提供 `POST /api/v1/learning/rollback-check`，并完成 21 个样本的受控降级实证。

## 5. 评审结论

1. 本次提审结果：`status=passed`
2. H-04 就绪度：`h04.ready=true`
3. 阻塞项统计：`p0=0, p1=0, warning=1`
4. 结论说明：
   - 当前已达成“学习辅助可用 + 个性化建议可解释 + 记忆沉淀可回放”的最小闭环。
   - BestBlogs 主路径按公开 API 稳定工作，21 个样本在 extract / recommend / memory / audit / rollback 口径下均通过。
   - 当前 warning 仅为 browser fallback 仍处占位状态，不影响本轮最小闭环提审。

## 6. 风险与回退

1. 风险：页面噪声内容导致价值判断失真。
2. 风险：插件端职责膨胀，后续维护复杂度上升。
3. 风险：个性化建议缺上下文导致泛化建议。
4. 回退策略：
   - 关闭学习模式（`learning_mode_enabled=false`）；
   - 降级为“仅解释/翻译，不写记忆”。

## 7. 后续动作（按结论执行）

1. 若 `passed`：
   - 先等待当前主推进签收或切换下一主推进项。
   - 若继续补 H-04 采集侧，再进入 browser fallback 真实执行与失败样本验证。
2. 若 `warning`：
   - 记录告警责任人/追踪号/到期时间；
   - 针对样本薄弱项补测复审。
3. 若 `failed`：
   - 暂停学习模式扩展；
   - 回退到无记忆写入模式并修复后再提审。
