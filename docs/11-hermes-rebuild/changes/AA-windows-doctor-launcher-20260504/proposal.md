# AA-change: Windows doctor 与一键启动体验

## 背景

Y-change 已补齐 MCP 能力入口与安全治理，Z-change 已补齐 Runtime diff apply 最小闭环。下一块短板是本地产品可用性：用户在 Windows 上需要稳定启动、清晰诊断和可见的服务状态，而不是在 Gateway、Runtime、Frontend 与 MCP 之间手工排错。

## 目标

1. 建立 Windows doctor 统一诊断口径，覆盖依赖、端口、Token、目录可写性、Gateway、Runtime、MCP 与模型服务商基础状态。
2. 将一键启动器调整为“先诊断、再启动、再验活”的流程。
3. 前端增加服务状态面板，集中展示 Gateway、Runtime、MCP、模型服务商、会话存储和知识库状态。
4. 诊断结果必须可读、可定位，并给出下一步处理提示。
5. 保持现有 Gateway 入口与 Token 认证要求。

## 非目标

1. 本 change 不实现完整安装包或自动升级器。
2. 本 change 不接入系统托盘、Windows Service 或后台守护进程。
3. 本 change 不实现 diff apply UI 预览；该项作为后续 `AB-diff-preview-ui` 候选 change。
4. 本 change 不实现 MCP server 自动安装市场。

## 验收标准

1. Gateway 提供服务状态 API，前端可展示关键服务健康状态。
2. doctor 能返回结构化检查结果，包含状态、严重级别、说明和建议动作。
3. 一键启动器启动前执行诊断，启动后等待 Gateway 与 Runtime 验活。
4. 前端状态面板能展示 Gateway、Runtime、MCP、模型服务商、会话存储和知识库状态。
5. `go test ./...` 与前端相关测试通过，文档记录验证证据。
