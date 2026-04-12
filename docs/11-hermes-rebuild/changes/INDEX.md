# Hermes Change 索引

更新时间：2026-04-12

这个文件用于标记当前执行主线下的活跃 change，避免多项变更并行时上下文漂移。

## 当前活跃 change

1. [E-frontend-experience-upgrade](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md)（2026-04-12 已恢复为前端重构主推进项）

## 保留观察项

1. [D-memory-skill-foundation](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/D-memory-skill-foundation/status.md)（历史进行中状态，暂不参与当前前端重构主线）

## 归档入口

1. [archive/2026-04-12/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-12/INDEX.md)（已收口 change 统一归档，避免影响后续开发口径）

## 选择规则

1. 继续任务时，优先读取“当前活跃 change”。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果 `INDEX.md` 与各 change 的 `status.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等以上变更后，将其加入索引。
2. 某个 change 进入主推进状态时，把它放到“当前活跃 change”第一位。
3. 某个 change 完成并收口后，移动到 `archive/<日期>/` 并补归档索引。
