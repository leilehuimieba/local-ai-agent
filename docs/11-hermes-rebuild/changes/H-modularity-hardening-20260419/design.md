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

#### `gateway/internal/api/router.go` 第一轮拆分方案

第一轮目标不是重构 handler 或抽 service，而是先把“总路由表”收敛成“装配入口 + 按域注册文件”。

##### 拆分目标文件

1. 保留：`gateway/internal/api/router.go`
2. 新增：
   - `gateway/internal/api/router_chat.go`
   - `gateway/internal/api/router_learning.go`
   - `gateway/internal/api/router_settings.go`
   - `gateway/internal/api/router_logs.go`
   - `gateway/internal/api/router_providers.go`

##### 第一轮职责边界

1. `router.go`
   - 只保留 `NewRouter(...)`
   - 只负责依赖初始化与分组注册调用
   - 不再承载大量 `mux.HandleFunc(...)`
2. `router_chat.go`
   - 负责 `registerChatRoutes(...)`
3. `router_learning.go`
   - 负责 `registerLearningRoutes(...)`
4. `router_settings.go`
   - 负责 `registerSettingsRoutes(...)`
   - 负责 `registerDiagnosticsRoutes(...)`
   - 如当前代码仍保持 settings/diagnostics 强耦合，允许先放在同一文件中
5. `router_logs.go`
   - 负责 `registerLogsRoutes(...)`
6. `router_providers.go`
   - 负责 `registerProviderSettingsRoutes(...)`
   - 负责 `registerProviderArticleRoutes(...)`

##### 第一轮不做什么

1. 不改 handler 函数签名。
2. 不改 API contract。
3. 不抽 learning service 层。
4. 不把 learning handler 全量迁入新目录。
5. 不把 diagnostics remediation 再细拆到 executor / response / planner。

##### 验证方式

1. 编译与回归优先使用：
   - `go test ./internal/api`
2. 若第一轮拆分涉及 provider 注册路径，再补：
   - `go test ./...`

##### 验收口径

1. `router.go` 只剩装配入口，不再承担域内展开。
2. Chat / Learning / Settings / Diagnostics / Logs / Providers 的注册边界可单文件定位。
3. 第一轮 diff 不改变任何已有 API 路径与行为。
4. `go test ./internal/api` 通过。

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
