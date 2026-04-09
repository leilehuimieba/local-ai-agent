# 5 条真实问句前后对比（R9：回答质量稳定性）

执行目标：只优化回答质量稳定性，不扩功能。  
修补前样本：`five-real-questions-raw-20260409-quick-r9-before.json`  
修补后样本：`five-real-questions-raw-20260409-quick-r9-after.json`

## 本轮锁定的前 2 类问题

1. 收口问句同义写法“哪3件事”未命中稳定模板，输出格式不稳定。
2. 新手问句同义写法“小白 + 该做啥”未命中稳定模板，第一步建议不够可执行。

## 修补动作（最小）

1. `crates/runtime-core/src/executors/context.rs`  
   在 `closeout_priority_template` 补 `3件事` 同义匹配。
2. `crates/runtime-core/src/executors/context.rs`  
   新增 `instability_decision_template` 与 `beginner_first_step_template`，补“暂停/继续判断线”“小白第一步”同义问法稳定命中。

## 对比结果

| 指标 | 修补前 | 修补后 |
|---|---:|---:|
| `plan_tool=agent_resolve` 条数 | 0 | 0 |
| `result_mode=system` 条数 | 0 | 0 |
| 通用 recovery 兜底条数 | 0 | 0 |
| 稳定模板命中条数 | 2 | 5 |
| `run_failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775748403640-342` -> `run-1775748582985-357`  
   变化：保持 answer，稳定模板继续命中。
2. Q2：`run-1775748405891-345` -> `run-1775748585206-360`  
   变化：普通模型答复 -> answer 稳定模板（按优先级三件事）。
3. Q3：`run-1775748414462-348` -> `run-1775748587323-363`  
   变化：普通模型答复 -> answer 稳定模板（暂停/继续判断线）。
4. Q4：`run-1775748428840-351` -> `run-1775748589468-366`  
   变化：保持 answer，稳定模板继续命中。
5. Q5：`run-1775748430983-354` -> `run-1775748591631-369`  
   变化：普通模型答复 -> answer 稳定模板（小白第一步）。

## 务实结论

1. 本轮把“同义问法模板覆盖边界”进一步收紧，5 条同题复测全部命中稳定模板。
2. 当前这一组样本下，回答质量波动已从“问法差异引起的不稳定”收敛到“外部 provider 波动持续观察”。

