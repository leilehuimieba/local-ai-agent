# 技术方案

## 影响范围

- 涉及模块：
  1. `crates/runtime-core/src/sensitive_data.rs`
  2. `crates/runtime-core/src/knowledge_store.rs`
  3. `crates/runtime-core/src/knowledge.rs`
  4. `crates/runtime-core/src/lib.rs`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/E-sensitive-pattern-expansion/*`

## 方案

- 核心做法：
  1. 新增 `sensitive_data` 共享模块，统一敏感识别入口 `contains_sensitive_text`。
  2. 保留原有密钥/令牌标记，同时扩展三类模式：
     - 邮箱格式（`local@domain`）
     - 中国手机号（11 位 `1[3-9]` 开头，兼容 `86` 前缀）
     - 身份证号（17 位数字 + 1 位数字或 `X/x`）
  3. `knowledge_store` 与 `knowledge` 均改为调用共享入口，避免双份规则漂移。
  4. 补充入库拦截与召回脱敏单测覆盖新增模式。
- 状态流转或调用链变化：
  1. 知识写入链路与外部召回链路无结构调整，仅增强敏感识别分支。

## 风险与回退

- 主要风险：
  1. 模式识别可能产生少量误判，导致边界文本被拦截或脱敏。
  2. 规则收紧后，短期内入库通过率可能下降。
- 回退方式：
  1. 回退 `sensitive_data` 模块扩展逻辑，保留密钥/令牌原规则。
  2. 若出现误判集中，可仅保留高置信规则并逐项恢复。
