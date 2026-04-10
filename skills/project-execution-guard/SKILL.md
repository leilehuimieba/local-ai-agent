---
name: project-execution-guard
description: 用于项目开发推进、阶段执行、继续上次任务、检查当前做到哪一步、提醒缺失文档或验收信息时使用。遇到“继续做”“看看进度”“下一步该干嘛”“我还缺什么信息”“按当前项目状态推进”“先别写代码先梳理状态”等请求时触发。优先读取项目执行入口、阶段计划和进行中的 change 工作区，再输出统一状态卡片、缺口提醒和最小下一步，不直接跳入大规模实现。
---

# Project Execution Guard

## 概览

先判断项目当前处于哪一阶段、哪一项 change 正在推进、还缺什么关键输入，再决定是否进入设计或实现。

不要把它当成写代码 skill。
它负责的是项目推进秩序、缺口提醒和状态同步。

## 本仓库默认入口

当前仓库按下面顺序读取：

1. `docs/README.md`
2. `docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md`
3. `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
4. `docs/11-hermes-rebuild/changes/INDEX.md`
5. `docs/11-hermes-rebuild/changes/`

如果执行入口与历史文档冲突，以 `docs/README.md` 指向的当前入口为准。

## 使用流程

1. 先确认当前执行主线、当前阶段和 Gate。
2. 再读取 `docs/11-hermes-rebuild/changes/INDEX.md`，判断当前活跃 change。
3. 如果存在 change，优先读该 change 的 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md`。
4. 如果缺少关键文件或字段，先输出缺口，不直接进入实现。
5. 只有在目标、方案、任务和验证口径足够清楚时，才继续做设计细化或代码实现。

## change 工作区约定

每个 change 目录默认包含下面五个文件：

1. `proposal.md`
2. `design.md`
3. `tasks.md`
4. `status.md`
5. `verify.md`
6. 在仓库级别，使用 `changes/INDEX.md` 标记当前活跃项

作用分别是：

1. `proposal.md`
   说明为什么做、做什么、不做什么、对应哪个阶段目标。
2. `design.md`
   说明怎么做、影响哪些模块、会改哪些 contract、风险与回退方式。
3. `tasks.md`
   维护任务清单和完成判据，建议控制在 5 到 15 项。
4. `status.md`
   维护已完成、进行中、阻塞点、下一步和最近更新时间。
5. `verify.md`
   记录测试方式、验证证据、与阶段 Gate 的映射关系。
6. `INDEX.md`
   记录当前活跃 change 和多变更时的优先级。

## 活跃 change 选择规则

按下面顺序决定当前应该读取哪个 change：

1. 用户明确点名某个 change 时，以用户指定为准。
2. 否则优先读取 `docs/11-hermes-rebuild/changes/INDEX.md` 中列出的“当前活跃 change”。
3. 如果 `INDEX.md` 中有多个活跃项，优先选择与当前阶段一致且最近更新时间最新的项。
4. 如果没有 `INDEX.md`，退化为检查最近更新且 `status.md` 显示为进行中的 change。
5. 如果索引、阶段和状态文件互相冲突，先输出冲突点，不直接推进实现。

## 缺口提醒规则

发现下面任一情况时，先提醒补充内容：

1. 缺 `proposal.md`
   提醒补目标、非目标、业务背景、验收标准。
2. 缺 `design.md`
   提醒补影响模块、状态流转、接口或合同变化、回退方案。
3. 缺 `tasks.md`
   提醒补任务拆解和每项任务的完成判据。
4. `tasks.md` 只有事项没有完成标准
   提醒补“完成后应该看到什么”。
5. 缺 `status.md`
   提醒补当前进度、阻塞点和下一步。
6. 缺 `verify.md`
   提醒补测试方式、证据位置和与 Gate 的对应关系。
7. 当前阶段已进入实现，但没有明确 Gate 证据
   提醒先定义“什么叫通过”。

## 输出格式

默认按下面顺序输出：

1. `当前状态`
2. `缺失项`
3. `下一步建议`

需要模板时，读取 `references/status-card-template.md`。
需要统一检查缺口时，读取 `references/gap-checklist.md`。

## 默认行为约束

1. 项目推进类请求默认先读状态，不直接开写。
2. 继续任务时默认先按 `INDEX.md` 选择活跃 change，而不是重新猜上下文。
3. 如果当前 change 已有明确任务清单，优先按清单推进，不擅自扩 scope。
4. 如果发现阶段目标、Gate 或 change 方案冲突，先指出冲突点并暂停推进。
