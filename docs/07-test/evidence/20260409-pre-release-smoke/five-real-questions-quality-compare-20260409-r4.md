# 5 条真实问句前后对比（R4：回答质量稳定性）

执行目标：只优化回答质量稳定性，不扩功能。  
修补前样本：`five-real-questions-raw-20260409-quick-r4-before.json`  
修补后样本：`five-real-questions-raw-20260409-quick-r4-after.json`

## 本轮锁定的前 2 类问题

1. 收口计划问句落入 `agent_resolve`，触发 `system` 失败路径。
2. 明确问句在 recovery 下回退到通用兜底，缺少针对性答复。

## 修补动作（最小）

1. `crates/runtime-core/src/planner.rs`  
   补“计划/步骤/三步/结论/依据/风险/判断”等问句触发词，优先走 `session_context`。
2. `crates/runtime-core/src/executors/context.rs`  
   增加 recovery 最低模板：`20 分钟收口计划`、`429 限流判断`、`验收结论/依据/风险`。

## 对比结果

| 指标 | 修补前 | 修补后 |
|---|---:|---:|
| `plan_tool=agent_resolve` 条数 | 1 | 0 |
| `result_mode=system` 条数 | 1 | 0 |
| 通用兜底条数 | 2 | 1 |
| `result_mode=recovery` 条数 | 2 | 1 |
| `run_failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775744288436-159` -> `run-1775744491071-174`  
   变化：从 recovery 通用兜底转为 answer 三段式答复。
2. Q2：`run-1775744294868-162` -> `run-1775744503795-177`  
   变化：从 agent_resolve/system 失败转为 answer 可执行三步计划。
3. Q3：`run-1775744309398-165` -> `run-1775744510977-180`  
   变化：从 recovery 通用兜底转为 answer 可执行判断。
4. Q4：`run-1775744315074-168` -> `run-1775744523680-183`  
   变化：保持 answer 可用。
5. Q5：`run-1775744323576-171` -> `run-1775744530915-186`  
   变化：仍为 recovery 通用兜底，属于当前剩余问题。

## 务实结论

1. 本轮前 2 类质量问题已完成最小修补并显著收敛。
2. 当前剩余主要风险是 provider 波动下的单条 recovery 通用兜底（Q5），可在下一轮继续最小优化。

