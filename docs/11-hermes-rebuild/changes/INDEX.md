# Hermes Change 索引

更新时间：2026-05-07

这个文件用于提供 change 目录导航。
“当前阶段 / 当前 Gate / 当前活跃 change”的状态统一以 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 为准。

## 当前活跃 change

1. [BV-layout-actions-split-20260507](BV-layout-actions-split-20260507/) — P1：布局 action 装配簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证

## 已有回归 / 输入 change

1. [AP-knowledge-answer-eval-pack-20260506](AP-knowledge-answer-eval-pack-20260506/) — P1：知识回答主链回归包，已完成真实入口回归资产与首轮失败模式沉淀

## Blueprint / 参考入口

1. [AG-agent-loop-memory-knowledge-20260505](AG-agent-loop-memory-knowledge-20260505/) — Blueprint：Agent 回路、记忆路由与知识检索优化蓝图

## 已完成实现入口（五刀）

1. [AH-agent-loop-minimal-upgrade-20260505](AH-agent-loop-minimal-upgrade-20260505/) — P1：第一刀主循环最小升级实现 change，仅覆盖 `PlanEnvelope + 小步回路 + replan 事件`
2. [AI-context-profile-upgrade-20260505](AI-context-profile-upgrade-20260505/) — P1：第二刀上下文 profile 实现 change，仅覆盖 `ask / act / repair / learn`
3. [AJ-memory-minimal-routing-20260505](AJ-memory-minimal-routing-20260505/) — P1：第三刀记忆最小路由实现 change，仅覆盖 `何时读 / 读哪层 / 最小 digest / route metadata`
4. [AK-knowledge-pack-minimal-20260505](AK-knowledge-pack-minimal-20260505/) — P1：第四刀知识检索增强实现 change，仅覆盖 `混合检索 + knowledge pack + citation-ready 输出`
5. [AL-verify-matrix-minimal-20260505](AL-verify-matrix-minimal-20260505/) — P1：第五刀 verify 矩阵实现 change，仅覆盖 `知识回答主链优先验证 + verification metadata + replan/handoff 收口`

## 当前结构治理 / 收口链路

1. [AR-state-realignment-and-modularity-closeout-20260506](AR-state-realignment-and-modularity-closeout-20260506/) — P1：状态校准、未归属改动归口、热点文件拆分备案、聚合回归与证据入口收口；已收口
2. [AX-mainline-control-agent-20260507](AX-mainline-control-agent-20260507/) — P1：主线总控 Agent 产品定义、迁移边界、插件信息架构、证据规则与实现切分；已完成文档收口
3. [AY-desktop-calendar-shell-20260507](AY-desktop-calendar-shell-20260507/) — P1：桌面日历插件常驻壳、收缩态 / 展开态最小入口；已完成第一轮实现与测试
4. [AZ-evidence-packet-loop-20260507](AZ-evidence-packet-loop-20260507/) — P1：晚间证据包最小闭环、主线总控壳入口承载与提交反馈；已完成第一轮实现与测试
5. [BA-key-evidence-and-probability-20260507](BA-key-evidence-and-probability-20260507/) — P1：关键证据录入、48 小时未知降级、概率更新与模块级调整建议；已完成第一轮实现与测试
6. [BB-temporary-mainline-switch-20260507](BB-temporary-mainline-switch-20260507/) — P1：临时切主线申请、复核/坚持记账与自动恢复原优先级结构；已完成第一轮实现与测试
7. [BC-capability-classification-and-entry-degrade-20260507](BC-capability-classification-and-entry-degrade-20260507/) — P1：能力分类落地、主入口叙事切换与非核心能力降级说明；已完成第一轮实现与测试
8. [BD-personalized-roi-followup-20260507](BD-personalized-roi-followup-20260507/) — P1：个性化推进反馈、ROI 跟进记录与“更适合你的推进方式”摘要卡；已完成第一轮实现与测试
9. [BE-time-budget-and-calendar-blocks-20260507](BE-time-budget-and-calendar-blocks-20260507/) — P1：现实时间预算、今日可用时间块与主线任务接管摘要卡；已完成第一轮实现与测试
10. [BF-next-day-plan-and-restore-20260507](BF-next-day-plan-and-restore-20260507/) — P1：明日计划生成、次日恢复原主线与计划依据摘要卡；已完成第一轮实现与测试
11. [BG-evening-review-and-late-evidence-branch-20260507](BG-evening-review-and-late-evidence-branch-20260507/) — P1：晚上固定查看、次日中午前补交与超时分流；已完成第一轮实现与测试
12. [BH-next-step-entry-priority-20260507](BH-next-step-entry-priority-20260507/) — P1：下一步动作主按钮、关键证据优先入口与明日计划联动；已完成第一轮实现与测试
13. [BI-action-followthrough-loop-20260507](BI-action-followthrough-loop-20260507/) — P1：动作后自动重判、连续引导提示与下一步联动；已完成第一轮实现与测试
14. [BJ-execution-state-tightening-20260507](BJ-execution-state-tightening-20260507/) — P1：执行态状态结构、主按钮区弱化非当前动作与今日完成态回写；已完成第一轮实现与测试
15. [BK-cross-day-reopen-alignment-20260507](BK-cross-day-reopen-alignment-20260507/) — P1：执行态跨天复位、次日自动重开与晚间闭环日期边界对齐；已完成第一轮实现与测试
16. [BL-plan-freshness-and-recalc-20260507](BL-plan-freshness-and-recalc-20260507/) — P1：计划目标日期、证据日期归属、计划失效提示与重算判断；已完成第一轮实现与测试
17. [BM-today-plan-takeover-and-writeback-20260507](BM-today-plan-takeover-and-writeback-20260507/) — P1：计划目标日接管今天、时间预算承接与执行结果回写；进行中
18. [BN-today-plan-card-and-evidence-reconciliation-20260507](BN-today-plan-card-and-evidence-reconciliation-20260507/) — P1：今日计划卡独立呈现、晚间证据对账与最小热点拆分；进行中
19. [BO-plan-history-and-store-split-20260507](BO-plan-history-and-store-split-20260507/) — P1：计划历史最小闭环与 store 计划簇拆分第一步；进行中
20. [BP-evidence-execution-split-and-history-refine-20260507](BP-evidence-execution-split-and-history-refine-20260507/) — P1：evidence / execution 子模块外提与最近计划历史对账细化；已完成首轮实现与验证
21. [BQ-plan-sync-and-takeover-split-20260507](BQ-plan-sync-and-takeover-split-20260507/) — P1：计划同步 / 今日接管簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
22. [BR-switch-and-personalized-split-20260507](BR-switch-and-personalized-split-20260507/) — P1：`switch / personalized` 簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
23. [BS-mainline-default-state-split-20260507](BS-mainline-default-state-split-20260507/) — P1：主线默认状态装配簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
24. [BT-mainline-bridge-split-20260507](BT-mainline-bridge-split-20260507/) — P1：主线桥接簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
25. [BU-mainline-actions-split-20260507](BU-mainline-actions-split-20260507/) — P1：主线 action 装配簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
26. [BV-layout-actions-split-20260507](BV-layout-actions-split-20260507/) — P1：布局 action 装配簇外提与 `store.ts` 热点继续压缩；已完成首轮实现与验证
2. [AS-verify-structure-split-20260506](AS-verify-structure-split-20260506/) — P1：`verify.rs` 结构拆分已完成实现与验证；已收口
3. [AT-query-engine-structure-split-20260506](AT-query-engine-structure-split-20260506/) — P1：`query_engine.rs` 已完成 follow-up / replan / tests 模块拆分与定向验证；默认不继续第三刀 `bootstrap.rs`，已收口
4. [AU-worktree-ownership-and-closeout-routing-20260506](AU-worktree-ownership-and-closeout-routing-20260506/) — P1：工作区未提交改动归属、热点备案、脚本/证据/tmp 收口入口与下一实现 change 裁决；已收口
5. [AV-memory-writeback-structure-closeout-20260506](AV-memory-writeback-structure-closeout-20260506/) — P1：memory 写回治理簇已完成结构收口、热点解除与聚合验证；已收口
6. [AW-post-av-closeout-routing-20260506](AW-post-av-closeout-routing-20260506/) — P1：当前主推进项，仅覆盖后 AV 收口路由、状态切换与下一入口裁决；已收口

## 已完成 change

1. [AF-browser-interaction-confirmation-20260505](AF-browser-interaction-confirmation-20260505/) — P1：浏览器交互动作与确认分层
2. [AE-browser-automation-runtime-20260505](AE-browser-automation-runtime-20260505/) — P1：浏览器自动化接入 Runtime 主链路
3. [AM-browser-risky-interaction-20260505](AM-browser-risky-interaction-20260505/) — P1：浏览器高风险交互增量
4. [AN-browser-recovery-governance-20260505](archive/2026-05-06/AN-browser-recovery-governance-20260505/) — P1：浏览器恢复治理细化
5. [AO-memory-writeback-governance-20260506](archive/2026-05-06/AO-memory-writeback-governance-20260506/) — P1：记忆写回治理，已完成实现、验证与签收并进入归档准备
6. [AQ-knowledge-answer-closure-20260506](archive/2026-05-06/AQ-knowledge-answer-closure-20260506/) — P1：知识问答从 SearchKnowledge 列表态收口到 citation-ready knowledge_answer 回答态，已完成实现、验证与签收并进入归档准备
7. [AL-verify-matrix-minimal-20260505](AL-verify-matrix-minimal-20260505/) — P1：第五刀 verify 矩阵实现 change
8. [AD-runtime-mcp-tool-registry-20260505](archive/2026-05-05/AD-runtime-mcp-tool-registry-20260505/) — P0：Runtime tool registry 对 MCP 的主链路接入
9. [AC-mcp-observability-ui-20260505](archive/2026-05-05/AC-mcp-observability-ui-20260505/) — P0：MCP 可观测与运营面板
10. [AB-diff-preview-ui-20260504](archive/2026-05-05/AB-diff-preview-ui-20260504/) — P1：diff apply UI 预览与确认体验
11. [AA-windows-doctor-launcher-20260504](AA-windows-doctor-launcher-20260504/) — P1：Windows doctor、一键启动器与服务状态自检
12. [Z-diff-apply-20260504](Z-diff-apply-20260504/) — P1：Aider/Codex 风格代码 diff apply 最小闭环
13. [Y-shortcomings-mcp-first-20260504](Y-shortcomings-mcp-first-20260504/) — 短板补齐第一阶段：MCP 添加/删除持久化、Runtime MCP 调用闭环与安全治理
14. [S-productization-phase1-20260502](S-productization-phase1-20260502/) — 产品化第一阶段：Error Boundary + Toast + 前端测试 + 用户文档 + Session 隔离
15. [T-productization-phase2-20260503](T-productization-phase2-20260503/) — 产品化第二阶段：后端结构化会话历史存储 + 移动端适配 + Composer 文件上传 UI
16. [U-assistant-msg-sync-20260503](U-assistant-msg-sync-20260503/) — Assistant 消息同步到后端
17. [V-history-session-list-20260503](V-history-session-list-20260503/) — 历史会话列表 UI
18. [W-knowledge-upload-backend-20260503](W-knowledge-upload-backend-20260503/) — 文件上传接入后端知识库
19. [X-playwright-mobile-e2e-20260503](X-playwright-mobile-e2e-20260503/) — Playwright E2E 测试覆盖移动端

## 归档入口

1. [archive/2026-04-27/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-27/INDEX.md)（P / Q / R 产品化治理与模块化收口归档入口）
2. [archive/2026-04-26/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-26/INDEX.md)（D~N 阶段收口项与 H-02 / H-03 保留观察归档入口，共 26 项）
3. [archive/2026-04-24/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-24/INDEX.md)（H-modularity-hardening、H-gateway-service-extraction 归档入口）
4. [archive/2026-04-23/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-23/INDEX.md)（前端重新设计、前端工作台重构归档入口）
5. [archive/2026-04-15/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-15/INDEX.md)（阶段 G 已收口项与 `F-memory-progressive-disclosure-20260414` 归档入口）
6. [archive/2026-04-14/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-14/INDEX.md)（`E-claudecode-shell-alignment`、`E-sensitive-pattern-expansion` 归档入口）
7. [archive/2026-04-13/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-13/INDEX.md)（本轮文档治理收口归档）
8. [archive/2026-04-12/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-12/INDEX.md)（更早已收口 change 归档）
9. [archive/2026-05-06/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/INDEX.md)（AN / AO / AQ 归档入口）

## 选择规则

1. 继续任务时，先读 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`，再定位对应 change 目录。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果 `INDEX.md` 或任意 `status.md` 与 `current-state.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等及以上变更后，将其加入索引。
2. 切换主推进项时，先更新 `current-state.md`，再更新本索引。
3. 某个 change 完成并收口后，移动到 `archive/<日期>/` 并补归档索引。
