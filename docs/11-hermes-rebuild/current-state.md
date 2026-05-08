# Hermes 当前执行状态（单一事实源）

更新时间：2026-05-07
状态：`自由迭代期：BV-layout-actions-split-20260507 已完成首轮实现与验证`

本文件是 `docs/11-hermes-rebuild/` 下关于"当前阶段 / 当前 Gate / 当前活跃 change"的唯一权威记录。

## 1. 当前执行状态

1. 当前阶段：阶段 I（可持续交付与工程治理）
2. 当前 Gate：Gate-I（已收口）
3. 当前活跃 change：`BV-layout-actions-split-20260507`
4. 当前主推进目录：`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BV-layout-actions-split-20260507/`
5. 当前判定：`BV-layout-actions-split-20260507` 已完成布局 action 装配簇外提，前端定向 / 全量回归与 `npx tsc --noEmit` 通过；当前等待裁决下一刀是否继续拆 UI store 壳层热点。
6. 当前范围冻结：只做布局 action 装配簇外提与最小回归，不扩后端持久化、统计图表或完整 `store.ts` 大拆分。

## 2. 当前主推进项判断

1. `BV` 是当前唯一允许进入继续推进的主推进项。
2. `BV` 的首要目标是继续压缩 `store.ts` 热点，并把布局 action 装配簇从 `store.ts` 外提。
3. `BV` 只承担布局 action 装配簇外提与最小回归，不扩后端持久化、统计图表或完整 `store.ts` 大拆分。
4. `AY`、`AZ`、`BA`、`BB`、`BC`、`BD`、`BE`、`BF`、`BG`、`BH`、`BI`、`BJ`、`BK`、`BL`、`BM`、`BN`、`BO`、`BP`、`BQ`、`BR`、`BS`、`BT`、`BU` 已完成总控壳、证据、切主线、入口语义、个性化反馈、现实时间预算、明日计划、补交流程、主按钮优先级、连续引导、执行态、跨天重开、计划保鲜、计划接管、今日计划独立呈现、最近计划历史最小闭环与 `evidence / execution / plan-sync / switch / personalized / default-state / bridge / mainline-actions` 外提，作为 `BV` 的运行基础继续保留。

## 3. 阶段 I 收口结论

1. I-01 已完成：25 个 Rust warning 已清理。
2. I-02 已完成：全量回归脚本 `run-full-regression.ps1` 已建立，6 项检查全绿。
3. I-03 已完成：`.gitattributes` 已添加，LF/CRLF warning 已消除。
4. I-04 已完成：基线标签 `v1.0.0-20260424` 已打。
5. 阶段 I 收口，项目进入自由迭代期。

## 4. 近期完成项（自由迭代期）

1. **O-change（2026-04-26）**：六轮清理 — 前端 API 错误统一、workspaceViewModel 拆分、router_release.go 业务下沉、useSettings 抽离、Go 工具函数收拢、tmp 清理。已归档。
2. **P-change（2026-04-27）**：router.go 聚合逻辑拆分 — 提取 `settings_response.go`，router.go 782 行 → 547 行。已归档。
3. **Q-change（2026-04-27）**：CSS 功能域拆分 — `app-views.css` 1564 → 440 行，`app-components.css` 1344 → 441 行，新增 12 个功能域 CSS 文件。已归档。
4. **R-change（2026-04-27）**：产品化 MVP 治理 — 敏感信息环境变量化、LICENSE/CHANGELOG、Rust release 构建优化、前端生产配置、最小启动认证、统一构建脚本、代码质量工具。已归档。
5. **S-change（2026-05-03）**：产品化第一阶段 — Error Boundary + 全局 Toast、前端 11 项单元测试（Vitest + RTL）、用户快速入门文档、按 session 隔离 localStorage 支持历史恢复。已归档。
6. **T-change（2026-05-03）**：产品化第二阶段 — 后端结构化会话历史存储（SQLite sessions + chat_messages + REST API）、移动端适配（底部导航/Sheet/Composer 折叠）、Composer 文件上传 UI。已归档。
7. **U-change（2026-05-03）**：Assistant 消息同步到后端 — applyEvent / cancelRun 中 finalize 时异步写入 assistant 消息。已归档。
8. **V-change（2026-05-03）**：历史会话列表 UI — LogsView 增加"会话历史"标签页，展示 fetchSessions 列表，点击恢复会话。已归档。
9. **W-change（2026-05-03）**：文件上传接入后端知识库 — handleFileSelect 中异步调用 uploadKnowledgeFile，扩展 accept 为 pdf/docx，size 限制 5MB。已归档。
10. **X-change（2026-05-03）**：Playwright E2E 测试覆盖移动端 — 安装 @playwright/test，4 项移动端布局/交互测试全绿。已归档。
11. **Y-change（2026-05-04）**：短板补齐第一阶段 — MCP 添加/删除持久化、Runtime MCP 调用闭环、allowlist/risk/audit 安全治理。已收口。
12. **Z-change（2026-05-04）**：diff apply 最小闭环 — Runtime `workspace_apply_patch` 支持 dry-run、多文件、新增、删除、rename、冲突报告和失败回滚。已收口。
13. **AA-change（2026-05-04）**：Windows doctor 与一键启动体验 — Gateway 服务状态 API、一键启动器 preflight/start/verify、前端服务状态面板。已收口。
14. **AB-change（2026-05-05）**：diff apply UI 预览与确认体验 — 前端验收入口正式化，confirmation 预览优先读取 `patch_preview_report_json`，并补齐 fallback 摘要与确认写入边界。已归档。
15. **AC-change（2026-05-05）**：MCP 可观测与运营面板 — 设置页 MCP 观测面板、验收入口、只读审计 API、最近动作、按服务器联动筛选与错误码聚合。已归档。
16. **AD-change（2026-05-05）**：Runtime tool registry 对 MCP 的接入 — request-scoped MCP ToolDefinition、统一模型 tools 出口、request-scoped capability catalog 与接口级验收。已归档。
17. **AE-change（2026-05-05）**：浏览器自动化接入 Runtime 主链路 — 真实 browser MCP server 接入 Gateway / 配置侧，request-scoped browser tool spec 注入 Runtime，并补齐 `open_page -> read_page` 最小联调证据。已收口。
18. **AF-change（2026-05-05）**：浏览器交互动作与确认分层 — `click` / `type` contract、request-scoped confirmation 型 MCP spec、审批后放行链与交互 E2E。已收口。
19. **AH-change（2026-05-05）**：主循环最小升级 — `PlanEnvelope`、2 到 4 步受控循环、replan 事件与 iteration metadata。已收口。
20. **AI-change（2026-05-05）**：上下文 profile 升级 — `ask / act / repair / learn` 四类装配 profile 与注入差异。已收口。
21. **AJ-change（2026-05-05）**：记忆最小路由 — `ask / act / repair / learn` 下的读层选择、最小 digest 与 route metadata。已收口。
22. **AK-change（2026-05-05）**：知识检索增强与 knowledge pack — `混合检索 + knowledge pack + citation-ready 输出 + ask/learn 最小注入`。已收口。
23. **AL-change（2026-05-05）**：verify 矩阵最小实现 — `knowledge/file/command/memory/browser` 五类最小 verify、verification metadata、知识回答 replan、浏览器单次回读与 handoff 收口。已收口。
24. **AM-change（2026-05-05）**：浏览器高风险交互增量 — `select/submit/upload` 最小 contract、risk/confirmation、Runtime verify 与端到端联调证据。已收口。
25. **AN-change（2026-05-06）**：浏览器恢复治理细化 — 浏览器失败分型、单次回读后的停止条件、handoff artifact 与失败提示治理。已归档准备完成，待工作区收口对齐。
26. **AO-change（2026-05-06）**：记忆写回治理 — 三层写回分层、准入/拒绝规则、治理 metadata、最小留痕与全量 `runtime-core` 回归。已归档准备完成，待工作区收口对齐。
27. **AP-change（2026-05-06）**：知识回答主链回归包 — 已完成最小评测资产、真实入口脚本与首轮失败留痕；当前作为输入 / 回归 change 保留。
28. **AQ-change（2026-05-06）**：知识问答收口治理 — 已完成 `SearchKnowledge -> knowledge_answer` 最小主链修复，真实入口 `5/5` 通过；已归档准备完成，待工作区收口对齐。
29. **AR-change（2026-05-06）**：状态校准与模块化收口 — 已完成状态校准、归属矩阵、热点拆分备案、聚合回归与证据入口收口，并裁决下一最小实现项为 `AS-verify-structure-split-20260506`。
30. **AS-change（2026-05-06）**：verify 结构拆分 — 已完成 `verify.rs` 模块拆分、测试迁移与定向验证；已切换下一主推进项。
31. **AT-change（2026-05-06）**：query engine 结构拆分 — 已完成 follow-up / replan / tests 模块拆分与定向验证，`query_engine.rs` 从约 `689` 行降到 `98` 行；热点红线已解除，默认不继续第三刀 `bootstrap.rs`，已收口。
32. **AU-change（2026-05-06）**：工作区归属与收口路由 — 已完成未提交改动归属、热点备案、脚本/证据/tmp 收口入口与下一实现 change 裁决；已切换下一主推进项。
33. **AV-change（2026-05-06）**：memory 写回治理结构收口 — 已完成 `memory_router / sqlite_store / memory` 三个热点文件的结构收口与聚合验证；已收口。
34. **AW-change（2026-05-06）**：后 AV 收口路由 — 已完成状态切换、剩余未收口主簇判断与下一入口裁决。
35. **AX-change（2026-05-07）**：主线总控 Agent 产品定义、规则收口与实现切分计划。已完成文档收口。
36. **AY-change（2026-05-07）**：桌面日历插件常驻壳、收缩态 / 展开态最小入口。已完成首轮实现与测试。
37. **AZ-change（2026-05-07）**：晚间证据包最小闭环、主线总控壳入口承载与提交反馈。已完成首轮实现与测试。
38. **BA-change（2026-05-07）**：关键证据录入、48 小时未知降级、概率更新与模块级调整建议。已完成首轮实现与测试。
39. **BB-change（2026-05-07）**：临时切主线申请、复核/坚持记账与自动恢复原优先级结构。已完成首轮实现与测试。
40. **BC-change（2026-05-07）**：能力分类落地、主入口叙事切换与非核心能力降级说明。已完成首轮实现与测试。
41. **BD-change（2026-05-07）**：个性化推进反馈、ROI 跟进记录与“更适合你的推进方式”摘要卡。已完成首轮实现与测试。
42. **BE-change（2026-05-07）**：现实时间预算、今日可用时间块与主线任务接管摘要卡。已完成首轮实现与测试。
43. **BF-change（2026-05-07）**：明日计划生成、计划依据展示与恢复原主线说明。已完成首轮实现与测试。
44. **BG-change（2026-05-07）**：晚上固定查看、次日上午补交与中午后超时分流。已完成首轮实现与测试。
45. **BH-change（2026-05-07）**：下一步动作主按钮、关键证据优先入口与明日计划联动。已完成首轮实现与测试。
46. **BI-change（2026-05-07）**：动作后自动重判、连续引导提示与下一步联动。已完成首轮实现与测试。
47. **BJ-change（2026-05-07）**：执行态状态结构、主按钮区弱化非当前动作、今日核心任务完成态与今天收尾态。已完成首轮实现与测试。
48. **BK-change（2026-05-07）**：执行态跨天复位、次日自动重开与晚间闭环日期边界对齐。已完成首轮实现与测试。
49. **BL-change（2026-05-07）**：计划目标日期、证据日期归属、计划失效提示与重算判断。已完成首轮实现与测试。

## 5. 阶段 H 收口结论

1. Gate-H 已签收（H-01~H-05 全部闭环）。
2. H-memory-object-review-20260423 已提审，等待主控裁决（不切主推进）。
3. H-runtime-strict-e2e-20260424 已收口并归档。
4. H-gateway-service-extraction-20260424 已归档。
5. 总路线 A~H 全部完成，代码已全量入库。

## 6. 口径边界

1. 除本文件外，其他文档中出现的阶段或 Gate 描述，默认按"历史记录或上下文说明"处理。
2. 如 `changes/INDEX.md`、各 change 的 `status.md` 与本文件冲突，以本文件为准。
3. `docs/archive/` 下所有阶段描述默认不参与当前状态判定。
4. 阶段 H 收口不表示项目停止演进，只表示原总路线定义的范围已完成。
5. H-02/H-03 的长期治理缺口继续按风险接受条件跟踪，不纳入阶段 H 收口阻塞。

## 7. 更新规则

1. 新建或切换主推进项时，先更新本文件，再更新 `changes/INDEX.md`。
2. 进入提审或 Gate 结论变更时，先更新本文件，再更新阶段计划或阶段计划状态。
3. 每次更新本文件时，同时补“更新时间”。
