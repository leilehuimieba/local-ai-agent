# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/evening-review-rules.ts`（新增）
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 在 `MainlineShellState` 中新增 `eveningReview` 状态：
    - 固定查看状态
    - 补交截止说明
    - 当前分流裁决
    - 最近一次提交归属日期
  - 新增规则文件 `evening-review-rules.ts`，负责：
    - 晚间查看窗口判断
    - 次日上午补交窗口判断
    - 中午后“先补昨天 / 继续今天”分流
    - 提交证据时写回“今天提交”或“补昨天提交”
  - 在主线总控壳中加入：
    - “晚上固定查看”摘要卡
    - 随状态变化的晚间证据入口文案
    - 对应的风险 banner

## 状态流转

1. 默认状态为“今晚待查看”
2. 晚上固定窗口内显示“现在进入固定查看窗口”
3. 若错过且到次日上午，进入“昨晚证据可补交”
4. 若超过次日中午仍未补交，进入“补交窗口已过”
5. 根据当前风险与时间预算，裁决：
   - 先补昨天
   - 或继续今天
6. 用户提交证据后，状态回到“今晚证据已提交”

## 风险与回退

- 主要风险：
  - 当前仍是前端内存态，跨重启后不会保留真实补交流程
  - “先补昨天 / 继续今天”仍是基于当前有限状态的轻量裁决
- 回退方式：
  - 若状态流转过于复杂，可回退为只显示“可补交 / 已超时”两态
  - 若分流判断不稳，可先固定超时后默认“继续今天”
