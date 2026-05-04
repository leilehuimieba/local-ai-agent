# Hermes Change 索引

更新时间：2026-05-04

这个文件用于提供 change 目录导航。
"当前阶段 / 当前 Gate / 当前活跃 change"的状态统一以 `docs/11-hermes-rebuild/current-state.md` 为准。

## 当前活跃 change

暂无。下一主推进项待新建 change 后切换。

## 已完成 change

1. [AA-windows-doctor-launcher-20260504](AA-windows-doctor-launcher-20260504/) — P1：Windows doctor、一键启动器与服务状态自检
2. [Z-diff-apply-20260504](Z-diff-apply-20260504/) — P1：Aider/Codex 风格代码 diff apply 最小闭环
3. [Y-shortcomings-mcp-first-20260504](Y-shortcomings-mcp-first-20260504/) — 短板补齐第一阶段：MCP 添加/删除持久化、Runtime MCP 调用闭环与安全治理
4. [S-productization-phase1-20260502](S-productization-phase1-20260502/) — 产品化第一阶段：Error Boundary + Toast + 前端测试 + 用户文档 + Session 隔离
5. [T-productization-phase2-20260503](T-productization-phase2-20260503/) — 产品化第二阶段：后端结构化会话历史存储 + 移动端适配 + Composer 文件上传 UI
6. [U-assistant-msg-sync-20260503](U-assistant-msg-sync-20260503/) — Assistant 消息同步到后端
7. [V-history-session-list-20260503](V-history-session-list-20260503/) — 历史会话列表 UI
8. [W-knowledge-upload-backend-20260503](W-knowledge-upload-backend-20260503/) — 文件上传接入后端知识库
9. [X-playwright-mobile-e2e-20260503](X-playwright-mobile-e2e-20260503/) — Playwright E2E 测试覆盖移动端

## 归档入口

1. [archive/2026-04-27/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-27/INDEX.md)（P/Q/R 产品化治理与模块化收口归档入口）
2. [archive/2026-04-26/INDEX](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-04-26/INDEX.md)（D~N 阶段收口项与 H-02/H-03 保留观察归档入口，共 26 项）
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
