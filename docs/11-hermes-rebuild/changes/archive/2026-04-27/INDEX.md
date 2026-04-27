# 2026-04-27 归档索引

## 本次归档变更

1. [O-change：六轮清理](../2026-04-26/) — 已归档至 2026-04-26
2. [P-change：router.go 聚合逻辑拆分](P-router-service-extraction-20260427/) — router.go 业务逻辑下沉
3. [Q-change：CSS 功能域拆分](Q-css-functional-split-20260427/) — 按功能域拆分 CSS
4. [R-change：产品化 MVP 治理](R-productization-mvp-20260427/) — 敏感信息、LICENSE/CHANGELOG、release 构建、前端生产配置、启动认证、构建脚本与质量工具

## 归档说明

1. P/Q/R 均已归档，当前活跃 change 以 `docs/11-hermes-rebuild/current-state.md` 为准。
2. Q-change 验证状态：25 files / 74 tests passed，TypeScript 检查通过。
3. R-change 验证状态：前端测试、Go token/knowledge 测试、`cargo check --workspace` 通过。
