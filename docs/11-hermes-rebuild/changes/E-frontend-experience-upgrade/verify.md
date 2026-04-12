# 验证记录

## 验证方式

- 构建验证：在 `frontend/` 目录执行 `npm run build`。
- 人工验证：按 `docs/frontend-acceptance.md` 的主工作流闭环与调查层闭环条目逐项核对。
- 人工验证：按 Wave 1 主题 1 范围核对主线程与调查层状态表达，不跨页面扩 scope。
- 接口样本复核：提取阶段 C/D 样本中的审计字段并落盘 `tmp/stage-e-audit-consumption/latest.json`。

## 本轮验证点（Wave 1 - 主题 1）

1. 提交后等待态
   - 提交任务后到首个事件到来前，主线程显示“等待首个事件”状态记录。
2. 事件到来后的调查层模式区分
   - 调查层显示“自动跟随最新 / 手动查看历史”模式提示，且手动查看时可回到最新事件。
3. 失败且无事件恢复态
   - 失败且本轮未形成事件时，调查层不回落空态，展示失败说明与恢复建议。
4. 完成态与失败态可扫读性
   - 主线程状态记录区分运行中 / 失败 / 完成三态，文案与视觉层级可快速识别。

## 本轮验证点（Wave 1 - 主题 2 / E-03）

1. 记录页审计筛选可用
   - 筛选栏新增 `确认链 / 工具耗时 / 治理字段` 维度，可按审计信号收敛时间线结果。
2. 时间线审计字段可见
   - 时间线标签可显示确认链步骤、工具耗时、治理状态或归档标记。
3. 详情栏确认链闭环可复核
   - 详情栏可直接查看 `confirmation_chain_step / confirmation_decision / confirmation_resume_strategy / checkpoint_id`。
4. 详情栏治理字段可复核
   - 详情栏可直接查看 `governance_version / governance_source / governance_status / governance_reason / archive_reason`。
5. 接口样本字段落盘
   - 已生成 `tmp/stage-e-audit-consumption/latest.json`，包含阶段 C 风险确认链与阶段 D 治理字段样本值。

## 本轮验证点（Wave 1 - 主题 3）

1. 任务页调查层真实挂载恢复
   - 任务页渲染路径恢复 `BottomPanel`，`renderBottomPanel` 不再固定返回 `null`。
2. 调查层状态接线可用
   - `BottomPanel` 接收 `isOpen/onOpenChange/events/runState/submitError`，与现有任务态联动一致。
3. 任务页关键场景自动展开可用
   - `openTaskPage/openTaskPageForConfirmation` 在确认、失败或已有事件场景下自动展开调查层。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 证据位置

- 测试记录：
  - `frontend/src/chat/ChatPanel.tsx`
  - `frontend/src/chat/chatResultModel.ts`
  - `frontend/src/workspace/BottomPanel.tsx`
  - `frontend/src/shell/workspaceViewModel.tsx`
  - `frontend/src/App.tsx`
  - `frontend/src/history/auditSignals.ts`
  - `frontend/src/history/useHistoryReview.ts`
  - `frontend/src/history/components/HistoryPageSections.tsx`
  - `frontend/src/history/components/HistoryTimelineSection.tsx`
  - `frontend/src/history/components/HistoryDetailRail.tsx`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/tasks.md`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/verify.md`
- 日志或截图：
  - 构建命令：`frontend/` 目录执行 `npm run build`。
  - 构建结果：`tsc -b && vite build` 执行成功，产物已生成到 `frontend/dist/`。
  - 接口样本提取：`tmp/stage-e-audit-consumption/latest.json`（来源：`tmp/stage-c-risk-audit-acceptance/latest.json`、`tmp/stage-d-day1/latest.json`）。

## Gate 映射

- 对应阶段 Gate：Gate-E（交互收口执行中，不做 Gate-E 完成声明）。
- 当前覆盖情况：
  - 已完成 Wave 1 主题 1「任务页状态闭环优化」代码落地与构建验证。
  - 已完成 E-03「前端确认链字段消费改造」代码落地与构建验证。
  - 已完成阶段 C/D 样本审计字段提取，确认前端消费字段与后端口径一致。
  - 已补齐本轮任务与状态文档同步，证据路径可复查。
  - Wave 1 其余主题尚未执行，Gate-E 尚未满足整体完成条件。
