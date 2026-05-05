# 技术方案

## 影响范围

- 涉及模块：`frontend/components/local-agent/views/settings-view.tsx`、`frontend/lib/local-agent/types.ts`、`frontend/lib/local-agent/api.ts`、必要时的 `gateway/internal/api/*` 只读字段。
- 涉及文档或 contract：`docs/frontend-acceptance.md`、`docs/11-hermes-rebuild/changes/INDEX.md`、`docs/11-hermes-rebuild/current-state.md`。

## 方案

- 核心做法：在设置页补一个只读的 MCP 可观测面板，优先展示工具名称、服务器来源、风险等级、allowlist 命中、审计状态和最近动作。
- 状态流转或调用链变化：优先复用现有 settings / mcp API；如果字段不足，只补最小只读字段，不改执行链、不改工具路由。

## 风险与回退

- 主要风险：信息维度过多导致设置页变得拥挤，或者前端字段缺失造成误读。
- 回退方式：保留现有 MCP 服务器列表与原设置项，只隐藏新的观测面板；如字段暂缺，先降级为“缺失值占位 + 只读列表”。
