# 本地智能体 - SQLite 主存储收口方案 V1

更新时间：2026-04-02

状态：`当前有效`

执行标记：

1. `当前存储收口阶段主方案`
2. `后续代码 AI 开始做存储收口前必须先读`
3. `如与历史 JSONL 基线冲突，以本文为准`

关联文档：

1. [产品落地差距清单与收口顺序_V1](D:/newwork/本地智能体/docs/archive/completed-v1-20260410/产品落地差距清单与收口顺序_V1.md)
2. [本地记忆与知识沉淀需求文档_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀需求文档_V1.md)
3. [本地记忆与知识沉淀开发任务书_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀开发任务书_V1.md)
4. [本地记忆与知识沉淀验收文档_V1](D:/newwork/本地智能体/docs/archive/completed-v1-20260410/本地记忆与知识沉淀验收文档_V1.md)
5. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)

---

## 1. 文档目的

本文只解决一件事：

> 把当前记忆与知识存储从“JSON / JSONL 基线可用”收口到“SQLite 主存储正式方案”。

后续不再讨论“要不要继续长期用 JSONL 做主存储”。
本方案明确：

1. 哪些数据进入 `SQLite`
2. 哪些数据继续保留 `JSONL`
3. 如何迁移
4. 如何分阶段切换

---

## 2. 当前现状

当前实际存储情况：

1. 短期工作记忆：会话 JSON 与工作记忆文件
2. 长期记忆：按工作区分区 `JSONL`
3. 知识库：按工作区分区 `JSONL`
4. artifact 索引：`JSONL`
5. 日志与事件：`JSONL`

当前代码基线：

1. [storage.rs](D:/newwork/本地智能体/crates/runtime-core/src/storage.rs) 负责 `read_jsonl / append_jsonl`
2. [paths.rs](D:/newwork/本地智能体/crates/runtime-core/src/paths.rs) 负责 `long_term_memory / knowledge_base / artifacts` 路径

当前问题：

1. `JSONL` 适合流水追加
2. 不适合长期承担正式查询主存储
3. 去重、更新、排序、过滤会越来越散
4. 后续思源接入也需要一个稳定的本地索引层

---

## 3. 当前正式结论

从现在开始，存储口径固定为：

1. `SQLite`：长期记忆与知识索引主存储
2. `JSONL`：日志、事件、artifact 索引、审计流水
3. `JSON`：会话文件与短期工作记忆缓存

一句话：

> `SQLite` 管结构化真相，`JSONL` 管过程流水，`JSON` 管轻量会话态。

---

## 4. 为什么选 SQLite

### 4.1 选型原因

选择 `SQLite` 的原因固定为：

1. 本地嵌入式数据库，不需要额外服务
2. 资源占用低，适合本地智能体
3. 读写性能对当前单机产品足够
4. 适合结构化查询、过滤、索引、去重、更新
5. Rust 和 Go 都容易接入
6. 后续可加 `FTS` 做轻量全文检索

### 4.2 为什么不继续长期只用 JSONL

原因：

1. `JSONL` 更适合 append-only 流水
2. 不适合高质量条件查询
3. 不适合去重与更新
4. 不适合做正式知识索引层
5. 数据量增长后治理成本会迅速上升

### 4.3 为什么现在不选更重的数据库

当前不选 `PostgreSQL / MySQL / MongoDB / Redis / 向量数据库` 的原因：

1. 本地单机第一版没有必要引入额外服务
2. 部署、维护、迁移复杂度更高
3. 内存与运维成本更高
4. 当前需求优先是轻量、稳定、可本地运行

---

## 5. 收口范围

### 5.1 进入 SQLite 的数据

从本阶段开始，以下数据进入 `SQLite`：

1. 长期记忆
2. 知识索引
3. 种子记忆
4. 记忆去重与召回所需元数据
5. 思源外挂知识库的摘要索引

### 5.2 继续保留 JSONL 的数据

以下数据继续保留 `JSONL`：

1. 运行事件
2. 调试日志
3. artifact 索引
4. 审计流水
5. 历史兼容导出文件

### 5.3 继续保留 JSON 的数据

以下数据继续保留 `JSON`：

1. 会话文件
2. 短期工作记忆缓存
3. 临时运行态快照

---

## 6. 表设计原则

本阶段表设计遵守以下原则：

1. 先少表、强字段、清边界
2. 先满足当前查询主链路
3. 不先做复杂关系网
4. 不把日志型数据塞进主表
5. 所有主表都带 `workspace_id`

---

## 7. 第一版表设计

### 7.1 `long_term_memory`

用途：

1. 存长期记忆主记录

建议字段：

1. `id`
2. `workspace_id`
3. `memory_type`
4. `title`
5. `summary`
6. `content`
7. `source`
8. `source_run_id`
9. `source_type`
10. `verified`
11. `priority`
12. `archived`
13. `created_at`
14. `updated_at`

建议索引：

1. `workspace_id + memory_type`
2. `workspace_id + archived`
3. `workspace_id + updated_at`
4. `workspace_id + priority`

### 7.2 `knowledge_base`

用途：

1. 存知识索引主记录

建议字段：

1. `id`
2. `workspace_id`
3. `knowledge_type`
4. `title`
5. `summary`
6. `content`
7. `tags`
8. `source`
9. `source_type`
10. `verified`
11. `priority`
12. `archived`
13. `created_at`
14. `updated_at`

建议索引：

1. `workspace_id + knowledge_type`
2. `workspace_id + source_type`
3. `workspace_id + archived`
4. `workspace_id + updated_at`

### 7.3 `knowledge_tags`

用途：

1. 为知识条目做标签筛选扩展位

建议字段：

1. `knowledge_id`
2. `tag`

第一版可选：

1. 如果实现成本高，可先把 tags 作为 JSON 字段保留
2. 后续再拆表

### 7.4 `seed_records`

用途：

1. 标记种子记忆与种子知识

建议字段：

1. `id`
2. `record_id`
3. `record_kind`
4. `seed_source`
5. `created_at`

第一版可选：

1. 也可直接在主表加 `is_seed`
2. 不强制独立拆表

---

## 8. 目录与文件落点

### 8.1 新增数据库文件

建议正式路径：

1. `data/storage/main.db`

如需按工作区分库，可后续扩展：

1. `data/storage/<workspace_id>.db`

第一版建议：

1. 先单库
2. 所有表带 `workspace_id`

### 8.2 路径层规则

`paths.rs` 需要新增：

1. `sqlite_db_path(request)`
2. 如有必要，新增数据库目录路径函数

---

## 9. 模块落点

### 9.1 Rust 落点

优先改这些文件：

1. `crates/runtime-core/src/storage.rs`
2. `crates/runtime-core/src/paths.rs`
3. `crates/runtime-core/src/memory.rs`
4. `crates/runtime-core/src/knowledge.rs`
5. `crates/runtime-core/src/memory_router.rs`
6. `crates/runtime-core/src/knowledge_store.rs`

如现有职责不够清晰，允许新增：

1. `crates/runtime-core/src/sqlite_store.rs`
2. `crates/runtime-core/src/sqlite_memory_store.rs`
3. `crates/runtime-core/src/sqlite_knowledge_store.rs`
4. `crates/runtime-core/src/storage_migration.rs`

### 9.2 Go 落点

Go 侧一般不直接操作数据库主表。
当前阶段只需保证：

1. 合同不受影响
2. 运行时返回结果和事件不受影响

如后续需要管理接口，再补：

1. 查询接口
2. 数据治理接口

---

## 10. 迁移策略

### 10.1 总原则

迁移必须满足：

1. 不破坏现有主链路
2. 允许旧 JSONL 数据导入
3. 迁移失败不影响日志与事件流水
4. 可以灰度切换

### 10.2 第一阶段迁移

先做：

1. 建表
2. 新写入双写
3. 旧数据导入工具

此阶段口径：

1. 新记录写入 `SQLite`
2. 保留 JSONL 兼容导出或只读读取能力

### 10.3 第二阶段迁移

再做：

1. 召回查询优先走 `SQLite`
2. JSONL 回退只作为兼容层

### 10.4 第三阶段迁移

最后做：

1. 停止长期记忆和知识库主写入 JSONL
2. JSONL 仅保留导出、历史迁移和调试用途

---

## 11. 读写策略

### 11.1 写入策略

长期记忆与知识库主写入策略：

1. 先做结构化整理
2. 再做去重判断
3. 再写入 `SQLite`
4. 如需兼容，再导出一份 JSONL 快照

### 11.2 查询策略

主查询固定为：

1. 先查 `SQLite`
2. 命中后返回摘要
3. 必要时再读取详细内容
4. 兼容层需要时再查旧 JSONL

### 11.3 去重策略

第一版至少做到：

1. 同工作区
2. 同类型
3. 同标题或同来源
4. 近似摘要重复拦截

---

## 12. 与思源接入的边界

本方案先明确：

1. `SQLite` 是主索引层
2. 思源不是主数据库
3. 思源正文进入知识链路时，先抽摘要写入 `SQLite`
4. 思源检索结果先走 `SQLite` 摘要，再按需打开正文

一句话：

> 思源管正文与人工整理，SQLite 管查询主链路。

---

## 13. 验收标准

### 13.1 必须通过

1. 已建立正式 `SQLite` 数据库文件
2. 长期记忆已写入 `SQLite`
3. 知识索引已写入 `SQLite`
4. 召回主链路优先查询 `SQLite`
5. 旧 JSONL 不再承担正式主查询职责
6. 日志、事件、artifact 仍稳定保留 `JSONL`

### 13.2 不通过情形

1. 只是新增了 SQLite 文件，但主链路仍主要靠 JSONL
2. 没有迁移或兼容策略
3. 去重、过滤、更新逻辑继续散落在文件读写逻辑里
4. 把日志流水也迁进主表，导致边界重新混乱

---

## 14. 推荐实现顺序

### Phase A

1. 新增 `SQLite` 路径与基础连接层
2. 建立 `long_term_memory / knowledge_base` 表
3. 接入最小写入链路

### Phase B

1. 接入最小查询链路
2. 让 recall 主流程优先查 `SQLite`
3. 保留 JSONL 兼容回退

### Phase C

1. 导入旧 JSONL 数据
2. 完成去重、归档、优先级治理
3. 收口 JSONL 角色为日志与兼容层

---

## 15. 当前正式结论

当前正式结论为：

1. `SQLite` 必须成为长期记忆与知识索引主存储
2. `JSONL` 不再继续承担长期正式主存储角色
3. 存储收口是当前最优先产品落地事项

一句话结论：

> 先把 `SQLite` 主存储做稳，后面的思源接入、知识治理、产品验收才会真正稳定。
