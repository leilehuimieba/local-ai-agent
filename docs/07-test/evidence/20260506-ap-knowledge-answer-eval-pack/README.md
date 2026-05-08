# 知识回答主链回归包证据

更新时间：2026-05-06

## 目标

固定一组最小但有杀伤力的知识回答问题，验证以下能力不会退化：

1. 回答能同时给出结论与 citation；
2. `事实 / 推断 / 建议` 边界仍然可见；
3. verification metadata 不丢失；
4. 低证据问题不会被硬编成看似可信的答案。

## 固定样例来源

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/fixtures/knowledge-answer-cases.json`

## 执行入口

### 预检

1. `cargo test -p runtime-core verify -- --nocapture`
2. `cargo test -p runtime-core run_verification_metadata -- --nocapture`

### 真实入口回归

- `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

## 证据产物

- 主报告：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
- 历史报告：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/<run-id>.json`

## 通过标准

`latest.json` 中至少满足：

1. `status = passed`
2. `checks.all_cases_passed = true`
3. 正常知识问答与工程判断题命中：
   - `verification_task_type = knowledge_answer`
   - `verification_has_citation = true`
   - `capability_risk_checked` 与 `permission_boundary_respected` 可见
   - `has_boundary = true`
4. 低证据问题命中：
   - `replan_requested = true`
   - 或 `handoff_visible = true`
   - 或 `low_evidence_visible = true`
   - 或终态为 `run_failed`

## 当前状态

1. 本轮已补脚本入口、固定 fixture 与证据目录。
2. 首轮真实入口执行结果以下一轮实际跑出的 `latest.json` 为准。
