# 设计：Playwright E2E 测试覆盖移动端

## 方案概述

在 frontend 中引入 `@playwright/test`，配置移动端视口，编写针对移动端适配的 E2E 测试。

## 基础设施

1. **安装依赖**：`pnpm add -D @playwright/test`
2. **安装浏览器**：`npx playwright install chromium`
3. **配置**：`frontend/playwright.config.ts`
   - 使用 `webServer` 启动静态文件服务器（`npx serve dist`）
   - 定义 `Mobile Chrome` 项目（视口 375×667）
4. **测试目录**：`frontend/e2e/`

## 测试用例

### `mobile-layout.spec.ts`

1. **底部导航栏可见**
   - 视口：375×667
   - 断言：底部固定导航栏存在，包含"任务""历史""知识""设置""新任务"

2. **左侧边栏隐藏**
   - 视口：375×667
   - 断言：左侧边栏（`aside`）不可见

3. **Sheet 面板交互**
   - 视口：375×667
   - 打开 Sheet（点击某个触发按钮）
   - 断言：Sheet 内容可见
   - 关闭 Sheet
   - 断言：Sheet 内容不可见

4. **Composer 存在**
   - 视口：375×667
   - 断言：文本输入区域和发送按钮存在

## 运行方式

```bash
cd frontend
pnpm exec playwright test
```
