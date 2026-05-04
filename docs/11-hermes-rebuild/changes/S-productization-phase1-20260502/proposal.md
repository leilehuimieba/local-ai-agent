# S-change: 产品化第一阶段 — MVP 稳定性与体验治理

## 背景

产品化成熟度评估 32/100，前端零测试、无 Error Boundary、Toast 未接入、用户文档缺失、session 历史无法恢复。

## 目标

将产品化成熟度提升至 55+/100，达到"可内部试用"水准。

## 范围

1. Error Boundary + 全局 Toast
2. 前端核心单元测试（Vitest + RTL）
3. 用户快速入门文档
4. 按 session 隔离 localStorage，支持历史恢复
