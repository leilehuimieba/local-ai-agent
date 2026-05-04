# 验证

## 代码变更

- `frontend/package.json`
  - 新增 devDependencies：`@playwright/test`、`serve`
  - 新增 script：`test:e2e`
- `frontend/playwright.config.ts`
  - Playwright 配置：Mobile Chrome（Pixel 5 视口），webServer 自动启动静态文件服务器
- `frontend/e2e/mobile-layout.spec.ts`
  - 4 项移动端 E2E 测试：底部导航栏、左侧边栏隐藏、Sheet 面板交互、Composer 存在

## 构建验证

- `pnpm exec playwright test`：4/4 通过

## 运行方式

```bash
cd frontend
npm run build
pnpm exec playwright test
# 或
pnpm test:e2e
```
