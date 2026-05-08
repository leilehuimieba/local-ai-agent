# 验证记录

## 验证方式

- 单元测试：
  - 暂不涉及代码级单测，优先验证文档与治理口径是否闭合
- 集成测试：
  - 后续实现时验证桌面插件、证据包、概率更新、临时切主线的联动
- 人工验证：
  - 检查主线定义是否可直接指导后续实现
  - 检查迁移边界是否不会破坏现有必要能力
  - 检查“当前主目标 + 安全通过概率”是否成为收缩态主展示

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/product-one-pager.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/proposal.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/design.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/calendar-plugin-information-architecture.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/implementation-change-split-plan.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/capability-classification.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/evidence-and-probability-rules.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/temporary-mainline-switch-rules.md`
- 日志或截图：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段 Gate：
  - 自由迭代期的产品语义收敛与迁移治理
- 当前覆盖情况：
  - 已覆盖主定义、核心场景、非核心边界、迁移边界、插件信息架构、证据规则、切主线规则、能力分类表与实现切分建议
  - 待补充：更细的能力分类表与下一实现 change 的代码级验证计划
