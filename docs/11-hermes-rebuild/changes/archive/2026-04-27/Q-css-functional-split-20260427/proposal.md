# Q-css-functional-split：CSS 按功能域拆分

## 背景

架构审计发现 `frontend/src/styles/app-views.css` 1564 行、`app-components.css` 1344 行，超过 1000 行红线。

## 目标

按功能域拆分 CSS，降低单文件维护难度。

## 范围

- 从 `app-views.css` 提取 release 相关样式 → `app-release.css`
- 从 `app-components.css` 提取 model dropdown 相关样式 → `app-model.css`
- 更新 `index.css` 导入顺序

## 回退方式

恢复 `index.css` 导入，删除新文件，将样式合并回原文件即可。
