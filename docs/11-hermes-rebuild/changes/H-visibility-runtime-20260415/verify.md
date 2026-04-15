# H-visibility-runtime-20260415（verify）

更新时间：2026-04-15
状态：草案

## 1. 验证方式

1. 合同测试：
   - runtime -> gateway -> frontend 字段一致性检查
2. 集成测试：
   - 成功链、等待确认链、失败重试链、人工接管链
3. 人工验证：
   - 真实任务观察“当前状态、当前工具、等待原因、下一步建议”是否可读

## 2. 验收矩阵

| 维度 | 指标 | 阈值 | 证据文件 |
|---|---|---|---|
| 可视化覆盖 | 主链路关键事件可视化覆盖率 | >= 98% | `tmp/stage-h-visibility/latest.json` |
| 透明反馈 | 无反馈卡住 >60s 占比 | <= 1% | `tmp/stage-h-visibility/stall.json` |
| 失败可接管 | 失败事件含“原因+下一步建议”比例 | = 100% | `tmp/stage-h-visibility/failure-route.json` |
| 合同一致性 | 三端字段一致性 | = 100% | `tmp/stage-h-visibility/contracts.json` |
| 回退可用 | 关闭 visibility_v1 后可回退旧展示 | = 100% | `tmp/stage-h-visibility/rollback.json` |

## 3. 证据位置（预留）

1. `tmp/stage-h-visibility/latest.json`
2. `tmp/stage-h-visibility/runtime.json`
3. `tmp/stage-h-visibility/gateway.json`
4. `tmp/stage-h-visibility/ui-state.json`
5. `tmp/stage-h-visibility/ui-detail.json`
6. `tmp/stage-h-visibility/stall.json`
7. `tmp/stage-h-visibility/failure-route.json`
8. `tmp/stage-h-visibility/contracts.json`
9. `tmp/stage-h-visibility/rollback.json`

## 4. Gate 映射

1. 对应阶段 Gate：Gate-H（子项 H-01）
2. 当前覆盖情况：
   - 仅文档草案，尚无执行证据
3. 通过判定：
   - 验收矩阵全部达阈值，且可复跑
