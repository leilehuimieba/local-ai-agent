# Phase 2 验证记录

## 后端 API 验证

```bash
# 1. 列表会话（空表返回空数组）
curl -H "X-Local-Agent-Token: $TOKEN" http://127.0.0.1:8897/api/v1/sessions
# → {"items":[]}

# 2. 添加消息（自动创建 session）
curl -X POST -H "Content-Type: application/json" -H "X-Local-Agent-Token: $TOKEN" \
  -d '{"role":"user","content":"hello"}' \
  http://127.0.0.1:8897/api/v1/sessions/test-sess/messages
# → {"id":"msg_...","session_id":"test-sess","role":"user","content":"hello","timestamp":"..."}

# 3. 获取消息
curl -H "X-Local-Agent-Token: $TOKEN" http://127.0.0.1:8897/api/v1/sessions/test-sess/messages
# → {"items":[{"id":"msg_...",...}]}
```

## 前端构建验证
- `cd frontend && npm run build` 静态导出成功
- `cd frontend && npm run test` 11/11 通过
- `cd frontend && npx tsc --noEmit` 无类型错误

## UI 验证
- Playwright 截图 `tmp/file-upload-ui.png`：文件上传按钮（📎）在 Composer 中正常显示
- Playwright iPhone 375×812：底部导航栏和 Sheet 侧滑正常
