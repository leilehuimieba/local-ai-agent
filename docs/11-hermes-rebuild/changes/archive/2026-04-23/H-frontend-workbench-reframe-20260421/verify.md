# 验证记录

## 验证方式

- 文档验证：
  1. 核对本 change 五件套是否齐备。
  2. 核对 `changes/INDEX.md` 是否已新增本 change，并明确其为并行 change，而不是当前主推进。
  3. 核对 `design.md` 是否明确写出“现状模块 -> 目标模块”映射，以及 Shell / 内容块的两层收口策略。
  4. 核对 `wave1-实施入口.md` 与 `wave1-文件级实施计划.md` 是否已把 Wave 1 落到可执行文件清单。
  5. 核对 `wave2-实施入口.md` 是否已明确 Wave 2 的目标、边界、风险与最小验收。
- 方案验证：
  1. 检查是否已明确本轮不改写后端 contract、不引入大型 UI 框架。
  2. 检查是否已明确 Wave 1 / Wave 2 / Wave 3 的顺序，而不是一次性全量重写。
- 实施前验证：
  1. 信息架构验证：用户是否可以更快识别导航、任务流、事件流、调查层、日志与设置入口。
  2. 一致性验证：消息块、工具块、状态块、日志块是否进入同一套视觉语义。
  3. 认知负担验证：新用户是否更容易理解“输入 -> 执行 -> 反馈 -> 追踪”的连续流程。
  4. 扩展性验证：新增一个 agent 模块时，是否能挂入现有 shell，而不用重新设计页面主结构。
- 证据建议：
  1. Wave 1 完成后补前后对比截图。
  2. Wave 2 完成后补典型任务 walkthrough。
  3. Wave 3 完成后补多视图一致性检查清单。

## 证据位置

- 当前文档证据：
  1. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/proposal.md`
  2. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/design.md`
  3. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/tasks.md`
  4. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/status.md`
  5. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/verify.md`
  6. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave1-实施入口.md`
  7. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave1-深色-token-草案.md`
  8. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave1-文件级实施计划.md`
  9. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave2-实施入口.md`
  10. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave2-收口评估.md`
  11. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-最小入口.md`
  12. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-文件级实施计划.md`
  13. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-收口评估.md`
  14. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-多视图一致性检查清单.md`
  15. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-截图补证入口.md`
  16. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-保守收口口径.md`
  17. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/当前阶段总整理.md`
  18. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/wave3-收口交接说明.md`
- 实现证据：
  1. `frontend/src/styles/tokens.css`
  2. `frontend/src/styles/base.css`
  3. `frontend/src/index.css`
  4. `frontend/src/shell/AppShell.tsx`
  5. `frontend/src/workspace/TopBar.tsx`
  6. `frontend/src/workspace/ContextSidebar.tsx`
  7. `frontend/src/workspace/BottomPanel.tsx`
  8. `frontend/src/chat/ChatPanel.tsx`
  9. `frontend/src/events/EventTimeline.tsx`
  10. `frontend/src/ui/primitives/InfoCard.tsx`
  11. `frontend/src/ui/primitives/index.ts`
  12. `frontend/src/index.css`
  13. `frontend/src/history/components/HistoryPageSections.tsx`
  14. `frontend/src/history/components/HistorySpotlight.tsx`
  15. `frontend/src/history/components/HistoryTimelineSection.tsx`
  16. `frontend/src/settings/SettingsPanel.tsx`
  17. `frontend/src/settings/StatusCard.tsx`
  18. `frontend/src/settings/SettingsSections.tsx`
  19. `frontend/src/resources/components/MemoryResourcesSection.tsx`
  20. `frontend/src/resources/components/ResourcesEntrySection.tsx`
  21. `frontend/src/history/components/HistoryDetailRail.tsx`
- 现状扫描依据：
  1. `frontend/package.json`
  2. `frontend/src/chat/ChatPanel.tsx`
  3. `frontend/src/events/EventTimeline.tsx`
  4. `frontend/src/logs/LogsPanel.tsx`
- 构建验证：
  1. `cd frontend && npm run build`（通过，2026-04-21）
  2. `cd frontend && npm run build`（通过，2026-04-22，Wave 2 第四轮结果块与事件 tone 收口后复验）
  3. `cd frontend && npm run build`（通过，2026-04-22，Wave 2 第五轮减噪收口后复验）
  4. `cd frontend && npm run build`（通过，2026-04-22，Wave 2 第六轮微调与收口评估后复验）
  5. `cd frontend && npm run build`（通过，2026-04-22，Wave 3 Round 1 LogsPanel 最小收口实现后复验）
  6. `cd frontend && npm run build`（通过，2026-04-22，Wave 3 Round 1 history/components 最小收口实现后复验）
  7. `cd frontend && npm run build`（通过，2026-04-22，Wave 3 Round 2 settings/* 最小收口实现后复验）
  8. `cd frontend && npm run build`（通过，2026-04-22，Wave 3 Round 2 resources/* 最小收口实现后复验）
  9. `cd frontend && npm run build`（通过，2026-04-22，HistoryDetailRail.tsx 外层语义收口后复验）
- 页面验证准备：
  1. 已启动本地预览：`cd frontend && npm run preview -- --host 127.0.0.1 --port 4173`（通过，2026-04-21）。
  2. 证据目录已建立：`docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/`。
  3. 本轮已通过本地一次性自动化脚本补齐 Wave 3 页面截图，未改动前端实现代码。
  4. 已补运行环境与补证阻塞复核记录：`docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-screenshot-readiness-20260422.txt`。

- 页面截图证据：
  1. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave1-home-20260422.png`
  2. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave1-task-20260422.png`
  3. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave1-investigation-open-20260422.png`
  4. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-logs-workspace-20260422.png`
  5. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-history-review-20260422.png`
  6. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-settings-workspace-20260422.png`
  7. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-settings-resources-20260422.png`
- 设置加载恢复证据：
  1. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/settings-response-20260422.json`
  2. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/settings-health-20260422.txt`
- 最小 walkthrough 证据：
  1. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave2-walkthrough-20260422.txt`
  2. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave2-walkthrough-logs-20260422.json`
  3. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-walkthrough-20260422.txt`
  4. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-walkthrough-20260422.json`
- 补证准备记录：
  1. `docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/wave3-screenshot-readiness-20260422.txt`
- 页面验证结论：
  1. 首页深色壳层、顶栏、侧栏已可见。
  2. 任务页壳层可达，任务工具条、主线程区、侧栏、调查轨道可见。
  3. 通过 `scripts/start-dev.ps1` 拉起本地 `gateway/runtime` 后，`/api/v1/settings` 已恢复为 `200`，此前 `502` 阻塞已解除。
  4. 已通过真实 `POST /api/v1/chat/run` + `GET /api/v1/logs` 补最小 walkthrough 证据，当前可证明任务提交与收口链路可用。
  5. 当前浏览器自动化会话被现有 DevTools 浏览器会话占用，因此本轮 walkthrough 以真实接口链路和运行日志为主，页面级任务截图待会话恢复后补充。
  6. 当前 `ChatPanel.tsx` 已补主线程 meta 与 composer meta，`EventTimeline.tsx` 已补 primary detail 与 tag 聚合，Wave 2 内容块层级开始形成统一语义。
  7. 当前 `ChatPanel.tsx` 已统一确认块/状态块语义，`EventTimeline.tsx` 已补 detail 行层级，Wave 2 内容块从“共享原语接入”推进到“状态与细节表达收口”。
  8. 当前 `ChatPanel.tsx` 已区分 answer / recovery / system 结果块语义，`EventTimeline.tsx` 已统一 latest / selected / tone 的视觉表达，Wave 2 已进入第四轮结果块与事件 tone 收口。
  9. 当前 `ChatPanel.tsx` 已压缩 detail 类辅助结果块噪音，`EventTimeline.tsx` 已收紧 tag 数量与 detail 密度，Wave 2 已进入第五轮减噪收口。
  10. 当前 `ChatPanel.tsx` 已进一步弱化辅助说明块，`EventTimeline.tsx` 已将 tag 收紧到 3 个以内、detail 收紧到 4 条以内，代码侧已基本具备 Wave 2 收口条件，当前剩余缺口主要是页面级 walkthrough 证据。
  11. 已新增 `wave2-收口评估.md` 与 `wave3-最小入口.md`，当前已能明确说明：Wave 2 在代码侧基本可收，Wave 3 可按既定边界启动，但正式切换前仍建议补页面级 walkthrough 证据。
  12. 已新增 `wave3-文件级实施计划.md`，当前已明确 Wave 3 的第一阶段应从 `LogsPanel.tsx + history/components/*` 开始，第二阶段再进入 `settings/* + resources/*`。
  13. 当前 `LogsPanel.tsx` 已补 workbench hero 与统一区块容器，Wave 3 Round 1 已开始把 Logs 页纳入统一工作台语言。
  14. 当前 `HistoryPageSections.tsx / HistorySpotlight.tsx / HistoryTimelineSection.tsx` 已完成最小表达层收口：筛选区、焦点卡和时间线头部开始共享 Logs Workspace 语义，timeline tag 已进一步收紧为最多 3 个标签加时间戳。
  15. 当前 `SettingsPanel.tsx / StatusCard.tsx / SettingsSections.tsx` 已完成最小壳层收口：Settings 页已补 Workspace hero、运行态说明与模块级标题语义，页面级截图也已补齐，当前剩余缺口转为直观 walkthrough 证据。
  16. 当前 `MemoryResourcesSection.tsx / ResourcesEntrySection.tsx` 已完成最小模块收口：资源区已补工作区说明、头部描述与列表区说明，记忆模块开始和 Settings / Logs 共享同一套工作台语义，页面级截图已补齐，当前剩余缺口同样转为 walkthrough 证据。
  17. 当前 `HistoryDetailRail.tsx` 已完成最小外层语义收口：详情栏已补 `Detail / 复盘详情栏` 头部说明，空状态文案更明确，detail sections 的生成逻辑保持原样；页面截图已能覆盖其与筛选台、焦点卡、稳定记录流的组合关系。
  18. 已新增 `wave3-收口评估.md`，当前已能明确说明：Wave 3 在代码侧已基本具备阶段性收口条件；现阶段页面截图与最小 walkthrough 已补齐，但仍不建议直接写成“Wave 3 已全部完成”，因为本并行 change 仍需按保守口径等待收口裁决。
  19. 已新增 `wave3-多视图一致性检查清单.md`，当前已能按 header、panel、status、empty-state 与资源模块边界五类维度检查 Wave 3 的跨视图一致性；当前保守结论可收敛为“代码侧基本通过，页面证据已显著补齐”。
  20. 已新增 `wave3-截图补证入口.md`，当前已能明确说明 Logs / History / Settings / Resources 的真实截图入口、推荐文件名与回填步骤；本轮已按该入口补入页面截图文件。
  21. 已新增 `wave3-保守收口口径.md`，当前已能明确说明：即使截图与最小 walkthrough 已补齐，Wave 3 仍应优先使用保守收口表述，不直接写成“已全部完成”。
  22. 已新增 `当前阶段总整理.md`，当前已能用单文档收拢这条并行 change 的实现、验证、缺口与推荐口径，后续新开对话或交接时可直接作为恢复上下文入口。
  23. 已于 2026-04-22 复核本地预览与 settings 接口健康，`4173` 与 `8897` 相关入口均返回 `200`；截图补齐前的直接阻塞是 DevTools 浏览器 profile 被占用，而不是前端或 runtime 异常，本轮已绕开该阻塞补齐证据。
  24. 结合 `当前阶段总整理.md`、`wave3-收口评估.md` 与 `wave3-保守收口口径.md`，主控当前可将本 change 的稳定结论收敛为：**Wave 3 代码侧已基本收口，页面证据已显著补齐，本并行 change 已具备可结束条件。**
  25. 已于 2026-04-22 补齐 Wave 3 页面级截图证据：Logs workspace、History / Review 视图区、Settings workspace、Settings 内 Resources 模块均已有截图落盘。
  26. 其中 `wave3-history-review-20260422.png` 已覆盖筛选台、焦点复盘卡、稳定记录流与复盘详情栏；`wave3-settings-resources-20260422.png` 已明确 Resources 是挂在 Settings 内的模块，而不是独立页面。
  27. 已于 2026-04-22 补齐 Wave 3 最小 walkthrough 证据：当前已能按 Logs → History / Review → Settings → Resources 的真实阅读路径回放页面结构与关键文案。
  28. `wave3-walkthrough-20260422.txt` 与 `wave3-walkthrough-20260422.json` 已明确记录两项结构事实：History / Review 挂在 Logs / Review Workspace 内，Resources 挂在 Settings 的“记忆与资源”模块内。
  29. 当前页面级截图与最小 walkthrough 已具备，Wave 3 的页面证据已显著补齐；主控当前判断可将本并行 change 标记为“可结束（并行）”，但仍不直接写成“Wave 3 已全部完成”。
  30. 已新增 `wave3-收口交接说明.md`，当前已能用最小文稿向主控侧或后续执行侧交代：真实状态、证据边界、结构事实、推荐口径与不应误写的表述。
  31. 当前主控裁决边界已明确：**可结束（并行）**、**不切主推进**、**不修改 `current-state.md`**、**不把本 change 上升表述为全项目前端已全部验收**。

- 主推进状态源：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## Gate 映射

- 对应阶段：
  1. 阶段 H（产品差异化与透明执行）
- 当前覆盖情况：
  1. 本 change 当前属于并行设计 / 重构准备工作区，不参与 Gate-H 聚合复核签收裁决。
  2. 本 change 的输出主要服务于后续前端实现收口与产品体验统一。
  3. 当前已完成 Wave 1 / Wave 2 / Wave 3 的代码侧最小收口，并已补齐 Logs / History / Settings / Resources 的页面级截图与最小 walkthrough 证据；主控判断本并行 change 已具备可结束条件，后续以引用和归档口径整理为主。
