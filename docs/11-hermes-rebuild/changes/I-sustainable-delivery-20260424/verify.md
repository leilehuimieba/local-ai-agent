# Verify

## 验证方式

1. 清理 warning 后重跑 `cargo test --workspace`，确认输出无 warning。
2. 执行 `scripts/run-full-regression.ps1`，确认 exit code 0 且报告完整。
3. `git diff` 任意文件后确认无 LF→CRLF warning。
4. `git checkout v1.0.0-20260424` 后确认三端编译通过。

## 证据位置

1. `tmp/stage-i-sustainable/cargo-test-clean.log`
2. `tmp/stage-i-sustainable/regression-report.json`
3. `tmp/stage-i-sustainable/tag-verify.log`
