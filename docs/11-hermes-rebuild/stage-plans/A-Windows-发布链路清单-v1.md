# A 阶段 Windows 发布链路清单 v1

更新时间：2026-04-10
阶段：A（冻结）
目标：定义 F 阶段必须收口的 Windows 发布最小链路

## 1. 目标与边界

1. 本清单定义“Windows 新机从 0 到可运行”的最小发布链路。
2. v1 先保证稳定可用，不追求一次覆盖全部分发渠道。
3. 默认先支持开发版与内测版发布。

## 2. 组件清单

1. Runtime：`runtime-host.exe`（Rust）
2. Gateway：`server.exe`（Go）
3. Launcher：`go run ./cmd/launcher` 或 `launcher.exe`
4. Frontend：`frontend/dist` 静态产物

## 3. 构建链路（v1）

1. Rust 构建

```powershell
cargo build -p runtime-host
```

2. Go 构建

```powershell
cd gateway
go build -o server.exe ./cmd/server
```

3. Frontend 构建

```powershell
cd frontend
npm install
npm run build
```

4. 一键开发启动

```powershell
pwsh -File scripts/start-dev.ps1
```

## 4. 启动与健康检查

1. Runtime 健康：`GET http://127.0.0.1:{runtime_port}/health`
2. Gateway 健康：`GET http://127.0.0.1:{gateway_port}/health`
3. 系统信息：`GET /api/v1/system/info`
4. 端口来源：`config/app.json` 与 `LOCAL_AGENT_RUNTIME_PORT` 覆盖

## 5. 发布包最小内容（v1）

1. `runtime-host.exe`
2. `gateway/server.exe`
3. `frontend/dist/*`
4. `config/app.json`
5. 启动脚本（`start-dev.ps1` 或发布脚本）
6. `README-运行说明.md`

## 6. 新机安装验收流程（10 分钟目标）

1. 拉取发布包到本地目录。
2. 执行启动脚本。
3. 打开网关首页。
4. 执行一次 `chat/run` 与 `events/stream` 验证。
5. 验证设置页 `runtime_status` 为可达。

## 7. 诊断清单（doctor v1 预案）

必须检查：

1. Go 版本可用。
2. Rust toolchain 可用。
3. Node/npm 可用。
4. `config/app.json` 存在且端口合法。
5. `frontend/dist/index.html` 存在。
6. runtime/gateway 健康可达。
7. logs 目录可写。

## 8. 回滚策略（v1）

1. 保留上一个稳定发布目录（N-1）。
2. 发布失败时切回 N-1 可执行文件和前端产物。
3. 若 schema 变更，必须提供回滚兼容说明。
4. 回滚流程必须有脚本化步骤，不依赖口头记忆。

## 9. F 阶段验收门槛映射

1. 安装成功率 >= 90%。
2. 启动成功率 >= 95%。
3. 新机首任务成功率 >= 90%。
4. 回滚演练 1 次成功。

## 10. 当前缺口（A 阶段记录）

1. 还没有正式 `doctor` 命令。
2. 还没有统一发布打包脚本。
3. 还没有安装器渠道（winget/scoop/msi）冻结决策。

## 11. 责任归属

1. 构建链 Owner：`runtime-host` + `gateway` + `frontend`。
2. 发布流程 Owner：`gateway/cmd/launcher`。
3. 诊断与回滚 Owner：F 阶段发布责任人。
