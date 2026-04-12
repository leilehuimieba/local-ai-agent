# Hermes Change 索引

更新时间：2026-04-12

这个文件用于标记当前执行主线下的活跃 change，避免多项变更并行时上下文漂移。

## 当前活跃 change

1. [E-frontend-experience-upgrade](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md)（2026-04-12 阶段 E 主推进，前端体验与确认链字段消费对齐）

## 最近收口

1. [D-memory-skill-foundation](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/D-memory-skill-foundation/status.md)（2026-04-12 完成 D-G1 批量验收准备并收口）
2. [C-tool-contract-unification](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/C-tool-contract-unification/status.md)（2026-04-12 完成 C-G1 评审签收并收口）
3. [C-roadmap-task-decomposition](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/C-roadmap-task-decomposition/status.md)（2026-04-12 全路线最小任务分解与执行协议收口）
4. [B-doc-reading-guide](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/B-doc-reading-guide/status.md)（2026-04-12 文档阅读与口径治理收口）
5. [B-checkpoint-resume](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/B-checkpoint-resume/status.md)（2026-04-12 按任务列表收口）

## 选择规则

1. 继续任务时，优先读取“当前活跃 change”。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果存在多个活跃项，优先选择与当前阶段一致且最近更新时间最新的 change。
4. 如果 `INDEX.md` 与各 change 的 `status.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等以上变更后，将其加入索引。
2. 某个 change 进入主推进状态时，把它放到“当前活跃 change”第一位。
3. 某个 change 完成或暂停后，及时更新索引说明。
