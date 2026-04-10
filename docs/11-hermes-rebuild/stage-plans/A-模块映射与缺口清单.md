# A 阶段模块映射与缺口清单

更新时间：2026-04-10
阶段：A（基线收口与架构冻结）
关联总路线：[Hermes重构总路线图_完整计划](D:/newwork/本地智能体/docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md)

## 1. 说明

1. 本清单用于回答“现有代码谁负责什么、下一步补什么、以什么标准算完成”。
2. Owner 先按模块责任域定义，后续可映射到具体人员。
3. A 阶段不改代码，只冻结映射关系和执行入口。

## 2. 模块映射总表

| 模块 | 责任域 | 当前状态 | 关键证据 | 主要缺口 | Owner（责任域） | 承接阶段 |
|---|---|---|---|---|---|---|
| `crates/runtime-core/src/lib.rs` | Runtime 主循环编排与事件主线 | 部分完成 | `simulate_run*` 主流程 | 恢复与断点机制未统一 | Runtime 内核 | B |
| `crates/runtime-core/src/query_engine.rs` | 阶段执行编排 | 部分完成 | stage 执行路径 | 阶段重入策略需明确 | Runtime 内核 | B |
| `crates/runtime-core/src/planner.rs` | 动作规划与路由 | 可用 | `PlannedAction` | 缺少策略版本与评测反馈 | Runtime 策略 | B |
| `crates/runtime-core/src/execution.rs` | 动作执行分发 | 可用 | executor 分流 | 错误模型与超时策略未统一 | Runtime 执行 | C |
| `crates/runtime-core/src/executors/*` | 工具动作落地 | 部分完成 | command/workspace/memory 等 | 缺少统一工具 contract 与重试治理 | 工具平台 | C |
| `crates/runtime-core/src/tool_registry.rs` | 能力注册与连接器目录 | 可用 | capability/connector catalog | contract 版本化缺失 | 工具平台 | C |
| `crates/runtime-core/src/risk.rs` | 风险评估与确认链 | 可用 | `RiskOutcome` | 风险规则粒度和误判治理不足 | 安全与策略 | C |
| `crates/runtime-core/src/verify.rs` | 执行验证与下一步建议 | 可用 | verification report | 验证覆盖集与门禁绑定不足 | 质量与验证 | B/C |
| `crates/runtime-core/src/memory.rs` | 长短期记忆存取 | 部分完成 | 写入/检索/治理字段 | 污染检测与治理基线需增强 | 记忆与知识 | D |
| `crates/runtime-core/src/memory_router.rs` | Finish 阶段记忆写回策略 | 部分完成 | `evaluate_finish_memory_writes` | 写回策略评测和回滚缺失 | 记忆与知识 | D |
| `crates/runtime-core/src/knowledge.rs` | 文档与沉淀知识检索 | 部分完成 | 本地文档+知识库混合检索 | 语义检索与效果评估未标准化 | 记忆与知识 | D |
| `crates/runtime-core/src/sqlite_store.rs` | SQLite 主存储落地 | 部分完成 | memory/sqlite 接口 | schema 迁移版本治理不足 | 数据与迁移 | B/D |
| `crates/runtime-core/src/storage_migration.rs` | 存储迁移 | 雏形 | migration 模块存在 | 迁移策略与回滚脚本未成套 | 数据与迁移 | F |
| `crates/runtime-host/src/main.rs` | Runtime Host 入口 | 雏形可用 | HTTP 路由基础 | 协议层鲁棒性与并发能力不足 | Host 层 | B/E |
| `gateway/internal/api/router.go` | 网关路由与系统配置 API | 可用 | `/settings` `/chat/*` `/events/stream` | API contract 版本化不足 | 网关 | E |
| `gateway/internal/api/chat.go` | run/confirm 会话控制 | 可用 | run context 与确认流程 | 对失败恢复与幂等需增强 | 网关 | E |
| `gateway/internal/runtime/client.go` | Gateway-Runtime 通信 | 部分完成 | runtime client | 超时/重试/熔断策略不足 | 网关 | C/E |
| `gateway/internal/state/*` | 设置、确认、凭据状态 | 部分完成 | store 族模块 | 状态一致性与并发写策略需强化 | 网关状态 | C/E |
| `frontend/src/workspace/*` | 工作台视图与交互壳层 | 可用 | workbench 组件 | 与 Gate 指标联动不完整 | 前端体验 | E |
| `frontend/src/events/*` | 事件流展示 | 可用 | timeline + event stream | 事件语义等级和过滤策略需统一 | 前端体验 | E |
| `frontend/src/settings/*` | 设置与诊断页 | 可用 | settings/status card | doctor 级自检联动不足 | 前端体验 | F |
| `frontend/src/history/*` | 历史回顾与资源区 | 部分完成 | history panels | 运行快照与恢复信息展示不足 | 前端体验 | E |

## 3. P0 缺口清单（必须优先）

1. Runtime checkpoint/resume 协议未定稿。
2. Tool Contract 规范未统一（参数、错误、超时、trace）。
3. 风险策略与权限确认链尚未形成版本治理。
4. Windows 安装、诊断、回滚链路尚未产品化。

## 4. P1 缺口清单（紧随 P0）

1. 记忆/知识召回效果评测集缺失。
2. CLI/TUI 主入口不完整，当前交互重心偏前端。
3. 网关多入口抽象与协议兼容策略不足。

## 5. 阶段 A 可执行任务（本周）

1. 冻结 `Runtime Run Contract v1` 草案，明确恢复字段。
2. 冻结 `Tool Contract v1` 草案，统一错误码与审计字段。
3. 冻结 `Risk Policy v1` 草案，定义风险等级和确认门槛。
4. 建立 `Windows 发布链路清单 v1`，定义安装、自检、回滚最小要求。

## 6. 验收标准（Gate-A 对齐）

1. 每个核心能力都有明确模块与后续阶段承接。
2. 每个 P0 缺口都有单独文档入口与冻结版本号。
3. 阶段 B 不再接收未入表的新核心能力诉求。

## 7. 当前判定

1. `模块映射完整性：通过`。
2. `Owner 可执行性：通过（责任域已定义，待具体人名映射）`。
3. `可进入 B 阶段前置条件：满足（需先补三份 v1 contract 草案）`。
