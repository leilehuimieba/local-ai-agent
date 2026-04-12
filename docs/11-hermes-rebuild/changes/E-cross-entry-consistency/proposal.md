# 变更提案

## 背景

- 用户要求暂停前端改造，继续推进后端执行链。
- 阶段 E 的 `E-04` 要求校验“跨入口上下文一致性”，当前缺少 CLI 与 gateway 的同 `run_id` 对比脚本与证据样本。

## 目标

- 增加 CLI 直连 runtime 与 gateway 入口的同 `run_id` 一致性对比脚本。
- 支持 `chat/run` 可选注入 `request_id/run_id/trace_id`，作为一致性校验锚点。
- 产出 `tmp/stage-e-consistency/latest.json` 作为 E-04 可复核证据。

## 非目标

- 本轮不修改前端页面与前端状态消费逻辑。
- 本轮不推进 E-05 联调报告收口，不做 Gate-E 完成声明。

## 验收口径

- `chat/run` 可接受可选身份字段并回传一致身份。
- `run-stage-e-consistency-acceptance.ps1` 可完成 CLI/runtime 与 gateway 对比并输出 `status=passed`。
- 证据中可复核同一 `run_id` 在 gateway 链路中的 run/session/trace 一致性与终态一致性。
