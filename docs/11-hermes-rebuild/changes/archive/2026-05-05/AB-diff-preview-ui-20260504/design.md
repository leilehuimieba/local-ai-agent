# 设计：diff apply UI 预览

## 范围

本 change 聚焦前端确认体验，复用 Z-change 已有 Runtime `workspace_apply_patch` 能力，不重写 patch 引擎。

## Runtime/Gateway 复用

1. 继续使用 `workspace_apply_patch` 的 `dry_run` 参数获取预览。
2. 继续使用现有 tool trace、confirmation 和 artifact 展示链路。
3. 若现有事件中缺少结构化 report，优先在前端解析 confirmation payload；仍缺少结构化 dry-run report 时，再由前端从 patch arguments 推断文件级摘要。

## Frontend 设计

1. 在任务视图中识别 diff apply tool result。
2. 为 dry-run report 增加专用预览组件。
3. 文件级展示字段：path、kind、before_chars、after_chars。
4. 失败级展示字段：stage、path、reason、rollback_attempted。
5. 用户确认区提供 apply / cancel 操作，并复用现有确认按钮样式。

## 当前接入口

1. 当前任务页入口为 `frontend/components/local-agent/views/task-view.tsx` 的 `MessageBubble`。
2. Z-change 的 dry-run report 当前主要落在助手消息最终文本中，格式为 `patch dry-run 完成` 加文件行。
3. 预览组件位于 `frontend/components/local-agent/diff-preview.tsx`，优先解析结构化 JSON，失败时解析 Runtime final_answer 文本行。
4. 本轮只做 apply 前确认与只读预览，实际写入仍由既有确认 API 驱动后端继续执行。
5. 失败报告同样由 `DiffPreview` 处理，识别 `success:false` JSON 并展示 stage、path、reason、rollback_attempted。
6. 确认后 apply 当前可复用 `ConfirmationCard` 的 approve/reject API。
7. Runtime / Gateway / Frontend confirmation payload 现已正式带上：
   - `tool_arguments_json`
   - `patch_preview_report_json`（`workspace_apply_patch` 场景）
8. 确认卡优先读取 `patch_preview_report_json`，缺省时从 `tool_arguments_json.diff` 推断接近 `DiffPreview` 的摘要视图；仅在推断失败时回退原始 patch 文本。

## 风险与回退

1. 如果 tool output 不是合法 JSON，降级展示原始文本。
2. 如果确认链路无法复用，先只做 read-only 预览，不执行 apply。
3. 若 UI 变更影响任务流，回退到现有 tool output 展示。
