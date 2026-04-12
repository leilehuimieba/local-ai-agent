# Hermes Change 索引

更新时间：2026-04-12

这个文件用于标记当前执行主线下的活跃 change，避免多项变更并行时上下文漂移。

## 当前活跃 change

1. [F-release-rollback-closure](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-release-rollback-closure/status.md)（2026-04-12 进入下一 change：warning 协议接入发布/回滚固定流程）
2. [E-backend-reverify-pack](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-backend-reverify-pack/status.md)（2026-04-12 后端一键复核包与 warning 样本链已收口，可作为 F 口径输入）

## 最近收口

1. [E-cli-cancel-slice2](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-cli-cancel-slice2/status.md)（2026-04-12 完成 E-01 CLI/TUI 中断切片后端收口）
2. [E-cli-history-slice1](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-cli-history-slice1/status.md)（2026-04-12 完成 E-01 CLI/TUI 历史切片后端收口）
3. [F-gate-f-signoff](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-gate-f-signoff/status.md)（2026-04-12 完成 F-G1 Gate-F 评审与发布决策）
4. [F-windows-10min-verification](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-windows-10min-verification/status.md)（2026-04-12 完成 F-05 Windows 新机 10 分钟验证）
5. [F-release-candidate-regression](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/F-release-candidate-regression/status.md)（2026-04-12 完成 F-04 发布候选回归与故障注入）

## 并行窗口

1. [E-frontend-experience-upgrade](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md)（2026-04-12 已暂停，不阻塞后端主线；由用户前端窗口恢复推进）

## 选择规则

1. 继续任务时，优先读取“当前活跃 change”。
2. 如果用户明确点名某个 change，以用户指定为准。
3. 如果存在多个活跃项，优先选择与当前阶段一致且最近更新时间最新的 change。
4. 如果 `INDEX.md` 与各 change 的 `status.md` 冲突，先指出冲突并暂停推进。

## 维护规则

1. 新建中等以上变更后，将其加入索引。
2. 某个 change 进入主推进状态时，把它放到“当前活跃 change”第一位。
3. 某个 change 完成或暂停后，及时更新索引说明。
