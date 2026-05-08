# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/time-budget-rules.ts`（新增）
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 在 `MainlineShellState` 中新增时间预算状态：
    - 今日唯一核心任务
    - 今日可用时间块草稿
    - 已提交时间块列表
    - 现实时间预算变化反馈
    - 时间预算 insight
  - 新增独立规则文件 `time-budget-rules.ts`，负责：
    - 时间块解析
    - 总可用时长统计
    - 预算状态文案
    - 缩量保主线建议生成
  - 在主线总控壳中加入：
    - “今日主线接管”摘要卡
    - “时间预算 / 时间块录入”面板

## 状态流转

1. 用户录入今日唯一核心任务与可用时间块
2. 前端规则层统计当天总可用时长
3. 主壳更新预算状态：宽松 / 正常 / 紧张 / 极紧
4. 若预算偏低，自动改写建议为“先降任务量，保主目标最核心部分”
5. 后续主线调整仍由用户与 Agent 协商，不自动切主线

## 风险与回退

- 主要风险：
  - 没接真实日历时，时间块仍依赖用户手填，可能与现实不完全同步
  - 预算紧张时如果提示过强，可能与现有个性化提示冲突
- 回退方式：
  - 本轮只做手动录入与前端可见接管，不做自动同步
  - 时间预算 insight 只作为主线接管辅助，不直接覆盖关键证据概率与临时切主线结果
  - 如信息密度过高，可回退为摘要卡 + 面板，先不长期常显全部细节
