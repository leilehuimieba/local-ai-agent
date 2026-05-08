# 技术方案

## 影响范围

- 涉及模块：
  - `MainlineShell`
  - UI store 的主线总控壳状态
  - 新增晚间证据包面板 / 表单组件
- 涉及文档或 contract：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/evidence-and-probability-rules.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AX-mainline-control-agent-20260507/product-one-pager.md`
  - 本 change 的 `proposal.md`、`tasks.md`、`status.md`、`verify.md`

## 方案

- 核心做法：
  - 在 AY 的 `MainlineShell` 展开态内增加“晚间证据包”打开能力
  - 新增一个轻量证据包面板组件
  - 第一轮只提交固定核心字段：
    - 今天做了什么
    - 可验证结果
    - 失败或卡点
    - 下一步调整
  - 表单先写入前端状态，不接后端
  - 提交后让总控壳显示“今晚证据已提交”

## 风险与回退

- 主要风险：
  - 证据包表单直接塞进主壳后，导致插件过重
  - 表单状态与原有运行态混在一起，造成后续难拆
- 回退方式：
  - 第一轮只保留最小字段
  - 表单状态先挂 UI store，后续再抽离
  - 若交互过重，允许先收缩成简单弹层或内嵌区块
