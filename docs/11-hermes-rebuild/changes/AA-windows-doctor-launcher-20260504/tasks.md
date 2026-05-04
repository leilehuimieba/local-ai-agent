# 任务清单

- [x] 建立 AA-change 工作区并切换活跃项
  完成判据：`current-state.md` 与 `changes/INDEX.md` 指向本 change。

- [x] 梳理现有 diagnostics 与 launcher 能力
  完成判据：明确复用文件、缺口和第一版改动范围。

- [x] 补齐 Gateway 服务状态/doctor 输出
  完成判据：诊断接口返回结构化状态项，覆盖 Gateway、Runtime、MCP、Provider、Session、Knowledge、Logs、Frontend。

- [x] 改造一键启动器为 preflight/start/verify
  完成判据：启动前先诊断，启动后验活，失败输出阶段、日志路径和建议动作。

- [x] 前端设置页增加服务状态面板
  完成判据：用户可刷新并看到关键服务健康状态、说明和建议。

- [x] 补测试与验证记录
  完成判据：Gateway/Frontend 相关测试通过，`verify.md` 记录证据。

- [x] 登记后续 diff apply UI 预览 change
  完成判据：本 change 明确不扩 scope，后续队列记录 `AB-diff-preview-ui`。
