# 阶段评审材料（T31）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`

## 1. 达标项

1. `G1` 基础可运行已达标（`T00-T11` 完成，鉴权、开关回退、最小 ingest/recall 冒烟通过）。
2. `G2` 主链路适配已达标（ingest/recall 适配、去重、重试、审计、单测补齐）。
3. `G3` 可视化消费已达标（Markdown 导出、关系链接、graphify 构图、多跳与可解释节点验收）。
4. `G4` 质量治理已达标（固定评测集批跑、低质量清洗、敏感信息拦截、回退演练）。
5. `G5` 交付收口推进中（`T29-T30` 已完成）。

## 2. 未达标项

1. `T32` 尚未完成（下一阶段 backlog 冻结待执行）。

## 3. 风险项

1. 中文 query 直召回稳定性仍需持续观测（`T25` 批次中存在 `fallback` 查询模式）。
2. 外部记忆服务依赖本地容器与网络环境，需维持回退可用。
3. 清洗规则当前以关键词/规则为主，后续需结合更细粒度质量评分。

## 4. 回退项

1. 外部记忆可通过 `external-memory-cortex.json` 一键回退到 `enabled=false`。
2. 回退演练脚本：`scripts/cortex/run-external-memory-rollback-drill.ps1`。
3. 回退后本地主链路连续性已验证通过（见 `tmp/stage-e-rollback-drill/latest.json`）。

## 5. 评审结论建议

1. 建议通过本轮评审并进入 `T32`（冻结下一阶段 backlog）。
