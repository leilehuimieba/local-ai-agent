# 当前状态

- 最近更新时间：2026-05-07
- 状态：已完成首轮实现，待后续与真实时间预算联动
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
- 已完成：
  - BB change 已创建并切换为当前主推进项
  - `MainlineShell` 已接入临时切主线面板与自动恢复按钮
  - `types.ts` / `store.ts` 已补齐切换表单、复核状态、原主线快照、切换历史与恢复逻辑
  - 新增 `frontend/lib/local-agent/mainline-switch-rules.ts`，集中承载复核、坚持记账、快照保存与恢复规则
  - 已实现复核通过切换、未通过但坚持记账切换、自动恢复原主线优先级结构
  - 前端测试已通过（5 files / 31 tests）
- 进行中：
  - 暂无；当前已达到 BB 最小闭环验收口径
- 阻塞点：
  - 当前仍是前端本地状态，未接真实日历时间块、后端持久化和定时恢复器
- 下一步：
  - 若继续主线推进，优先进入 `BC-capability-classification-and-entry-degrade-20260507`
