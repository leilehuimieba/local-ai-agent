# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`（新增）
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 新增 `mainline-entry-rules.ts`，统一根据当前主壳状态生成：
    - 主按钮指向哪个入口
    - 主按钮文案
    - 当前这一步为什么优先
  - 在主线总控壳中加入：
    - “下一步动作”摘要卡
    - 一个会随状态变化的主按钮
  - 保留现有功能按钮区，但不再让用户自己猜“先点哪个”

## 路由规则

1. 概率未知：
   - 主按钮优先切到关键证据录入
2. 晚间证据已提交且概率已知：
   - 主按钮优先切到明日计划
3. 补交窗口已过且裁决为“继续今天”：
   - 主按钮优先切到时间预算接管
4. 次日上午仍在补交窗口内且概率已知：
   - 主按钮优先切到补交昨晚证据
5. 默认：
   - 主按钮优先切到今晚证据包

## 风险与回退

- 主要风险：
  - 若规则太多，主按钮切换逻辑可能变得难理解
- 回退方式：
  - 如体验不稳定，可回退为只显示 helper 文案，不自动切主按钮
  - 若优先级争议较大，可只保留“未知 -> 关键证据”“已提交 -> 明日计划”两条强规则
