# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/run-stage-f-gate-acceptance.ps1`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/F-gate-f-signoff-20260414/*`
  4. `tmp/stage-f-gate/latest.json`

## 方案

- 核心做法：
  1. 修复 Gate-F 脚本里的 `statusDocs` 映射，改为当前 change 命名（`*-20260414`）。
  2. 补齐脚本容错字段 `exists`，避免路径缺失时直接崩溃且保留定位信息。
  3. 复跑 Gate-F 验收并验证 `install/doctor/release_candidate/windows/no_open_p0_p1` 五项均达标。
- 状态流转或调用链变化：
  1. 本刀只改验收脚本与文档，不修改运行时主链路。
  2. Gate-F 通过后，进入阶段切换评审（是否转入阶段 G）。

## 风险与回退

- 主要风险：
  1. 脚本路径修复若不完整，会导致 Gate-F 报告误判或直接失败。
  2. 分项证据更新时序不一致，可能出现“脚本通过但证据过期”争议。
- 回退方式：
  1. 若脚本失败，先回退到上一个可执行版本并保留失败日志。
  2. 若结果争议，按分项证据逐项重跑（install/doctor/rc/windows）后再判定。
