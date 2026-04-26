# Proposal: 前端工作台统一与导航重组

## 背景

当前前端存在 "home" 与 "task" 两个独立视图，功能高度重叠：
- home：Hero 文案 + Composer + 示例卡片
- task：左侧导航 + 聊天流 + Composer

用户进入 home 后点击"开始任务"会跳转到 task，造成不必要的页面切换。

## 目标

将 home 内容合并到 task 视图空闲态中，重组导航结构，使"任务页"成为唯一主工作区。

## 范围

5 个 Phase：
1. Phase 1：合并 home + task 视图（类型清理 + UI 整合）
2. Phase 2：重组导航（左侧 Rail 承载全局切换，删除底部 Dock）
3. Phase 3：时间线改造
4. Phase 4：抽屉层（知识库/设置/日志）
5. Phase 5：视觉 polish

## 验收标准

- TypeScript 编译零错误
- 69 个前端测试全部通过
- 无功能回退（home 所有能力在 task 空闲态中可访问）
