# A 阶段能力对标矩阵

更新时间：2026-04-10
阶段：A（基线收口与架构冻结）
关联总路线：[Hermes重构总路线图_完整计划](D:/newwork/本地智能体/docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md)

## 1. 评估口径

1. 目标不是逐行复刻 Hermes，而是达到同级能力并在 Windows 原生体验上可超越。
2. 评分采用 `0-3`：`0=缺失`，`1=雏形`，`2=可用`，`3=成熟`。
3. A 阶段只做事实盘点和缺口冻结，不做超范围功能开发。

## 2. 能力对标矩阵

| 维度 | Hermes 目标基线 | 当前项目现状 | 当前评分 | 证据（代码/文档） | 主要缺口 | 对应阶段 |
|---|---|---|---|---|---|---|
| Runtime 主循环 | 稳定 `analyze-plan-act-verify-finish`，可恢复 | 已有主循环与阶段事件 | 2 | `crates/runtime-core/src/lib.rs`、`crates/runtime-core/src/query_engine.rs` | checkpoint/resume 机制未产品化 | B |
| 动作规划 | 规则与上下文结合，支持自然语言路由 | 具备 `PlannedAction` + 前缀/语义分流 | 2 | `crates/runtime-core/src/planner.rs` | 规划策略缺少策略版本与评测集 | B |
| 工具执行层 | 统一工具契约与执行回放 | 已按 executor 分层执行 | 2 | `crates/runtime-core/src/execution.rs`、`crates/runtime-core/src/executors/*` | 工具协议字段与错误码未统一治理 | C |
| 工具注册与能力元信息 | 能力与连接器可枚举 | 已有 capability/connector catalog | 2 | `crates/runtime-core/src/tool_registry.rs`、`capabilities/*` | 缺少版本化 contract 与兼容策略 | C |
| 风险控制与确认链 | 高风险动作可拦截、可确认 | 已有风险评估与确认请求 | 2 | `crates/runtime-core/src/risk.rs`、`gateway/internal/api/chat.go` | 风险策略粒度仍偏粗，误判评估不足 | C |
| 验证链 | 每步执行后有验证结论 | 已有 verify 报告与 next_step | 2 | `crates/runtime-core/src/verify.rs` | 验证策略覆盖不足，需与回归集绑定 | B/C |
| 会话与状态持久化 | 支持恢复、追溯、审计 | 已有 session/storage/sqlite 模块 | 2 | `crates/runtime-core/src/session.rs`、`sqlite_store.rs`、`storage.rs` | 断点恢复入口缺少统一 API | B |
| 记忆系统 | 短期/长期/知识分层，治理可解释 | 三层结构已存在，含治理字段 | 2 | `crates/runtime-core/src/memory.rs`、`memory_router.rs` | 记忆污染防控与评估基线要强化 | D |
| 知识检索 | 文件+沉淀知识混合召回 | 已支持本地文档+沉淀知识 | 2 | `crates/runtime-core/src/knowledge.rs` | 召回质量评估和语义扩展未成体系 | D |
| 模型与 Provider 路由 | 多 provider 可切换可校验 | 网关侧已支持 provider 解析 | 2 | `gateway/internal/api/chat.go`、`provider_settings.go` | 失败降级和健康探测链路不完整 | B/C |
| API/Gateway | 稳定 API、事件流、多入口基础 | 已有 `/chat/run`、`/confirm`、`/events/stream` | 2 | `gateway/internal/api/router.go`、`chat.go` | 网关协议版本化与多入口抽象不足 | E |
| CLI/TUI | 高可用交互、补全、历史、中断体验 | 当前偏 API + 前端形态 | 1 | `docs/10-v2/*`、当前仓库结构 | 缺少产品级 CLI/TUI 主入口 | E |
| 前端工作台 | 实时事件、确认、日志、设置 | 已有工作台与分域模块 | 2 | `frontend/src/workspace/*`、`events/*`、`settings/*` | 与 Runtime Gate 指标联动不足 | E |
| 可观测与诊断 | 诊断命令、故障定位、追踪闭环 | 已有 `runtime_status` 与 diagnostics 接口 | 2 | `gateway/internal/api/router.go` | 缺少统一 doctor 命令与发布自检链 | F |
| Windows 一等支持 | 原生运行稳定、安装便捷 | 已具备 Win 开发环境基础 | 1 | 仓库在 Windows 路径运行 | ConPTY/安装分发/一键诊断未收口 | F |
| 发布与回滚 | 可发布、可回滚、可迁移 | 已有基础构建产物 | 1 | `gateway/cmd/server/server-latest.exe` | 缺少发布门禁、迁移脚本、回滚 SOP | F |
| 自动化测试与评测 | 分层测试+场景回归+故障注入 | 存在部分测试与大量证据文件 | 1 | `gateway/internal/memory/store_test.go`、`docs/archive/*` | 缺少标准化评测集与门禁流水线 | A/B/F |

## 3. A 阶段结论

1. 当前项目整体成熟度可判定为 `2-可用雏形成型`，并非从零开始。
2. 最高优先缺口是 `Runtime 恢复链 + 工具治理 + 发布级 Windows 支持`。
3. 要达到 Hermes 同级，关键不在“补更多功能”，而在“把已有能力产品化收口”。

## 4. A 阶段必须冻结的优先级

1. P0：B 阶段先固化 Runtime 状态机、checkpoint、恢复 API。
2. P0：C 阶段统一工具契约与权限确认链，不再分散定义。
3. P1：D 阶段补记忆检索评测与污染治理。
4. P1：E 阶段补 CLI/TUI 产品入口，和前端/网关共享会话协议。
5. P0：F 阶段把 Windows 安装、诊断、回滚做成正式发布能力。

## 5. Gate-A 判定

### 5.1 通过条件

1. 每个能力维度都已映射到仓库模块与后续阶段。
2. 每个 P0 缺口均有明确后续阶段承接。
3. 不存在“无人负责的核心能力”。

### 5.2 当前判定

1. `Gate-A/能力对标子项：通过（待与模块 owner 表联合确认）`。
