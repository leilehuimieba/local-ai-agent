# 验证记录

## 验证方式

- 单元测试：
  - 主入口说明条强调主线总控语义
  - 任务首屏说明卡强调主目标优先与辅助入口降级
- 人工验证：
  - 进入首页时，能看到“主线总控 Agent”品牌与能力分层说明
  - 任务首屏 idle 态能看到“主目标接管 / 关键证据 / 临时切主线”为核心链路

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/app-entry-branding.test.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/task-view-confirmation.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/app/layout.tsx`
  - `D:/newwork/本地智能体/frontend/app/page.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/brand-strip.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/task-entry-card.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/top-bar.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/views/task-view.tsx`
- 命令输出：
  - `npm test -- app-entry-branding task-view-confirmation`
  - `npm test`

## Gate 映射

- 对应阶段 Gate：
  - 主入口叙事从泛本地智能体切换到主线总控 Agent
- 当前覆盖情况：
  - 已完成 metadata、入口说明条、任务首屏说明卡和品牌口径的最小前端收口
  - 后续如需更强证据，可补浏览器可视化截图与更多旧文案替换检查
