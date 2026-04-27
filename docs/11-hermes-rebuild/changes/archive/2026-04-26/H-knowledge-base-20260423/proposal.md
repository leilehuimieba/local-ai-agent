# Knowledge Base 独立页面实现（proposal）

更新时间：2026-04-23

## 背景

当前 Memory（记忆）与 Knowledge Base（知识库）混在同一页面，用户无法主动管理专业资料。知识库应作为用户主动添加、编辑、分类、搜索专业资料的独立入口，与运行时自动沉淀的 Memory 分离。

## 目标

1. 新增独立"知识库"页面，与首页/任务/记录/设置并列
2. 支持知识库列表、条目卡片网格、添加/编辑/删除/搜索
3. 前端先用 localStorage 做 mock 数据层，不阻塞后端 API 开发
4. 不阻塞 Gate-H 聚合复核

## 非目标

1. 不替换现有 Memory 功能
2. 不实现文件上传（PDF/Word/TXT）
3. 不实现与后端 API 的对接（前端 mock 阶段）
4. 不修改 Gate-H 相关状态

## 影响范围

- `frontend/src/shell/LeftNav.tsx`
- `frontend/src/shell/workspaceViewModel.tsx`
- `frontend/src/App.tsx`
- `frontend/src/knowledge-base/`（新增目录）

## 验收标准

1. 左侧导航出现"知识库"图标，点击可切换
2. 知识库页面展示条目卡片网格，支持分类筛选和搜索
3. 可添加/编辑/删除知识条目，数据持久化到 localStorage
4. 空状态有友好提示
