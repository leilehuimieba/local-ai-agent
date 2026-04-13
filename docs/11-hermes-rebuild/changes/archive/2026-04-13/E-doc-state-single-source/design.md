# 技术方案

## 影响范围

- 直接影响：
  - `docs/11-hermes-rebuild/current-state.md`（新增）
  - `docs/README.md`
  - `docs/11-hermes-rebuild/文档阅读与执行指引.md`
  - `docs/11-hermes-rebuild/changes/INDEX.md`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md`
  - `docs/11-hermes-rebuild/changes/D-memory-skill-foundation/status.md`

## 方案

### 1. 建立单一状态主记录

- 新增 `docs/11-hermes-rebuild/current-state.md`。
- 仅在该文件维护三项状态字段：
  1. 当前阶段
  2. 当前 Gate
  3. 当前活跃 change

### 2. 入口文档改为引用主记录

- `docs/README.md`：把主记录加入“当前唯一执行入口”，并写明状态字段以主记录为准。
- `文档阅读与执行指引.md`：移除硬编码阶段描述，改成“读取主记录”流程。
- `changes/INDEX.md`：保留索引角色，但把“当前活跃 change”判定改成引用主记录。

### 3. 未归档 change 状态头部去硬编码

- 将未归档 change 的 `status.md` 中“当前阶段”改为主记录引用表达。
- change 内部任务进展和验证内容保持原样，不改历史证据。

## 风险与回退

- 主要风险：主记录更新后未同步索引，短期内仍可能产生阅读歧义。
- 回退方式：如发现流程不适配，先回退入口文档引用改动，保留主记录文件作为并行试运行口径。
