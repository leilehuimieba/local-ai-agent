# 本地智能体 - 运行时 Harness 收口方案 V1

更新时间：2026-04-08

状态：`当前有效`

执行标记：

1. `运行时治理与收口入口`
2. `基于当前代码现状的 Harness 差距评审`
3. `后续 Rust / Go / 前端合同补强优先按本文收口`

关联文档：

1. [docs/README.md](D:/newwork/本地智能体/docs/README.md)
2. [本地适配架构原则_V1](D:/newwork/本地智能体/docs/02-architecture/本地适配架构原则_V1.md)
3. [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)
4. [产品级冻结与下一阶段规划入口文档_V1](D:/newwork/本地智能体/docs/06-development/产品级冻结与下一阶段规划入口文档_V1.md)
5. [关键入口文档](D:/newwork/本地智能体/docs/07-test/关键入口文档.md)

代码证据入口：

1. [lib.rs](D:/newwork/本地智能体/crates/runtime-core/src/lib.rs)
2. [query_engine.rs](D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs)
3. [context_builder.rs](D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs)
4. [tool_registry.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_registry.rs)
5. [risk.rs](D:/newwork/本地智能体/crates/runtime-core/src/risk.rs)
6. [verify.rs](D:/newwork/本地智能体/crates/runtime-core/src/verify.rs)
7. [tool_trace.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_trace.rs)
8. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs)
9. [contracts.rs](D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs)
10. [chat.go](D:/newwork/本地智能体/gateway/internal/api/chat.go)
11. [contracts.go](D:/newwork/本地智能体/gateway/internal/contracts/contracts.go)
12. [contracts.ts](D:/newwork/本地智能体/frontend/src/shared/contracts.ts)

---

## 1. 文档目的

本文只解决一件事：

> 基于当前仓库已有实现，评估运行时 Harness 哪些已经成立，哪些仍然只是“骨架存在但壳没有收紧”，并给出最小收口顺序。

这里的 `Harness` 不等于提示词。

这里说的 `Harness` 指：

1. 主循环阶段边界
2. 上下文按需装配
3. 工具协议与工具结果结构
4. 权限与确认恢复主线
5. 验证闭环
6. artifact 外置
7. 记忆与知识路由
8. 事件与三端合同
9. 长任务交接与状态重建

---

## 2. 当前 Harness 判断

当前仓库的运行时状态，不应再判断为“没有运行时主干”。

当前更准确的判断是：

1. `主循环骨架已成立`
2. `工具协议骨架已成立`
3. `确认主线已成立`
4. `事件与三端合同基本对齐`
5. `但关键 Harness 仍未完全收口`

一句话：

> 当前差距已经不在“有没有壳”，而在“这些壳是否真的按需、独立、可恢复、可审计”。

---

## 3. 已成立的壳

### 3.1 主循环已具备显式阶段

当前运行时已经不是隐式一坨逻辑。

证据：

1. [lib.rs](D:/newwork/本地智能体/crates/runtime-core/src/lib.rs) 已显式产出 `Analyze -> Plan -> Execute -> Observe -> Verify -> Finish` 阶段事件。
2. [query_engine.rs](D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs) 已把 `bootstrap_run` 和 `execute_stage` 收到单一入口。
3. `blocked`、`awaiting_confirmation`、`failed`、`finished` 都已进入统一返回结构。

结论：

1. 主循环已经具备正式收口基础。
2. 后续重点不是再发明新 loop，而是收紧每个阶段的质量。

### 3.2 工具协议和工具可见性已具备单一入口

证据：

1. [tool_registry.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_registry.rs) 已统一 `visible_tools`、`capability_specs`、`plan_tool_call`。
2. [contracts.rs](D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs) 已定义 `CapabilitySpec`、`ToolCallSnapshot`、`VerificationSnapshot`。
3. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs) 已能把工具快照和验证快照送入事件。

结论：

1. 工具不再是散落函数。
2. 后续重点是收紧工具结果真实性和验证证据，而不是重新分散协议。

### 3.3 权限确认已有恢复主线

证据：

1. [risk.rs](D:/newwork/本地智能体/crates/runtime-core/src/risk.rs) 已对首次进入工作区、高风险删除、危险命令给出确认请求。
2. [chat.go](D:/newwork/本地智能体/gateway/internal/api/chat.go) 会保存待确认请求，并在批准后携带原 `RunRequest` 与 `ConfirmationDecision` 继续执行。
3. `confirmation_required` 事件已经进入 SSE 主链路。

结论：

1. 确认不再是纯前端旁路交互。
2. “确认后恢复原主线”这一层壳已经具备基本能力。

### 3.4 事件合同已基本三端对齐

证据：

1. [contracts.rs](D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs)
2. [contracts.go](D:/newwork/本地智能体/gateway/internal/contracts/contracts.go)
3. [contracts.ts](D:/newwork/本地智能体/frontend/src/shared/contracts.ts)

当前三端已经对齐了这些关键结构：

1. `RunEvent`
2. `RuntimeContextSnapshot`
3. `ToolCallSnapshot`
4. `VerificationSnapshot`
5. `ConfirmationRequest`

结论：

1. 合同层已经具备继续治理的基础。
2. 后续重点是让字段内容更“真实有效”，而不是再改字段名。

---

## 4. 当前主要差距

以下差距按收口优先级排序。

### 4.1 上下文装配仍偏“默认全拉”，还不是严格按需装配

这是当前最明显的 Harness 薄弱层。

证据：

1. [context_builder.rs](D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs) 在每次 `build_runtime_context` 时默认执行：
   `session_summary`
   `recall_memory_digest(request, &request.user_input, 3)`
   `knowledge_hits(request)`
   `tool_preview(visible_tools)`
2. `knowledge_hits` 未命中时会回退到固定查询：`项目 智能体 本地 主干 架构 运行时`。
3. 当前上下文没有按 `PlannedAction`、阶段或工具类别做差异化装配。

问题本质：

1. 当前是“先把常见上下文都准备好”，而不是“先判断下一步真正需要什么，再按需加载”。
2. 这会让后续长任务更容易被固定噪声污染。
3. 这也会让 `memory digest` 和 `knowledge digest` 看起来存在，但不一定真的与当前动作最相关。

正式判断：

> 当前上下文层已经有了封装结构，但还没有完成 `progressive disclosure` 式收口。

### 4.2 Verify 阶段已经存在，但还不是独立验证闭环

这是第二个高优先级差距。

证据：

1. [verify.rs](D:/newwork/本地智能体/crates/runtime-core/src/verify.rs) 当前验证主要依赖：
   `trace.result.success`
   `trace.result.summary`
   `trace.result.reasoning_summary`
   `summary` 是否包含 `已执行单次恢复`
2. 当前 verify 没有拉入独立测试、合同校验、外部证据或回归结果。
3. `VerificationReport` 仍然主要是对执行结果的结构化包装，不是独立证据层。

问题本质：

1. 现在的 Verify 更像“执行后解释”，还不像“执行后审查”。
2. 生成和验证仍然高度同源，容易过度乐观。
3. 这会导致 `verification_completed` 事件存在，但 `verification` 的可信度还不够强。

正式判断：

> 当前 Verify 已经成为阶段，但还没有成为真正独立的 Harness。

### 4.3 权限闸门只覆盖少数高风险动作，普通写入边界仍偏粗

证据：

1. [risk.rs](D:/newwork/本地智能体/crates/runtime-core/src/risk.rs) 当前主要拦截：
   首次进入工作区
   `DeletePath`
   含危险关键词的 `RunCommand`
2. `WriteFile` 在 `standard` 模式下没有进一步按路径范围、覆盖行为、批量写入特征做分层确认。
3. `RunCommand` 的危险识别仍主要依赖字符串关键词。

问题本质：

1. 当前权限层已经有主线，但风险建模仍偏“少数特判”。
2. 这适合作为 `V1` 起点，不适合作为后续治理终点。

正式判断：

> 权限与确认壳已经成立，但仍停留在基础拦截层，离“收紧到产品级默认安全”还有距离。

### 4.4 Artifact 外置已经接入，但覆盖面仍偏窄

证据：

1. [tool_trace.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_trace.rs) 当前主要对 `execution.final_answer` 做 artifact 判断。
2. [artifacts.rs](D:/newwork/本地智能体/crates/runtime-core/src/artifacts.rs) 只有文本长度超过 `240` 字符才会外置。
3. 当前未把以下内容系统性外置：
   验证报告
   长日志
   交接包
   结构化中间结果

问题本质：

1. 现在 artifact 更像“长文本兜底”，还不像“主链路默认分流机制”。
2. 这会限制后续长任务交接与证据留存。

正式判断：

> Artifact 能力已接入，但还没有完成“主链路只留摘要、详细结果统一外置”的收口。

### 4.5 长任务交接包仍然缺位

这是当前最明显缺失、但尚未单独成模块的一层壳。

证据：

1. 当前代码中已有 `session summary`、`memory digest`、`knowledge digest`、`artifact path`。
2. 但尚未看到统一的 handoff artifact 结构，用来保存：
   当前计划
   已完成项
   未决风险
   下一步
   关键证据路径
3. 当前 `run_finished` 与 `verification_completed` 事件虽然携带 metadata，但还不是专门为长任务续跑设计的交接包。

问题本质：

1. 现在系统能“完成一轮”，但还不擅长“跨多轮稳定续跑”。
2. 这会在后续复杂任务中暴露上下文老化问题。

正式判断：

> 长任务 Harness 目前仍是缺位项，不应再继续后置太久。

### 4.6 Context Snapshot 合同比实际填充值更丰富

证据：

1. [contracts.rs](D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs)
2. [contracts.go](D:/newwork/本地智能体/gateway/internal/contracts/contracts.go)
3. [contracts.ts](D:/newwork/本地智能体/frontend/src/shared/contracts.ts)
4. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs) 在构造 `RuntimeContextSnapshot` 时，把：
   `prompt_static`
   `prompt_project`
   `prompt_dynamic`
   全部写成空字符串

问题本质：

1. 合同已经预留了更强的上下文证据面。
2. 但当前事件里并没有真实填充这些字段。
3. 这会造成“合同看起来很完整，但快照内容还不够真”的情况。

正式判断：

> 当前事件合同已经预留未来收口位，但快照真实性仍需补齐。

---

## 5. 收口优先级

当前建议不要平均用力，而是按下面顺序推进。

### Phase A：收紧上下文与验证

优先做：

1. 上下文按动作与阶段按需装配
2. Verify 从“执行包装”升级为“独立证据层”
3. Context Snapshot 填真实内容或收缩空字段承诺

原因：

1. 这是当前最明显的 Harness 薄弱层。
2. 不先收紧这两层，后面的 artifact 和记忆治理都会继续掺噪声。

建议落点：

1. [context_builder.rs](D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs)
2. [query_engine.rs](D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs)
3. [verify.rs](D:/newwork/本地智能体/crates/runtime-core/src/verify.rs)
4. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs)

### Phase B：收紧权限与 artifact

优先做：

1. `WriteFile` 等普通写入动作的分级确认
2. artifact 外置范围扩展到验证报告、长日志和交接包
3. 把 artifact 从“长文本兜底”提升为“主链路默认分流”

建议落点：

1. [risk.rs](D:/newwork/本地智能体/crates/runtime-core/src/risk.rs)
2. [tool_trace.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_trace.rs)
3. [artifacts.rs](D:/newwork/本地智能体/crates/runtime-core/src/artifacts.rs)
4. [chat.go](D:/newwork/本地智能体/gateway/internal/api/chat.go)

### Phase C：补长任务交接

优先做：

1. 定义 handoff artifact
2. 定义交接包写入时机
3. 定义续跑时最小状态重建入口

建议交接包最小字段：

1. `task_title`
2. `current_plan`
3. `completed_steps`
4. `open_risks`
5. `next_step`
6. `key_artifacts`
7. `verification_summary`

建议落点：

1. [lib.rs](D:/newwork/本地智能体/crates/runtime-core/src/lib.rs)
2. [artifacts.rs](D:/newwork/本地智能体/crates/runtime-core/src/artifacts.rs)
3. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs)
4. [session.rs](D:/newwork/本地智能体/crates/runtime-core/src/session.rs)

---

## 6. 当前不建议做的事

当前不要借收口之名，把范围再次扩散。

不建议现在进入：

1. 多智能体编排
2. 浏览器自动化深链路
3. 复杂自动 fallback
4. 重型记忆评分体系
5. 重新改造 Go / Rust 主分工

一句话：

> 当前最该做的是把已有壳收紧，而不是继续加新壳。

---

## 7. 建议的验收证据

这份收口方案对应的证据，不应只靠主观描述。

至少要补：

1. 一组上下文装配样本
   证明不同动作不会再默认拉同一套记忆和知识。

2. 一组验证闭环样本
   证明 verify 不再只是执行成功包装。

3. 一组权限确认恢复样本
   证明高风险动作批准后能恢复原主线，拒绝后能稳定收口。

4. 一组 artifact 证据样本
   证明长输出、验证报告或交接包会统一外置。

5. 一组三端合同消费样本
   证明 Rust、Go、前端都能稳定读取 `context_snapshot`、`tool_call_snapshot`、`verification_snapshot`。

建议回归入口：

1. [V1回归检查入口_V1.md](D:/newwork/本地智能体/docs/07-test/V1回归检查入口_V1.md)
2. [智能体框架主干总体验收文档_V1.md](D:/newwork/本地智能体/docs/07-test/智能体框架主干总体验收文档_V1.md)

---

## 8. 当前正式结论

当前运行时 Harness 的正式结论应写成：

1. 主循环已成立
2. 工具协议已成立
3. 确认主线已成立
4. 事件与合同已基本对齐
5. 当前最大差距在：
   上下文按需装配
   独立验证
   artifact 默认分流
   长任务交接
6. 当前阶段应进入 `运行时 Harness 收口`，而不是回到“主链路尚未成立”的叙事

一句话执行要求：

> 后续运行时治理的重点，不是再搭骨架，而是把已经存在的骨架收紧成真正可持续的 Harness。

---

## 9. 2026-04-08 收口进展

本轮已按既定优先级推进以下 4 项，不扩多智能体、浏览器自动化、自动 fallback 或重型记忆评分。

### 9.1 上下文按需装配

本轮已新增：

1. [context_policy.rs](D:/newwork/本地智能体/crates/runtime-core/src/context_policy.rs)
2. [context_builder.rs](D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs)
3. [query_engine.rs](D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs)

本轮落地效果：

1. 规划阶段与执行阶段已使用不同 `ContextAssemblyPolicy`。
2. `ProjectAnswer / ContextAnswer / AgentResolve / knowledge / memory / workspace` 已按动作类型选择不同上下文密度。
3. `knowledge_hits` 不再在未命中时回退到固定全局查询，减少默认噪声注入。
4. 事件中的 `context_snapshot` 已补充：
   `assembly_profile`
   `includes_session`
   `includes_memory`
   `includes_knowledge`
   `includes_tool_preview`

正式判断：

> 上下文层已从“默认全拉”向“按动作、按阶段装配”推进，收口方向已进入真实主链路。

### 9.2 Verify 独立化

本轮已新增或收紧：

1. [verify.rs](D:/newwork/本地智能体/crates/runtime-core/src/verify.rs)
2. [events.rs](D:/newwork/本地智能体/crates/runtime-core/src/events.rs)
3. [contracts.rs](D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs)
4. [contracts.go](D:/newwork/本地智能体/gateway/internal/contracts/contracts.go)
5. [contracts.ts](D:/newwork/本地智能体/frontend/src/shared/contracts.ts)

本轮落地效果：

1. `VerificationOutcome` 已新增：
   `policy`
   `evidence`
2. Verify 已按能力输出不同验证策略，例如：
   `confirm_write_effect`
   `inspect_command_result`
   `check_result_relevance`
3. `verification_snapshot` 已可回流策略与证据，而不再只回流 `passed / summary`。

正式判断：

> Verify 还不是完整外部验证系统，但已经从“执行结果包装”升级为“带策略和证据的独立验证层”。

### 9.3 artifact 默认分流

本轮已新增或收紧：

1. [artifacts.rs](D:/newwork/本地智能体/crates/runtime-core/src/artifacts.rs)
2. [tool_trace.rs](D:/newwork/本地智能体/crates/runtime-core/src/tool_trace.rs)

本轮落地效果：

1. 新增 `externalize_json_artifact`，支持结构化 artifact 外置。
2. `tool_trace` 外置内容不再只看 `final_answer`，而是覆盖：
   `action_summary`
   `result_summary`
   `final_answer`
   `reasoning_summary`
   `cache_status`
   `cache_reason`
3. 已为 handoff artifact 提供统一外置入口。

正式判断：

> Artifact 已从“长文本兜底”向“结构化默认分流”推进，但验证报告的独立 artifact 化仍可继续增强。

### 9.4 长任务交接

本轮已新增：

1. [handoff.rs](D:/newwork/本地智能体/crates/runtime-core/src/handoff.rs)
2. [session.rs](D:/newwork/本地智能体/crates/runtime-core/src/session.rs)
3. [lib.rs](D:/newwork/本地智能体/crates/runtime-core/src/lib.rs)

本轮落地效果：

1. `run_finished` 前已生成 handoff artifact。
2. 交接包已覆盖最小字段：
   `task_title`
   `current_plan`
   `completed_steps`
   `open_risks`
   `next_step`
   `key_artifacts`
   `verification_summary`
3. 会话短期状态已记录 handoff path，便于后续续跑时做最小状态重建。

正式判断：

> 长任务交接已从“缺位”进入“有默认交接包留证”的状态，但续跑时的自动重建入口仍可后续补强。

### 9.5 本轮验证证据

执行时间：

1. `2026-04-08 19:19:48`

本轮最小验证：

1. `cargo test -p runtime-core`：通过
2. `cargo build -p runtime-core`：通过
3. `go build ./...`：通过
4. `frontend/npm run build`：通过

样本与日志入口：

1. [sample-summary.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/sample-summary.json)
2. [frontend-build.log](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/frontend-build.log)
3. [context-answer-true.response.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/context-answer-true.response.json)
4. [knowledge-search.response.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/knowledge-search.response.json)
5. [workspace-write.response.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/workspace-write.response.json)
6. [workspace-write.session.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/workspace-write.session.json)
7. [handoff-1775647188172.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/handoff-1775647188172.json)
8. [write-1775647188170.txt](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/write-1775647188170.txt)

样本结论：

1. `context_answer-true.response.json` 里的 `action_requested` 事件已真实回流 `assembly_profile=context_answer`，并且 `includes_session=true / includes_memory=false / includes_knowledge=false / includes_tool_preview=false`，证明会话续推已切到按需装配。
2. `knowledge-search.response.json` 里的 `action_requested` 事件已真实回流 `assembly_profile=knowledge`，并且只保留 `includes_knowledge=true`，证明知识检索不再默认夹带会话和工具预览。
3. `workspace-write.response.json` 里的 `verification_completed` 事件已回流 `verification_snapshot.policy=confirm_write_effect`，`verification_snapshot.evidence` 已包含 `summary`、`reasoning`、`artifact`、`cache_status` 四类证据。
4. 同一条 `workspace_write` 样本已生成 `artifact_path=D:\\newwork\\本地智能体\\data\\artifacts\\harness-write-session\\harness-write-run\\write-1775647188170.txt`，证明长结果默认分流到 artifact，而不是继续压进主事件正文。
5. `run_finished` 事件与 [handoff-1775647188172.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/handoff-1775647188172.json) 已形成一组长任务交接证据，`workspace-write.session.json` 也已把 handoff 路径写回 `short_term.recent_tool_result`，证明交接包已进入默认工作流。

验证说明：

1. 本轮已补齐 Rust / Go / 前端三端构建验证，并留下单独前端构建日志。
2. 本轮样本只覆盖上下文按需装配、Verify 独立化、artifact 默认分流、长任务交接四项收口，不扩展到多智能体、浏览器自动化、自动 fallback 或重型记忆评分。
3. `project_answer` 也已留下对照样本 [context-answer.response.json](D:/newwork/本地智能体/docs/07-test/evidence/20260408-runtime-harness/context-answer.response.json)，可证明项目状态问答分支会切到 `assembly_profile=project_answer`，但本轮主证据仍以前述 4 项收口样本为准。
