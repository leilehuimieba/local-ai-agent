# 设计：文件上传接入后端知识库

## 方案概述

在 `handleFileSelect` 中，读取文件文本后增加异步调用 `uploadKnowledgeFile`，成功或失败均通过 sonner toast 提示用户。

## 修改点

### `frontend/components/local-agent/views/task-view.tsx`

1. **导入**：增加 `uploadKnowledgeFile` from api
2. **`handleFileSelect`**：
   - 扩展文件类型过滤：`.txt,.md,.pdf,.docx`
   - 扩展大小限制：`5 * 1024 * 1024`（5MB）
   - 读取文本后，对每个文件调用 `uploadKnowledgeFile`
   - 成功 toast：`「filename」已保存到知识库`
   - 失败 toast：`「filename」保存到知识库失败`
3. **文件 input**：`accept=".txt,.md,.pdf,.docx"`

## 时序

```
用户点击 Paperclip → 选择文件
  → handleFileSelect
    → file.text() 读取内容
    → setAttachedFiles() 附加到 Composer
    → uploadKnowledgeFile(file) 异步上传
      → 成功: toast.success
      → 失败: toast.error
用户发送消息
  → 文件内容拼接到消息（现有逻辑不变）
```
