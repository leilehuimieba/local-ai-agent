# 阶段性提审包（H-remediation-playbook-20260415）

更新时间：2026-04-17（冻结观察口径已收紧）  
提审类型：阶段 H 子项提审草案（H-02 低成本修复与接管指引）  
评审状态：草案（已同步到三类真实修复样本 + 一类配置自动写入最小闭环样本）

## 1. 提审范围

本次提审草案覆盖 H-02 change 工作区初始化、H02-01/H02-02 冻结，以及 `logs_not_writable`、`frontend_dist_missing`、`gateway_unreachable` 三类真实修复样本证据，以及 `config_missing_or_invalid` 的最小自动写入修复子样本；不包含更高风险配置写入修复与 Gate-H 签收。

覆盖项：

1. H-02 目标、边界、验收阈值草案。
2. H-02 最小链路设计：诊断、分类、修复/接管、校验、回放。
3. H02-01 冻结项：五类问题、自动修复白名单、人工接管模板、证据目录。
4. H02-02 冻结项：样本矩阵、自动/人工分流、优先级与证据映射。
5. H02-03/H02-06 已落的三类真实修复样本与回放证据。
6. H02-S05 配置类最小自动写入修复子样本与回放证据。
7. H-02 任务拆分、证据目录与提审占位。

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`

## 3. 核心证据

### 3.1 聚合报告

1. `D:/newwork/本地智能体/tmp/stage-h-remediation/latest.json`

### 3.2 子证据

1. `D:/newwork/本地智能体/tmp/stage-h-remediation/manual-guides/*.md`
2. `D:/newwork/本地智能体/tmp/stage-h-remediation/replay-results.json`
3. `D:/newwork/本地智能体/tmp/stage-h-remediation/manual-guide-eval.json`

### 3.3 构建/测试记录（按实际回填）

1. `go test -run 'TestRemediateLogsWritable|TestRemediateFrontendDist|TestRemediateGatewayUnreachable|TestInspectConfigRemediation|TestGenerateH02RemediationEvidence' ./internal/api`（cwd=`D:/newwork/本地智能体/gateway`）
2. `go test -run 'TestGenerateH02RemediationEvidence' ./internal/api`（已刷新 `manual-guide-eval.json`）

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| H02-01 设计冻结完整性 | = 100% | 100% | PASS | `design.md` |
| H02-02 样本矩阵完整性 | = 100% | 100% | PASS | `design.md` |
| 人工接管模板覆盖率 | = 100% | 100%（5/5） | PASS | `tmp/stage-h-remediation/manual-guides/*.md` |
| 已落真实修复样本数 | >= 3 | 3（另含 1 个配置最小自动写入闭环子样本） | PASS | `tmp/stage-h-remediation/latest.json` |
| 已落 replay 通过数 | >= 6 | 8 | PASS | `tmp/stage-h-remediation/replay-results.json` |
| 自动修复成功率 | >= 70% | 100%（3/3 当前自动修复样本） | PASS | `tmp/stage-h-remediation/latest.json` |
| 配置类最小自动写入闭环通过率 | = 100%（样本） | 100%（1/1） | PASS | `tmp/stage-h-remediation/latest.json` |
| 指引可执行性评分 | >= 4.5/5 | 4.75/5（8 条扩样） | PASS | `tmp/stage-h-remediation/manual-guide-eval.json` |

## 5. 评审结论

1. 本次提审结果：`status=warning`
2. 就绪度判定：`h02.ready=false`
3. 阻塞项统计：`p0=0, p1=0, warning=1`
4. 结论说明（必填）：
   - 当前已完成 H-02 change 草案初始化，并冻结 H02-01/H02-02 设计输入。
   - 已形成 5 个样本、5 份手动接管模板，以及 `logs_not_writable`、`frontend_dist_missing`、`gateway_unreachable` 三类真实修复样本证据。
   - 当前已有 8 条 replay 通过，其中自动修复成功 4 条、自动降级人工接管 3 条、只读校验通过 1 条。
   - 人工接管模板实测评分已扩样，当前 `guide_score_avg=4.75/5`，8 条人工接管样本均通过。
   - 当前建议不是立刻继续做高风险实现，而是先冻结“高风险配置写入边界 + 权限类最小必测验收集 + 再提审最低门槛”。
   - 上述签收级缺口与门槛现已完成冻结，且后续路径已裁决：哪些继续禁止、哪些后续可验证、哪些保持人工接管已写清。
   - 本轮进一步选定第二个且仅一个受限验证窗口：`frontend_dist_missing_build_ready_rebuild`，其原因是该场景同样位于当前用户工作区、无需管理员、失败可立即降级人工接管，且已有 build_ready 成功证据与前置不足降级证据支撑。
   - 之所以它优先于其他候选，是因为它比 `P-B` 的权限修改动作更低风险、比 `gateway_unreachable` 的进程/端口联动面更小、比配置类候选更不容易触发高风险写入耦合。
   - 其他场景本轮仍不进入：高风险配置写入继续禁止或观察，`P-C/P-D` 继续人工接管，`P-B` 虽可作为未来候选，但本轮不作为第二窗口，`gateway_unreachable` 也不作为第二窗口首选。
   - 当前对该窗口仅保留一条 `aborted_manual_takeover` 结果记录：它证明了前置条件核查已做，但没有产生新的合格受限样本，也没有形成成功闭环。
   - 中止原因是：当前可见 success 样本中的 `dist/index.html` 已存在，不满足“缺失后重建”；同时 `package.json` 未提供可直接核验的 build 脚本内容，不能在受限边界内继续扩展动作。
   - 因此，当前不足以签收的原因是：第二窗口没有带来新的成功验证结论，H-02 只能继续维持“冻结但可继续观察”的 warning；H-02 仍不能写成 ready，Gate-H 仍不能写成可签收。

## 6. 风险与回退

1. 风险：
   - H02-02 样本若选取失衡，会导致后续成功率指标失真。
   - 当前自动修复成功率样本量仍偏小，容易高估整体可用性。
2. 回退触发条件：
   - 未按已冻结样本矩阵推进，直接扩到高风险系统修复。
   - 证据目录无法稳定产出聚合报告。
3. 回退动作：
   - 退回 docs 冻结态，只保留问题分类、白名单与模板定义。
   - 暂停自动修复，仅保留人工接管模板与样本清单设计。

## 7. 后续动作

1. 若 `passed`：
   - 不适用，本提审包当前不以签收为目标。
2. 若 `warning`：
   - 责任人：`待定`
   - 追踪编号：`H02-DRAFT-001`
   - 到期时间：`2026-04-18T18:00:00+08:00`
   - 补证动作：当前已先冻结签收级缺口、后续路径与再提审门槛，并已完成首个窗口执行；第二窗口当前仅保留候选准备与一条 `aborted_manual_takeover` 观察记录。本轮到此停止，不继续追加正式窗口，也不据此回刷 Gate-H。
3. 若 `failed`：
   - 暂停 H-02 实现推进。
   - 回到阶段 H 路线重新收紧范围。

## 8. Gate 映射

1. 对应 Gate：`Gate-H`
2. 覆盖项：
   - H-02 change 工作区初始化
   - H02-01 设计冻结
   - H02-02 样本矩阵冻结
   - H02-03/H02-06 三类真实修复样本
   - H02-S05 配置类最小自动写入闭环子样本
   - H-02 任务与证据口径占位
   - 签收级缺口矩阵与再提审门槛冻结
3. 未覆盖项（如有）：
   - 更高风险配置写入的签收级验证结论
   - 更广权限类场景的签收级验证结论

## 9. 签收记录（评审后回填）

1. 评审人：`待定`
2. 评审时间：`待定`
3. 最终结论：`warning`
4. 签收备注：当前已冻结 H02-01/H02-02，并落三类真实修复样本 + 一类配置最小自动写入闭环子样本与 8 条 replay；签收级缺口与门槛虽已冻结，但不代表 H-02 已通过。
