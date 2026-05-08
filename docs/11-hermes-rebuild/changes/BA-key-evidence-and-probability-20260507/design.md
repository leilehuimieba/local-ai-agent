# 技术方案

## 影响范围

- 涉及模块：
  - `frontend/components/local-agent/mainline-shell.tsx`
  - `frontend/lib/local-agent/types.ts`
  - `frontend/lib/local-agent/store.ts`
  - `frontend/lib/local-agent/mainline-rules.ts`（新增）
  - `frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/evidence-and-probability-rules.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/calendar-plugin-information-architecture.md`
  - 本 change 的 `proposal.md`、`tasks.md`、`status.md`、`verify.md`

## 方案

- 核心做法：
  - 为主线总控壳新增“关键证据面板”，承载模考 / 真题结果录入
  - 关键证据分两条路径：
    - 完整且可信：更新概率、刷新风险级别、生成模块级调整建议
    - 非完整考试条件或字段不全：只记录反馈信息，不改概率
  - 新增纯前端规则文件，统一处理：
    - 可信性门槛
    - 48 小时过期判断
    - 概率展示降级
    - 模块级建议生成
  - 概率展示采用“前端演示型启发式规则”，只承担 V1 最小闭环，不冒充完整智能诊断系统

## 状态流转

1. 用户点击“补关键证据”
2. 录入做题日期、总分、考试条件、用时、分项得分
3. store 调用规则层判断：
   - 满足强关键证据门槛：更新概率与调整建议
   - 不满足：仅记录为弱参考
4. 收缩态与展开态统一读取规则层派生后的展示状态
5. 超过 48 小时无新的关键证据时，概率降为“未知”

## 风险与回退

- 主要风险：
  - 前端直接写启发式概率规则，后续可能需要替换为更真实的策略层
  - 主壳承载晚间证据包与关键证据双面板后，结构复杂度上升
- 回退方式：
  - 关键证据逻辑集中在 `mainline-rules.ts`，便于后续整体替换
  - 面板保持前端本地状态，不接后端即可快速回退到 AZ 状态
  - 若交互过重，可将关键证据面板拆成独立子组件继续收口
