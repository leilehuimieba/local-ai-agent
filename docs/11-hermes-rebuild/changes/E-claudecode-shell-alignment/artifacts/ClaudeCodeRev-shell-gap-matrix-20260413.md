# ClaudeCodeRev 源码对照差距矩阵（2026-04-13）

## 1. 对照范围

1. 命令输出回灌机制（`stdout/stderr`）
2. 工具结果预算与外置
3. 权限判定与确认链
4. 上下文压缩边界

## 2. 差距矩阵

| 维度 | ClaudeCodeRev 机制（证据） | 当前实现（证据） | 结构性差距 | 后续任务 |
|---|---|---|---|---|
| 工具输出预算 | 全局阈值：`50k` 单结果、`100k token`、`200k` 单消息聚合预算（`tmp/ccr-snippets/constants__toolLimits.ts:13`） | 仅通用 artifact 外置阈值 `content_chars >= 240`（`crates/runtime-core/src/artifacts.rs:23`） | 缺少 shell 专项预算与“单消息聚合预算” | T07 |
| 大输出外置 | 大结果持久化，模型侧回 `<persisted-output>`，含路径+preview（`tmp/ccr-snippets/utils__toolResultStorage.ts:192`） | `run_command` 只回摘要文案；artifact 存的是 `action_summary/result_summary/final_answer` 拼接（`crates/runtime-core/src/executors/command.rs:105`，`crates/runtime-core/src/tool_trace.rs:57`） | 缺少“原始 stdout/stderr 可回放”的侧链 | T04/T05 |
| Shell 双轨展示 | Bash/PowerShell 均 `maxResultSizeChars: 30_000`，有 `persistedOutputPath`（`tmp/ccr-snippets/tools__BashTool__BashTool.tsx:644`，`tmp/ccr-snippets/tools__PowerShellTool__PowerShellTool.tsx:403`） | `ToolCallResult` 无 `stdout/stderr/persisted_output` 字段（`crates/runtime-core/src/capabilities/spec.rs:17`） | 三端合同缺“命令输出明细层” | T04 |
| UI 展示能力 | UI 解包 `<persisted-output>` 并可展开 shell 输出（`tmp/ccr-snippets/components__messages__UserBashOutputMessage.tsx:12`，`tmp/ccr-snippets/components__shell__ExpandShellOutputContext.tsx:11`） | 前端时间线主要消费 `summary/result_summary/artifact_path`（`frontend/src/events/EventTimeline.tsx:147`） | 用户端看不到统一 stdout/stderr 展开体验 | T05 |
| 权限规则层 | `alwaysAllow/alwaysDeny/alwaysAsk` 多源规则（`tmp/ccr-snippets/Tool.ts:123`） | 风险判定集中在 `assess_risk`（模式+关键词）`Proceed/RequireConfirmation/Blocked`（`crates/runtime-core/src/risk.rs:12`） | 缺少规则层与策略层分离 | T06 |
| ask 决策编排 | ask 链路串联 hooks/classifier/交互，含竞态防重（`resolveOnce + claim`）（`tmp/ccr-snippets/hooks__toolPermission__handlers__interactiveHandler.ts:70`） | 确认链路可用，但无同级的“自动判定 + 交互竞态编排” | 高阶权限路径缺位，后续易漂移 | T06 |
| 压缩边界 | 有 compact boundary、microcompact、snip replay，且边界后释放前序消息（`tmp/ccr-snippets/QueryEngine.ts:917`，`tmp/ccr-snippets/commands__compact__compact.ts:98`） | 会话压缩为最近轮次摘要拼接（`crates/runtime-core/src/compaction.rs:9`） | 缺少边界化压缩与预算协同 | T07 |

## 3. 最小实现切片（冻结）

1. T04 工具结果合同升级
2. T05 命令输出双轨展示
3. T06 权限决策链收口
4. T07 压缩边界与预算补齐

## 4. 约束

1. 每刀仅覆盖一个主题，不跨主题并改。
2. 每刀必须补 `verify.md` 证据，不以“描述完成”代替“验证完成”。
3. 未完成 T04 前，不并行推进 T05。
