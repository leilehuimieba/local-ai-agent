# 技术方案

## 影响范围

- 诊断命令：`scripts/doctor.ps1`
- 验收脚本：`scripts/run-stage-f-doctor-acceptance.ps1`
- 文档索引：`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. doctor 命令

- 新增 `doctor.ps1`，默认检查：
  1. `go` 可用
  2. `cargo` 可用
  3. `node` 可用
  4. `npm` 可用
  5. `config/app.json` 存在
  6. 网关与运行时端口合法
  7. `frontend/dist/index.html` 存在
  8. `runtime /health` 可达
  9. `gateway /health` 可达
  10. `logs` 目录可写
- 输出结构化 JSON：`checks`、`versions`、`artifacts`。
- 支持 `-OutFile` 落盘和 `-RequirePass` 强制失败退出。

### 2. F-02 验收脚本

- 新增 `run-stage-f-doctor-acceptance.ps1`：
  1. 隔离端口构建并启动 runtime/gateway。
  2. 执行 `doctor.ps1 -RequirePass`。
  3. 校验关键检查项并落盘 `tmp/stage-f-doctor/latest.json`。

## 风险与回退

- 风险：当前 `doctor` 聚焦核心项，不覆盖网络外部依赖连通性深诊断。
- 缓解：将外部依赖细分检查放入后续 `F-03/F-04` 发布前检查项。
- 回退：若 `doctor` 某检查误报，可临时关闭 `-RequirePass` 保持报告输出，再修正规则。
