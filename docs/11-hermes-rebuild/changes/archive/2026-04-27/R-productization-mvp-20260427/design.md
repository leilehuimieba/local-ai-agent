# R-change 设计文档

## 1. 敏感信息治理

### 当前问题
`config/app.json` 硬编码 `api_key` 和绝对路径。

### 方案
1. 新增 `.env.example` 模板，列出所有可配置变量
2. 扩展 `gateway/internal/config/config.go` 的 `applyEnvOverrides`，支持 provider 级 `api_key`、`base_url` 覆盖
3. `config/app.json` 中 provider 的 `api_key` 改为空字符串或 `"${ENV}"` 占位符
4. gateway 启动时若 key 为空，读取 `LOCAL_AGENT_API_KEY_<PROVIDER>` 环境变量

### 环境变量清单
```
LOCAL_AGENT_GATEWAY_PORT=8897
LOCAL_AGENT_RUNTIME_PORT=8898
LOCAL_AGENT_WORKSPACE_ROOT=
LOCAL_AGENT_API_KEY_SCNET=
LOCAL_AGENT_API_KEY_OPENAI=
LOCAL_AGENT_API_KEY_LOCAL_LLAMA=
```

## 2. LICENSE + 元数据 + CHANGELOG

### 方案
1. 根目录新增 `LICENSE`（MIT 全文）
2. `frontend/package.json` 补充 `description`、`license`、`repository`
3. `crates/runtime-core/Cargo.toml` 和 `crates/runtime-host/Cargo.toml` 补充 `description`、`license`、`repository`、`authors`
4. 根目录新增 `CHANGELOG.md`（Keep a Changelog 格式），首条记录 0.1.0

## 3. Rust Release 构建

### 方案
1. `Cargo.toml` 新增 `[profile.release]`：
   - `lto = true`
   - `codegen-units = 1`
   - `panic = "abort"`
2. `install-local-agent.ps1` 中 `cargo build` 改为 `cargo build --release -p runtime-host`
3. CI 发布 workflow 已用 `--release`，无需改动

## 4. 前端生产配置

### 方案
1. 新增 `frontend/.env.example` 和 `frontend/.env.production`
2. `vite.config.ts` 补充：
   - `build.outDir: "dist"`（默认即可，显式声明）
   - `base: "/"`
   - `build.rollupOptions.output.manualChunks` 做基础代码分割
3. 新增 `frontend/public/favicon.svg`（使用品牌色 `#ff6b35` 的极简图标）
4. `index.html` 补充：
   - `<meta name="description">`
   - `<meta name="theme-color" content="#ff6b35">`
   - `<link rel="icon" type="image/svg+xml" href="/favicon.svg">`
5. 前端 API base URL 从开发代理切换为生产环境变量 `VITE_API_BASE_URL`

## 5. 最小启动认证

### 方案
1. gateway 启动时生成 32 字节随机 token（hex 编码），写入 `data/.gateway_token`
2. 所有 `/api/*` 请求需带 `X-Local-Agent-Token: <token>` header
3. 前端 `index.html` 由 gateway 托管时，gateway 将 token 注入 `<meta name="local-agent-token">`
4. 前端 fetch 拦截器自动读取该 meta 并附加到请求头
5. 无 token 或 token 不符的请求返回 `401 Unauthorized`

### 例外
- `/health` 端点无需认证（供外部探针使用）

## 6. 统一构建脚本

### 方案
新建 `scripts/build-all.ps1`：
```powershell
param(
    [ValidateSet("debug","release")]
    [string]$Profile = "release",
    [string]$OutputDir = "./build"
)
# 1. 前端 build
# 2. Go gateway build (当前平台)
# 3. Rust runtime-host build ($Profile)
# 4. 产物复制到 $OutputDir
# 5. 生成 BUILD_INFO.txt
```

`install-local-agent.ps1` 改为调用 `build-all.ps1`，分离构建与安装职责。

## 7. 代码质量工具

### 前端
- ESLint（`@eslint/js` + `typescript-eslint` + `eslint-plugin-react-hooks`）
- Prettier（基础配置）
- `package.json` 新增 `lint`、`format` 脚本

### Go
- `golangci-lint` 配置文件 `.golangci.yml`
- CI 新增 `go-lint` job

### Rust
- `rustfmt.toml`（默认即可）
- CI 新增 `rust-fmt` 和 `rust-clippy` job

### CI 调整
`ci.yml` 新增 job：
- `frontend-lint`
- `go-lint`
- `rust-fmt`
- `rust-clippy`
