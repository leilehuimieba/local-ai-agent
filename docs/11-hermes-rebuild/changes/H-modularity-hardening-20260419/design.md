# 技术方案

## 影响范围

- 涉及模块：
  1. `gateway/internal/api/*`
  2. `gateway/internal/providers/*`
  3. `crates/runtime-core/src/*`
  4. `frontend/src/*`
  5. `docs/11-hermes-rebuild/*`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`（只读引用）
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/*`
  4. 本 change 工作区文件

## 方案

- 核心做法：
  1. 优先解决文档单一事实源漂移，确保 `current-state.md -> changes/INDEX.md -> 当前 active change 工作区` 一致。
  2. 将 gateway 的结构收敛拆成三层：
     - 路由注册层；
     - handler 层；
     - 子域目录层（learning / diagnostics）。
  3. 将 runtime-core 的结构收敛聚焦在热点文件目录化：
     - `observation.rs`
     - `checkpoint.rs`
     - `planner.rs`
     - `memory_router.rs`
  4. 将 frontend 的结构收敛聚焦在热点 view model / section / 样式文件减重：
     - `workspaceViewModel.tsx`
     - `SettingsSections.tsx`
     - `index.css`
  5. 将 GitHub 与工程护栏收敛为补充项：
     - README / About / topics
     - `tmp/README.md`
     - 热点文件体量治理规则

- 状态流转或调用链变化：
  1. 当前 change 只作为“待启动 / 草案”存在，不改变 active change。
  2. 后续若正式启动，应优先拆“低风险高收益”的入口层文件，再进入 runtime-core 热点文件目录化。
  3. 每一轮模块化收敛都应保持“先可验证，再扩范围”，避免形成新的大重构主线。

## 分批建议

### 第一批：口径一致性与 gateway 装配层

1. 修 `H-gate-h-signoff-20260416` 工作区与全局状态口径漂移。
2. 拆 `gateway/internal/api/router.go`。
3. 将 learning 相关 handler 收口到独立目录或独立路由注册层。

### 第二批：runtime-core 热点文件目录化

1. 拆 `crates/runtime-core/src/observation.rs`。
2. 拆 `crates/runtime-core/src/checkpoint.rs`。
3. 拆 `crates/runtime-core/src/memory_router.rs`。
4. 若仍有余量，再拆 `planner.rs`。

### 第三批：frontend 与仓库对外表达

1. 拆 `frontend/src/shell/workspaceViewModel.tsx`。
2. 拆 `frontend/src/settings/SettingsSections.tsx`。
3. 拆 `frontend/src/index.css`。
4. 补仓库 README / About / topics / `tmp/README.md`。

## 风险与回退

- 主要风险：
  1. 一次性拆分过多热点文件，导致验证范围失控。
  2. 在未固定 active change 的情况下，把草案误写成当前主推进。
  3. gateway / runtime-core 拆分时引入不必要的新抽象层，扩大 diff。

- 回退方式：
  1. 拆分按批进行，每批只覆盖一个模块域。
  2. 草案阶段只允许增量补文档，不触碰全局状态口径。
  3. 如某一批结构拆分收益不明显，回退为“热点文件冻结 + 下轮再拆”。
