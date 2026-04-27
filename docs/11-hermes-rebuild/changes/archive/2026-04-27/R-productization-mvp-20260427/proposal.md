# R-change 提案：产品化 MVP 治理

## 背景

项目已完成 Hermes 重构（A~H 阶段）及后续工程治理（O/P/Q）。当前处于自由迭代期，但距产品化仍有显著差距：
- 硬编码 API key 于配置文件中
- 无 LICENSE、CHANGELOG、favicon
- Rust 产物为 debug 模式
- 零代码质量工具（lint/format）
- 版本号全项目停滞于 `0.1.0`

## 目标

在 **6 个工作日**内完成最小可行产品化治理，使项目达到"可公开发布"底线。

## 范围

### 纳入
1. 敏感信息治理（API key 环境变量化）
2. LICENSE + 包元数据 + CHANGELOG
3. Rust release 构建优化
4. 前端生产配置（.env、favicon、vite build）
5. 最小启动认证（gateway 生成 token）
6. 统一构建脚本（`scripts/build-all.ps1`）
7. 代码质量工具接入（ESLint + Prettier + golangci-lint + clippy/rustfmt）

### 不纳入
- 多平台 CI 产物（Windows/macOS/ARM64）
- Docker 支持
- e2e 测试
- 用户手册/文档重写

## 验收标准

- [ ] `config/app.json` 中无真实 API key
- [ ] 根目录存在 `LICENSE`（MIT）
- [ ] `package.json` / `Cargo.toml` 含 `description`、`license`、`repository`
- [ ] `CHANGELOG.md` 存在且含 0.1.0 条目
- [ ] `cargo build --release` 产物可正常运行
- [ ] `npm run build` 产物含 favicon 且 API 指向正确
- [ ] gateway 启动时生成随机 token，前端首次请求带校验
- [ ] `scripts/build-all.ps1` 可独立产出前端 + Go + Rust 产物
- [ ] CI 新增 lint/fmt 检查 job
