# CET4 端到端演示记录（T30）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`  
场景：学习 4 级备考（固定样例集）

## 1. 演示目标

1. 复现“写入 -> 召回 -> 可视化”完整链路。
2. 证明在固定业务样例下，知识链路可稳定运行并可追溯。

## 2. 输入与环境

1. 样例集：`fixtures/cet4-acceptance-cases.jsonl`（9 条）。
2. 召回评测 agent：`eval-cet4-t25`。
3. 图谱验收批次：`data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/`。

## 3. 执行步骤

1. 召回批跑：
   - 命令：`powershell -NoProfile -File scripts/run-stage-e-knowledge-recall-eval.ps1`
   - 证据：`tmp/stage-e-knowledge-recall-eval/latest.json`
2. 图谱构建：
   - 命令：`powershell -NoProfile -File scripts/build-graphify-input.ps1 -BatchDir data/exports/knowledge-markdown/20260413-190819-cet4-acceptance`
   - 证据：`data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/graphify/`
3. 图谱可视化验收快照：
   - 证据：`tmp/stage-e-knowledge-graph/latest.json`

## 4. 结果

1. 召回结果：
   - `total_cases=9`
   - `hit_cases=9`
   - `top5_hit_rate=100%`
   - `recall_p95_latency_ms=46`
2. 图谱结果：
   - `markdown_files=9`
   - `nodes=30`
   - `edges=45`
   - 多跳样例：`doc/knowledge_recall__cet4-q1-plan -> topic/cet4 -> doc/knowledge_recall__cet4-q7-adjust-next -> conclusion/cet4-q7-adjust-next`

## 5. 判定

1. CET4 业务样例链路复现成功。
2. 满足 `T30` 完成判据：学习 4 级备考场景全链路可复现且有证据留存。
