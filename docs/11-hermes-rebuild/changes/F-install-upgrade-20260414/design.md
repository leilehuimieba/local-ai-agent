# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/install-local-agent.ps1`
  2. `scripts/run-stage-f-install-acceptance.ps1`
  3. `scripts/doctor.ps1`（仅必要联动，非本刀主改动）
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/F-install-upgrade-20260414/*`
  4. `tmp/stage-f-install/latest.json`

## 方案

- 核心做法：
  1. 先盘点当前安装脚本路径与依赖前置条件（PowerShell 权限、目录结构、版本写入点）。
  2. 以 `run-stage-f-install-acceptance.ps1` 为主验证入口，固定“安装成功 + 首次启动可达”的最小通过链。
  3. 若发现脚本缺口，仅做最小修复，确保回退路径可执行。
- 状态流转或调用链变化：
  1. 本刀优先改脚本与文档，不引入 runtime/gateway 结构性改动。
  2. 验证通过后再进入 `F-02 doctor` 补齐。

## 风险与回退

- 主要风险：
  1. Windows 环境差异导致安装脚本在不同机器行为不一致。
  2. 依赖缺失时错误提示不明确，影响首轮验收效率。
- 回退方式：
  1. 安装脚本失败时回退到上一稳定 tag 对应安装步骤，保持人工安装可用。
  2. 验收不通过时冻结当前脚本改动，仅保留诊断信息回写与失败样本。
