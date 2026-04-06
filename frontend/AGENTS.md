# 前端目录规则

## 作用域与优先级

1. 本文件作用于 `frontend/` 及其所有子目录。
2. 本文件补充仓库根目录 [AGENTS.md](d:\newwork\本地智能体\AGENTS.md) 的规则。
3. 前端目录内发生冲突时，以本文件为准。

## P4 前端规则

1. React 组件只使用函数组件，不使用 class 组件或 `React.Component`。
2. 新增跨组件或跨页面状态管理时，统一使用 `zustand`。
3. 组件内部的局部临时状态，继续使用 `useState`、`useReducer` 等 React 原生能力。
