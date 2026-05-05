# 任务清单

- [x] 建立 AB-change 工作区并切换活跃项
  完成判据：`current-state.md` 与 `changes/INDEX.md` 指向本 change。

- [x] 梳理现有 tool result / confirmation UI
  完成判据：明确 diff apply report 应接入的组件和数据来源。

- [x] 实现 diff dry-run 预览组件
  完成判据：新增、修改、删除、rename 能清晰展示。

- [x] 实现失败 report 展示
  完成判据：stage、path、reason、rollback_attempted 可读。

- [x] 接入确认后 apply 流程
  完成判据：用户确认后才执行 apply，取消不会写文件。

- [x] 补前端测试和验证记录
  完成判据：相关测试或类型检查通过，`verify.md` 记录证据。
