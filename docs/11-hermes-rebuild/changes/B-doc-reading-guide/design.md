# 技术方案

## 影响范围

- 涉及模块：
  - `docs/README.md`
  - `docs/11-hermes-rebuild/changes/INDEX.md`
  - `docs/11-hermes-rebuild/文档阅读与执行指引.md`（新增）
- 涉及文档或 contract：
  - `docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md`
  - `docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  - `docs/11-hermes-rebuild/changes/B-checkpoint-resume/*`

## 方案

- 核心做法：
  - 新增一份聚合指引文档，按“先判断口径 -> 再定位当前任务 -> 最后按需回看历史”的流程组织。
  - 在指引中给出三类清单：当前执行输入、历史参考输入、需要警惕的旧口径输入。
  - 明确以日期和文件状态为判断依据，减少“看名字猜状态”的不确定性。
  - 将该文档接入 `docs/README.md` 和 `changes/INDEX.md`。
- 状态流转或调用链变化：
  - 无代码状态流转变化。
  - 文档入口变化为：`docs/README.md` -> `文档阅读与执行指引` -> `总路线/阶段计划/活跃 change`。

## 风险与回退

- 主要风险：
  - 如果指引和后续实际推进不同步，可能再次产生双口径。
- 回退方式：
  - 保留历史入口不删减；若指引失效，可仅回退 `README` 与 `INDEX` 的链接，不影响其他文档。
