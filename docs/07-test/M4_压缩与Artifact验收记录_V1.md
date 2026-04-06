# 本地智能体 - M4 压缩与 Artifact 验收记录 V1

更新时间：2026-04-02

状态：已完成实现，待最终人工复核。

## 1. 本轮目标

按 M4 要求，把长输出外置为 artifact，并让会话摘要通过 compaction 控制密度。

## 2. 实现范围

1. 新增 `ArtifactRecord`
2. 新增 `externalize_text_artifact`
3. 新增 `artifact_index_path`
4. 新增 `CompactionResult`
5. 会话压缩改为统一走 `compact_session_turns`

## 3. 修改文件清单

1. `crates/runtime-core/src/artifacts.rs`
2. `crates/runtime-core/src/compaction.rs`
3. `crates/runtime-core/src/paths.rs`
4. `crates/runtime-core/src/execution.rs`
5. `crates/runtime-core/src/session.rs`

## 4. 运行链路变化

调整前：

1. artifact 只是在执行层局部落盘
2. 会话摘要压缩逻辑散落在 `session.rs`

调整后：

1. 长文本统一通过 `externalize_text_artifact()` 外置
2. artifact 会写入索引文件
3. 会话压缩统一通过 `compact_session_turns()`

## 5. 手工测试步骤

1. 发起长输出任务
2. 检查 `data/artifacts/...` 是否生成产物
3. 检查 `data/artifacts/index.jsonl` 是否有记录
4. 查看事件中的 `artifact_path`
5. 连续发多轮对话，检查 session 文件中的 `compressed_summary`

## 6. 构建验证结果

1. `cargo build -p runtime-host`：通过
2. `go build ./...`：通过
3. `npm run build`：通过

## 7. 未完成项

1. artifact 类型目前以文本为主，没有继续细分 `command_output / file_snapshot / verification_report`
2. compaction 还没有根据 token 或大小做更精细分层

## 8. 偏离本规范的地方

1. 没有单独新增 `artifacts.rs` 之外的 artifact API 出口
2. artifact 详情目前主要依赖日志与文件路径，不是独立查询接口

## 9. 是否通过本里程碑验收

结论：通过

判断依据：

1. 大输出已经外置
2. 事件回流只保留摘要与路径
3. 会话压缩已有统一入口
