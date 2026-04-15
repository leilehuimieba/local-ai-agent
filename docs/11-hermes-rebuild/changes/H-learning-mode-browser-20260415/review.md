# 阶段性提审包（H-learning-mode-browser-20260415）

更新时间：2026-04-15  
提审类型：阶段 H 子项提审（H-04 学习模式与浏览器辅助）  
评审状态：草案（待实证回填）

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
| 页面解析成功率 | >= 95% | 待回填 | 待判定 |
| 解释/翻译准确率 | >= 90% | 待回填 | 待判定 |
| 学习建议相关性 | >= 85% | 待回填 | 待判定 |
| 记忆命中有效率 | >= 80% | 待回填 | 待判定 |
| trace 全链路贯通率 | = 100% | 待回填 | 待判定 |
| 学习模式回退可用率 | = 100% | 待回填 | 待判定 |

## 5. 评审结论（模板）

1. 本次提审结果：`status=<passed|warning|failed>`
2. H-04 就绪度：`h04.ready=<true|false>`
3. 结论说明（必填）：
   - 是否达成“学习辅助可用 + 个性化建议可解释 + 记忆沉淀可回放”。

## 6. 风险与回退

1. 风险：页面噪声内容导致价值判断失真。
2. 风险：插件端职责膨胀，后续维护复杂度上升。
3. 风险：个性化建议缺上下文导致泛化建议。
4. 回退策略：
   - 关闭学习模式（`learning_mode_enabled=false`）；
   - 降级为“仅解释/翻译，不写记忆”。

## 7. 后续动作（按结论执行）

1. 若 `passed`：
   - 进入 H-05（记忆路由深化）或 H-G1 聚合准备。
2. 若 `warning`：
   - 记录告警责任人/追踪号/到期时间；
   - 针对样本薄弱项补测复审。
3. 若 `failed`：
   - 暂停学习模式扩展；
   - 回退到无记忆写入模式并修复后再提审。
