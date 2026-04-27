# Hermes Change 索引

更新时间：2026-04-26

这个文件用于提供 change 目录导航。
"当前阶段 / 当前 Gate / 当前活跃 change"的状态统一以 `docs/11-hermes-rebuild/current-state.md` 为准。

## 当前活跃 change

1. [P-router-service-extraction-20260427](P-router-service-extraction-20260427/)：router.go 聚合逻辑拆分至 api/settings_response.go

## 归档入口

1. [archive/2026-04-26/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-26/INDEX.md)（D~N 阶段收口项与 H-02/H-03 保留观察归档入口，共 26 项）
2. [archive/2026-04-24/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-24/INDEX.md)（H-modularity-hardening、H-gateway-service-extraction 归档入口）
3. [archive/2026-04-23/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-23/INDEX.md)（前端重新设计、前端工作台重构归档入口）
4. [archive/2026-04-15/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-15/INDEX.md)（阶段 G 已收口项与 `F-memory-progressive-disclosure-20260414` 归档入口）
5. [archive/2026-04-14/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-14/INDEX.md)（`E-claudecode-shell-alignment`、`E-sensitive-pattern-expansion` 归档入口）
6. [archive/2026-04-13/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-13/INDEX.md)（本轮文档治理收口归档）
7. [archive/2026-04-12/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-12/INDEX.md)（更早已收口 change 归档）

## 选择规则

1. 继续任务时，先读 `current-state.md`，再定位对应 change 目录。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果 `INDEX.md` 或任意 `status.md` 与 `current-state.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等及以上变更后，将其加入索引。
2. 切换主推进项时，先更新 `current-state.md`，再更新本索引。
3. 某个 change 完成并收口后，移动到 `archive/<日期>/` 并补归档索引。
