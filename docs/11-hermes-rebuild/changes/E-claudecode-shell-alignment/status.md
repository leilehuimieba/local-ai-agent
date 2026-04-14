# 当前状态

- 最近更新时间：2026-04-14
- 状态：进行中（`T04-T07` 已完成）
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：
  1. 完成 `ClaudeCodeRev` 关键源码机制抓取与证据定位。
  2. 完成当前仓库 `run_command/权限/压缩` 现状对照。
  3. 已输出差距矩阵与最小 task 分解。
  4. 完成 `T04`：`run_command` 结果合同补齐 `detail_preview/raw_output_ref`，并完成 runtime/gateway/frontend 三端 contract 对齐。
  5. 完成 `T05`：新增网关产物内容读取接口，前端支持基于 `raw_output_ref/artifact_path` 展开命令原文输出，时间线详情优先显示 `detail_preview`。
  6. 完成 `T06`：ask 链路补齐 `confirmation_approved` 事件与统一审计元数据，前端/日志消费侧完成 `permission_* + confirmation_*` 展示收口与状态判定校准（确认通过不再误判为“待确认”）。
  7. 完成 `T07`：会话压缩补齐聚合预算边界（900 字符 / 最近 4 轮），`run_command` 补齐单结果预算阈值（30,000 字符），并把预算命中证据写入可回放 metadata（`result_chars/single_result_budget_*`）。
- 进行中：
  1. 无（本 change 的 `T04-T07` 均已完成）。
- 阻塞点：
  1. 无硬阻塞。
- 下一步：
  1. 等待你确认是否提交本次 `T07` 第二刀改动，或切换到下一个 change。
