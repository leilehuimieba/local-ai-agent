# 项目映射

把 harness 清单映射到本仓库现有文档和模块，减少重复摸索。

## 1. 先读哪些文档

默认顺序：

1. `docs/README.md`
2. `docs/02-architecture/本地适配架构原则_V1.md`
3. `docs/06-development/智能体框架主干开发任务书_V1.md`

涉及验收时，再看：

1. `docs/07-test/验收标准_V1.md`
2. `docs/07-test/智能体框架主干总体验收文档_V1.md`
3. `docs/07-test/V1回归检查入口_V1.md`

## 2. 清单到模块的对应关系

### 主循环

优先看：

1. `crates/runtime-core/src/lib.rs`
2. `crates/runtime-core/src/planner.rs`
3. `crates/runtime-core/src/execution.rs`
4. `crates/runtime-core/src/session.rs`

需要回答：

1. 主循环阶段是否清晰
2. 等待确认和失败是否能回主线

### 上下文装配

优先看：

1. `crates/runtime-core/src/context_builder.rs`
2. `crates/runtime-core/src/repo_context.rs`
3. `crates/runtime-core/src/prompt.rs`
4. `crates/runtime-core/src/compaction.rs`

需要回答：

1. 上下文是否按需装配
2. 是否存在过量注入
3. 是否有压缩和 preview 机制

### 工具协议

优先看：

1. `crates/runtime-core/src/tool_registry.rs`
2. `crates/runtime-core/src/execution.rs`
3. `crates/runtime-core/src/contracts.rs`
4. `crates/runtime-core/src/tool_trace.rs`

需要回答：

1. 工具输入输出结构是否统一
2. 风险和确认信息是否成为协议一部分

### 权限与确认

优先看：

1. `crates/runtime-core/src/risk.rs`
2. `gateway/internal/state/confirmation_store.go`
3. `gateway/internal/api/chat.go`
4. `gateway/internal/runtime/client.go`

需要回答：

1. 确认是否前置
2. 确认结果是否能恢复运行态

### 记忆与知识路由

优先看：

1. `crates/runtime-core/src/memory.rs`
2. `crates/runtime-core/src/memory_router.rs`
3. `crates/runtime-core/src/memory_recall.rs`
4. `crates/runtime-core/src/knowledge.rs`
5. `crates/runtime-core/src/knowledge_store.rs`

需要回答：

1. 是否按需召回
2. 是否存在全量注入倾向
3. 写回条件是否收紧

### Artifact 外置

优先看：

1. `crates/runtime-core/src/artifacts.rs`
2. `crates/runtime-core/src/storage.rs`
3. `crates/runtime-core/src/paths.rs`

需要回答：

1. 大结果是否从主链路分流
2. 是否只回流摘要和引用

### 事件与合同

优先看：

1. `crates/runtime-core/src/events.rs`
2. `crates/runtime-core/src/contracts.rs`
3. `gateway/internal/contracts/contracts.go`
4. `gateway/internal/session/bus.go`
5. `frontend/src/shared/contracts.ts`

需要回答：

1. 三端事件字段是否对齐
2. 是否依赖易变字符串

## 3. 实施优先级

如果改动范围较大，按这个顺序收口：

1. 主循环
2. 工具协议
3. 权限与确认
4. 上下文装配
5. 记忆与知识路由
6. artifact 外置
7. 事件合同补齐

理由：

1. 没有主循环，后面的壳无处挂载
2. 没有工具协议和权限闸门，系统会继续旁路执行
3. 没有 artifact 和事件结构，后续验证和前端消费会持续失稳

## 4. 评审输出模板

做方案评审或代码评审时，优先按下面结构输出：

1. `当前补的是哪层壳`
2. `当前最明显的薄弱层`
3. `最小收口路径`
4. `需要新增的验证证据`
5. `明确后置的高级能力`
