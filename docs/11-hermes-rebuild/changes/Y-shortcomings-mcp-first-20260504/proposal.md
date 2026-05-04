# Y-change: 短板补齐第一阶段

## 背景

2026-05-04 的竞品对比结论显示，当前项目要从“可演示本地智能体”继续走向“可用的本地 Agent 控制台”，最先需要补齐 MCP 生态入口、Runtime 自动工具调度、安全治理、代码修改闭环和 Windows 产品化体验。

## 目标

1. 先完成 MCP 添加/删除持久化与 Gateway Manager 热重载。
2. 为后续 Runtime tool registry 集成 MCP 工具建立配置与状态基础。
3. 将 MCP 工具 allowlist、risk level、审计字段纳入下一步设计范围。
4. 将 Aider/Codex 风格 diff apply 和 Windows doctor/启动器作为后续 P1，不塞入本次首个实现切片。

## 非目标

1. 本 change 首轮不实现 Runtime 自动选择 MCP 工具。
2. 本 change 首轮不接 stdio MCP transport。
3. 本 change 首轮不实现代码 diff apply。
4. 本 change 首轮不重做 Windows 安装器。

## 验收标准

1. 设置页添加 MCP Server 后，`config/app.json` 持久化新增项。
2. 设置页删除 MCP Server 后，`config/app.json` 持久化移除项。
3. Gateway 内存中的 MCP Manager 在新增/删除后热替换服务器列表并重新连接。
4. 后端测试覆盖新增、重复 URL 拒绝、删除和配置写回。
5. change 下 `tasks.md`、`status.md`、`verify.md` 持续记录实现与验证证据。
