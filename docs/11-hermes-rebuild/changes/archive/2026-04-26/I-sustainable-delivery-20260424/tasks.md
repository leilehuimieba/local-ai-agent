# Tasks

- [x] I-01 清理 Rust `unused`/`dead_code` warning（25 个）
  - 完成判据：`cargo test --workspace` 无 warning 输出。
  - 验证方式：运行测试并检查输出。
  - 证据落点：`tmp/stage-i-sustainable/cargo-test-clean.log`
  - 实际结果：2026-04-24 完成，169 passed / 0 failed，warning 清零。

- [x] I-02 建立全量回归脚本 `scripts/run-full-regression.ps1`
  - 完成判据：单脚本可串行跑完 Rust/Go/前端/E2E 四项检查。
  - 验证方式：执行脚本，exit code 0。
  - 证据落点：`tmp/stage-i-sustainable/regression-report.json`
  - 实际结果：2026-04-24 完成，6 项检查全部 passed（含 E2E strict_runtime_terminal）。

- [x] I-03 治理 LF/CRLF 混用
  - 完成判据：`git diff` 不再输出 LF→CRLF warning。
  - 验证方式：任意修改后 `git diff --stat` 无 warning。
  - 证据落点：`.gitattributes` + `git add --renormalize .` 执行记录
  - 实际结果：2026-04-24 完成，新增 `.gitattributes`，仅 1 个文件需标准化。

- [x] I-04 建立发布标签规范并打基线标签
  - 完成判据：存在 `v1.0.0-20260424` 标签，且 `git checkout` 后可编译。
  - 验证方式：`git tag` 查看并 checkout 验证编译。
  - 证据落点：`tmp/stage-i-sustainable/tag-verify.log`
