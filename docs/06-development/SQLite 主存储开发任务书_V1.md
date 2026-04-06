# 本地智能体 - SQLite 主存储开发任务书 V1

更新时间：2026-04-02

状态：`当前有效`

执行标记：

1. `当前 SQLite 收口阶段开发主入口`
2. `后续代码 AI 开始实现 SQLite 收口时直接执行`
3. `优先级高于历史 JSONL 主存储基线`

关联文档：

1. [SQLite 主存储收口方案_V1](D:/newwork/本地智能体/docs/06-development/SQLite%20主存储收口方案_V1.md)
2. [产品落地差距清单与收口顺序_V1](D:/newwork/本地智能体/docs/06-development/产品落地差距清单与收口顺序_V1.md)
3. [本地记忆与知识沉淀开发任务书_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀开发任务书_V1.md)
4. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)

---

## 1. 任务定位

本任务书只解决一件事：

> 把长期记忆与知识索引主存储从当前 `JSONL` 基线收口到 `SQLite`。

当前任务不是做：

1. embedding 检索
2. 重型数据库集群
3. 向量数据库
4. 多租户数据平台
5. 复杂 ORM 抽象层

当前任务要做的是：

1. `SQLite` 建库
2. 主表落地
3. 主查询切换
4. JSONL 兼容迁移
5. 存储边界收口

---

## 2. 当前阶段目标

本阶段唯一目标：

> 让长期记忆和知识索引正式进入 `SQLite` 主存储，并让运行时主查询优先走 `SQLite`。

做到这一步后，系统应具备：

1. 正式数据库文件
2. 正式表结构
3. 正式查询入口
4. JSONL 兼容迁移能力
5. 不影响现有事件与日志链路

---

## 3. 本阶段硬约束

开始实现前必须满足：

1. `SQLite` 只接管长期记忆与知识索引
2. 日志、事件、artifact 继续保留 `JSONL`
3. 不把短期工作记忆强行迁入 `SQLite`
4. 不引入重型 ORM 或复杂依赖
5. 不破坏现有 Rust / Go / Frontend 合同

---

## 4. 本阶段必须完成的 5 个模块

### 4.1 `sqlite-bootstrap`

职责：

1. 建立数据库文件
2. 建立连接入口
3. 初始化表和索引

### 4.2 `sqlite-memory-store`

职责：

1. 持久化长期记忆
2. 查询长期记忆
3. 去重与归档基础能力

### 4.3 `sqlite-knowledge-store`

职责：

1. 持久化知识索引
2. 查询知识索引
3. 支持来源类型和标签字段

### 4.4 `storage-migration`

职责：

1. 导入旧 JSONL 数据
2. 做基础兼容转换
3. 保留回退与灰度切换能力

### 4.5 `query-cutover`

职责：

1. 让 recall 主查询优先走 `SQLite`
2. JSONL 只作为兼容回退

---

## 5. 推荐实现顺序

### Phase A：数据库基础层

先做：

1. `sqlite-bootstrap`
2. 数据库路径
3. 建表与建索引

通过标准：

1. 存在正式 `main.db`
2. 表结构可初始化

### Phase B：主写入链路

再做：

1. 长期记忆写入
2. 知识索引写入
3. 基础去重

通过标准：

1. 新数据可稳定写入 `SQLite`
2. JSONL 可继续兼容保留

### Phase C：主查询切换

最后做：

1. recall 优先查 `SQLite`
2. JSONL 兼容回退
3. 导入旧数据

通过标准：

1. 主链路优先命中 `SQLite`
2. 旧数据不阻断查询

---

## 6. 文件级落点

### 6.1 Rust 落点

优先改：

1. `crates/runtime-core/src/storage.rs`
2. `crates/runtime-core/src/paths.rs`
3. `crates/runtime-core/src/memory.rs`
4. `crates/runtime-core/src/knowledge.rs`
5. `crates/runtime-core/src/memory_router.rs`
6. `crates/runtime-core/src/knowledge_store.rs`

允许新增：

1. `crates/runtime-core/src/sqlite_store.rs`
2. `crates/runtime-core/src/sqlite_memory_store.rs`
3. `crates/runtime-core/src/sqlite_knowledge_store.rs`
4. `crates/runtime-core/src/storage_migration.rs`

### 6.2 Go 落点

本阶段一般不要求 Go 直接操作数据库主表。
只要求：

1. 不破坏合同
2. 不破坏运行事件回流

---

## 7. 文件级任务清单

### 7.1 `crates/runtime-core/src/paths.rs`

任务：

1. 新增 `sqlite_db_path(request)`
2. 新增数据库目录路径函数
3. 保留旧 JSONL 路径函数

### 7.2 `crates/runtime-core/src/storage.rs`

任务：

1. 保留 JSONL 读写能力
2. 新增 SQLite 初始化入口
3. 提供基础连接与执行入口

### 7.3 `crates/runtime-core/src/sqlite_store.rs`

任务：

1. 初始化数据库
2. 创建表
3. 创建索引
4. 提供通用执行工具

### 7.4 `crates/runtime-core/src/sqlite_memory_store.rs`

任务：

1. 写入长期记忆
2. 查询长期记忆
3. 按工作区、类型、优先级过滤
4. 提供基础去重判断

### 7.5 `crates/runtime-core/src/sqlite_knowledge_store.rs`

任务：

1. 写入知识索引
2. 查询知识索引
3. 支持 `source_type`
4. 支持 tags 或 tags JSON

### 7.6 `crates/runtime-core/src/storage_migration.rs`

任务：

1. 读取旧 JSONL
2. 转换成 SQLite 结构
3. 导入并记录导入结果

### 7.7 `crates/runtime-core/src/memory.rs`

任务：

1. 长期记忆主写入切到 `SQLite`
2. 长期记忆主查询切到 `SQLite`
3. JSONL 只做兼容回退

### 7.8 `crates/runtime-core/src/knowledge.rs`

任务：

1. 知识索引主查询切到 `SQLite`
2. 保留文件命中优先策略
3. JSONL 只做兼容层

### 7.9 `crates/runtime-core/src/memory_router.rs`

任务：

1. recall 主流程优先走 `SQLite`
2. 保持摘要返回逻辑不变

---

## 8. 表结构要求

### 8.1 `long_term_memory`

至少包含：

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

### 8.2 `knowledge_base`

至少包含：

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

---

## 9. 查询要求

主查询必须满足：

1. 先查 `SQLite`
2. 先返回摘要
3. 需要时再展开正文
4. 支持工作区过滤
5. 支持归档过滤

---

## 10. 迁移要求

第一版迁移必须做到：

1. 能导入旧 JSONL 长期记忆
2. 能导入旧 JSONL 知识索引
3. 能识别重复数据
4. 能保留 JSONL 文件不删除

不要求：

1. 一次性彻底清理所有旧脏数据
2. 全量自动版本治理

---

## 11. 验收标准

### 11.1 必须通过

1. 已存在正式 `main.db`
2. 长期记忆主写入走 `SQLite`
3. 知识索引主写入走 `SQLite`
4. recall 主查询优先走 `SQLite`
5. JSONL 继续承担日志与兼容层职责

### 11.2 不通过情形

1. 只是建了库，但主链路没切
2. 查询仍主要依赖 JSONL
3. 日志和事件也被错误迁入主表
4. 没有迁移兼容策略

---

## 12. 给实现型 AI 的明确要求

1. 不要做重型抽象层
2. 不要引入与当前项目不匹配的数据库栈
3. 不要把短期工作记忆强行迁表
4. 先收口主写入与主查询，再谈高级检索
5. 完成后必须补对应验收文档

一句话要求：

> 当前任务不是做复杂数据库平台，而是把长期记忆和知识索引正式收口到 `SQLite`。
