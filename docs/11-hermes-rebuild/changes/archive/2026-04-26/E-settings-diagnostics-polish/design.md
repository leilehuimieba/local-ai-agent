# 技术方案

## 影响范围

- 涉及模块：
  1. `frontend/src/settings/api.ts`
  2. `frontend/src/settings/SettingsSections.tsx`
  3. `frontend/src/history/useHistoryReview.ts`
  4. `frontend/src/ui/primitives/MetaGrid.tsx`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`

## 方案

- 核心做法：
  1. 诊断摘要动作改为直接导出 JSON 文件，复用下载链路，移除对 `window.open` 的成功依赖。
  2. 诊断时间展示增加统一格式化函数，输出本地时间字符串。
  3. 记录页搜索字段补齐 `error` 相关字段、中文状态词字段，提升检索命中。
  4. `MetaGrid` key 改为带索引的稳定 key，避免同标签冲突导致 React 告警。
- 状态流转或调用链变化：
  1. 设置页“打开诊断信息”由“打开新窗口”变为“触发下载并回填成功提示”。
  2. 记录筛选搜索仍走原 `useHistoryReview` 过滤链，仅扩展 `searchableText` 输入字段。

## 风险与回退

- 主要风险：
  1. 下载行为在部分浏览器策略下需要用户交互触发，若调用链改动不当可能被拦截。
  2. 搜索字段扩展后可能引入过宽匹配，影响筛选精度。
- 回退方式：
  1. 保留接口与组件结构不变，必要时可按文件级回退到上一提交。
  2. 搜索扩展点集中在 `useHistoryReview.ts`，可局部回滚字段增量。
