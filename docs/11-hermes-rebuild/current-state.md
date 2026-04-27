# Hermes 当前执行状态（单一事实源）

更新时间：2026-04-27
状态：`自由迭代期：无当前活跃 change`

本文件是 `docs/11-hermes-rebuild/` 下关于"当前阶段 / 当前 Gate / 当前活跃 change"的唯一权威记录。

## 1. 当前执行状态

1. 当前阶段：阶段 I（可持续交付与工程治理）
2. 当前 Gate：Gate-I（已收口）
3. 当前活跃 change：[R-productization-mvp-20260427](docs/11-hermes-rebuild/changes/R-productization-mvp-20260427/)
4. 上一主推进目录：`docs/11-hermes-rebuild/changes/archive/2026-04-27/`
5. 当前主推进目录：`docs/11-hermes-rebuild/changes/R-productization-mvp-20260427/`

## 2. 阶段 I 收口结论

1. I-01 已完成：25 个 Rust warning 已清理。
2. I-02 已完成：全量回归脚本 `run-full-regression.ps1` 已建立，6 项检查全绿。
3. I-03 已完成：`.gitattributes` 已添加，LF/CRLF warning 已消除。
4. I-04 已完成：基线标签 `v1.0.0-20260424` 已打。
5. 阶段 I 收口，项目进入自由迭代期。

## 3. 近期完成项（自由迭代期）

1. **O-change（2026-04-26）**：六轮清理 — 前端 API 错误统一、workspaceViewModel 拆分、router_release.go 业务下沉、useSettings 抽离、Go 工具函数收拢、tmp 清理。已归档。
2. **P-change（2026-04-27）**：router.go 聚合逻辑拆分 — 提取 `settings_response.go`，router.go 782 行 → 547 行。已归档。
3. **Q-change（2026-04-27）**：CSS 功能域拆分 — `app-views.css` 1564 → 440 行，`app-components.css` 1344 → 441 行，新增 12 个功能域 CSS 文件。已归档。

## 4. 阶段 H 收口结论

1. Gate-H 已签收（H-01~H-05 全部闭环）。
2. H-memory-object-review-20260423 已提审，等待主控裁决（不切主推进）。
3. H-runtime-strict-e2e-20260424 已收口并归档。
4. H-gateway-service-extraction-20260424 已归档。
5. 总路线 A~H 全部完成，代码已全量入库。

## 5. 口径边界

1. 除本文件外，其他文档中出现的阶段或 Gate 描述，默认按"历史记录或上下文说明"处理。
2. 如 `changes/INDEX.md`、各 change 的 `status.md` 与本文件冲突，以本文件为准。
3. `docs/archive/` 下所有阶段描述默认不参与当前状态判定。
4. 阶段 H 收口不表示项目停止演进，只表示原总路线定义的范围已完成。
5. H-02/H-03 的长期治理缺口继续按风险接受条件跟踪，不纳入阶段 H 收口阻塞。

## 6. 更新规则

1. 新建或切换主推进项时，先更新本文件，再更新 `changes/INDEX.md`。
2. 进入提审或 Gate 结论变更时，先更新本文件，再更新阶段计划或阶段计划状态。
3. 每次更新本文件时，同时补"更新时间"。
