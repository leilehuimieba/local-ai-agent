# H-stage-definition-prep-20260415（design）

更新时间：2026-04-15
状态：草案

## 设计概览

本 change 不做功能开发，仅建立 H 阶段的执行合同：

- 阶段目标合同
- Gate-H 验收合同
- 状态切换合同（谁先改、谁后改）

## 核心设计点

### 1. 阶段合同

H 阶段主线固定为：

1) H-01 透明执行
2) H-02 修复与接管
3) H-03 MCP/Skills 质量
4) H-04 学习模式与浏览器辅助
5) H-05 记忆路由与知识沉淀
6) H-G1 阶段签收

### 2. 切换时序合同

1. 先落盘阶段定义文档与本 change 评审。
2. 再更新 `current-state.md`（阶段/Gate/活跃 change）。
3. 再更新 `changes/INDEX.md`（主推进项与并行项）。
4. 再创建 H-01/H-04 等执行类 change。

### 3. 冲突处理合同

若 `INDEX.md`、子 change `status.md` 与 `current-state.md` 不一致：

- 统一以 `current-state.md` 为准
- 暂停推进并输出冲突点

## 回退设计

1. 若切换后发现文档不一致：
   - 回滚 `current-state.md` 到 G 口径
   - 将 H 线 change 标记为预备
2. 禁止“口头切换、文档未切换”的隐式状态
