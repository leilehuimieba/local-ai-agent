# 任务清单

- [ ] T01 建立模块化收敛草案工作区
  完成判据：proposal/design/tasks/status/verify 五件套齐备，并加入 `changes/INDEX.md` 的待启动区。
- [ ] T02 固定口径一致性问题列表
  完成判据：明确列出 `current-state.md`、`changes/INDEX.md`、当前 active change 工作区的同步检查项。
- [ ] T03 形成 gateway 模块化第一批拆分方案
  完成判据：至少给出 `router.go`、learning handler 收口、diagnostics remediation 拆层的目标文件清单。
- [ ] T03-01 固定 `router.go` 第一轮拆分边界
  完成判据：明确第一轮只做“总装配入口 + 按域注册文件”，不改 handler 逻辑、不抽 service。
- [ ] T03-02 明确 `router.go` 目标文件清单
  完成判据：至少明确 `router.go / router_chat.go / router_learning.go / router_settings.go / router_logs.go / router_providers.go` 的职责归属。
- [ ] T03-03 明确第一轮验证与验收口径
  完成判据：明确 `go test ./internal/api` 为最小回归入口，并明确“不改变 API 路径与行为”。
- [ ] T04 形成 runtime-core 热点拆分方案
  完成判据：至少给出 `observation.rs`、`checkpoint.rs`、`memory_router.rs` 的目录化拆分落点。
- [ ] T05 形成 frontend 热点减重方案
  完成判据：至少给出 `workspaceViewModel.tsx`、`SettingsSections.tsx`、`index.css` 的拆分顺序与验证方式。
- [ ] T06 形成仓库对外表达与工程护栏补强方案
  完成判据：明确 README / About / topics / `tmp/README.md` / 热点文件体量规则的落点。
- [ ] T07 输出正式启动前的最小进入条件
  完成判据：明确“什么时候可以把本草案提升为正式推进项”，且不与 Gate-H 当前结论冲突。
