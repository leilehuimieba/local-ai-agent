# 任务清单

## 草案阶段

- [x] T01 建立模块化收敛草案工作区
  完成判据：proposal/design/tasks/status/verify 五件套齐备，并加入 `changes/INDEX.md` 的待启动区。
- [x] T02 固定口径一致性问题列表
  完成判据：明确列出 `current-state.md`、`changes/INDEX.md`、当前 active change 工作区的同步检查项。
- [x] T03 形成 gateway 模块化第一批拆分方案
  完成判据：至少给出 `router.go`、learning handler 收口、diagnostics remediation 拆层的目标文件清单。
- [x] T03-01 固定 `router.go` 第一轮拆分边界
  完成判据：明确第一轮只做"总装配入口 + 按域注册文件"，不改 handler 逻辑、不抽 service。
- [x] T03-02 明确 `router.go` 目标文件清单
  完成判据：至少明确 `router.go / router_chat.go / router_learning.go / router_settings.go / router_logs.go / router_providers.go` 的职责归属。
- [x] T03-03 明确第一轮验证与验收口径
  完成判据：明确 `go test ./internal/api` 为最小回归入口，并明确"不改变 API 路径与行为"。
- [x] T04 形成 runtime-core 热点拆分方案
  完成判据：至少给出 `observation.rs`、`checkpoint.rs`、`memory_router.rs` 的目录化拆分落点。
- [x] T05 形成 frontend 热点减重方案
  完成判据：至少给出 `workspaceViewModel.tsx`、`SettingsSections.tsx`、`index.css` 的拆分顺序与验证方式。
- [x] T06 形成仓库对外表达与工程护栏补强方案
  完成判据：明确 README / About / topics / `tmp/README.md` / 热点文件体量规则的落点。
- [x] T07 输出正式启动前的最小进入条件
  完成判据：明确"什么时候可以把本草案提升为正式推进项"，且不与 Gate-H 当前结论冲突。

## 第一批：gateway 装配层拆分

- [x] T08 拆分 `router.go` 为按域注册文件
  完成判据：新建 6 个 router_*.go 文件，`router.go` 只保留装配入口 + `registerCoreRoutes` + handler 实现。
- [x] T08-01 新建 `router_chat.go`
  完成判据：`registerChatRoutes` 已移入，编译通过。
- [x] T08-02 新建 `router_learning.go`
  完成判据：`registerLearningRoutes` 已从 `learning_extract.go` 移入，编译通过。
- [x] T08-03 新建 `router_settings.go`
  完成判据：settings + diagnostics + external-connections 路由已拆分，编译通过。
- [x] T08-04 新建 `router_logs.go`
  完成判据：logs + system info + artifacts 路由已拆分，编译通过。
- [x] T08-05 新建 `router_memory.go`
  完成判据：`registerMemoryRoutes` 已移入，编译通过。
- [x] T08-06 新建 `router_providers.go`
  完成判据：`registerProviderSettingsRoutes` + `registerProviderArticleRoutes` 已从原文件移入，编译通过。
- [x] T09 第一批回归验证
  完成判据：`go build ./...` 通过；关键测试（logs / learning / provider / chat / remediation / artifact）通过。

## 第二批：runtime-core 热点文件目录化

- [x] T10 目录化 `checkpoint.rs`
  完成判据：`checkpoint.rs` → `checkpoint/mod.rs` + `checkpoint/tests.rs`，编译通过，测试通过。
- [x] T11 目录化 `observation.rs`
  完成判据：`observation.rs` → `observation/mod.rs` + `observation/tests.rs`，编译通过，测试通过。
- [x] T12 目录化 `memory_router.rs`
  完成判据：`memory_router.rs` → `memory_router/mod.rs` + `memory_router/tests.rs`，编译通过，测试通过。
- [x] T13 修复并行测试隔离
  完成判据：`memory_object_store.rs` 的 `sample_request()` 使用 `std::process::id()` + `timestamp_now()` 组合确保临时目录唯一性，flaky test 稳定通过。
- [x] T14 第二批回归验证
  完成判据：`cargo test -p runtime-core --lib` 175/175 全部通过。

## 第三批：frontend 热点减重

- [x] T15 拆分 `workspaceViewModel.tsx`
  完成判据：提取 `homeModel.ts`（~270 行），原文件从 856 → 547 行；类型定义导出；App.tsx import 更新；编译通过。
- [x] T16 拆分 `SettingsSections.tsx`
  完成判据：提取 `settingsHelpers.tsx`（~320 行），原文件从 780 → 444 行；共享辅助函数导出；编译通过。
- [x] T17 拆分 `index.css`
  完成判据：提取 6 个功能域 CSS 文件（layout / components / views / confirmations / home-task / knowledge-base），原文件从 4270 → 1 行（入口注释）；`styles/index.css` 聚合入口更新；编译通过。
- [x] T18 第三批回归验证
  完成判据：`npm run build` 通过；JS/CSS 输出大小无异常增长。

## 第四批：仓库对外表达与工程护栏

- [x] T19 创建根目录 README.md
  完成判据：包含项目简介、技术栈、项目结构、快速开始、文档导航。
- [x] T20 创建 tmp/README.md
  完成判据：说明 tmp 目录用途、子目录约定、清理规则；.gitignore 同步更新（!tmp/README.md）。
- [x] T21 补充 AGENTS.md 工程护栏
  完成判据：新增"热点文件体量红线"（源文件 600 行 / CSS 1000 行）和"临时目录管理"规则。
- [x] T22 第四批验证
  完成判据：tmp/README.md 可被 Git 跟踪；AGENTS.md 语法正确；无编译/构建破坏。
