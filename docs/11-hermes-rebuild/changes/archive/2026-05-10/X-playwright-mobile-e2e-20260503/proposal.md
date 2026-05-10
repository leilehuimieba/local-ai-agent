# X-change: Playwright E2E 测试覆盖移动端

## 背景

Phase 2 完成了移动端适配（底部导航栏、Sheet 面板、Composer 折叠、触摸目标等），但缺少自动化回归测试。Playwright 是项目已经使用的浏览器自动化工具（`.playwright-cli/` 和 `.playwright-mcp/` 目录存在），但尚未在 frontend 中建立正式的 E2E 测试套件。

## 目标

建立 Playwright E2E 测试基础设施，并编写覆盖移动端关键交互的测试用例。

## 范围

- **前端**：安装 `@playwright/test`，创建 `playwright.config.ts` 和 `e2e/` 测试目录
- **后端**：无需修改

## 验收标准

1. `pnpm exec playwright test` 可运行
2. 至少覆盖以下移动端场景：
   - 底部导航栏渲染和切换
   - 左侧边栏在移动端隐藏
   - Sheet 面板打开/关闭
   - Composer 区域存在且可交互
