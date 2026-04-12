# 变更提案

## 背景

- 当前 A-F 主线已收口，但缺少“后端一键复核”入口，复跑要手工串多个脚本。
- `E-01` 已拆出历史/中断两条切片，`F-G1` 也有门禁脚本，适合做统一聚合。
- `E-04` 跨入口一致性虽已有脚本，但尚未纳入后端复核包统一入口。
- 用户要求继续后端推进，本轮不改前端。

## 目标

- 新增一键复核脚本，统一聚合：
  - `E-01` 历史切片
  - `E-01` 中断切片
  - `E-04` 跨入口一致性
  - `F-G1` Gate-F 门禁
- 提供严格门禁模式：
  - 校验证据 `checked_at` 时效。
  - 校验关键结构字段完整性（run 身份、终态、Gate-F 关键字段）。
- 产出单份报告：`tmp/stage-backend-reverify/latest.json`。

## 非目标

- 不改前端页面。
- 不改变 E/F 既有验收标准，仅做聚合复核层。

## 验收口径

- `run-stage-backend-reverify-pack.ps1 -RequirePass` 执行通过。
- `run-stage-backend-reverify-pack.ps1 -StrictGate -RequirePass` 执行通过。
- 报告中 `backend_reverify_ready=true`。
