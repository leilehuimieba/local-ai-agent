# H-visibility-runtime-20260415（verify）

更新时间：2026-04-15
状态：部分通过（实现轮进行中）

## 1. 验证方式

1. 合同测试：
   - runtime -> gateway -> frontend 字段一致性检查
2. 集成测试：
   - 成功链、等待确认链、失败重试链、人工接管链
3. 人工验证：
   - 观察“当前状态、当前工具、等待原因、下一步建议”是否可读

## 2. 验收矩阵

| 维度 | 指标 | 阈值 | 当前结果 | 证据文件 |
|---|---|---|---|---|
| 合同冻结 | 字段与映射冻结完成率 | = 100% | 100%（已通过） | `design.md` |
| 可视化覆盖 | 主链路关键事件可视化覆盖率 | >= 98% | 100%（runtime/gateway 主链路） | `tmp/stage-h-visibility/latest.json` |
| 透明反馈 | 无反馈卡住 >60s 占比 | <= 1% | 通过（已覆盖 35/75/130s 三阈值样本） | `tmp/stage-h-visibility/stall.json` |
| 失败可接管 | 失败事件含“原因+下一步建议”比例 | = 100% | 通过（`retry/manual/stop` 三路可见） | `tmp/stage-h-visibility/failure-route.json` |
| 合同一致性 | 三端字段一致性 | = 100% | 100%（H01-02/H01-03 范围） | `tmp/stage-h-visibility/contracts.json` |
| 回退可用 | 关闭 visibility_v1 后可回退旧展示 | = 100% | 100%（回退说明已补齐） | `tmp/stage-h-visibility/rollback.json` |
| 前端最小消费 | 任务卡可显示 `task_title/activity_state/waiting_reason/next_action_hint` | = 100% | 100%（成功链路样本） | `tmp/stage-h-visibility/ui-state.json` |
| waiting 分支覆盖 | `confirmation_required` 样本包含 `waiting_reason=confirmation` 且网关事件/日志可见 | = 100% | 100%（runtime/gateway/log 全覆盖） | `tmp/stage-h-visibility/ui-state-waiting.json` + `tmp/stage-h-visibility/latest.json` |
| 详情抽屉字段 | `evidence_ref/raw_output_ref/artifact_path` 字段可映射并消费 | = 100% | 100%（已补齐） | `tmp/stage-h-visibility/ui-detail.json` |
| 上下文预算 | 默认上下文预算提升到 512k token 且可从 runtime 预算字段可见 | = 100% | 100%（已通过） | `tmp/stage-h-visibility/context-budget-runtime-core-tests.txt` |

## 3. 证据位置

1. 文档证据：
   - `docs/11-hermes-rebuild/changes/H-visibility-runtime-20260415/design.md`
   - `docs/11-hermes-rebuild/changes/H-visibility-runtime-20260415/tasks.md`
   - `docs/11-hermes-rebuild/changes/H-visibility-runtime-20260415/status.md`
2. 实现证据（预留）：
   - `tmp/stage-h-visibility/latest.json`
   - `tmp/stage-h-visibility/runtime.json`
   - `tmp/stage-h-visibility/runtime-confirmation.json`
   - `tmp/stage-h-visibility/gateway.json`
   - `tmp/stage-h-visibility/gateway-confirmation.json`
   - `tmp/stage-h-visibility/ui-state.json`
   - `tmp/stage-h-visibility/ui-state-waiting.json`
   - `tmp/stage-h-visibility/ui-detail.json`
   - `tmp/stage-h-visibility/stall.json`
   - `tmp/stage-h-visibility/failure-route.json`
   - `tmp/stage-h-visibility/contracts.json`
   - `tmp/stage-h-visibility/rollback.json`
   - `tmp/stage-h-visibility/context-budget-runtime-core-tests.txt`

## 4. Gate 映射

1. 对应阶段 Gate：Gate-H（子项 H-01）
2. 当前覆盖情况：
   - 文档合同层已通过（H01-01）
   - runtime/gateway 合同实现层已通过（H01-02/H01-03）
   - frontend 可视化层已通过（H01-04/H01-05/H01-06/H01-07）
3. 当前结论：
   - `h01.contract_ready=true`
   - `h01.implementation_ready=true`
   - `h01.context_budget_512k_ready=true`
   - `h01.ready_for_signoff=true`
