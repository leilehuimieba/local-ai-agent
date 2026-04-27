# R-change 任务清单

## R-01 敏感信息治理

- [x] 新增 `.env.example`
- [x] 扩展 `gateway/internal/config/config.go` 环境变量覆盖（支持 `LOCAL_AGENT_API_KEY_<PROVIDER>` 和 `LOCAL_AGENT_BASE_URL_<PROVIDER>`）
- [x] 清理 `config/app.json` 中的真实 API key（scnet 改为空字符串）
- [x] 验证 gateway 编译通过（`go vet ./...` 无错误），前端测试 25/74 全绿

## R-02 LICENSE + 元数据 + CHANGELOG

- [x] 根目录新增 `LICENSE`（MIT）
- [x] `frontend/package.json` 补充元数据（description、license、repository）
- [x] `crates/runtime-core/Cargo.toml` 补充元数据（description、license、repository）
- [x] `crates/runtime-host/Cargo.toml` 补充元数据（description、license、repository）
- [x] 根目录新增 `CHANGELOG.md`

## R-03 Rust Release 构建优化

- [x] 根 `Cargo.toml` 新增 `[profile.release]`（lto、codegen-units=1、panic=abort）
- [x] `install-local-agent.ps1` 改用 `cargo build --release`
- [x] 验证 release 产物：3.11 MB（对比 debug 5.66 MB，减小 45%）

## R-04 前端生产配置

- [x] 新增 `frontend/.env.example` 和 `.env.production`
- [x] `vite.config.ts` 补充生产优化（sourcemap、manualChunks vendor 分割）
- [x] 新增 `frontend/public/favicon.svg`
- [x] `index.html` 补充 meta description、theme-color、favicon 链接
- [x] 前端 API 使用相对路径（生产环境由 gateway 同源托管，无需 base URL 切换）

## R-05 最小启动认证

- [x] gateway 启动时生成随机 token（`token.LoadOrCreate`）
- [x] 所有 `/api/*` 请求校验 token（`token.Middleware`，`/health` 豁免）
- [x] gateway 向 `index.html` 注入 token meta（`injectTokenAndServe`）
- [x] 前端 fetch 拦截器自动附加 token（`main.tsx` 全局拦截）
- [x] launcher 读取 token 调用 `/api/v1/system/info`
- [x] 单元测试验证：无 token 返回 401、有 token 通过、/health 豁免

## R-06 统一构建脚本

- [x] 新建 `scripts/build-all.ps1`（支持 debug/release 配置、产物复制、BUILD_INFO.txt）

## R-07 代码质量工具

- [x] 前端 ESLint + Prettier 配置（`eslint.config.js`、`.prettierrc`、package.json 依赖）
- [x] 前端 `lint` / `format` 脚本
- [x] Go `.golangci.yml` 配置
- [x] Rust `rustfmt.toml` 配置
- [x] CI 新增 lint/fmt/clippy job（`ci.yml`）

## R-08 回归验证

- [x] `npm test -- --run`：25 文件 / 74 测试全绿
- [x] `go test ./internal/token/...`：5 测试全绿，`go vet ./...` 无错误
- [x] `cargo check --workspace`：无错误
- [x] `npx tsc --noEmit`：无错误
