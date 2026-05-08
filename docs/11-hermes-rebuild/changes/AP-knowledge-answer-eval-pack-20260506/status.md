# 当前状态

- 最近更新时间：2026-05-06
- 状态：进行中
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
- 已完成：已新建独立 change `AP-knowledge-answer-eval-pack-20260506`。
- 已完成：已冻结本 change 只覆盖知识回答主链回归包，不回 `AK`、`AL`、`AO` 混做实现。
- 已确认：当前优先级不是继续扩主链行为，而是先固定最小可复跑评测资产。
- 已完成：最小风险行为、固定问题集、评分规则、inspect 信号与输出位置已收敛。
- 已完成：已补固定样例文件 `fixtures/knowledge-answer-cases.json`。
- 已确认：自动化补位路径采用“两层回归”——先跑 `runtime-core` 定向预检，再补真实入口脚本 `scripts/run-knowledge-answer-eval-pack.ps1`。
- 已完成：已新增真实入口脚本 `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`。
- 已完成：已新增证据说明 `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`，并已登记到 `D:/newwork/本地智能体/scripts/README.md`。
- 已完成：预检已通过：`cargo test -p runtime-core verify -- --nocapture`、`cargo test -p runtime-core run_verification_metadata -- --nocapture`。
- 已完成：首轮真实入口回归已执行，产物为 `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`。
- 已确认：首轮结果失败，`5` 题中仅 `1` 题通过；`KA-01 ~ KA-04` 全部失败，`KA-05` 低证据收口通过。
- 已确认：当前失败模式不是脚本或鉴权问题，而是主链行为问题：正常知识问答仍落到 `verification_task_type = generic`，输出是“本地知识检索结果”列表，而不是 `knowledge_answer + citation-ready + 事实/推断/建议` 的回答收口。
- 当前进行中：把首轮失败模式沉淀为下一把实现 change 的输入。
- 阻塞点：暂无脚本阻塞；当前阻塞转为主链行为不满足 AP 评测口径。
- 下一步：不要继续扩 AP；应新建一把独立实现 change，专门修正“知识问答从 SearchKnowledge 收口到 citation-ready answer”的主链行为。
