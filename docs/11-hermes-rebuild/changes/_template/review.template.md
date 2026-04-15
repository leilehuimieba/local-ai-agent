# 阶段性提审包（<change-id>）

更新时间：<YYYY-MM-DD>  
提审类型：<阶段子项提审 / Gate签收提审 / 阶段切换提审>  
评审状态：<草案|待评审|已签收|未通过>

## 1. 提审范围

本次提审仅覆盖 `<scope>`，不包含 `<out-of-scope>`。

覆盖项：

1. <覆盖点1>
2. <覆盖点2>
3. <覆盖点3>

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/<change-id>/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/<change-id>/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/<change-id>/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/<change-id>/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/<change-id>/verify.md`

## 3. 核心证据

### 3.1 聚合报告

1. `D:/newwork/本地智能体/tmp/<stage-or-change>/latest.json`

### 3.2 子证据

1. `D:/newwork/本地智能体/tmp/<stage-or-change>/<evidence-1>.json`
2. `D:/newwork/本地智能体/tmp/<stage-or-change>/<evidence-2>.json`
3. `D:/newwork/本地智能体/tmp/<stage-or-change>/<evidence-3>.json`

### 3.3 构建/测试记录（按实际回填）

1. `<cmd-1>`
2. `<cmd-2>`
3. `<cmd-3>`

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| <metric-1> | <threshold-1> | <actual-1> | <result-1> | `<path-1>` |
| <metric-2> | <threshold-2> | <actual-2> | <result-2> | `<path-2>` |
| <metric-3> | <threshold-3> | <actual-3> | <result-3> | `<path-3>` |

## 5. 评审结论

1. 本次提审结果：`status=<passed|warning|failed>`
2. 就绪度判定：`<gate_or_subitem>.ready=<true|false>`
3. 阻塞项统计：`p0=<n>, p1=<n>, warning=<n>`
4. 结论说明（必填）：
   - <为什么通过/为什么警告/为什么失败>

## 6. 风险与回退

1. 风险：
   - <risk-1>
   - <risk-2>
2. 回退触发条件：
   - <trigger-1>
   - <trigger-2>
3. 回退动作：
   - <rollback-step-1>
   - <rollback-step-2>

## 7. 后续动作

1. 若 `passed`：
   - <next-step-pass-1>
   - <next-step-pass-2>
2. 若 `warning`：
   - 责任人：`<owner>`
   - 追踪编号：`<tracking-id>`
   - 到期时间：`<iso8601>`
   - 补证动作：<warning-fix>
3. 若 `failed`：
   - <next-step-fail-1>
   - <next-step-fail-2>

## 8. Gate 映射

1. 对应 Gate：`<Gate-X>`
2. 覆盖项：
   - <covered-1>
   - <covered-2>
3. 未覆盖项（如有）：
   - <uncovered-1>（原因：<reason>）

## 9. 签收记录（评审后回填）

1. 评审人：`<reviewer>`
2. 评审时间：`<YYYY-MM-DDTHH:mm:ss+08:00>`
3. 最终结论：`<passed|warning|failed>`
4. 签收备注：<note>
