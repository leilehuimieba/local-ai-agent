# 技术方案

## 影响范围

- 聚合脚本：`scripts/run-stage-f-gate-acceptance.ps1`
- 门禁证据：`tmp/stage-f-gate/latest.json`
- 提审文档：`docs/11-hermes-rebuild/changes/F-gate-f-signoff/*.md`
- 状态同步：`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. Gate-F 聚合验收

- 输入证据：
  - `tmp/stage-f-install/latest.json`（F-01）
  - `tmp/stage-f-doctor/latest.json`（F-02）
  - `tmp/stage-f-rc/latest.json`（F-04）
  - `tmp/stage-f-windows/latest.json`（F-05）
- 规则：
  - 安装/升级链路检查通过。
  - doctor 核心检查通过。
  - 发布候选回归与故障注入通过。
  - Windows 新机 10 分钟验证通过。
  - F 阶段变更状态文档无阻塞项（以“阻塞点为无”为准）。
- 输出：
  - `gate_f.install_ready`
  - `gate_f.doctor_ready`
  - `gate_f.release_candidate_ready`
  - `gate_f.windows_10min_ready`
  - `gate_f.no_open_p0_p1`
  - `gate_f.ready`

### 2. 提审签收

- 在 `review.md` 固化 Gate-F 评审结论与证据映射。
- 回写 `INDEX.md` 和总表，标记 `F-G1` 为已完成。

## 风险与回退

- 风险：当前门禁以阶段验收样本为主，不等同长期生产流量稳态。
- 缓解：保留 Gate-F 聚合脚本作为回归入口，发布前可一键复跑。
- 回退：若后续复测失败，冻结发布动作，回退至 `F-04/F-05` 已通过基线重新定位。
