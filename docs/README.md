# 本地智能体 Docs 导航

更新时间：2026-04-08

状态：`docs 当前导航与状态索引`

使用规则：

1. 先看本文，再决定读哪份文档。
2. 不要默认把所有文档都当成“当前执行入口”。
3. 当前有效文档优先，历史文档按需参考。
4. 本文只负责总导航，不替代具体执行文档。
5. 除前端专项实现外，`V1` 当前唯一上层执行口径固定为 [产品级冻结与下一阶段规划入口文档_V1](D:/newwork/本地智能体/docs/06-development/产品级冻结与下一阶段规划入口文档_V1.md)。

当前阶段正式结论：

1. `V1` 已冻结通过。
2. 当前进入稳定治理与可审计维护阶段。
3. 运行时 Harness 四项收口已完成并留证。
4. 当前默认推进顺序为：整理 `V1` 唯一执行口径、做可发布候选、执行真实场景脚本、开始一轮内测。

---

## 0. 当前唯一执行口径

当前默认按下面这套口径执行，不再并列认多个“主入口”：

1. 顶层导航入口：本文。
2. `V1` 治理阶段唯一上层执行文档：[产品级冻结与下一阶段规划入口文档_V1](D:/newwork/本地智能体/docs/06-development/产品级冻结与下一阶段规划入口文档_V1.md)。
3. 当前运行时唯一执行口径：[运行时 Harness 收口方案_V1](D:/newwork/本地智能体/docs/06-development/运行时 Harness 收口方案_V1.md)。
4. 当前前端唯一执行口径：[V2前端当前执行入口_V1](D:/newwork/本地智能体/docs/10-v2/V2前端当前执行入口_V1.md)。
5. 当前测试与验收唯一入口：[关键入口文档](D:/newwork/本地智能体/docs/07-test/关键入口文档.md)。
6. 其他文档默认为按需参考、历史留证或讨论稿，不单独升级为“当前主口径”。

---

## 1. 当前最重要的文档

如果你现在要继续推进项目，先看上面的“当前唯一执行口径”，再按领域进入下面这些文档：

### 当前产品基线文档

1. [需求冻结稿_V1](D:/newwork/本地智能体/docs/00-charter/需求冻结稿_V1.md)
2. [产品边界与入口定义_V1](D:/newwork/本地智能体/docs/01-boundary/产品边界与入口定义_V1.md)
3. [MVP功能清单_V1](D:/newwork/本地智能体/docs/01-boundary/MVP功能清单_V1.md)
4. [需求口径升级变更记录_V1](D:/newwork/本地智能体/docs/00-charter/需求口径升级变更记录_V1.md)
5. [V2产品需求文档](D:/newwork/本地智能体/docs/10-v2/V2产品需求文档.md)
6. [V2实现附录](D:/newwork/本地智能体/docs/10-v2/V2实现附录.md)

### 当前前端执行域

1. [V2前端当前执行入口_V1](D:/newwork/本地智能体/docs/10-v2/V2前端当前执行入口_V1.md)
2. [V2首页状态化方案_V1](D:/newwork/本地智能体/docs/10-v2/V2首页状态化方案_V1.md)
3. [V2前端视觉与界面实现方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端视觉与界面实现方案_V1.md)
4. [V2前端体验与架构优化方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端体验与架构优化方案_V1.md)
5. [V2前端定制工作流_V1](D:/newwork/本地智能体/docs/10-v2/V2前端定制工作流_V1.md)
6. [首页状态化验收文档_V1](D:/newwork/本地智能体/docs/07-test/首页状态化验收文档_V1.md)

### 当前运行时执行域

1. [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)
2. [运行时 Harness 收口方案_V1](D:/newwork/本地智能体/docs/06-development/运行时 Harness 收口方案_V1.md)
3. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)
4. [当前项目本地化架构方案_V1](D:/newwork/本地智能体/docs/02-architecture/当前项目本地化架构方案_V1.md)
5. [本地记忆与知识沉淀需求文档_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀需求文档_V1.md)
6. [本地记忆与知识沉淀开发任务书_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀开发任务书_V1.md)
7. [ClaudeCode本地智能体框架架构设计文档](D:/newwork/本地智能体/docs/ClaudeCode本地智能体框架架构设计文档.md)

---

## 2. 目录说明

### `00-charter`

定位：

1. 项目最初需求冻结
2. 只负责边界和原则

状态：

1. `当前最高优先级约束`
2. `定义 V1 正式冻结边界`
3. `所有后续文档冲突时以本目录冻结稿为准`

### `01-boundary`

定位：

1. 产品边界
2. MVP 范围
3. 正式入口定义

状态：

1. `历史基线`
2. `仍有效`
3. `边界参考`

### `02-architecture`

定位：

1. 总体分层架构
2. Go / Rust / 前端职责分离
3. 本地适配原则

状态：

1. `架构主参考`
2. `当前仍有效`
3. `运行时实现前必须先读本地适配原则`

当前优先读：

1. [前端状态与模块分层设计文档_V1](D:/newwork/本地智能体/docs/02-architecture/前端状态与模块分层设计文档_V1.md)
2. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)
3. [当前项目本地化架构方案_V1](D:/newwork/本地智能体/docs/02-architecture/当前项目本地化架构方案_V1.md)

### `03-runtime`

定位：

1. Rust 运行时设计草案

状态：

1. `运行时主参考`
2. `当前仍有效`

### `04-api`

定位：

1. API 与事件合同基线

状态：

1. `合同主参考`
2. `当前仍有效`

### `05-migration`

定位：

1. 旧项目迁移边界

状态：

1. `历史参考`
2. `按需阅读`

### `06-development`

定位：

1. 开发任务书
2. 实施计划
3. 阶段执行文档

状态：

1. `部分当前有效`
2. `部分已归档`

当前优先读：

1. [开发文档收口导航_V1](D:/newwork/本地智能体/docs/06-development/开发文档收口导航_V1.md)
2. [产品级冻结与下一阶段规划入口文档_V1](D:/newwork/本地智能体/docs/06-development/产品级冻结与下一阶段规划入口文档_V1.md)
3. [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)
4. [第二阶段产品定位与开发重点清单_V1](D:/newwork/本地智能体/docs/06-development/第二阶段产品定位与开发重点清单_V1.md)
5. [第二阶段短期可用能力开发任务书_V1](D:/newwork/本地智能体/docs/06-development/第二阶段短期可用能力开发任务书_V1.md)
6. [第二阶段需求文档_V1](D:/newwork/本地智能体/docs/06-development/第二阶段需求文档_V1.md)
7. [忠实用户转化导向开发任务书_V1](D:/newwork/本地智能体/docs/06-development/忠实用户转化导向开发任务书_V1.md)

历史只读：

1. 历史任务书与可删除候选，统一见 [开发文档收口导航_V1](D:/newwork/本地智能体/docs/06-development/开发文档收口导航_V1.md)

### `07-test`

定位：

1. 验收标准
2. 历史验收记录
3. 问题清单

状态：

1. `以历史记录为主`
2. `当前只按需参考`

当前优先读：

1. [关键入口文档](D:/newwork/本地智能体/docs/07-test/关键入口文档.md)
2. [产品级总体验收文档_V1](D:/newwork/本地智能体/docs/07-test/产品级总体验收文档_V1.md)
3. [验收标准_V1](D:/newwork/本地智能体/docs/07-test/验收标准_V1.md)
4. [需求文档对照完成度清单_V1](D:/newwork/本地智能体/docs/07-test/需求文档对照完成度清单_V1.md)
5. [V1回归检查入口_V1](D:/newwork/本地智能体/docs/07-test/V1回归检查入口_V1.md)
6. [里程碑验收与首版候选汇总_V1](D:/newwork/本地智能体/docs/07-test/里程碑验收与首版候选汇总_V1.md)
7. [本地记忆与知识沉淀验收文档_V1](D:/newwork/本地智能体/docs/07-test/本地记忆与知识沉淀验收文档_V1.md)
8. [SQLite 主存储验收文档_V1](D:/newwork/本地智能体/docs/07-test/SQLite 主存储验收文档_V1.md)
9. [思源外挂知识库接入验收文档_V1](D:/newwork/本地智能体/docs/07-test/思源外挂知识库接入验收文档_V1.md)
10. [工作区与授权策略产品化验收文档_V1](D:/newwork/本地智能体/docs/07-test/工作区与授权策略产品化验收文档_V1.md)
11. [前端产品化验收文档_V1](D:/newwork/本地智能体/docs/07-test/前端产品化验收文档_V1.md)
12. [首页状态化验收文档_V1](D:/newwork/本地智能体/docs/07-test/首页状态化验收文档_V1.md)
13. [智能体框架主干总体验收文档_V1](D:/newwork/本地智能体/docs/07-test/智能体框架主干总体验收文档_V1.md)
14. [第二阶段短期可用能力验收文档_V1](D:/newwork/本地智能体/docs/07-test/第二阶段短期可用能力验收文档_V1.md)

### `10-v2`

定位：

1. 当前产品形态
2. 当前前端执行域
3. 当前前端唯一执行入口所在目录

状态：

1. `当前前端执行域`
2. `前端实现优先`
3. `与冻结稿冲突时遵循冻结稿`

---

## 3. 当前文档层级

### P0 - 直接执行

1. [需求冻结稿_V1](D:/newwork/本地智能体/docs/00-charter/需求冻结稿_V1.md)
2. [产品边界与入口定义_V1](D:/newwork/本地智能体/docs/01-boundary/产品边界与入口定义_V1.md)
3. [MVP功能清单_V1](D:/newwork/本地智能体/docs/01-boundary/MVP功能清单_V1.md)
4. [验收标准_V1](D:/newwork/本地智能体/docs/07-test/验收标准_V1.md)

### P1 - 领域入口与实现参考

说明：

1. 只有“当前唯一执行口径”一节中点名的入口，默认作为直接执行入口。
2. 本层其余文档用于领域实现、方案细化与回溯参考，不与唯一入口并列竞争。

1. [V2产品需求文档](D:/newwork/本地智能体/docs/10-v2/V2产品需求文档.md)
2. [V2前端视觉与界面实现方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端视觉与界面实现方案_V1.md)
3. [V2前端体验与架构优化方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端体验与架构优化方案_V1.md)
4. [V2前端定制工作流_V1](D:/newwork/本地智能体/docs/10-v2/V2前端定制工作流_V1.md)
5. [V2前端当前执行入口_V1](D:/newwork/本地智能体/docs/10-v2/V2前端当前执行入口_V1.md)
6. [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)
7. [运行时 Harness 收口方案_V1](D:/newwork/本地智能体/docs/06-development/运行时 Harness 收口方案_V1.md)
8. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)
9. [本地记忆与知识沉淀需求文档_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀需求文档_V1.md)
10. [本地记忆与知识沉淀开发任务书_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀开发任务书_V1.md)
11. [开发文档收口导航_V1](D:/newwork/本地智能体/docs/06-development/开发文档收口导航_V1.md)
12. [前端重构开发任务书_V1](D:/newwork/本地智能体/docs/06-development/前端重构开发任务书_V1.md)
13. [产品级冻结与下一阶段规划入口文档_V1](D:/newwork/本地智能体/docs/06-development/产品级冻结与下一阶段规划入口文档_V1.md)
14. [关键入口文档](D:/newwork/本地智能体/docs/07-test/关键入口文档.md)
15. [ClaudeCode本地智能体框架架构设计文档](D:/newwork/本地智能体/docs/ClaudeCode本地智能体框架架构设计文档.md)
16. [Rust运行时设计草案_V1](D:/newwork/本地智能体/docs/03-runtime/Rust运行时设计草案_V1.md)

### P2 - 补充参考

1. `04-api/`
2. `05-migration/`

### P3 - 历史归档

1. M1 开发任务书
2. M1 验收记录
3. 已知非阻断问题
4. 历史阶段计划

---

## 4. 当前建议阅读顺序

如果你是做前端：

1. [需求冻结稿_V1](D:/newwork/本地智能体/docs/00-charter/需求冻结稿_V1.md)
2. [V2前端当前执行入口_V1](D:/newwork/本地智能体/docs/10-v2/V2前端当前执行入口_V1.md)
3. [V2首页状态化方案_V1](D:/newwork/本地智能体/docs/10-v2/V2首页状态化方案_V1.md)
4. [V2前端体验与架构优化方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端体验与架构优化方案_V1.md)
5. [V2前端定制工作流_V1](D:/newwork/本地智能体/docs/10-v2/V2前端定制工作流_V1.md)
6. [前端状态与模块分层设计文档_V1](D:/newwork/本地智能体/docs/02-architecture/前端状态与模块分层设计文档_V1.md)
7. [前端重构开发任务书_V1](D:/newwork/本地智能体/docs/06-development/前端重构开发任务书_V1.md)
8. [第一阶段状态模型落地任务拆解清单_V1](D:/newwork/本地智能体/docs/06-development/第一阶段状态模型落地任务拆解清单_V1.md)
9. [前端产品化验收文档_V1](D:/newwork/本地智能体/docs/07-test/前端产品化验收文档_V1.md)
10. [首页状态化验收文档_V1](D:/newwork/本地智能体/docs/07-test/首页状态化验收文档_V1.md)
11. [V2前端视觉与界面实现方案_V1](D:/newwork/本地智能体/docs/10-v2/V2前端视觉与界面实现方案_V1.md)

如果你是做运行时：

1. [需求冻结稿_V1](D:/newwork/本地智能体/docs/00-charter/需求冻结稿_V1.md)
2. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)
3. [当前项目本地化架构方案_V1](D:/newwork/本地智能体/docs/02-architecture/当前项目本地化架构方案_V1.md)
4. [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)
5. [运行时 Harness 收口方案_V1](D:/newwork/本地智能体/docs/06-development/运行时 Harness 收口方案_V1.md)
6. [本地记忆与知识沉淀需求文档_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀需求文档_V1.md)
7. [本地记忆与知识沉淀开发任务书_V1](D:/newwork/本地智能体/docs/06-development/本地记忆与知识沉淀开发任务书_V1.md)
8. [ClaudeCode本地智能体框架架构设计文档](D:/newwork/本地智能体/docs/ClaudeCode本地智能体框架架构设计文档.md)
9. [Rust运行时设计草案_V1](D:/newwork/本地智能体/docs/03-runtime/Rust运行时设计草案_V1.md)

如果你是做验收：

1. 当前阶段任务书
2. [验收标准_V1](D:/newwork/本地智能体/docs/07-test/验收标准_V1.md)
3. 对应阶段验收记录

---

## 5. 当前整理策略

当前先采用：

1. `不大规模移动文件`
2. `先通过导航和状态标记收口`
3. `等后续阶段更稳定后，再做物理归档整理`

这样做的原因：

1. 风险最低
2. 不会打断当前开发
3. 不会让已有引用路径全部失效
