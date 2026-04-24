# 验证记录

## 验证方式

- 文档验证：
  1. 以 `docs/11-hermes-rebuild/current-state.md` 作为当前活跃 change 来源，核对 Gate-H 工作区文档是否与 active change=`H-gate-h-signoff-20260416` 保持一致。
  2. 核对 H-02 的 `status.md`、`verify.md` 与人工接管手册，确认其当前口径为开发阶段 ready，且高风险/权限类场景已由主控裁决接受为人工接管替代验收。
  3. 核对 H-03 的 `status.md`、`verify.md`、`review.md`、`formal-execution-entry.md` 与结构性缺口说明，确认其当前口径为开发阶段 ready，且结构性缺口已由主控裁决接受为风险接受条件。
- 脚本验证：
  1. 执行 `scripts/run-stage-h-gate-acceptance.ps1`，确认生成 `tmp/stage-h-gate/latest.json`。
  2. 执行 `scripts/run-stage-h-signoff-acceptance.ps1 -RequireSignoff`，确认生成 `tmp/stage-h-signoff/latest.json`。
  3. 检查 gate JSON 当前保持 `status=development_ready`，`gate_h.ready=true`，`h02_ready=true`，`h03_ready=true`。
  4. 检查 signoff JSON 当前保持 `status=signed_off`，`signoff_ready=true`，`development_ready=true`。
  5. 检查 signoff JSON 中 `h02_manual_takeover_accepted=true`、`h03_structural_gap_accepted=true`。
  6. 检查两份 JSON 的双语输出约定是否稳定：英文结构字段作为机器可读主结构，中文说明字段仅作人工复核与提审说明。
- 一致性验证：
  1. 检查 Gate-H 文档是否明确：H-02 / H-03 当前都已提升为 `development_ready`。
  2. 检查 Gate-H 文档是否明确：H-02 / H-03 的已知缺口已由主控裁决转为替代验收或后续治理。
  3. 检查 Gate-H 文档是否明确：当前 `signoff_ready=true`，Gate-H 已签收。
  4. 检查 Gate-H 文档是否明确：签收不扩大 H-02 自动修复边界，也不取消 H-03 长期校准。

## 证据位置

- 当前文档证据：
  1. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/proposal.md`
  2. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/design.md`
  3. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/tasks.md`
  4. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/status.md`
  5. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/verify.md`
  6. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/review.md`
  7. `scripts/run-stage-h-gate-acceptance.ps1`
  8. `scripts/run-stage-h-signoff-acceptance.ps1`
  9. `tmp/stage-h-gate/latest.json`
  10. `tmp/stage-h-signoff/latest.json`
- H-02 替代验收证据：
  1. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`
  3. `tmp/stage-h-remediation/manual-guides/high-risk-config-write.md`
  4. `tmp/stage-h-remediation/manual-guides/permission-elevation-required.md`
  5. `tmp/stage-h-remediation/manual-guides/baijiacms-db-prereq-missing.md`
  6. `tmp/stage-h-remediation/h02-baijiacms-sample-pass-summary-20260421.json`
- H-03 替代验收证据：
  1. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
  3. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
  4. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
  5. `tmp/stage-h-mcp-skills/latest.json`
  6. `tmp/stage-h-mcp-skills/h03-38-batch1-execution.json`
  7. `tmp/stage-h-mcp-skills/h03-39-handoff-check.json`
  8. `tmp/stage-h-mcp-skills/evals/institutional-review-minimum-closure.json`
  9. `tmp/stage-h-mcp-skills/structural-gap-acceptance-20260424.md`
- 唯一主推进状态源：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段 Gate：
  1. `Gate-H`
- 当前覆盖情况：
  1. H-01 已签收。
  2. H-02 已提升为开发阶段 ready：十一个低风险受限验证窗口全部闭环，高风险配置写入和权限类场景已由永久人工接管手册覆盖。
  3. H-03 已提升为开发阶段 ready：数量门槛 30/24/16 已达标，skill_hit_effective 三维度已代码化并通过测试，制度化多评审最小闭环已形成，剩余结构性缺口已由风险接受条件覆盖。
  4. H-04/H-05 已签收。
  5. Gate-H 已完成主控裁决，`signoff_ready=true`。

## 2026-04-24 主控裁决记录

1. H-02 人工接管手册可替代当前高风险/权限类 runtime 验收。
   - 替代对象：`C-B`~`C-F` 高风险配置写入、`P-C`/`P-D` 权限提升类场景。
   - 替代性质：风险接受与人工接管，不是自动修复能力通过。
   - 后续要求：继续保持自动化停止、人工接管、回退记录与后续新授权重开 change 的边界。
2. H-03 结构性缺口说明可替代上线前长期校准闭合要求。
   - 替代对象：当前剩余 `business-task-chain`、`skill-false-positive`、`manual-review` 样本缺口。
   - 替代性质：结构性资源缺口的风险接受，不是长期校准取消。
   - 后续要求：新 runtime 观测样本出现后优先回填 `manual-review` 缺口，并维持 skill guard 可观测字段。
3. Gate-H 签收裁决：
   - `signoff_ready=true`。
   - 允许 Gate-H 签收。

## 本轮结论

1. H-02 / H-03 在开发阶段标准下均已 ready。
2. H-02 / H-03 的上线前阻塞项已由主控裁决转为替代验收或后续治理。
3. Gate-H 已签收。

## 后续接手边界

1. H-02：按"开发阶段 ready，高风险/权限类场景由永久人工接管手册覆盖"的口径继续引用。
2. H-03：按"开发阶段 ready，结构性缺口由风险接受条件覆盖，长期校准转后续治理"的口径继续引用。
3. Gate-H：当前 `signoff_ready=true`，但不自动放行后续阶段切换；阶段切换仍需先更新 `current-state.md` 与 `changes/INDEX.md`。
