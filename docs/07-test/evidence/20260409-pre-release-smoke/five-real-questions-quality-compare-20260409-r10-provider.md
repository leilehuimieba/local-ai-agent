# 5 条真实问句对比（R10：新 Provider 接入后快测）

执行目标：切换新 `base_url + api_key` 后，按同题 5 条验证回答质量是否回退。  
基线样本：`five-real-questions-raw-20260409-quick-r9-after.json`  
新 Provider 样本：`five-real-questions-raw-20260409-quick-r10-provider.json`

## 对比结果

| 指标 | R9 基线 | R10 新 Provider |
|---|---:|---:|
| `result_mode=answer` 条数 | 5 | 5 |
| `result_mode=recovery` 条数 | 0 | 0 |
| `result_mode=system` 条数 | 0 | 0 |
| 稳定模板命中条数 | 5 | 5 |
| `completion_status=failed` 条数 | 0 | 0 |

## 样本对照

1. Q1：`run-1775748582985-357` -> `run-1775750059453-381`  
   变化：保持 answer，稳定模板继续命中。
2. Q2：`run-1775748585206-360` -> `run-1775750061709-384`  
   变化：保持 answer，稳定模板继续命中。
3. Q3：`run-1775748587323-363` -> `run-1775750063860-387`  
   变化：保持 answer，稳定模板继续命中。
4. Q4：`run-1775748589468-366` -> `run-1775750066034-390`  
   变化：保持 answer，稳定模板继续命中。
5. Q5：`run-1775748591631-369` -> `run-1775750068224-393`  
   变化：保持 answer，稳定模板继续命中。

## 务实结论

1. 新 provider 接入后，当前同题 5 条未出现质量回退。  
2. 当前风险主要不在“问句覆盖”层，而在外部 provider 波动本身（需持续观察）。

