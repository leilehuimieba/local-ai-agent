# H-visibility-runtime-20260415（tasks）

更新时间：2026-04-15
状态：进行中

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| H01-01 | 冻结透明执行合同字段与事件映射 | 设计 | done | 字段与映射冻结并文档化 | `design.md` |
| H01-02 | runtime 事件补齐 `stage/activity/heartbeat` | 实现 | done | 事件含最小字段集合 | `tmp/stage-h-visibility/runtime.json` |
| H01-03 | gateway 合同映射与透传 | 实现 | done | API/stream/logs 字段一致 | `tmp/stage-h-visibility/gateway.json` + `tmp/stage-h-visibility/contracts.json` |
| H01-04 | 前端状态条+任务卡 | 实现 | doing | 可显示阶段/任务/等待原因 | `tmp/stage-h-visibility/ui-state.json`（待补） |
| H01-05 | 工具详情抽屉（预览+原文引用） | 实现 | todo | 可跳转 raw_output_ref/artifact_path | `tmp/stage-h-visibility/ui-detail.json` |
| H01-06 | 卡住检测与提示策略 | 实现 | todo | 30/60/120s 阈值行为正确 | `tmp/stage-h-visibility/stall.json` |
| H01-07 | 失败分流可视化 | 实现 | todo | retry/manual/stop 可见 | `tmp/stage-h-visibility/failure-route.json` |
| H01-08 | 回归与提审材料 | 验证 | todo | 指标达标并可复跑 | `verify.md` + `review.md` |

## 执行顺序

1. 主链路：H01-01 -> H01-02 -> H01-03 -> H01-04 -> H01-05 -> H01-06 -> H01-07 -> H01-08。
2. 可并行项：H01-05 与 H01-06 在 H01-04 后可并行。
3. 阻塞项：字段合同未冻结前，不进入前端实现（已解除）。
