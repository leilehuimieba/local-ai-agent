# tmp 目录说明

本目录用于存放临时文件、实验代码、构建缓存与中间产物。

## 目录约定

1. **实验性子目录**：以具体任务或日期命名（如 `cargo-target/`、`frontend-wave1-regression/`）。
2. **自动清理**：CI 不保留本目录内容；本地开发应定期清理超过 7 天的子目录。
3. **禁止提交**：除本 `README.md` 外，其他内容不应进入 Git 跟踪。

## 当前子目录

| 目录 | 用途 | 建议保留期限 |
|------|------|-------------|
| `cargo-target/` | Rust 构建缓存 | 可长期保留 |
| `ccr-snippets/` | 代码片段与实验 | 7 天 |
| `frontend-wave1-regression/` | 前端回归测试产物 | 每次发布后清理 |

## 清理脚本

```powershell
# 清理 7 天未修改的非保留目录
Get-ChildItem -Directory | Where-Object {
  $_.Name -notin @("cargo-target") -and
  $_.LastWriteTime -lt (Get-Date).AddDays(-7)
} | Remove-Item -Recurse -Force
```
