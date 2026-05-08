# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`（新增）
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 在 `MainlineShellState` 中新增明日计划状态：
    - 是否已生成
    - 明日唯一核心任务
    - 明日补充任务
    - 计划依据说明
    - 恢复状态说明
  - 新增独立规则文件 `next-day-plan-rules.ts`，负责：
    - 结合关键证据、个性化 insight、时间预算 insight
    - 生成明日核心任务与补充任务
    - 在临时主线恢复后优先回到原主线
  - 在主线总控壳中加入：
    - “明日计划”摘要卡
    - “生成明日计划”操作入口

## 状态流转

1. 用户提交晚间证据 / 关键证据 / 个性化反馈 / 时间预算后
2. 点击“生成明日计划”
3. 规则层综合现有状态输出明日方案
4. 主壳展示：
   - 明日唯一核心任务
   - 明日补充任务
   - 计划依据
   - 是否为恢复原主线后的方案
5. 次日进入主壳时，可继续基于这份方案执行

## 风险与回退

- 主要风险：
  - 当前仍是前端内存态，生成后的明日计划只在当前会话中可见
  - 如果证据不足，明日计划可能显得模板化
- 回退方式：
  - 计划摘要明确写出“依据”，避免伪装成高精度判断
  - 若信息不足，回退到“先补关键证据 / 先保唯一核心任务”的保守计划
  - 如复杂度过高，可回退为只展示核心任务与一句依据说明
