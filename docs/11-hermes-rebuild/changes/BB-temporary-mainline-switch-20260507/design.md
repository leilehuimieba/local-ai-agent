# 技术方案

## 影响范围

- 涉及模块：
  - `frontend/components/local-agent/mainline-shell.tsx`
  - `frontend/lib/local-agent/types.ts`
  - `frontend/lib/local-agent/store.ts`
  - `frontend/lib/local-agent/mainline-switch-rules.ts`（新增）
  - `frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/temporary-mainline-switch-rules.md`
  - 本 change 的 `proposal.md`、`tasks.md`、`status.md`、`verify.md`

## 方案

- 核心做法：
  - 为主线总控壳新增“临时切主线”面板
  - 新增前端规则层，集中处理：
    - 是否属于紧急且重要
    - 未通过复核时的坚持记账
    - 原主线快照保存
    - 自动恢复原优先级结构
  - 切换后把当前主目标切为临时目标，同时清空概率到 `未知`
  - 恢复时从原主线快照回填目标、概率、风险级别和调整建议

## 状态流转

1. 用户打开临时切主线面板
2. 填写临时目标、原因、截止时间，并勾选紧急 / 重要
3. store 调用规则层复核：
   - 紧急且重要：直接通过复核并切换
   - 不满足：若未坚持，仅返回拒绝反馈；若坚持，记账后切换
4. 切换成功后：
   - 临时目标升为当前主目标
   - 原主线快照进入 `temporaryMainline.originalSnapshot`
   - 反馈区提示已复核或已记账
5. 点击恢复后：
   - 原主线优先级结构恢复
   - 当前临时切主线记录标记 `restoredAt`

## 风险与回退

- 主要风险：
  - 纯前端快照恢复可能与未来后端状态源出现偏差
  - 临时切主线、关键证据、晚间证据三套面板叠加后，主壳复杂度上升
- 回退方式：
  - 切主线规则集中在 `mainline-switch-rules.ts`，可整体替换
  - 恢复只基于主壳快照，不改底层会话或后端状态
  - 若主壳过重，可后续继续拆出独立子组件
