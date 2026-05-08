# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 在主壳状态中新增“动作跟进”状态：
    - 最近完成的动作
    - 自动重判后的下一步
    - 对应的说明文案
  - 在关键动作提交函数中写回跟进状态：
    - 提交晚间证据
    - 更新关键证据
    - 生成明日计划
  - 在主壳中新增“连续引导”提示卡

## 联动规则

1. 晚间证据提交后：
   - 若概率未知，下一步优先补关键证据
   - 若概率已知，下一步优先生成明日计划
2. 关键证据更新后：
   - 若时间预算极紧，下一步优先时间预算接管
   - 否则优先生成明日计划
3. 明日计划生成后：
   - 明确提示是否已基于最新关键证据

## 风险与回退

- 主要风险：
  - 若跟进提示过多，主壳会显得重复
- 回退方式：
  - 如信息噪音过大，可回退为只保留最近一次动作提示
  - 若联动判断不稳，可只保留证据提交后的自动重判
