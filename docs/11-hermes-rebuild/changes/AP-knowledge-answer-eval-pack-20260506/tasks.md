# 任务清单

- [x] 任务 1：冻结 AP 的范围与执行入口
  完成判据：`proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md` 对“只做知识回答主链回归包”保持一致。
- [x] 任务 2：收敛最小风险行为列表
  完成判据：已明确最优先覆盖的主链行为与退化信号，不再泛化成大 benchmark。
- [x] 任务 3：定义最小场景集
  完成判据：至少包含正常问答、工程判断、证据不足、边界约束四类场景，并给出固定候选问题。
- [x] 任务 4：定义评分与检查规则
  完成判据：已明确 Pass / Concern / Fail 口径，以及需要人工检查的产物和字段。
- [x] 任务 5：定义复跑入口与证据位置
  完成判据：已明确固定问题集、输出位置、复跑步骤与基线比较方式。
- [x] 任务 6：决定自动化补位路径
  完成判据：已确定“核心定向预检 + 真实入口脚本”两层回归路径，并明确参考现有脚本与报告组织方式。
- [x] 任务 7：补固定 case fixture
  完成判据：已新增固定样例文件，可直接作为后续脚本与人工复跑的唯一问题集来源。
- [x] 任务 8：实现最小真实入口回归脚本
  完成判据：新增 `scripts/run-knowledge-answer-eval-pack.ps1`，能基于固定 case 集输出 `tmp/knowledge-answer-evals/latest.json`。
- [x] 任务 9：补首轮执行证据与脚本登记
  完成判据：已补 `docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`，并把脚本写入 `scripts/README.md`；已完成首轮真实入口执行并产出 `tmp/knowledge-answer-evals/latest.json`。
- [ ] 任务 10：收敛首轮失败模式并决定下一把 change
  完成判据：已把首轮失败模式写回 `status.md` / `verify.md`，并明确下一步是新建实现 change 修正主链，而不是继续扩 AP 本身。
