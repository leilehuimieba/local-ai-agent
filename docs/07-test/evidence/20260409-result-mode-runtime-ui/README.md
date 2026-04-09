# 本目录已失效

本目录不是本轮“结果包装层 + 前端消费层”验收的有效证据，请不要继续引用。

失效原因：

1. 采样时前后台仍运行旧进程，不是当前源码对应的运行实例。
2. 旧样本里的终态事件存在 `metadata.result_mode = null` 的情况。
3. 这会把 observe 阻止样本错误推回前端兜底路径，导致页面分层失真。

本目录保留仅用于说明“上一轮为何会出现误判”，不再作为通过依据。

本轮唯一有效证据目录：

- [20260409-result-mode-runtime-ui-closure](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/README.md)
