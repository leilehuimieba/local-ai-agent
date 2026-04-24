# 验证记录

## 验证方式

- 文档验证：
  1. 以 `docs/11-hermes-rebuild/current-state.md` 作为唯一主推进状态源，核对 Gate-H 工作区文档是否与当前 active change=`H-gate-h-signoff-20260416` 保持一致。
  2. 核对 H-02 的 `status.md`、`verify.md`，确认其当前口径已提升为"开发阶段 ready"，十一个受限验证窗口均已闭环，高风险场景已冻结为人工接管。
  3. 核对 H-03 的 `status.md`、`verify.md`、`review.md` 与 `formal-execution-entry.md`，确认其当前口径已提升为"开发阶段 ready"，数量门槛 30/24/16 已达标，skill_hit_effective 三维度已通过测试，制度化多评审最小闭环已形成。
- 脚本验证：
  1. 执行 `scripts/run-stage-h-gate-acceptance.ps1`，确认生成 `tmp/stage-h-gate/latest.json`。
  2. 执行 `scripts/run-stage-h-signoff-acceptance.ps1`，确认生成 `tmp/stage-h-signoff/latest.json`。
  3. 检查 gate JSON 当前保持 `status=development_ready`，`gate_h.ready=true`，`h02_ready=true`，`h03_ready=true`。
  4. 检查 signoff JSON 当前保持 `status=development_ready`，`signoff_ready=false`，`development_ready=true`。
  5. 检查两个 JSON 在保留英文结构字段的同时，包含 `summary_zh`、`status_zh` 以及阻塞项中文说明字段。
  6. 检查两份 JSON 的双语输出约定是否稳定：英文结构字段作为机器可读主结构，中文说明字段仅作人工复核与提审说明，不替代英文结构字段。
- 一致性验证：
  1. 检查 Gate-H 文档是否明确：H-02 / H-03 当前都已提升为 `development_ready`。
  2. 检查 Gate-H 文档是否明确：H-02 / H-03 的已知缺口已记录，上线前需补验收。
  3. 检查 Gate-H 文档是否明确：当前虽已开发阶段通过，但 `signoff_ready=false`，不可正式上线签收。
  4. 检查 Gate-H 文档是否明确：本工作区开发阶段通过不等于阶段完成，上线前验收是阶段 H 的最后一步。
- 收紧验证：
  1. 检查 Gate-H 文档是否已移除"非当前主推进 / 聚合复核候选"等旧口径。
  2. 检查 Gate-H 文档是否仍坚持：开发阶段通过但上线前不可签收。
  3. 检查 Gate-H 文档是否已把 H-03 从"warning"更新到 `development_ready` 后的当前权威强度。

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
- H-02 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`
  3. `tmp/stage-h-remediation/h02-baijiacms-db-prereq-takeover-20260421.json`
  4. `tmp/stage-h-remediation/h02-baijiacms-db-prereq-guide-20260421.json`
  5. `tmp/stage-h-remediation/manual-guides/baijiacms-db-prereq-missing.md`
  6. `tmp/stage-h-remediation/h02-baijiacms-siteid-host-check-20260421.json`
  7. `tmp/stage-h-remediation/h02-baijiacms-homepage-check-20260421.json`
  8. `tmp/stage-h-remediation/h02-baijiacms-sample-pass-summary-20260421.json`
- H-03 状态证据：
  1. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
  2. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
  3. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
  4. `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
  5. `tmp/stage-h-mcp-skills/latest.json`
  6. `tmp/stage-h-mcp-skills/h03-38-batch1-execution.json`
  7. `tmp/stage-h-mcp-skills/h03-39-handoff-check.json`
  8. `tmp/stage-h-mcp-skills/evals/institutional-review-minimum-closure.json`
- 唯一主推进状态源：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段 Gate：
  1. `Gate-H`
- 当前覆盖情况：
  1. H-01 已签收。
  2. H-02 已提升为开发阶段 ready：十一个低风险受限验证窗口全部闭环，高风险配置写入和权限类场景已冻结为人工接管，上线前需补 runtime 验收。
  3. H-03 已提升为开发阶段 ready：数量门槛 30/24/16 已达标，skill_hit_effective 三维度已代码化并通过测试，制度化多评审最小闭环已形成，manual-review 剩余 8 条结构化回指缺口为已知技术债。
  4. H-04/H-05 已签收。
  5. Gate-H 已开发阶段通过，但 `signoff_ready=false`，上线前需补验收方可签收。

## 本轮结论

1. 本轮目标是在开发阶段口径下，完成 Gate-H 聚合判断并统一工作区口径，明确已知缺口，释放资源转向后续开发任务。
2. H-02 / H-03 在开发阶段标准下均已 ready，Gate-H 开发阶段通过。
3. 上线前验收条件已明确记录，验收完成前 `signoff_ready` 保持为 `false`。

## 当前仍不可正式上线签收的原因

1. H-02 高风险配置写入场景（`C-B`~`F`）和权限类场景（`P-C`/`P-D`）尚未形成 runtime 验证结论。
2. H-03 manual-review 剩余 8 条结构化回指缺口尚未补齐，命中有效性分布仍需长期校准，多评审制度化流程仍需正式化。
3. Gate-H 作为阶段聚合判断，在上线前验收未完成前不可签收。

## 若主控后续接手时的入口边界

1. H-02：按"开发阶段 ready，上线前需补高风险场景 runtime 验收"的口径继续引用。
2. H-03：按"开发阶段 ready，上线前需补 manual-review 缺口 + 长期校准 + 制度化流程正式化"的口径继续引用。
3. Gate-H：当前已开发阶段通过，但上线前验收完成前 `signoff_ready=false`。
## 2026-04-24 证据重跑记录

1. 已重跑 `scripts/run-stage-h-gate-acceptance.ps1`，输出 `tmp/stage-h-gate/latest.json`。
   - `status=development_ready`
   - `status_zh=开发阶段通过`
   - `gate_h.ready=true`
2. 已重跑 `scripts/run-stage-h-signoff-acceptance.ps1`，输出 `tmp/stage-h-signoff/latest.json`。
   - `status=development_ready`
   - `status_zh=开发阶段通过`
   - `gate_h_signoff.development_ready=true`
   - `gate_h_signoff.signoff_ready=false`
3. 中文说明字段已从 base64 解码常量改为直接中文文本，避免重新生成证据时出现乱码或语义损坏。
4. 本轮重跑不改变 Gate-H 结论：开发阶段通过，上线前不可签收。
