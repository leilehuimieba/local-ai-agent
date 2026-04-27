# 验证记录

## R-01 敏感信息治理

- `config/app.json` 中 scnet provider 的 `api_key` 已清空
- `.env.example` 已创建，列出所有可配置环境变量
- `gateway/internal/config/config.go` 的 `applyEnvOverrides` 已扩展，支持 `LOCAL_AGENT_API_KEY_<PROVIDER>` 和 `LOCAL_AGENT_BASE_URL_<PROVIDER>`
- `go vet ./...` 无错误

## R-02 LICENSE + 元数据 + CHANGELOG

- 根目录 `LICENSE`（MIT）已创建
- `frontend/package.json` 已补充 description、license、repository
- `crates/runtime-core/Cargo.toml` 和 `crates/runtime-host/Cargo.toml` 已补充 description、license、repository
- `CHANGELOG.md` 已创建（Keep a Changelog 格式）

## R-03 Rust Release 构建优化

- 根 `Cargo.toml` 新增 `[profile.release]`（lto、codegen-units=1、panic=abort）
- `install-local-agent.ps1` 改用 `cargo build --release`
- release 产物 3.11 MB（对比 debug 5.66 MB，减小 45%）

## R-04 前端生产配置

- `frontend/.env.example` 和 `.env.production` 已创建
- `vite.config.ts` 补充 sourcemap 和 manualChunks（vendor 分割）
- `frontend/public/favicon.svg` 已创建（品牌色笑脸图标）
- `index.html` 补充 meta description、theme-color、favicon 链接
- 生产构建成功，产物含 vendor chunk（189.68 kB）

## R-05 最小启动认证

- `gateway/internal/token` 包已创建，支持生成/读取/验证 token
- gateway 启动时自动生成 32 字节 hex token，写入 `data/.gateway_token`
- 所有 `/api/*` 请求校验 `X-Local-Agent-Token` header，`/health` 豁免
- `spaHandler` 向 `index.html` 注入 `<meta name="local-agent-token">`
- 前端 `main.tsx` 全局拦截 fetch，自动附加 token header
- launcher `systemInfo` 读取 token 文件并附加 header
- 单元测试验证：无 token 返回 401、有 token 通过、/health 豁免

## R-06 统一构建脚本

- `scripts/build-all.ps1` 已创建，支持 debug/release 配置、产物复制、BUILD_INFO.txt

## R-07 代码质量工具

- 前端：`eslint.config.js`、`.prettierrc`、`lint`/`format` 脚本、package.json 依赖已配置
- Go：`.golangci.yml` 已配置（启用 errcheck、gosimple、govet、ineffassign、staticcheck、unused、gofmt）
- Rust：`rustfmt.toml` 已配置（edition=2024、max_width=120）
- CI：`ci.yml` 新增 frontend-lint、go-lint、rust-fmt、rust-clippy job

## 综合回归验证

- `npm test -- --run`：25 文件 / 74 测试全绿
- `go test ./internal/token/...`：5 测试全绿
- `go vet ./...`：无错误
- `cargo check --workspace`：无错误
- `npx tsc --noEmit`：无错误
