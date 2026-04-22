# Wave 3 截图补证入口

## 目的

本文件用于保留 Wave 3 页面级截图补证的执行入口，并说明本轮已完成的补证结果。

当前目标不是继续改代码，而是记录以下证据已经按本入口补齐：

1. Logs 页截图
2. History / Review 视图区截图
3. Settings 页截图
4. Settings 内 Resources 模块截图

本文件只服务于当前并行 change：

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/`

不改变主推进状态，不修改 `current-state.md`。

## 当前页面入口口径

根据当前前端实现，页面入口应按下面口径理解：

1. **Logs**
   - 当前应用内存在 `logs` 视图入口
   - 由顶部导航或工作台动作进入

2. **History / Review**
   - 当前不是独立顶层路由
   - 它挂在 Logs / Review Workspace 内部
   - 主要由筛选台、焦点卡、稳定记录流、详情栏组成

3. **Settings**
   - 当前应用内存在 `settings` 视图入口

4. **Resources**
   - 当前不是独立页面
   - 它挂在 Settings 视图内部的“记忆与资源”模块

因此截图补证时，不要把 History 和 Resources 按“独立页面”处理。

## 截图前准备

### 1. 运行环境

建议确认：

1. 前端预览可用
2. `gateway/runtime` 已启动
3. 设置接口已恢复为 `200`

可复用当前已有口径：

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/settings-response-20260422.json`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/settings-health-20260422.txt`

### 2. 推荐截图命名

建议直接落到：

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/evidence/`

推荐文件名：

1. `wave3-logs-workspace-20260422.png`
2. `wave3-history-review-20260422.png`
3. `wave3-settings-workspace-20260422.png`
4. `wave3-settings-resources-20260422.png`

如果当日不是 2026-04-22，则用实际截图日期替换。

## 具体截图清单

### A. Logs 页截图

入口：

1. 从应用顶部导航进入“记录”
2. 或通过工作台动作进入 Logs 页

截图内容至少应包含：

1. Logs workspace hero
2. 记录摘要 strip
3. 复盘筛选台
4. 稳定记录流
5. 右侧焦点卡或详情轨道的一部分

截图目的：

1. 证明 Logs 已进入 workbench 语言
2. 证明 Logs 与主任务页属于同一产品

建议文件名：

- `wave3-logs-workspace-20260422.png`

### B. History / Review 视图区截图

入口：

1. 进入 Logs 视图
2. 保证左侧已有记录
3. 选中一条稳定记录

截图内容至少应包含：

1. 筛选台
2. 焦点复盘卡
3. 稳定记录流
4. 复盘详情栏

截图目的：

1. 证明 History 已形成 review workspace 语义
2. 证明 detail rail、spotlight、timeline 已形成闭环

建议文件名：

- `wave3-history-review-20260422.png`

### C. Settings 页截图

入口：

1. 从顶部导航进入“设置”

截图内容至少应包含：

1. Settings workspace hero
2. 设置运行态卡
3. 至少 2 组 settings module

截图目的：

1. 证明 Settings 已从传统设置页进入 workbench settings 语义
2. 证明模块头、状态卡、hero 语言已统一

建议文件名：

- `wave3-settings-workspace-20260422.png`

### D. Settings 内 Resources 模块截图

入口：

1. 进入 Settings 页
2. 滚动到“记忆与资源”模块

截图内容至少应包含：

1. 资源工作区说明块
2. `Memory / Resources Workspace` 头部
3. 记忆入口列表
4. 如可能，展开一条资源详情

截图目的：

1. 证明 Resources 已按模块级进入统一工作台语言
2. 证明它不是独立页面，但也不再像一块孤立功能堆叠

建议文件名：

- `wave3-settings-resources-20260422.png`

## 当前补证结果

本轮已按本入口补齐并落盘：

1. `wave3-logs-workspace-20260422.png`
2. `wave3-history-review-20260422.png`
3. `wave3-settings-workspace-20260422.png`
4. `wave3-settings-resources-20260422.png`

同时已补：

1. `wave3-walkthrough-20260422.txt`
2. `wave3-walkthrough-20260422.json`

## 截图完成后如何回填文档

### 1. 更新 `verify.md`

至少补：

1. 页面截图证据列表新增 4 张图
2. 页面验证结论新增：
   - Logs 页已补截图
   - History / Review 区已补截图
   - Settings 页已补截图
   - Settings 内 Resources 模块已补截图

### 2. 更新 `status.md`

将“进行中 / 下一步”中的 Wave 3 证据缺口切换为：

1. 当前已具备可结束（并行）条件
2. 后续仅保留最小引用说明或人工复看

### 3. 是否可关闭 Wave 3

满足以下条件后再评估：

1. `wave3-收口评估.md` 已有
2. `wave3-多视图一致性检查清单.md` 已有
3. 本文件所列截图已补齐
4. `verify.md` 已回填

当前主控已判断本并行 change 可结束（并行），但仍不建议直接写成“Wave 3 已全部完成”。

## 若浏览器自动化仍受阻

如果后续浏览器自动化继续被现有 DevTools 会话占用，则建议：

1. 先保留本文件作为已执行过的截图补证入口
2. 优先引用现有截图与 walkthrough
3. 当前维持口径：
   - **Wave 3 代码侧已基本收口，页面证据已显著补齐；本并行 change 可结束（并行）**
