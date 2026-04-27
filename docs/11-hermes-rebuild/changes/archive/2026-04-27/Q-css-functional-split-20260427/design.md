# 设计文档

## 拆分策略

按选择器前缀提取功能域：

| 新文件 | 来源 | 选择器前缀 | 行数 |
|---|---|---|---|
| `app-release.css` | app-views.css | `.release-*` | 105 |
| `app-model.css` | app-components.css | `.model-*` | 141 |
| `app-timeline.css` | app-views.css | `.timeline-*` | 98 |
| `app-chat.css` | app-components.css | `.chat-*` `.thread-*` `.assistant-*` `.result-*` `.idle-*` `.composer-*` `.state-*` | 351 |
| `app-nav.css` | app-components.css | `.task-*` `.nav-*` `.history-*` | 212 |
| `app-logs.css` | app-views.css | `.logs-*` | 56 |
| `app-settings.css` | app-views.css | `.settings-*` | 42 |
| `app-status.css` | app-views.css | `.status-*` | 33 |

## 保留原则

跨功能域共享的选择器组（如 `.page-shell-header, .section-header, .composer-header`）保留在原文件，避免重复定义。
