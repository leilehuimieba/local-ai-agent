# 归档索引 — 2026-04-24

## 当日归档

1. [H-modularity-hardening-20260419](H-modularity-hardening-20260419/status.md)
   - 内容：仓库模块化收敛与热点文件减重
   - 完成项：gateway router.go 按域拆分、runtime-core 热点文件目录化、frontend 热点减重、仓库对外表达与工程护栏补齐
   - 归档原因：四批全部完成并验证通过，无遗留阻塞
   - 原始路径：`docs/11-hermes-rebuild/changes/H-modularity-hardening-20260419/`

2. [H-gateway-service-extraction-20260424](H-gateway-service-extraction-20260424/status.md)
   - 内容：gateway `api` 包业务逻辑抽取到独立 `service` 层
   - 完成项：
     - provider_settings.go / diagnostics.go 业务逻辑下沉、handler 层纯化、测试文件迁移适配；
     - Chat 核心链路（context/provider/retry/confirmation/execution/execute/events）全下沉；
     - Learning 系列（value_score/recommend/explain/translate）全下沉，api 层改为类型别名 + 转发。
   - 归档原因：编译与测试全部通过，handler 行数符合目标
   - 原始路径：`docs/11-hermes-rebuild/changes/H-gateway-service-extraction-20260424/`
