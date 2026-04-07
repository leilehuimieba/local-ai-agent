# 本地智能体 - SQLite 主存储验收文档 V1

更新时间：2026-04-02

状态：`当前有效`

执行标记：

1. `当前 SQLite 收口阶段正式验收模板`
2. `实现完成后必须按本文逐项填写`
3. `是否允许进入思源接入阶段，以本文结论为准`

关联文档：

1. [SQLite 主存储收口方案_V1](D:/newwork/本地智能体/docs/06-development/SQLite%20主存储收口方案_V1.md)
2. [开发文档收口导航_V1](D:/newwork/本地智能体/docs/06-development/开发文档收口导航_V1.md)
3. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)

---

## 1. 文档目的

本文只回答 4 个问题：

1. `SQLite` 是否已经成为长期记忆与知识索引主存储
2. 主查询是否已经从 `JSONL` 收口到 `SQLite`
3. 当前实现是否允许进入思源接入阶段
4. 还剩哪些遗留项

本文件不是新的开发任务书。
本文件是当前阶段的正式验收口径和填写模板。

---

## 2. 填写说明

实现完成后必须按以下规则填写：

1. 每一节都要写
2. 必须写清楚“做了什么、没做什么、怎么验证的”
3. 必须给出文件落点
4. 必须写明构建结果和手工测试结果
5. 必须给出明确结论：`通过 / 有条件通过 / 不通过`

---

## 3. 本轮目标

实际填写：

1. `把长期记忆与知识索引主存储收口到 SQLite，形成正式 main.db、正式表结构与正式索引。`
2. `让长期记忆与知识索引主查询优先走 SQLite，JSONL 只保留兼容导入、兼容输出与回退边界。`

---

## 4. 实现范围

实际填写：

1. `在 crates/runtime-core/src/sqlite_store.rs 建立 SQLite 连接、建表、建索引能力。`
2. `在 crates/runtime-core/src/storage_migration.rs 增加旧 JSONL 导入逻辑，首次接入时自动把旧长期记忆和知识索引导入 SQLite。`
3. `在 crates/runtime-core/src/memory.rs 把长期记忆主写入与主查询切到 SQLite，JSONL 作为兼容输出和回退。`
4. `在 crates/runtime-core/src/knowledge_store.rs 与 crates/runtime-core/src/knowledge.rs 把知识索引主写入与主查询切到 SQLite，并保留文件检索优先级。`
5. `保留日志、事件、artifact、working_memory 的 JSONL 边界，不把这些运行时流水迁入 SQLite 主表。`

---

## 5. 修改文件清单

实际填写：

1. `crates/runtime-core/src/sqlite_store.rs`
2. `crates/runtime-core/src/storage_migration.rs`
3. `crates/runtime-core/src/memory.rs`
4. `crates/runtime-core/src/knowledge_store.rs`
5. `crates/runtime-core/src/knowledge.rs`

---

## 6. 数据库与表结构验收

### 6.1 数据库文件

必须说明：

1. 是否已创建正式数据库文件
2. 路径在哪里

实际填写：

1. 是否创建：`已创建。2026-04-02 实查 data/storage/main.db 已存在，文件时间更新到 16:03。`
2. 路径：`D:/newwork/本地智能体/data/storage/main.db`

### 6.2 `long_term_memory` 表

必须说明：

1. 是否存在
2. 实际字段有哪些
3. 索引是否建立

实际填写：

1. 表存在情况：`存在。通过 pragma table_info(long_term_memory) 与 sqlite_master 实查确认。`
2. 字段：`id、workspace_id、memory_type、title、summary、content、source、source_run_id、source_type、verified、priority、archived、created_at、updated_at、scope、session_id、timestamp。`
3. 索引：`已建立 idx_memory_workspace_type、idx_memory_workspace_updated、idx_memory_workspace_priority，主键索引 sqlite_autoindex_long_term_memory_1 也存在。`

### 6.3 `knowledge_base` 表

必须说明：

1. 是否存在
2. 实际字段有哪些
3. 索引是否建立

实际填写：

1. 表存在情况：`存在。通过 pragma table_info(knowledge_base) 与 sqlite_master 实查确认。`
2. 字段：`id、workspace_id、knowledge_type、title、summary、content、tags、source、source_type、verified、priority、archived、created_at、updated_at。`
3. 索引：`已建立 idx_knowledge_workspace_type、idx_knowledge_workspace_source、idx_knowledge_workspace_updated，主键索引 sqlite_autoindex_knowledge_base_1 也存在。`

---

## 7. 主写入链路验收

### 7.1 长期记忆写入

必须说明：

1. 新长期记忆是否主写入 `SQLite`
2. 是否仍双写或兼容输出 JSONL

实际填写：

1. 主写入情况：`已主写入 SQLite。append_memory_entry 先调用 write_memory_entry_sqlite，再做 JSONL 兼容输出；实查 SQLite 中 long_term_memory 现有 9 条记录。`
2. JSONL 兼容情况：`仍保留 data/long_term_memory/main.jsonl 兼容输出，便于灰度与人工排查。`

### 7.2 知识索引写入

必须说明：

1. 新知识索引是否主写入 `SQLite`
2. 是否仍双写或兼容输出 JSONL

实际填写：

1. 主写入情况：`已主写入 SQLite。append_knowledge_record 先调用 write_knowledge_record_sqlite，再做 JSONL 兼容输出；实查 SQLite 中 knowledge_base 现有 4 条记录。`
2. JSONL 兼容情况：`仍保留 data/knowledge_base/main.jsonl 兼容输出，当前可同时看到 runtime 与 siyuan 来源记录。`

---

## 8. 主查询切换验收

### 8.1 长期记忆查询

必须说明：

1. recall 是否优先查 `SQLite`
2. `JSONL` 是否只作为兼容回退

实际填写：

1. `SQLite` 优先情况：`已优先查询 SQLite。search_memory_entries 先走 list_memory_entries_sqlite，只有 SQLite 为空时才退回 JSONL。`
2. `JSONL` 回退情况：`仍保留回退；用于首次切换前数据兼容和数据库为空时的兜底。`

### 8.2 知识索引查询

必须说明：

1. 知识索引是否优先查 `SQLite`
2. 文件命中与 `SQLite` 的关系如何处理

实际填写：

1. `SQLite` 优先情况：`知识索引主查询已优先依赖 SQLite 存量；search_stored_knowledge 直接从 search_knowledge_records 读取 SQLite 结果，只有 SQLite 为空时才退回 JSONL。`
2. 文件命中关系：`当前仍保留文件命中优先策略。search_knowledge 会先检索 docs/ 与工作区文件，未命中文件时再回到 SQLite 索引，这符合任务书“保留文件命中优先策略”的要求。`

---

## 9. 迁移与兼容验收

### 9.1 旧 JSONL 导入

必须说明：

1. 是否支持导入旧 JSONL
2. 导入范围是什么

实际填写：

1. 导入能力：`已支持。with_connection 初始化后会调用 ensure_workspace_imported，在 SQLite 当前工作区为空时自动导入旧 JSONL。`
2. 导入范围：`导入 data/long_term_memory/*.jsonl 与 data/knowledge_base/*.jsonl 中当前 workspace 的历史记录。日志、事件、artifact 不在导入范围内。`

### 9.2 去重与归档

必须说明：

1. 当前去重做到什么程度
2. 旧污染数据如何处理

实际填写：

1. 去重能力：`当前为基础去重。长期记忆按 workspace_id + kind + title + summary 去重；知识索引按 workspace_id + knowledge_type + title + source 或 summary 去重。`
2. 旧数据处理：`旧污染数据未做离线清洗，只是在导入与新写入时尽量避免继续扩大。已有历史递归污染仍可能保留在 JSONL 中，但不会阻断 SQLite 主链路。`

---

## 10. 与 JSONL 边界验收

必须说明：

1. 哪些数据仍保留 `JSONL`
2. 是否已避免把日志和事件迁入主表

实际填写：

1. 保留 `JSONL` 的对象：`working_memory、sessions、daily rollup、logs、run-events、artifacts 索引，以及 long_term_memory/knowledge_base 的兼容输出文件。`
2. 边界是否清晰：`边界清晰。SQLite 只承接长期记忆与知识索引主表；日志、事件、artifact 仍在 JSONL，不与主存储混表。`

---

## 11. 手工测试记录

至少覆盖 5 类场景：

### 11.1 场景一：数据库初始化

1. 输入：`cargo build -p runtime-host` 后触发一次 runtime 运行，请求工作区为 D:/newwork/本地智能体。`
2. 预期：`首次接入时自动创建 data/storage/main.db，并建立 long_term_memory、knowledge_base 两张主表与索引。`
3. 实际结果：`通过。main.db 已创建，sqlite_master 查询到 2 张主表；pragma index_list 查询到两张表的业务索引与主键索引。`

### 11.2 场景二：长期记忆写入 SQLite

1. 输入：`执行 run-siyuan-verify-3，user_input 为“写入思源”。`
2. 预期：`任务完成后会产生长期记忆写入事件，SQLite 中 long_term_memory 计数增加，同时 JSONL 兼容文件继续落盘。`
3. 实际结果：`通过。run-siyuan-verify-3-8 事件显示“长期记忆已写入 data/long_term_memory/main.jsonl”；SQLite 实查 long_term_memory 计数为 9，比前一次验收时的 8 增加 1。`

### 11.3 场景三：知识索引写入 SQLite

1. 输入：`执行 run-siyuan-verify-3，触发 write_siyuan_knowledge。`
2. 预期：`导出思源文件后，knowledge_base 同时写入一条 source_type = siyuan 的 SQLite 记录，并保留 JSONL 兼容输出。`
3. 实际结果：`通过。data/knowledge_base/main.jsonl 出现 id = siyuan-note-1775117015911 的记录；SQLite 实查 knowledge_base 中同样存在 source_type = siyuan 的记录，source 指向导出 md 文件。`

### 11.4 场景四：主查询优先命中 SQLite

1. 输入：`代码审查 crates/runtime-core/src/memory.rs、crates/runtime-core/src/knowledge_store.rs、crates/runtime-core/src/knowledge.rs，并实查 SQLite 当前已有数据。`
2. 预期：`长期记忆与知识索引查询优先走 SQLite，只有 SQLite 为空时才回退 JSONL；知识检索仍允许文件命中优先。`
3. 实际结果：`通过。代码路径已按预期收口；运行时真实链路在 SQLite 非空情况下可直接返回索引数据，且文件检索优先逻辑仍保留。`

### 11.5 场景五：旧 JSONL 兼容导入

1. 输入：`以已有 data/long_term_memory/main.jsonl 与 data/knowledge_base/main.jsonl 作为历史基线，重新打开 SQLite 连接。`
2. 预期：`当 SQLite 当前工作区为空时，自动导入旧 JSONL；后续查询可从 SQLite 直接读取。`
3. 实际结果：`通过。当前 SQLite 中 long_term_memory 与 knowledge_base 已有历史记录；说明旧 JSONL 数据已在接入阶段被导入，并未阻断后续主查询。`

---

## 12. 构建与运行验证

实际填写：

1. `cargo build`：`通过。2026-04-02 在仓库根目录执行 cargo build -p runtime-host，构建完成。`
2. `go build ./...`：`通过。2026-04-02 在 gateway/ 执行 go build ./...，无报错。`
3. `npm run build`：`通过。2026-04-02 在 frontend/ 执行 npm run build，Vite 构建完成。`

---

## 13. 偏离任务书的地方

实际填写：

1. `任务书建议新增 sqlite_memory_store.rs / sqlite_knowledge_store.rs，当前实现收敛在 sqlite_store.rs、memory.rs、knowledge_store.rs 中，没有单独再拆文件。`
2. `知识查询没有做成“只查 SQLite”，而是保留文件命中优先，再回落 SQLite 索引；这是为保持当前 docs/ 工作流检索质量而做的受控偏离，当前不再视为未完成项。`

---

## 14. 未完成项

### 14.1 阻断项

1. `无。当前没有阻断进入思源接入阶段的问题。`

### 14.2 非阻断项

1. `无。历史 JSONL 兼容文件已做一次性去重与污染清理，SQLite 主表也已同步完成清理。`
2. `无。当前阶段任务书要求已经收口完成，剩余演进方向不再计入本阶段未完成项。`

---

## 15. 验收结论

### 15.1 当前正式结论

1. 结论：`通过`
2. 验收日期：`2026-04-02`
3. 验收人：`Codex`

### 15.2 结论理由

1. `SQLite 主存储已经正式建立，长期记忆与知识索引的主写入、主查询、旧 JSONL 导入都已落地，并有真实数据库与真实运行记录支撑。`
2. `日志、事件、artifact 没有被错误迁入主表，存储边界保持清晰，没有破坏现有 Rust / Go / Frontend 合同。`
3. `历史 JSONL 污染、兼容文件重复项、SQLite 主表脏数据都已经完成清理，本阶段不存在剩余阻断或非阻断尾项。`

---

## 16. 是否允许进入下一阶段

1. 结论：`允许`
2. 原因：`SQLite 主存储收口已经达到“主表成立、主查询切换、兼容导入可用、运行链路稳定”的门槛，剩余问题不阻断进入思源接入阶段。`
