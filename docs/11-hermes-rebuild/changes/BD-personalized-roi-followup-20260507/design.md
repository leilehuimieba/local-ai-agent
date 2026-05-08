# 技术方案

## 影响范围

- 涉及模块：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/personalized-followup-rules.ts`（新增）
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
  - 本 change 的五件套

## 方案

- 核心做法：
  - 在 `MainlineShellState` 中新增个性化反馈状态：
    - 当前记录草稿
    - 历史记录
    - 可见 insight
    - 最近一次反馈信息
  - 新增独立规则文件 `personalized-followup-rules.ts`，负责：
    - 任务类型标签
    - 记录解析
    - insight 汇总
    - 推荐文案生成
  - 在主线总控壳加入两个新可见层：
    - “更适合你的推进方式”摘要卡
    - “个性化推进记录”面板

## 状态流转

1. 用户完成或尝试一项任务后
2. 在主壳中记录任务类型、是否完成、是否带来短期收益等反馈
3. 前端规则层生成一条标准化记录并更新历史
4. 主壳摘要卡重新计算：
   - 更易完成项
   - 短期收益最高项
   - 当前阻力偏高项
5. 后续计划可基于这些反馈继续细化，但当前不自动重排计划

## 风险与回退

- 主要风险：
  - 样本量很小的时候，个性化提示容易显得过度自信
  - 若把“更容易完成”和“更值得押注”混为一谈，会误导用户理解
- 回退方式：
  - 通过 `basedOnCount` 与默认文案明确“先通用、后个性化”
  - 把提示限定为辅助 insight，不直接自动改主目标和概率
  - 如效果不稳定，可回退为只记录历史、不展示偏好推断
