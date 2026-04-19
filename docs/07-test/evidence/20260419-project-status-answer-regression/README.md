# 项目状态问答回归证据

更新时间：2026-04-19

## 目标

固定一条“真实用户接手项目时的状态问答”回归样例，验证系统在真实入口下能够稳定回答当前主线，而不是回退到历史测试证据或旧阶段文档。

本回归只验证以下三类结果是否同时命中当前主线：

1. 最终回答 `final_answer`
2. 计划阶段知识摘要 `plan_knowledge_digest`
3. 结果与验证摘要 `result_summary` / `verification_summary`

## 固定问句

`我现在接手这个项目，请直接告诉我：当前停在什么状态、为什么不能继续默认推进、以及以后满足什么条件才值得重启。`

## 执行入口

脚本：

- `D:/newwork/本地智能体/scripts/run-project-status-answer-regression.ps1`

运行方式：

- `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-project-status-answer-regression.ps1`

## 证据产物

本轮最新产物：

- `D:/newwork/本地智能体/tmp/project-status-answer-regression/latest.json`

脚本会在独立端口上拉起隔离的 `runtime-host` 与 `gateway`，再通过真实入口：

- `POST /api/v1/chat/run`
- `GET /api/v1/logs`

完成一次端到端回归。

## 通过标准

`latest.json` 中以下检查项必须全部为 `true`：

1. `answer_hits_stage_h`
2. `answer_hits_gate_h`
3. `answer_hits_not_signoff`
4. `knowledge_hits_current_state`
5. `summary_hits_current_state`
6. `verification_hits_current_state`

同时要求：

1. `status = passed`
2. `terminal_event_type = run_finished`

## 本轮结果

本轮已通过，关键命中如下：

1. `final_answer` 已指向 `阶段 H / Gate-H / 未签收 / 不可签收`
2. `plan_knowledge_digest` 已优先指向 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
3. `result_summary` 已优先指向 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
4. `verification_summary` 已优先指向 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`

## 作用边界

这份证据只用于回归“项目状态问答”主线是否稳定命中当前权威状态源。

它不代表：

1. Gate-H 可签收
2. 阶段 H 已完成
3. H-02 或 H-03 已 ready
