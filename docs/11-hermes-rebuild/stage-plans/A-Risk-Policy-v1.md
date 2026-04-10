# A 阶段 Risk Policy v1

更新时间：2026-04-10
阶段：A（冻结）
适用模块：`risk.rs`、`chat.go`、`confirmation_store.go`

## 1. 目标

1. 冻结风险分级、确认策略、模式拦截规则。
2. 避免 C 阶段改造时出现策略散落与行为漂移。
3. 确保“高风险动作必须可拦截、可确认、可追溯”。

## 2. 风险等级定义

1. `low`：只读操作，默认放行。
2. `medium`：潜在修改，按模式和上下文决定。
3. `high`：高影响动作，默认要求确认。
4. `irreversible`：不可逆动作，必须确认。

## 3. 模式策略矩阵

1. `observe`

- 拒绝所有修改性动作。
- 返回 `blocked_by_mode`。

2. `standard`

- 允许常规读写。
- 高风险和不可逆动作要求确认。

3. `full_access`

- 允许全部动作。
- 仍保留高风险确认链。

## 4. 确认策略（v1）

### 4.1 工作区首次接触确认

触发条件：

1. `context_hints.workspace_first_seen=true`
2. 本次 run 未携带通过确认决策

结果：

1. `kind=workspace_access`
2. `risk_level=medium`
3. `status=awaiting_confirmation`

### 4.2 高风险动作确认

触发条件：

1. `PlannedAction::DeletePath`
2. `PlannedAction::RunCommand` 命中危险命令特征

结果：

1. `kind=high_risk_action`
2. `risk_level=high|irreversible`
3. 必须返回 `target_paths`、`hazards`、`alternatives`

## 5. 危险命令识别（v1）

沿用当前实现关键字：

1. `remove-item`
2. `del `
3. ` rd `
4. `rm `
5. `rm-`
6. `rmdir`
7. `erase `

规则：

1. 仅作为 v1 基线，C 阶段扩展为规则表配置。
2. 识别命中即进入高风险确认，不直接执行。

## 6. 决策语义

允许值：

1. `approve`
2. `reject`
3. `cancel`

约束：

1. `approve` 才允许继续执行。
2. `reject/cancel` 必须发布结束事件。
3. `remember=true` 仅允许在 `workspace_access` 场景持久化。

## 7. 审计要求

每次风险判定必须可在事件或日志中还原：

1. `risk_level`
2. `confirmation_id`
3. `kind`
4. `reason`
5. `decision`

## 8. 失败保护

1. 策略异常时默认提高保护级别，不允许直接放行高风险动作。
2. 确认链路异常时返回 `awaiting_confirmation`，禁止隐式降级执行。

## 9. C 阶段扩展点（冻结预留）

1. 目录级 allowlist 与规则版本号。
2. 命令 AST 级风险识别。
3. 策略热更新与回滚版本。

## 10. 验收清单

1. `observe` 模式下修改动作必被阻断。
2. 删除动作必触发确认。
3. 危险命令关键词样本必触发确认。
4. `reject/cancel` 后 run 必正确收口。

## 11. 责任归属

1. 策略 Owner：`runtime-core/risk.rs`。
2. 确认流 Owner：`gateway/internal/api/chat.go`。
3. 状态存储 Owner：`gateway/internal/state/confirmation_store.go`。
