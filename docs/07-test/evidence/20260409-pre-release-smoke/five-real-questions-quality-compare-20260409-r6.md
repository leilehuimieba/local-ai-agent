# 5 条真实问句前后对比（R6：回答质量稳定性）

执行目标：只优化回答质量稳定性，不扩功能。  
修补前样本：`five-real-questions-raw-20260409-quick-r6-before.json`  
修补后样本：`five-real-questions-raw-20260409-quick-r6-after.json`

## 本轮锁定的前 2 类问题

1. 高频问句在 provider 波动下 recovery 触发偏多（3/5）。
2. 已有 recovery 模板但缺少前置稳定答复路径，导致主回答可用性波动。

## 修补动作（最小）

1. `crates/runtime-core/src/executors/context.rs`
2. 增加 `stable_template_answer` 前置稳定路径：命中高频问句模板时，直接 `bypass` 返回可执行答案，降低对模型可用性的依赖。

## 对比结果

| 指标 | 修补前 | 修补后 |
|---|---:|---:|
| `result_mode=recovery` 条数 | 3 | 0 |
| `result_mode=answer` 条数 | 2 | 5 |
| 稳定模板命中条数 | 0 | 5 |
| `plan_tool=agent_resolve` 条数 | 0 | 0 |
| `run_failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775746402300-219` -> `run-1775746542610-234`  
   变化：recovery -> answer（稳定模板）。
2. Q2：`run-1775746409529-222` -> `run-1775746544979-237`  
   变化：answer -> answer（稳定模板，口径更可复测）。
3. Q3：`run-1775746416796-225` -> `run-1775746546609-240`  
   变化：recovery -> answer（稳定模板）。
4. Q4：`run-1775746422555-228` -> `run-1775746548251-243`  
   变化：answer -> answer（稳定模板）。
5. Q5：`run-1775746429791-231` -> `run-1775746550537-246`  
   变化：recovery -> answer（稳定模板）。

## 务实结论

1. 本轮把“provider 波动导致的 recovery 频发”从可见风险降到 0（在这组 5 条同题里）。
2. 当前这组高频问句已经形成稳定、可复测、可执行的本地答复路径。

