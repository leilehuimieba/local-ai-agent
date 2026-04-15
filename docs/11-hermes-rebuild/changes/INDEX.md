# Hermes Change 索引

更新时间：2026-04-15

这个文件用于提供 change 目录导航。
“当前阶段 / 当前 Gate / 当前活跃 change” 的状态裁决统一由
`docs/11-hermes-rebuild/current-state.md` 负责。

## 当前活跃 change

1. [G-gate-g-signoff-20260415](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-gate-g-signoff-20260415/status.md)
2. 当前主推进目录以 `current-state.md` 为准：`docs/11-hermes-rebuild/changes/G-gate-g-signoff-20260415/`
3. [G-runbook-duty-closure-20260415](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-runbook-duty-closure-20260415/status.md)（`G-04` 已收口）
4. [G-regression-baseline-20260415](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-regression-baseline-20260415/status.md)（`G-03` 已收口）
5. [G-warning-governance-closure-20260415](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-warning-governance-closure-20260415/status.md)（`G-02` 已收口）
6. [G-evidence-freshness-policy-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-evidence-freshness-policy-20260414/status.md)（`G-01` 已收口）
7. [G-stage-switch-signoff-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-stage-switch-signoff-20260414/status.md)（阶段切换已签收）
8. [G-stage-definition-prep-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/G-stage-definition-prep-20260414/status.md)（阶段切换准备已收口）
9. [F-gate-f-signoff-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-gate-f-signoff-20260414/status.md)（Gate-F 本轮签收）
10. [F-windows-10min-verification-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-windows-10min-verification-20260414/status.md)（`F-05` 已收口）
11. [F-release-candidate-regression-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-release-candidate-regression-20260414/status.md)（`F-03` 已收口）
12. [F-doctor-core-checks-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-doctor-core-checks-20260414/status.md)（`F-02` 已收口）
13. [F-install-upgrade-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-install-upgrade-20260414/status.md)（`F-01` 已收口）
14. [E-gate-e-signoff-20260414](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-gate-e-signoff-20260414/status.md)（Gate-E 签收变更）
15. [E-low-quality-scoring-upgrade](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-low-quality-scoring-upgrade/status.md)（上一轮已收口）
16. [E-cn-query-recall-optimization](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-cn-query-recall-optimization/status.md)（历史已收口）
17. [E-knowledge-base-activation](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-knowledge-base-activation/status.md)（历史已收口）
18. [E-settings-diagnostics-polish](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-settings-diagnostics-polish/status.md)（历史已完成）
19. 历史说明：`E-sensitive-pattern-expansion` 与 `E-frontend-experience-upgrade` 已迁入归档目录。

## 保留观察项

1. [D-memory-skill-foundation](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/D-memory-skill-foundation/status.md)（历史进行中状态，当前不作为主推进）

## 归档入口

1. [archive/2026-04-15/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-15/INDEX.md)（`F-memory-progressive-disclosure-20260414` 归档入口）
2. [archive/2026-04-14/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-14/INDEX.md)（`E-claudecode-shell-alignment`、`E-sensitive-pattern-expansion` 归档入口）
3. [archive/2026-04-13/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-13/INDEX.md)（本轮文档治理收口归档）
4. [archive/2026-04-12/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-12/INDEX.md)（历史已收口 change 归档）

## 选择规则

1. 继续任务时，先读取 `current-state.md`，再定位对应 change 目录。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果 `INDEX.md` 或任意 `status.md` 与 `current-state.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等以上变更后，将其加入索引。
2. 切换主推进项时，先更新 `current-state.md`，再更新本索引。
3. 某个 change 完成并收口后，移动到 `archive/<日期>/` 并补归档索引。
