# 5 条真实问句前后对比（R8：回答质量稳定性）

执行目标：只优化回答质量稳定性，不扩功能。  
修补前样本：`five-real-questions-raw-20260409-quick-r8-before.json`  
修补后样本：`five-real-questions-raw-20260409-quick-r8-after.json`

## 本轮锁定的前 2 类问题

1. “提测结论 + 两条理由”问句在 provider 波动下仍会回退到通用 recovery。
2. “今天收口先做哪三件事”问句没有稳定命中可执行模板，优先级输出不稳定。

## 修补动作（最小）

1. `crates/runtime-core/src/executors/context.rs`  
   新增 `release_check_template`，命中“提测 + 一句结论 + 两条理由”时走稳定模板。
2. `crates/runtime-core/src/executors/context.rs`  
   新增 `closeout_priority_template`，命中“今天收口 + 三件事 + 优先级”时走稳定模板。

## 对比结果

| 指标 | 修补前 | 修补后 |
|---|---:|---:|
| `plan_tool=agent_resolve` 条数 | 0 | 0 |
| `result_mode=system` 条数 | 0 | 0 |
| 通用 recovery 兜底条数 | 1 | 0 |
| 稳定模板命中条数 | 3 | 5 |
| `run_failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775747589673-282` -> `run-1775747956808-327`  
   变化：recovery 通用兜底 -> answer 稳定模板（提测结论）。
2. Q2：`run-1775747595595-285` -> `run-1775747959057-330`  
   变化：泛化答复 -> answer 稳定模板（按优先级三件事）。
3. Q3：`run-1775747603986-288` -> `run-1775747961181-333`  
   变化：保持 answer，稳定模板继续命中。
4. Q4：`run-1775747606135-291` -> `run-1775747963349-336`  
   变化：保持 answer，稳定模板继续命中。
5. Q5：`run-1775747608257-294` -> `run-1775747965470-339`  
   变化：保持 answer，稳定模板继续命中。

## 务实结论

1. 本轮把 Q1 的通用 recovery 回退清零，同时把 Q2 收紧为固定可执行格式。
2. 当前同题 5 条复测已全部稳定为 `answer`，且全部命中 `session_context`，没有扩接口和能力域。

