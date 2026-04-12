# 技术方案

## 影响范围

- `crates/runtime-core/`：记忆模型 schema、检索路由、写回治理入口。
- `gateway/internal/`：记忆查询与写回透传接口。
- `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/`：阶段 D 执行工作区。

## D-01 冻结目标

- 冻结三层记忆 schema v1：`短期(会话)`、`长期(记忆)`、`知识(知识库)`。
- 冻结“读写与检索口径”最小闭环：先关键词主路径，语义检索只保留预留位。
- 冻结迁移策略草案：兼容历史 JSONL + SQLite，支持分步上线与快速回退。

## 三层 schema v1（冻结稿）

### L1：短期记忆（Session / Working Memory）

数据落点：`data/session/*.json`（由 `SessionMemory` 落盘）。  
主结构来源：`crates/runtime-core/src/session.rs`。

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `session_id` | string | 是 | 会话主键。 |
| `compressed_summary` | string | 否 | 最近轮次压缩摘要。 |
| `short_term.current_goal` | string | 否 | 当前目标。 |
| `short_term.current_plan` | string | 否 | 当前计划。 |
| `short_term.open_issue` | string | 否 | 当前阻塞问题。 |
| `short_term.recent_observation` | string | 否 | 最近观察。 |
| `short_term.recent_tool_result` | string | 否 | 最近工具结果。 |
| `short_term.pending_confirmation` | string | 否 | 待确认动作摘要。 |
| `short_term.current_phase` | string | 否 | 当前执行阶段。 |
| `short_term.last_run_status` | string | 否 | 最近一次运行状态。 |
| `short_term.handoff_artifact_path` | string | 否 | 长任务交接包路径。 |
| `recent_turns[]` | array | 否 | 最近 6 轮对话快照。 |
| `recent_turns[].user_input` | string | 是 | 用户输入。 |
| `recent_turns[].final_answer` | string | 是 | 最终回答。 |
| `recent_turns[].summary` | string | 是 | 轮次摘要。 |
| `recent_turns[].timestamp` | string | 是 | 时间戳。 |

冻结规则：

1. `recent_turns` 最多保留 6 条，超出即淘汰最旧记录。
2. `current_phase/last_run_status` 在恢复链路中允许保留原值，不被规划阶段覆盖。
3. L1 仅用于会话连续性，不参与跨工作区共享。

### L2：长期记忆（Long-term Memory）

数据落点：SQLite `long_term_memory`（主），JSONL `memory/*.jsonl`（兼容回放）。  
主结构来源：`crates/runtime-core/src/memory.rs` + `crates/runtime-core/src/sqlite_store.rs` + `gateway/internal/memory/store.go`。

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `id` | string | 是 | 记忆主键。 |
| `memory_type(kind)` | string | 是 | 规范类型。 |
| `title` | string | 是 | 标题。 |
| `summary` | string | 是 | 可检索摘要。 |
| `content` | string | 是 | 正文。 |
| `scope` | string | 是 | 作用域名称。 |
| `workspace_id` | string | 是 | 工作区主键。 |
| `session_id` | string | 是 | 来源会话。 |
| `source_run_id` | string | 是 | 来源运行。 |
| `source` | string | 是 | 来源标识（如 `run:*` 或文档路径）。 |
| `source_type` | string | 是 | 来源类型（`runtime/seed/...`）。 |
| `source_title` | string | 否 | 来源标题。 |
| `source_event_type` | string | 否 | 来源事件类型。 |
| `source_artifact_path` | string | 否 | 来源产物路径。 |
| `governance_version` | string | 否 | 治理版本。 |
| `governance_reason` | string | 否 | 治理原因。 |
| `governance_source` | string | 否 | 治理来源。 |
| `governance_at` | string | 否 | 治理时间。 |
| `archive_reason` | string | 否 | 归档原因。 |
| `verified` | bool | 是 | 是否验证通过。 |
| `priority` | int | 是 | 检索优先级。 |
| `archived` | bool | 是 | 是否归档。 |
| `archived_at` | string | 否 | 归档时间。 |
| `created_at` | string | 是 | 创建时间。 |
| `updated_at` | string | 是 | 更新时间。 |
| `timestamp` | string | 是 | 兼容时间戳。 |

`memory_type` 规范值（v1）：

- `preference`
- `project_rule`
- `workspace_summary`
- `workflow_pattern`
- `project_knowledge`
- `lesson_learned`
- `daily_note`
- `task_outcome`

冻结规则：

1. L2 查询路径固定为“关键词打分 + 优先级 + 更新时间”排序。
2. `workspace_id` 为硬隔离键，禁止跨工作区混检。
3. 治理字段缺失时必须走归一化补齐，保持可审计。
4. 历史低价值运行时噪音可被过滤或归档，不进入主召回集。

### L3：知识层（Knowledge Base）

数据落点：SQLite `knowledge_base`（主），JSONL `knowledge/*.jsonl`（兼容回放）。  
主结构来源：`crates/runtime-core/src/knowledge_store.rs` + `crates/runtime-core/src/sqlite_store.rs`。

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `id` | string | 是 | 知识主键。 |
| `knowledge_type` | string | 是 | 知识类型。 |
| `title` | string | 是 | 标题。 |
| `summary` | string | 是 | 摘要。 |
| `content` | string | 是 | 正文。 |
| `tags` | string[] | 是 | 标签。 |
| `source` | string | 是 | 来源。 |
| `source_type` | string | 是 | 来源类型（`runtime/siyuan/...`）。 |
| `verified` | bool | 是 | 是否验证通过。 |
| `workspace_id` | string | 是 | 工作区主键。 |
| `priority` | int | 是 | 优先级。 |
| `archived` | bool | 是 | 是否归档。 |
| `created_at` | string | 是 | 创建时间。 |
| `updated_at` | string | 是 | 更新时间。 |

`knowledge_type` 规范值（v1）：

- `knowledge_recall`
- `project_status`
- `user_curated`

冻结规则：

1. 未通过验证（`verified=false`）的记录不入知识层。
2. 摘要过短与“项目说明回显”类低价值条目直接拦截。
3. 可选思源同步只作为旁路输出，不影响主存储口径。

## 读写与检索路由 v1

1. 写入顺序：先 L1，再按策略写 L2/L3。
2. L2 召回：`search_memory_entries` 关键词打分主路径。
3. L3 召回：按工作区列出并参与知识型问题补充。
4. 汇总策略：同一工作区内先高优先级，再新鲜度，再文本匹配分。
5. 语义召回：当前只预留接口，不进入默认主路径。

## 迁移策略草案（D-02 输入）

### 目标

- 不破坏既有数据读取。
- 先兼容再收敛，避免一次性重写。
- 可灰度开启、可快速回退。

### 分步迁移

1. `M0-盘点`：以当前 Rust/Go 结构为基线，冻结字段与类型。
2. `M1-扩展`：保留 `alter table add column` 扩展式迁移，避免破坏旧表。
3. `M2-回填`：对 L2 治理字段执行 backfill，补齐 `governance_*` 与 `archive_reason`。
4. `M3-双读`：读取优先 SQLite，缺失时回落 JSONL，保持历史可复盘。
5. `M4-双写收口`：维持 SQLite + JSONL 双写，待 D 阶段验证后再评估单写收敛。

### 兼容与回退

1. 兼容策略：新增字段一律有默认值，旧记录通过归一化函数补齐。
2. 回退策略：若迁移异常，关闭新增写入策略，保留旧读取与关键词主路径。
3. 数据安全：任何清理动作仅处理重复/噪音项，不删除唯一有效记录。

## 风险与控制

1. 风险：字段语义漂移导致跨端不一致。  
   控制：以本冻结稿作为 D 阶段唯一 schema 口径，变更必须先改文档再改实现。
2. 风险：迁移清理误删高价值记录。  
   控制：只按“重复键 + 明确噪音规则”清理，并保留 tombstone 轨迹。
3. 风险：语义检索提前接入导致不稳定。  
   控制：D 阶段默认禁用语义主路径，先完成关键词链路验收。

## D-05 技能加载与版本治理（最小实现）

### 目标

1. 让运行态可识别 workspace 技能目录，不再只保留“skill 接口预留”。
2. 对技能版本增加 pin 治理，避免同名技能版本漂移。
3. 对技能入口路径增加隔离校验，阻断跨工作区越界路径。

### 实现口径

1. 新增 `crates/runtime-core/src/skill_catalog.rs`：
   - 加载 manifest：默认 `data/skills/<workspace_id>.json`，也支持 `context_hints.skill_manifest_path` 覆盖。
   - 版本治理：支持 `context_hints.skill_version_pins`（格式 `skill_id@x.y.z,...`）。
   - 隔离校验：入口路径统一走 `resolve_workspace_path`，仅允许当前 workspace 内路径。
2. 运行态接入：
   - `bootstrap_run` 启动时加载技能目录，写入 `RuntimeEnvelope.skill_catalog`。
   - 执行阶段可读取 `skill_catalog_brief`，作为后续技能执行链路扩展落点。

### 回退策略

1. 若 manifest 解析失败或字段异常，降级为“无技能加载”，主链路继续执行。
2. 版本不匹配和路径越界仅跳过对应技能，不影响普通工具执行。

## D-06 跨会话连续性首轮样本（1 天）

### 目标

1. 复现最小跨会话链路：会话 A 记忆写入，会话 B 记忆召回。
2. 产出可重复执行、可独立复核的脚本化证据。

### 验收口径

1. 脚本：`scripts/run-stage-d-day1-acceptance.ps1`。
2. 证据：`tmp/stage-d-day1/latest.json`。
3. 关键判据：
   - `cross_session=true`
   - `write_has_memory_written=true`
   - `recall_has_memory_recalled=true`
   - `recall_hit=true`

## D-G1 批量阈值映射（7 天）

### 脚本与输入

1. 脚本：`scripts/run-stage-d-gate-batch.ps1`
2. 默认输入：
   - `Days=7`
   - `MinContinuityDays=7`
   - `MinPassRate=0.90`
   - `MinRecallHitRate=0.90`
   - `RequireGateD=true`

### 输出与判定

1. 输出：`tmp/stage-d-batch/latest.json`
2. 关键字段：
   - `gate_d.ready`
   - `gate_d.days / gate_d.pass_days`
   - `gate_d.pass_rate`
   - `gate_d.recall_hit_rate`
3. 判定口径：
   - `pass_days >= MinContinuityDays`
   - `pass_rate >= MinPassRate`
   - `recall_hit_rate >= MinRecallHitRate`
