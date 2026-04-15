# 阶段性提审包（H-visibility-runtime-20260415）

更新时间：2026-04-15  
提审类型：阶段 H 子项提审（H-01 透明执行可视化）  
评审状态：草案（待实证回填）

## 1. 提审范围

本次提审仅覆盖 H-01 透明执行主线，不包含学习模式、记忆路由和 MCP/Skills 扩展实现。

覆盖范围：

1. 运行状态可视化合同（stage/activity/task/heartbeat/stall）。
2. 当前任务卡与等待原因展示。
3. 工具执行详情可展开（摘要 + 原文引用）。
4. 卡住检测与提示策略（30/60/120 秒阈值）。
5. 失败分流可视化（retry/manual/stop）。

## 2. 核心证据（回填区）

1. 聚合报告：
   - `tmp/stage-h-visibility/latest.json`
2. 子证据：
   - `tmp/stage-h-visibility/runtime.json`
   - `tmp/stage-h-visibility/gateway.json`
   - `tmp/stage-h-visibility/ui-state.json`
   - `tmp/stage-h-visibility/ui-detail.json`
   - `tmp/stage-h-visibility/stall.json`
   - `tmp/stage-h-visibility/failure-route.json`
   - `tmp/stage-h-visibility/contracts.json`
   - `tmp/stage-h-visibility/rollback.json`
3. 构建/测试（按实际回填）：
   - `cargo test -p runtime-core`
   - `go test ./...`
   - `npm run build`

## 3. 指标判定（回填区）

| 指标 | 阈值 | 实测 | 结论 |
|---|---|---|---|
| 主链路可视化覆盖率 | >= 98% | 待回填 | 待判定 |
| 无反馈卡住 >60s 占比 | <= 1% | 待回填 | 待判定 |
| 失败事件含“原因+下一步”比例 | = 100% | 待回填 | 待判定 |
| 三端合同一致性 | = 100% | 待回填 | 待判定 |
| 回退开关可用率 | = 100% | 待回填 | 待判定 |

## 4. 评审结论（模板）

1. 本次提审结果：`status=<passed|warning|failed>`
2. H-01 就绪度：`h01.ready=<true|false>`
3. 结论说明（必填）：
   - 通过：达到阈值，证据可复跑；
   - 警告：不阻塞但需限期补齐；
   - 失败：存在阻塞项，不得推进 H-G1。

## 5. 风险与回退

1. 风险：状态字段跨端漂移导致 UI 显示错误。
2. 风险：心跳阈值不合理造成误报“卡住”。
3. 回退策略：
   - 启用 `visibility_v1_enabled=false` 回退旧展示链路；
   - 保留新字段兼容读取，避免历史记录不可见。

## 6. 后续动作（按结论执行）

1. 若 `passed`：
   - 进入 H-02 或并行推进已批准的 H 子项；
   - 更新对应 `status.md`、`verify.md`。
2. 若 `warning`：
   - 记录责任人、追踪号、到期时间；
   - 限期补证后复审。
3. 若 `failed`：
   - 标记 blocker；
   - 回退到最近稳定版本，重新提审。
