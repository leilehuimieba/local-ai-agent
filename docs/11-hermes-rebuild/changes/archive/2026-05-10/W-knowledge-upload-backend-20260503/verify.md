# 验证

## 代码变更

- `frontend/components/local-agent/views/task-view.tsx`
  - 导入增加 `uploadKnowledgeFile`
  - `handleFileSelect`：读取文件内容后异步调用 `uploadKnowledgeFile`，附带 toast 成功/失败提示
  - 文件 input `accept` 扩展为 `.txt,.md,.pdf,.docx`
  - 文件大小限制从 1MB 扩展到 5MB

## 构建验证

- `npx tsc --noEmit`：通过
- `npm run build`：通过

## 逻辑验证

1. 文件内容继续拼接到用户消息（现有行为不变）
2. 同一文件对象同时上传至后端知识库
3. 上传结果通过 Toast 反馈给用户
4. 后端已支持 pdf/docx 提取，前端 accept 与之对齐
