# Hermes 当前执行状态（单一事实源）

更新时间：2026-04-13  
状态：`当前有效`

本文件是 `docs/11-hermes-rebuild/` 下关于“当前阶段 / 当前 Gate / 当前活跃 change”的唯一权威记录。

## 1. 当前执行状态

1. 当前阶段：阶段 E（交互与网关统一）
2. 当前 Gate：Gate-E（执行中，未做完成声明）
3. 当前活跃 change：`E-low-quality-scoring-upgrade`
4. 当前活跃 change 路径：`docs/11-hermes-rebuild/changes/E-low-quality-scoring-upgrade/`

## 2. 口径边界

1. 除本文件外，其他文档中出现的阶段或 Gate 描述，默认按“历史记录或上下文说明”处理。
2. 如 `changes/INDEX.md`、`文档阅读与执行指引.md`、各 change 的 `status.md` 与本文件冲突，以本文件为准。
3. `docs/archive/` 下所有阶段描述默认不参与当前状态判定。

## 3. 更新规则

1. 新建或切换主推进项时，先更新本文件，再更新 `changes/INDEX.md`。
2. 进入提审或 Gate 结论变更时，先更新本文件，再更新阶段计划或 change 状态。
3. 每次更新本文件时，同时补“更新时间”。
