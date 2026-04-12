# 变更提案

## 背景

- `F-01` 安装/升级主路径已完成并有验收证据。
- `F-02` 要求补齐 `doctor` 核心检查接入，当前仓库缺少统一可执行的诊断命令。
- 本轮继续按后端主线推进，不涉及前端页面改动。

## 目标

- 新增可独立执行的 `doctor` 命令，输出结构化诊断结果。
- 新增 `F-02` 验收脚本，产出 `tmp/stage-f-doctor/latest.json`。
- 覆盖核心检查项：依赖、配置、端口、前端产物、服务健康、日志可写。

## 非目标

- 本轮不做自动修复能力，只做只读诊断。
- 本轮不推进发布/回滚文档收口（归属 `F-03`）。
- 本轮不修改前端 UI 与交互。

## 验收口径

- `scripts/doctor.ps1` 可直接执行，并输出 JSON 结果。
- `scripts/run-stage-f-doctor-acceptance.ps1` 执行通过，`tmp/stage-f-doctor/latest.json` 为 `status=passed`。
- 诊断报告中关键检查项全部可复核。
