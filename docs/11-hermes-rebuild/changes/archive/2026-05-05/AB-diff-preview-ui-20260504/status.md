# 状态记录

更新时间：2026-05-05

## 当前状态

可收口。已完成 tool result / confirmation UI 梳理、只读 diff dry-run 预览组件、失败 report 展示、确认卡直接消费 patch confirmation payload，以及正式前端验收入口。

## 已完成

1. 新建 diff apply UI 预览 change 工作区。
2. 明确本 change 复用 Z-change Runtime patch 能力，不扩 patch 引擎。
3. 已确认第一版接入点为任务页 `MessageBubble`，数据来源为助手消息最终文本中的 patch dry-run 结果。
4. 已新增 `DiffPreview` 组件，支持结构化 JSON 与 Runtime 文本行双解析。
5. 已在助手消息气泡中挂载只读 diff dry-run 预览。
6. 已补组件测试覆盖 JSON、文本、apply 结果不展示、删除和 rename 标签展示。
7. 已支持 `success:false` 失败 report 卡片，展示 stage、path、reason、rollback_attempted。
8. 已补失败 report 解析与渲染测试。
9. 已将 `tool_arguments_json` 与 `patch_preview_report_json` 正式接入 Runtime / Gateway / Frontend confirmation payload。
10. 已为 `workspace_apply_patch` 补风险确认入口，确认请求可直接携带 dry-run report。
11. 已在 `ConfirmationCard` 中优先渲染 `patch_preview_report_json`，缺省时回退展示 `tool_arguments_json` 中的 patch 参数。
12. 已将 patch 场景确认文案收口为“确认应用 / 取消应用”，与后端“确认后 apply、拒绝不写入”语义对齐。
13. 已支持从 `tool_arguments_json.diff` 推断 create / modify / delete / rename 摘要，fallback 展示接近 `DiffPreview`。
14. 已新增浏览器联调用调试页，准备补本地页面证据。

## 进行中

1. 等待提交与归档动作。

## 阻塞点

1. 暂无阻塞。

## 下一步

1. 提交 AB-change 代码与文档。
2. 归档或切换到下一主推进项。
