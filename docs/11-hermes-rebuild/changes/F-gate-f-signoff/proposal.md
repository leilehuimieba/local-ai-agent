# 变更提案

## 背景

- 阶段 F 的 `F-01`、`F-02`、`F-03`、`F-04`、`F-05` 已完成并有证据落盘。
- 总表中 `F-G1` 仍待办，缺口是 Gate-F 的统一判定与评审签收。
- 用户要求继续后端主线推进，本轮不改前端。

## 目标

- 新增 Gate-F 聚合验收脚本，形成可复跑、可复核的阶段门禁证据。
- 产出 `tmp/stage-f-gate/latest.json`，明确 `gate_f.ready` 判定。
- 输出阶段 F 提审结论，给出发布决策与边界说明。

## 非目标

- 本轮不改前端页面与交互。
- 本轮不新增发布渠道或安装形态。
- 本轮不替代长期稳态压测，仅完成阶段门禁签收。

## 验收口径

- `run-stage-f-gate-acceptance.ps1 -RequireGateF` 执行通过。
- 报告中 `status=passed` 且 `gate_f.ready=true`。
- `review.md` 明确 Gate-F 判定、证据映射、风险与发布建议。
