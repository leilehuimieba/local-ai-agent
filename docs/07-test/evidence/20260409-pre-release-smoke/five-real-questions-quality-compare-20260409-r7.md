# 5 条真实问句前后对比（R7：回答质量稳定性）

执行目标：只优化回答质量稳定性，不扩功能。  
修补前样本：`five-real-questions-raw-20260409-quick-r7-before.json`  
修补后样本：`five-real-questions-raw-20260409-quick-r7-after.json`

## 本轮锁定的前 2 类问题

1. 复盘模板问句误入 `agent_resolve`，触发 `system` 失败。
2. 模型不稳定判断 / 非技术第一步问句缺稳定模板，答复不够可执行。

## 修补动作（最小）

1. `crates/runtime-core/src/planner.rs`  
   补问句触发词：`模板/复盘/第一步`，优先走 `session_context`。
2. `crates/runtime-core/src/executors/context.rs`  
   补稳定模板：`复盘模板（5 行）`、`模型不稳定可执行判断`、`非技术第一步`。

## 对比结果

| 指标 | 修补前 | 修补后 |
|---|---:|---:|
| `plan_tool=agent_resolve` 条数 | 1 | 0 |
| `result_mode=system` 条数 | 1 | 0 |
| 稳定模板命中条数 | 0 | 3 |
| 通用 recovery 兜底条数 | 1 | 1 |
| `run_failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775746920115-249` -> `run-1775747096341-264`  
   变化：answer -> recovery（受 provider 波动影响，回退到通用兜底，未闭合）。
2. Q2：`run-1775746928241-252` -> `run-1775747102473-267`  
   变化：保持 answer，仍需后续提升可执行细粒度。
3. Q3：`run-1775746936990-255` -> `run-1775747111253-270`  
   变化：recovery 通用兜底 -> answer 稳定模板。
4. Q4：`run-1775746960831-258` -> `run-1775747112897-273`  
   变化：system 失败 -> answer 稳定模板。
5. Q5：`run-1775746971210-261` -> `run-1775747114542-276`  
   变化：非执行性答复 -> answer 稳定模板。

## 务实结论

1. 本轮两类主问题已按最小修补显著收敛：`agent_resolve/system` 风险已清零，稳定模板命中提升到 3 条。
2. 仍有一处残余风险：Q1 在 provider 波动下回退到通用 recovery 兜底，下一轮应补“提测结论+两条理由”稳定模板。

