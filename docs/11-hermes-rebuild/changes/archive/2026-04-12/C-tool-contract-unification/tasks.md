# 任务清单

- [x] 激活阶段 C 主推进 change 并更新索引
  完成判据：`changes/INDEX.md` 的“当前活跃 change”指向本 change。
- [x] 补齐本 change 五件套并冻结范围
  完成判据：`proposal/design/tasks/status/verify` 文件齐全，且范围对齐阶段 C。
- [x] 盘点跨端 Tool Contract 差异并产出字段对齐表
  完成判据：形成 runtime/gateway/frontend 三端字段差异表与统一口径。
- [x] 补齐 `RuntimeContextSnapshot` 跨端缺失字段
  完成判据：gateway/frontend 已同步 `phase_label`、`selection_reason`、`prefers_artifact_context`、`artifact_hint` 字段并通过构建。
- [x] 补齐 `tool_elapsed_ms` 成功/失败双路径
  完成判据：runtime 能采集工具执行耗时，并在成功链路（`verification_completed`、`run_finished`）与失败链路输出 `tool_elapsed_ms`。
- [x] 完成工具合同最小实现与测试
  完成判据：至少覆盖参数、错误码、trace id、耗时字段，并通过对应测试。
- [x] 补齐 `tool_elapsed_ms` 接口级验收样本
  完成判据：`chat/run -> logs` 样本中可复核 `verification_completed` 与 `run_finished` 的 `metadata.tool_elapsed_ms`。
- [x] 接入风险分级确认链最小闭环并补齐审计字段
  完成判据：高风险动作能稳定触发确认链，日志可追溯到调用、确认与结果。
- [x] 补齐 Gate-C 对应验证证据
  完成判据：`verify.md` 中包含拦截准确率、失败可定位率、审计字段完整性证据。
- [x] 输出阶段 C 提审收口包并给出阶段切换判定
  完成判据：形成 `review.md`，包含 Gate-C 判定、证据映射、风险回退与阶段切换建议。
