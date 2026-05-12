# AF-change：Go errcheck 逐个修复

## 背景

`.golangci.yml` 中通过 `exclude-rules` 统一排除了大量 `errcheck` 告警（`defer Close/Remove/RemoveAll`、`io.Copy` 等）。这些排除规则降低了代码质量，掩盖了真实的错误处理缺口。

## 目标

逐个修复所有被排除的 errcheck 告警，然后移除 `.golangci.yml` 中的 errcheck 排除规则，使 linter 通过零排除。

## 范围

Gateway 目录下所有 Go 源文件中，被当前 `.golangci.yml` errcheck 排除规则掩盖的未处理错误返回值。

## 方案

1. 临时移除 `.golangci.yml` 中的 errcheck 排除规则
2. 运行 `golangci-lint run` 收集全部 errcheck 告警
3. 按文件分组，逐个修复：
   - `defer x.Close()` → 提取为具名函数或添加 `_ = x.Close()`（若确实无需处理）
   - `os.Remove/RemoveAll` → 添加显式错误处理或日志
   - `io.Copy` → 检查返回值
4. 验证修复后 golangci-lint 全绿
5. 更新 `.golangci.yml`，永久移除 errcheck 排除规则

## 验收标准

1. `golangci-lint run ./gateway/...` 0 errors
2. `.golangci.yml` 中无 errcheck 相关 exclude-rules
3. Go test 全绿
