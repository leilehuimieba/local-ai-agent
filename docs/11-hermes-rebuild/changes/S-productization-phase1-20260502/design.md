# 设计文档

## 1. Error Boundary + Toast

- `frontend/components/error-boundary.tsx`: React class ErrorBoundary，捕获渲染错误显示降级 UI（重试按钮）
- `frontend/app/layout.tsx`: 用 ErrorBoundary 包裹 children，挂载 `<Toaster position="top-right" richColors closeButton />`
- `frontend/components/local-agent/views/task-view.tsx`: handleSend / handleCancel / onStreamError 中添加 `toast.error()`

## 2. 前端测试

- 测试框架: Vitest 4 + jsdom + @testing-library/react + @testing-library/jest-dom
- `vitest.config.ts`: @vitejs/plugin-react 处理 JSX，jsdom 环境
- `vitest.setup.ts`: 导入 `@testing-library/jest-dom`
- 测试覆盖: store 核心 action（addMessage, cancelRun, resumeSession）、LightweightMarkdown 渲染

## 3. Session 隔离

- `localStorage` key 从单一 `"la:session"` 改为 `"la:session:${sessionId}"`
- 保留旧 key 作为 backward compatibility（当前活跃 session 的 fallback）
- `resumeSession`: 先 save 当前 session，再从目标 session key load
- `clearSession`: 删除当前 session 的 key（legacy + session-specific）
