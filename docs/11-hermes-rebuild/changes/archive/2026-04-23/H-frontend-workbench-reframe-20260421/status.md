# 当前状态

- 最近更新时间：2026-04-23
- 状态：已归档（2026-04-23）
- 状态口径：当前主推进仍以 `docs/11-hermes-rebuild/current-state.md` 为准，本 change 仅作为前端工作台重构并行工作区。

## 当前状态

1. 已完成：
   - 已确认本次前端重构不应挂到 `H-gate-h-signoff-20260416`，避免与 Gate-H 聚合复核边界冲突。
   - 已确认当前前端已有工作台雏形：`shell / workspace / chat / events / logs / settings / history / resources / ui / styles` 结构完整，适合做收口而非推翻重写。
   - 已完成首轮现状扫描，确认前端技术栈为 React + Vite + TypeScript + Zustand，当前未引入大型 UI 框架。
   - 已形成初步方向：采用 Codex 风格工作台作为参考，先做 shell 和 token 收口，再做内容块收口。
   - 已修正 `changes/INDEX.md` 索引口径，并将本 change 作为并行草案加入索引。
   - 已新增 `wave1-实施入口.md`，冻结 Wave 1 必改文件、目标、风险、回退与最小验收。
   - 已新增 `wave1-深色-token-草案.md`，明确深色工作台 token 的第一版建议值。
   - 已新增 `wave1-文件级实施计划.md`，将 Wave 1 拆到具体文件、类名挂点、回退点与最小验证。
   - 已完成 Wave 1 首轮代码改造，覆盖 `tokens.css / base.css / index.css / AppShell.tsx / TopBar.tsx / ContextSidebar.tsx / BottomPanel.tsx`。
   - 已完成前端构建验证：`npm run build` 通过。
   - 已补首页、任务页、调查层展开态截图，证据已落到 `evidence/`。
   - 已新增 `wave2-实施入口.md`，冻结 Wave 2 的范围、风险、回退与最小验收。
   - 已通过 `scripts/start-dev.ps1` 拉起本地 `gateway/runtime`，确认 `http://127.0.0.1:8897/api/v1/settings` 与 `http://127.0.0.1:4173/api/v1/settings` 均返回 `200`。
   - 已补设置加载恢复证据：`settings-response-20260422.json`、`settings-health-20260422.txt`。
   - 已补最小 walkthrough 证据：`wave2-walkthrough-20260422.txt`、`wave2-walkthrough-logs-20260422.json`。
   - 已完成 Wave 2 第一轮最小 primitives 收口：新增 `ui/primitives/InfoCard.tsx`，并接入 `ChatPanel.tsx`、`EventTimeline.tsx`。
   - 已完成 Wave 2 第二轮最小层级收口：`ChatPanel.tsx` 新增主线程 meta 与 composer meta，`EventTimeline.tsx` 收口 primary detail 与 tag 聚合。
   - 已完成 Wave 2 第三轮最小语义收口：`ChatPanel.tsx` 统一确认块与状态块语义，`EventTimeline.tsx` 增加 detail 行层级与事件头收口。
   - 已完成 Wave 2 第四轮结果块与事件 tone 收口：`ChatPanel.tsx` 已区分 answer / recovery / system 结果块语义，`EventTimeline.tsx` 已统一 latest / selected / tone 的状态表达。
   - 已完成 Wave 2 第五轮减噪收口：`ChatPanel.tsx` 已压缩 detail 类辅助块噪音，`EventTimeline.tsx` 已收紧 tag 数量与 detail 密度。
   - 已完成 Wave 2 第六轮微调：进一步弱化辅助结果块权重，timeline tag 已收紧到 3 个以内、detail 已收紧到 4 条以内。
   - 已新增 `wave2-收口评估.md`，明确当前代码侧已基本具备 Wave 2 收口条件，剩余缺口主要为页面级 walkthrough 证据。
   - 已新增 `wave3-最小入口.md`，冻结 `Logs / History / Settings / Resources` 的进入顺序、边界与最小验收。
   - 已新增 `wave3-文件级实施计划.md`，明确 Phase 1 先从 `LogsPanel.tsx + history/components/*` 开始，Phase 2 再进入 `settings/* + resources/*`。
   - 已启动 Wave 3 Round 1：`LogsPanel.tsx` 已补 workbench hero 与统一区块容器，Logs 页开始进入统一工作台语言。
   - 已推进 Wave 3 Round 1 第二步：`HistoryPageSections.tsx / HistorySpotlight.tsx / HistoryTimelineSection.tsx` 已完成最小表达层收口，History 视图开始与 Logs Workspace 共用筛选台、焦点卡与稳定记录流语义。
   - 已推进 Wave 3 Round 2 第一轮：`SettingsPanel.tsx / StatusCard.tsx / SettingsSections.tsx` 已补 Settings Workspace hero、运行态说明与模块头语义，设置页开始从传统配置页收口为统一工作台视图。
   - 已推进 Wave 3 Round 2 第二轮：`MemoryResourcesSection.tsx / ResourcesEntrySection.tsx` 已补资源工作区说明、模块头与列表区描述，记忆与资源区开始按工作台语言表达，而不再只是 settings 内的卡片堆叠。
   - 已轻触 `HistoryDetailRail.tsx`：详情栏已补 `Detail / 复盘详情栏` 头部语义与空状态文案，当前仍保持 detail 生成逻辑不变。
   - 已新增 `wave3-收口评估.md`，明确当前 Wave 3 在代码侧已基本具备阶段性收口条件，当前主要缺口转为页面截图与多视图一致性证据。
   - 已新增 `wave3-多视图一致性检查清单.md`，当前已能按 Logs / History / Settings / Resources 四类视图检查 header、panel、status、empty-state 与资源模块边界的一致性。
   - 已新增 `wave3-截图补证入口.md`，明确 Logs / History / Settings / Resources 的截图入口、建议文件名与回填步骤。
   - 已新增 `wave3-保守收口口径.md`，当前已明确本 change 应采用“Wave 3 代码侧已基本收口，页面证据待补”的稳定表述，不将其直接写成全部完成。
   - 已新增 `当前阶段总整理.md`，当前已能用一份总整理文稿收拢本并行 change 的已实现、已验证、待补证与推荐口径，便于后续交接与恢复上下文。
   - 已新增 `wave3-收口交接说明.md`，当前已能用单文档向主控侧或后续执行侧说明：Wave 3 代码侧已收口、页面截图与最小 walkthrough 已补齐、当前仍按保守口径等待裁决。
   - 已完成前端构建验证：`npm run build` 通过（2026-04-22）。
   - 已于 2026-04-22 再次复核本地前端预览与设置接口健康：`http://127.0.0.1:4173`、`http://127.0.0.1:4173/api/v1/settings`、`http://127.0.0.1:8897/api/v1/settings` 均可访问，补证准备记录已落到 `evidence/wave3-screenshot-readiness-20260422.txt`。
2. 进行中：
   - 当前已无新增实现动作；本并行 change 已进入主控裁决后的收口挂账状态，后续仅保留按需引用和最小补充说明。
3. 阻塞点：
   - 当前无代码或页面证据阻塞；仅保留“并行 change 不切主推进”的治理边界。
4. 下一步：
   - 当前主控已给出“可结束（并行）”判断；后续如需动作，仅在本 change 下补最小引用说明，不继续扩大代码 diff。
   - 不修改 `docs/11-hermes-rebuild/current-state.md`，不切主推进，不把本并行 change 写成“前端工作台重构已全部验收”。
