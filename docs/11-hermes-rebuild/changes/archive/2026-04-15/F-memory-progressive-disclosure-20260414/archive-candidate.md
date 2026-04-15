# 归档候选（并行专项）

- change：`F-memory-progressive-disclosure-20260414`
- 候选状态：可归档（待串行执行）
- 记录时间：2026-04-15
- 说明：本文档仅给出归档候选结论与执行建议；不修改锁文件，不执行目录迁移。

## 1. 归档前置条件核对

### 1.1 里程碑与 Gate

- M0：已完成
- M1：已完成
- M2：已完成
- M3：已完成
- M4：已完成
- M5：已完成
- Gate-M5：验收证据已补齐（见 `verify.md`）

### 1.2 change 五件套与提审文档

- `proposal.md`：已存在
- `design.md`：已存在
- `tasks.md`：已存在，`MEM-01 ~ MEM-25` 已勾选
- `status.md`：已收口
- `verify.md`：已补齐 M3~M5 验收证据
- `review.md`：已存在
- `risk-rollback-register.md`：已存在

### 1.3 关键验证结果（摘录）

- 检索评测：`Top-5 命中率 = 100.0%`（阈值 70%）
  - 证据：`tmp/stage-mem-eval/latest.json`
- 分层注入节省：`saved_percent = 58.3778...`（阈值 50%）
  - 证据：`tmp/stage-mem-m4/ab-test.json`
- 隐私与回退链路：脱敏 / private 排除 / feature flag 回退均通过
  - 证据：
    - `tmp/stage-mem-m5/privacy-redact.json`
    - `tmp/stage-mem-m5/private-skip.json`
    - `tmp/stage-mem-m5/rollback.json`

## 2. 归档建议动作（仅建议，不在本并行分支执行）

> 触发条件：主线确认“执行归档迁移”。

1. 串行切换后，更新：
   - `docs/11-hermes-rebuild/current-state.md`
   - `docs/11-hermes-rebuild/changes/INDEX.md`
2. 将 `changes/F-memory-progressive-disclosure-20260414` 迁移到：
   - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414`
3. 在 `docs/11-hermes-rebuild/changes/archive/<归档日期>/INDEX.md` 增加条目：
   - 归档原因：并行专项目标完成（M0~M5 + Gate-M5）
   - 保留入口：证据路径与提审文档路径
4. 保持证据目录可追溯（`tmp/stage-mem-*`、`tmp/stage-mem-eval`）并在索引中注明。

## 3. 归档后保留入口建议

- 方案与设计：
  - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414/proposal.md`
  - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414/design.md`
- 验收与风险：
  - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414/verify.md`
  - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414/review.md`
  - `docs/11-hermes-rebuild/changes/archive/<归档日期>/F-memory-progressive-disclosure-20260414/risk-rollback-register.md`
- 证据目录（运行产物）：
  - `tmp/stage-mem-m3/`
  - `tmp/stage-mem-m4/`
  - `tmp/stage-mem-m5/`
  - `tmp/stage-mem-eval/`

## 4. 风险与回退可追溯点

1. 若归档后出现回归或口径争议，优先回看：
   - `verify.md` 的 Gate-M5 证据链
   - `risk-rollback-register.md` 的回退触发与步骤
2. 若需快速恢复为活跃 change：
   - 从 archive 路径原位恢复目录；
   - 在 `changes/INDEX.md` 恢复活跃条目；
   - 在 `current-state.md` 恢复阶段口径并标记“恢复日期/原因”。
3. 本 change 的代码开关回退路径已内建（feature flag），可先走运行时回退再决定是否反向迁移文档状态。

## 5. 结论

该专项满足归档候选条件。建议在主线窗口以串行方式执行归档迁移；当前并行执行上下文保持“仅候选，不落锁文件”。
