# 阶段性提审包（H-memory-routing-kb-20260415）

更新时间：2026-04-16  
提审类型：阶段 H 子项提审（H-05 记忆路由与知识沉淀）  
评审状态：已签收

## 1. 提审范围

本次提审仅覆盖 H-05 记忆路由与知识沉淀，不包含 H-04 学习模式采集实现与前端联调。

覆盖项：

1. 记忆分类与写入判据。
2. 命中解释与注入预算。
3. 回退演练与证据口径。

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-memory-routing-kb-20260415/verify.md`

## 3. 核心证据

### 3.1 聚合报告

1. `D:/newwork/本地智能体/tmp/stage-h-memory-routing/latest.json`

### 3.2 子证据

1. `D:/newwork/本地智能体/tmp/stage-h-memory-routing/injection-audit.json`
2. `D:/newwork/本地智能体/tmp/stage-h-memory-routing/rollback-drill.json`
3. `D:/newwork/本地智能体/tmp/stage-h-learning/memory-routing.json`

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| 记忆命中有效率 | >= 80% | 100%（21/21） | PASS | `tmp/stage-h-memory-routing/latest.json` |
| 误注入率 | <= 5% | 0%（`over_budget_count=0`） | PASS | `tmp/stage-h-memory-routing/injection-audit.json` |
| 回退可用率 | = 100% | 100%（2/2 drill cases） | PASS | `tmp/stage-h-memory-routing/rollback-drill.json` |

## 5. 评审结论

1. 本次提审结果：`status=passed`
2. 就绪度判定：`h05.ready=true`
3. 签收结论：`h05.signed_off=true`
4. 阻塞项统计：`p0=0, p1=0, warning=1`
5. 结论说明：
   - 当前已完成 H-05 最小闭环：写入阈值、命中解释、注入预算与回退演练均已有独立证据。
   - 21 个 BestBlogs 学习样本全部通过，当前独立证据显示 `effective_hit_rate=100%`、`over_budget_count=0`。
   - 当前 warning 为 recall 仍停留在“同文 article_id/title 命中”，尚未覆盖“同主题不同文章”的复用场景，但不影响本轮最小治理闭环签收。

## 6. 风险与回退

1. 风险：
   - 学习文章摘要长度与主题粒度不稳定时，仍可能造成注入质量波动。
   - recall 仅按同文命中，跨文章主题复用能力仍不足。
2. 回退触发条件：
   - 出现注入噪声升高或误注入率超阈值。
   - 学习记忆开始干扰主执行上下文。
3. 回退动作：
   - 关闭自动写入或降级为 `skip/manual_confirm`。
   - 注入侧只保留预览，不自动并入主执行上下文。

## 7. 后续动作

1. 若 `passed`：
   - 当前已完成 H-05 签收，可切换到下一个 H 阶段主推进项，或进入 H-G1 聚合准备。
   - 若继续深化 H-05，再补“同主题不同文章” recall 与误注入样本。
2. 若 `warning`：
   - 责任人：`待定`
   - 追踪编号：`H05-recall-broadening`
   - 到期时间：`待下一主推进项确定后回填`
   - 补证动作：补跨文章主题 recall 与误注入样本。
3. 若 `failed`：
   - 回退到仅保留学习写入预览的最小模式。
   - 暂停自动注入并补测后再提审。

## 8. Gate 映射

1. 对应 Gate：`Gate-H`
2. 覆盖项：
   - H-05 记忆路由最小治理闭环
   - 独立聚合证据、注入预算审计、回退演练
3. 未覆盖项（如有）：
   - 同主题跨文章 recall（原因：本轮保持最小规则版，不扩到语义召回）
