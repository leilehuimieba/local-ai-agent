# 验证记录

## 验证方式

- 后端回归：`go test ./...`（工作目录：`gateway/`）。
- 接口验收（成功链）：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1`。
- 接口验收（一致性链）：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1`。
- 接口验收（失败链）：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry-failure-acceptance.ps1`。
- 人工复核：核对三份 `latest.json` 的 `status`、关键检查项和事件链。

## 证据位置

- 验收脚本：
  - `scripts/run-stage-e-entry1-acceptance.ps1`
  - `scripts/run-stage-e-consistency-acceptance.ps1`
  - `scripts/run-stage-e-entry-failure-acceptance.ps1`
- 证据样本：
  - `tmp/stage-e-entry1/latest.json`
  - `tmp/stage-e-consistency/latest.json`
  - `tmp/stage-e-entry-failure/latest.json`
- 日志与构建输出：
  - `tmp/stage-e-entry1/logs/*`
  - `tmp/stage-e-consistency/logs/*`
  - `tmp/stage-e-entry-failure/logs/*`

## Gate 映射

- 对应阶段 Gate：Gate-E（进行中，不做完成声明）。
- 当前覆盖情况：
  - `E-02` 入口协议成功链已复核通过。
  - `E-04` 跨入口一致性链已复核通过。
  - `E-05` 失败样本与回退路径已收口并形成评审文档。
  - 尚未执行 `E-G1` 批量验收与 Gate-E 正式判定。
