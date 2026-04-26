# H-modularity-hardening-20260419（status）

最近更新时间：2026-04-24
状态：第三批已完成并验证通过；等待收尾决策
状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`

## 当前状态

1. 已完成：
   - 草案工作区已建立，五件套齐备。
   - Gate-H 已开发阶段通过，阻塞解除；本 change 作为并行项启动。
   - 第一批：gateway `router.go` 按域拆分已完成（6 个新文件，router.go 从 840 → 808 行）。
   - 第二批：runtime-core 热点文件目录化已完成。
     - `checkpoint.rs` → `checkpoint/mod.rs` + `checkpoint/tests.rs`（998 → 593 行，-40.4%）
     - `observation.rs` → `observation/mod.rs` + `observation/tests.rs`（1903 → 1670 行，-12.2%）
     - `memory_router.rs` → `memory_router/mod.rs` + `memory_router/tests.rs`（876 → 786 行，-10.3%）
     - 修复 `memory_object_store.rs` 测试隔离：`sample_request()` 使用 `std::process::id()` + `timestamp_now()` 组合确保临时目录唯一性。
   - 第三批：frontend 热点减重已完成。
     - `workspaceViewModel.tsx` 提取 `homeModel.ts`（~270 行），原文件 856 → 547 行（-36.1%）
     - `SettingsSections.tsx` 提取 `settingsHelpers.tsx`（~320 行），原文件 780 → 444 行（-43.1%）
     - `index.css` 提取 6 个功能域 CSS 文件，原文件 4270 → 1 行（-99.9%）
   - 编译验证：`go build ./...` 通过；`cargo test -p runtime-core --lib` 175/175 通过；`npm run build` 通过。
2. 进行中：
   - 无。
3. 阻塞点：
   - 无。
4. 下一步：
   - 方案 A：收尾本 change，更新 INDEX.md 标记为已完成，回切主控。
   - 方案 B：补仓库对外表达（README / About / topics / `tmp/README.md`），作为本 change 的收尾项。

## 三批验证摘要

| 域 | 文件 | 原行数 | 新行数 | 降幅 |
|----|------|--------|--------|------|
| gateway | router.go | 840 | 808 | -3.8% |
| runtime-core | checkpoint.rs | 998 | 593 | -40.4% |
| runtime-core | observation.rs | 1903 | 1670 | -12.2% |
| runtime-core | memory_router.rs | 876 | 786 | -10.3% |
| frontend | workspaceViewModel.tsx | 856 | 547 | -36.1% |
| frontend | SettingsSections.tsx | 780 | 444 | -43.1% |
| frontend | index.css | 4270 | 1 | -99.9% |

## 第四批：仓库对外表达与工程护栏

- [x] T19 创建根目录 README.md
- [x] T20 创建 tmp/README.md
- [x] T21 补充 AGENTS.md 工程护栏
- [x] T22 第四批验证

## 收尾状态

- 收尾时间：2026-04-24
- 操作：更新 `changes/INDEX.md` 标记为已完成；回切主控。
- 所有四批任务已验证通过，无遗留阻塞。
