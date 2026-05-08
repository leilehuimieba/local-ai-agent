# 技术方案

## 影响范围

- 涉及模块：
  - 桌面日历插件 / 常驻壳
  - 插件收缩态与展开态展示层
  - 晚间证据包入口跳转
  - 补关键证据入口跳转
  - 开机启动与轻量常驻逻辑
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/calendar-plugin-information-architecture.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/product-one-pager.md`
  - `implementation-landing-notes.md`
  - `code-change-plan-v1.md`
  - 本 change 的 `proposal.md`、`tasks.md`、`status.md`、`verify.md`

## 方案

- 核心做法：
  - 先实现一个轻量常驻壳，而不是先接复杂逻辑
  - 收缩态只展示当前主目标与安全通过概率 / 未知状态
  - 若概率未知，则展示“请尽快补充关键证据”
  - 展开态只开放最小必要入口，不塞大量历史信息
  - 晚间证据包与补关键证据先做入口承载，允许后续 change 再补完整行为
- 状态流转或调用链变化：
  - 开机后插件常驻
  - 默认收缩显示
  - 用户点击后展开
  - 展开态可跳到晚间证据包入口
  - 概率未知时可跳到模考 / 真题结果录入入口

## 当前最小实现落点

### 页面挂载点

- 第一挂载点：
  - `D:/newwork/本地智能体/frontend/app/page.tsx`
- 原因：
  - 这是当前全局主壳，天然跨 `task / logs / knowledge / settings` 视图

### 新增组件建议

- 建议新增：
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 职责：
  - 渲染收缩态 / 展开态
  - 承载“当前主目标 + 安全通过概率 / 未知状态”
  - 作为晚间证据与补关键证据的全局入口壳

### 状态层建议

- 第一状态落点：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
- 建议：
  - 为 AY 增加轻量主线总控壳状态
  - 不把复杂概率逻辑与聊天运行态直接耦合

### 不推荐落点

1. `TaskView`：
   - 当前已经很重，不适合承担全局常驻壳
2. `RightDrawer`：
   - 当前只在 `task` 视图启用，不适合做跨视图总控插件
3. `LeftSidebar`：
   - 导航职责更强，不适合长期显示主线状态卡

## 第一轮代码改动口径

第一轮建议只改下面几类文件：

1. `frontend/lib/local-agent/types.ts`
2. `frontend/lib/local-agent/store.ts`
3. `frontend/components/local-agent/mainline-shell.tsx`
4. `frontend/app/page.tsx`
5. `frontend/app/globals.css`

如无必要，不在 AY 第一轮改：

1. `TaskView`
2. `RightDrawer`
3. `LeftSidebar`
4. 复杂后端与网关逻辑

## 风险与回退

- 主要风险：
  - 常驻壳做重，破坏“轻量常驻”目标
  - 展示字段过多，导致插件失去一眼可见的价值
  - 先接入过多复杂逻辑，导致第一刀实现边界失控
- 回退方式：
  - 优先保留静态展示壳与入口跳转，复杂逻辑延后
  - 若常驻模式不稳定，允许先降为手动启动的总控面板原型
  - 若展开态过重，收回到只保留最小入口集合
