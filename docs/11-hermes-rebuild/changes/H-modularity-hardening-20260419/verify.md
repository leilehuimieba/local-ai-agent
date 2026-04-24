# 验证记录

## 验证方式

- 文档验证：
  1. 检查本 change 是否完整包含 `proposal/design/tasks/status/verify`。
  2. 检查本 change 是否被加入 `changes/INDEX.md` 的待启动 / 草案区域，而非当前活跃区域。
  3. 检查本 change 是否明确声明"不切当前 active change"。

- 结构验证：
  1. 检查草案是否覆盖三类模块化热点：gateway、runtime-core、frontend。
  2. 检查草案是否区分"先拆什么、后拆什么、暂不动什么"。
  3. 检查草案是否明确仓库对外表达与工程护栏补强项。

- 第一批拆分验证：
  1. `go build ./...` 通过。
  2. 关键回归测试通过：`TestLogsHandler|TestBestblogs|TestLearningExtract|TestRemediateLogs|TestCancelStopsRunning|TestResolveArtifactPath`。
  3. `router.go` 从 840 行缩减到 808 行，路由注册职责已按域拆分。
  4. API 路径与行为未改变。

- 第二批拆分验证：
  1. `cargo test -p runtime-core --lib` 175/175 通过。
  2. 三个热点文件已目录化，测试代码独立到 `tests.rs`：
     - `checkpoint.rs` (998 行) → `checkpoint/mod.rs` (593 行) + `checkpoint/tests.rs` (405 行)
     - `observation.rs` (1903 行) → `observation/mod.rs` (1670 行) + `observation/tests.rs`
     - `memory_router.rs` (876 行) → `memory_router/mod.rs` (786 行) + `memory_router/tests.rs`
  3. `lib.rs` 中模块声明保持 `mod checkpoint;` / `mod memory_router;` / `mod observation;`，Rust 自动识别目录形式。
  4. 并行测试隔离已修复：`memory_object_store.rs` 的 `sample_request()` 使用 `std::process::id()` + `timestamp_now()` 组合确保临时目录唯一性，flaky test 稳定通过。

## 证据位置

1. `docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/proposal.md`
2. `docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/design.md`
3. `docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/tasks.md`
4. `docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/status.md`
5. `docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/verify.md`
6. `gateway/internal/api/router_chat.go`
7. `gateway/internal/api/router_learning.go`
8. `gateway/internal/api/router_settings.go`
9. `gateway/internal/api/router_logs.go`
10. `gateway/internal/api/router_memory.go`
11. `gateway/internal/api/router_providers.go`
12. `gateway/internal/api/router.go`（已缩减）
13. `crates/runtime-core/src/checkpoint/mod.rs` / `checkpoint/tests.rs`
14. `crates/runtime-core/src/observation/mod.rs` / `observation/tests.rs`
15. `crates/runtime-core/src/memory_router/mod.rs` / `memory_router/tests.rs`
16. `crates/runtime-core/src/memory_object_store.rs`（测试隔离修复）

## Gate 映射

1. 对应阶段 Gate：
   - 当前只作为阶段 H 的结构收敛候选，不承担 Gate-H 签收结论。
2. 当前覆盖情况：
   - 已明确本 change 不改写当前 Gate-H 结论；
   - 已明确本 change 不接手当前 active change；
   - 第一批 gateway 装配层拆分已完成并验证通过；
   - 第二批 runtime-core 热点文件目录化已完成并验证通过。

- 第三批拆分验证：
  1. `npm run build` 通过，无新增 TypeScript 错误和 warning。
  2. `workspaceViewModel.tsx` 提取 `homeModel.ts`（~270 行），原文件从 856 → 547 行。类型定义导出（`export type`），App.tsx import 路径更新。
  3. `SettingsSections.tsx` 提取 `settingsHelpers.tsx`（~320 行），原文件从 780 → 444 行。共享辅助函数导出（`export function`），包含 JSX 的辅助函数保留在 `.tsx` 文件中。
  4. `index.css` 提取 6 个功能域 CSS 文件：
     - `styles/app-layout.css`（~300 行）
     - `styles/app-components.css`（~1200 行）
     - `styles/app-views.css`（~1300 行）
     - `styles/app-confirmations.css`（~690 行）
     - `styles/app-home-task.css`（~304 行）
     - `styles/app-knowledge-base.css`（~477 行）
  5. `styles/index.css` 从 `@import "../index.css"` 改为直接聚合 6 个新文件，保持加载顺序不变。
  6. JS/CSS 输出大小无异常增长（dist/assets/index-*.js 约 388 kB，dist/assets/index-*.css 约 68 kB）。

## 证据位置（第三批）

17. `frontend/src/shell/workspaceViewModel.tsx`（已缩减）
18. `frontend/src/shell/workspaceViewModel/homeModel.ts`
19. `frontend/src/settings/SettingsSections.tsx`（已缩减）
20. `frontend/src/settings/settingsHelpers.tsx`
21. `frontend/src/index.css`（已清空为入口注释）
22. `frontend/src/styles/app-layout.css`
23. `frontend/src/styles/app-components.css`
24. `frontend/src/styles/app-views.css`
25. `frontend/src/styles/app-confirmations.css`
26. `frontend/src/styles/app-home-task.css`
27. `frontend/src/styles/app-knowledge-base.css`
28. `frontend/src/styles/index.css`（聚合入口已更新）

## Gate 映射（更新）

1. 对应阶段 Gate：
   - 当前只作为阶段 H 的结构收敛候选，不承担 Gate-H 签收结论。
2. 当前覆盖情况：
   - 已明确本 change 不改写当前 Gate-H 结论；
   - 已明确本 change 不接手当前 active change；
   - 第一批 gateway 装配层拆分已完成并验证通过；
   - 第二批 runtime-core 热点文件目录化已完成并验证通过；
   - 第三批 frontend 热点减重已完成并验证通过。

## 第四批验证：仓库对外表达与工程护栏

1. 根目录 `README.md` 已创建，包含项目简介、技术栈表、项目结构、快速开始命令、文档导航。
2. `tmp/README.md` 已创建，包含目录约定、子目录说明、清理脚本；`.gitignore` 已添加 `!tmp/README.md` 例外，确保可被 Git 跟踪。
3. `AGENTS.md` 已新增"工程护栏"小节：
   - 热点文件体量红线：Rust/Go/TS 源文件 600 行、CSS 1000 行，超过应评估拆分。
   - 临时目录管理：`tmp/` 实验目录 7 天清理，长期缓存集中存放。
4. 无编译/构建破坏：Rust/Go/Frontend 构建未受影响。

## 证据位置（第四批）

29. `README.md`
30. `tmp/README.md`
31. `.gitignore`
32. `AGENTS.md`
