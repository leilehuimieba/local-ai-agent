# A 阶段 Tool Contract v1

更新时间：2026-04-10
阶段：A（冻结）
适用模块：`capabilities/*`、`tool_registry`、`execution`、`tool_trace`

## 1. 目标

1. 冻结工具元信息、调用快照、执行结果、错误语义。
2. 统一工具行为，避免 C 阶段出现“同类工具返回不一致”。
3. 为审计与回放提供最小统一字段集。

## 2. 工具定义（ToolDefinition）

来源：`crates/runtime-core/src/capabilities/spec.rs`

必填字段：

1. `tool_name`
2. `display_name`
3. `category`
4. `risk_level`
5. `input_schema`
6. `output_kind`
7. `requires_confirmation`

约束：

1. `tool_name` 全局唯一。
2. `risk_level` 允许：`low`、`medium`、`high`、`irreversible`。
3. `category` 使用受控枚举，不允许自由文本扩散。

## 3. 工具可见性策略

来源：`crates/runtime-core/src/capabilities/registry.rs`

1. `observe`：只允许只读工具。
2. `standard`：禁止高级写入工具（如 `memory_write`、`write_siyuan_knowledge`）。
3. `full_access`：允许全部工具。

## 4. 工具执行结果（ToolCallResult）

来源：`crates/runtime-core/src/capabilities/spec.rs`

必填字段：

1. `summary`
2. `final_answer`
3. `retryable`
4. `success`
5. `reasoning_summary`
6. `cache_status`
7. `cache_reason`

可选字段：

1. `artifact_path`
2. `error_code`
3. `memory_write_summary`

约束：

1. `success=false` 时必须有 `error_code`。
2. `retryable` 必须和错误语义一致。
3. `artifact_path` 存在时必须可读。

## 5. 工具执行追踪（ToolExecutionTrace）

字段：

1. `tool`
2. `action_summary`
3. `result`

约束：

1. 每次工具调用都必须产生 trace。
2. trace 必须可回放到 `RunEvent` 与日志。

## 6. 错误码规范（v1 最小集）

1. `invalid_input`
2. `path_not_found`
3. `permission_denied`
4. `command_failed`
5. `tool_timeout`
6. `serialization_failed`
7. `unsupported_tool`
8. `runtime_error`

规则：

1. 错误码稳定优先，文案可变。
2. 所有错误码必须映射到 `retryable`。

## 7. 审计字段（v1 必填）

在 `RunEvent.metadata` 中至少包含：

1. `tool_name`
2. `tool_category`
3. `risk_level`
4. `cache_status`
5. `output_kind`

C 阶段新增建议字段：

1. `tool_elapsed_ms`
2. `tool_attempt`
3. `tool_backend`

## 8. 与 Risk Policy 的关系

1. `requires_confirmation=true` 的工具必须由风险策略二次确认。
2. 即便 `requires_confirmation=false`，若命中高风险上下文也可升级确认。

## 9. 验收清单

1. 工具定义字段在 capability 输出中可完整读取。
2. 同类失败在不同工具上有一致错误码语义。
3. `tool_trace` 到 `RunEvent` 的关键字段映射完整。

## 10. 责任归属

1. Tool Contract Owner：`runtime-core/capabilities`。
2. 执行结果映射 Owner：`runtime-core/tool_trace`。
3. 网关消费契约 Owner：`gateway/internal/contracts`。
