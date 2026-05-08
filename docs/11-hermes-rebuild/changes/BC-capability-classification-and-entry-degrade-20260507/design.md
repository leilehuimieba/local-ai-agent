# 技术方案

## 影响范围

- 涉及模块：
  - `frontend/app/layout.tsx`
  - `frontend/app/page.tsx`
  - `frontend/components/local-agent/top-bar.tsx`
  - `frontend/components/local-agent/views/task-view.tsx`
  - `frontend/components/local-agent/__tests__/task-view-confirmation.test.tsx`
  - `frontend/components/local-agent/__tests__/app-entry-branding.test.tsx`（新增）
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/capability-classification.md`
  - 本 change 的 `proposal.md`、`tasks.md`、`status.md`、`verify.md`

## 方案

- 核心做法：
  - 将全局 metadata、顶栏品牌名、任务视图 idle 态文案切换到“主线总控 Agent”语义
  - 在任务主视图首屏加入一块“主线总控说明卡”，明确：
    - 主目标是核心
    - 关键证据 / 晚间证据 / 临时切主线是主链
    - 知识库 / 历史 / 设置等继续保留，但降级为辅助能力
  - 保持现有视图结构不动，只切叙事与说明层，避免无谓扩 scope

## 状态流转

1. 用户进入首页
2. 顶栏、文档标题、首屏文案优先强调主线总控
3. 用户仍可进入知识、日志、设置等旧能力页
4. 这些能力继续保留，但不再作为首页核心承诺

## 风险与回退

- 主要风险：
  - 叙事切换过快但缺少说明，可能让旧用户误解现有能力被删除
  - 在不改结构的前提下，品牌和文案可能出现新旧混杂
- 回退方式：
  - 本轮优先改 metadata、品牌文案和任务首屏说明层，不碰底层能力
  - 若口径仍混杂，可在后续继续拆出独立的 capability banner 或 onboarding 卡片
