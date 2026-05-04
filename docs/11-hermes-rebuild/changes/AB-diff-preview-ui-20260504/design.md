# 设计：diff apply UI 预览

## 范围

本 change 聚焦前端确认体验，复用 Z-change 已有 Runtime `workspace_apply_patch` 能力，不重写 patch 引擎。

## Runtime/Gateway 复用

1. 继续使用 `workspace_apply_patch` 的 `dry_run` 参数获取预览。
2. 继续使用现有 tool trace、confirmation 和 artifact 展示链路。
3. 若现有事件中缺少结构化 report，优先在前端解析 tool output JSON。

## Frontend 设计

1. 在任务视图中识别 diff apply tool result。
2. 为 dry-run report 增加专用预览组件。
3. 文件级展示字段：path、kind、before_chars、after_chars。
4. 失败级展示字段：stage、path、reason、rollback_attempted。
5. 用户确认区提供 apply / cancel 操作，并复用现有确认按钮样式。

## 风险与回退

1. 如果 tool output 不是合法 JSON，降级展示原始文本。
2. 如果确认链路无法复用，先只做 read-only 预览，不执行 apply。
3. 若 UI 变更影响任务流，回退到现有 tool output 展示。
