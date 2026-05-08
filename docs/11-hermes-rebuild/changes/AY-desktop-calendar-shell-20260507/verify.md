# 验证记录

## 验证方式

- 单元测试：
  - 视实现载体决定；如当前以前端组件为主，至少补最小渲染与状态展示验证
- 集成测试：
  - 验证插件可进入收缩态 / 展开态并触达晚间证据入口与补证入口
- 人工验证：
  - 开机或启动后能看到插件
  - 收缩态字段正确
  - 概率未知时提示正确
  - 展开态入口可达

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AY-desktop-calendar-shell-20260507/implementation-landing-notes.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AY-desktop-calendar-shell-20260507/code-change-plan-v1.md`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 日志或截图：
  - `D:/newwork/本地智能体/frontend/app/page.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/top-bar.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/left-sidebar.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/right-drawer.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/views/task-view.tsx`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`

## Gate 映射

- 对应阶段 Gate：
  - 自由迭代期第一实现项：桌面入口常驻闭环
- 当前覆盖情况：
  - 已完成设计、任务拆解、最小实现落点勘察、第一轮前端壳实现与最小测试验证
  - 待补：如需更强证据，可补人工截图或页面级验收记录
