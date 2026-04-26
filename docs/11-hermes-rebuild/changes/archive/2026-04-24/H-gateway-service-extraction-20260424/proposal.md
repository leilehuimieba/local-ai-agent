# 提案

将 gateway `internal/api` 包中混有 HTTP 协议与业务逻辑的 handler 文件，按域抽取为 `internal/service` 层的纯业务函数，使 service 函数可独立单元测试，handler 层仅保留路由注册、请求解码、参数校验和响应写入。
