# 任务清单

- [x] T01 建立前端工作台重构 change 工作区
  完成判据：`proposal/design/tasks/status/verify` 五件套齐备，并加入 `changes/INDEX.md`，同时明确“不切当前主推进”。
- [x] T02 输出前端现状模块 -> 目标模块映射表
  完成判据：至少覆盖 shell、sidebar、topbar、bottom panel、chat、events、logs、settings、history、resources、styles/tokens 六类以上关键模块，且每项包含当前问题与目标形态。
- [x] T03 冻结工作台设计原则与最小 token 范围
  完成判据：明确深色工作台主题、文字层级、边框层级、状态色、间距、圆角与等宽字体策略。
- [x] T04 拆分实现波次：Shell 收口波次
  完成判据：明确 P0 文件清单、改动边界和完成后应看到的骨架变化。
- [x] T05 拆分实现波次：内容块收口波次
  完成判据：明确 P1/P2 文件清单，说明 Chat / EventTimeline / Logs / History / Settings / Resources 的收口顺序。
- [x] T06 定义验证口径与证据要求
  完成判据：verify 中明确至少包含信息架构、一致性、认知负担、扩展性四类验证。
- [x] T07 形成首轮实施入口
  完成判据：能明确回答“如果今天开始改，先改哪几个文件、为什么、风险是什么”。

## 实施顺序建议

1. Wave 1：`tokens.css + AppShell.tsx + ContextSidebar.tsx + TopBar.tsx + BottomPanel.tsx`
2. Wave 2：`ChatPanel.tsx + EventTimeline.tsx + ui/primitives/*`
3. Wave 3：`LogsPanel.tsx + history/* + settings/* + resources/*`

- [x] T08 输出 Wave 1 实施入口文档
  完成判据：新增 `wave1-实施入口.md`，明确 Wave 1 必改文件、目标、风险、回退与最小验收。
- [x] T09 输出深色 token 草案
  完成判据：新增 `wave1-深色-token-草案.md`，冻结 Wave 1 使用的背景、文本、边框、状态、圆角、间距 token 草案。
- [x] T10 输出 Wave 1 文件级实施计划
  完成判据：新增 `wave1-文件级实施计划.md`，明确每个文件的先改点、视觉目标、回退点与最小验证。

- [x] T11 同步 Wave 1 实施证据状态
  完成判据：`status.md` 与 `verify.md` 对齐，明确截图证据已补齐、当前缺口为 walkthrough 与运行时配置异常。
- [x] T12 输出 Wave 2 实施入口文档
  完成判据：新增 `wave2-实施入口.md`，明确 `ChatPanel.tsx / EventTimeline.tsx / ui/primitives/*` 的目标、风险、回退与最小验收。
- [x] T13 补最小 walkthrough 证据
  完成判据：补充一组可回放的最小任务链路证据，至少包含 `chat/run` 提交、运行日志、结论落点。
- [x] T14 启动 Wave 2 第一轮 primitives 收口
  完成判据：`ChatPanel.tsx`、`EventTimeline.tsx` 至少接入一层共享 primitives，并构建通过。
- [x] T15 推进 Wave 2 第二轮内容块层级收口
  完成判据：`ChatPanel.tsx` 明确主线程 meta / composer meta，`EventTimeline.tsx` 明确 detail / tag 层级，且构建通过。
- [x] T16 推进 Wave 2 第三轮状态块与事件细节收口
  完成判据：统一状态块/确认块语义与事件 detail 行层级，且构建通过。
- [x] T17 推进 Wave 2 第四轮结果块与事件 tone 收口
  完成判据：`ChatPanel.tsx` 明确 answer / recovery / system 结果语义，`EventTimeline.tsx` 明确 latest / selected / tone 的统一表达，且构建通过。
- [x] T18 推进 Wave 2 第五轮减噪收口
  完成判据：`ChatPanel.tsx` 压缩辅助结果块噪音，`EventTimeline.tsx` 收紧 tag 与 detail 密度，且构建通过。
- [x] T19 推进 Wave 2 第六轮微调与收口评估
  完成判据：继续弱化辅助信息视觉权重、收紧 timeline 标签/细节数量，并给出 Wave 2 是否可结束的判断。
- [x] T20 输出 Wave 2 收口评估文稿
  完成判据：新增 `wave2-收口评估.md`，明确代码侧是否已收口、验证侧缺口与正式关闭 Wave 2 的条件。
- [x] T21 输出 Wave 3 最小入口文档
  完成判据：新增 `wave3-最小入口.md`，明确 `Logs / History / Settings / Resources` 的进入顺序、边界与最小验收。
- [x] T22 输出 Wave 3 文件级实施计划
  完成判据：新增 `wave3-文件级实施计划.md`，明确 `Logs / History / Settings / Resources` 的文件级改造顺序、先改点、风险与回退点。
- [x] T23 启动 Wave 3 Round 1：LogsPanel 最小收口实现
  完成判据：`LogsPanel.tsx` 完成 workbench 头部与区块容器收口，且构建通过。
- [x] T24 推进 Wave 3 Round 1：history/components 最小收口实现
  完成判据：`HistoryPageSections.tsx / HistorySpotlight.tsx / HistoryTimelineSection.tsx` 完成最小表达层收口，并与 Logs 工作区语言对齐，且构建通过。
- [x] T25 推进 Wave 3 Round 2：settings/* 最小收口实现
  完成判据：`SettingsPanel.tsx / StatusCard.tsx / SettingsSections.tsx` 完成工作区壳层与模块头部的最小语义收口，且构建通过。
- [x] T26 推进 Wave 3 Round 2：resources/* 最小收口实现
  完成判据：`MemoryResourcesSection.tsx / ResourcesEntrySection.tsx` 完成资源模块级头部、说明与列表区的最小语义收口，且构建通过。
- [x] T27 轻触 Wave 3 Detail Rail：HistoryDetailRail.tsx 外层语义收口
  完成判据：`HistoryDetailRail.tsx` 完成标题、说明与空状态文案的最小收口，不触碰 detail 生成逻辑，且构建通过。
- [x] T28 输出 Wave 3 收口评估文稿
  完成判据：新增 `wave3-收口评估.md`，明确 Wave 3 当前代码侧是否已基本收口、验证侧缺口以及正式关闭 Wave 3 的条件。
- [x] T29 输出 Wave 3 多视图一致性检查清单
  完成判据：新增 `wave3-多视图一致性检查清单.md`，明确 Logs / History / Settings / Resources 的跨视图一致性检查项、当前观察与补证建议。
- [x] T30 输出 Wave 3 截图补证入口文稿
  完成判据：新增 `wave3-截图补证入口.md`，明确 Logs / History / Settings / Resources 的截图入口、推荐文件名与回填方式。
- [x] T31 输出 Wave 3 保守收口口径文稿
  完成判据：新增 `wave3-保守收口口径.md`，明确“代码侧已收口、页面证据待补”的稳定表述方式，以及当前不能写成全部完成的边界。
- [x] T32 输出并行 change 当前阶段总整理文稿
  完成判据：新增 `当前阶段总整理.md`，收拢当前并行 change 的已实现、已验证、待补证与推荐口径，便于后续交接与恢复上下文。
- [x] T33 输出 Wave 3 收口交接说明
  完成判据：新增 `wave3-收口交接说明.md`，收拢当前 Wave 3 的真实状态、页面证据、结构边界与推荐交接口径，便于主控侧或后续对话直接使用。
