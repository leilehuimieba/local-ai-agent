# Wave 3 多视图一致性检查清单

## 目的

本清单用于验证 Wave 3 的核心目标是否真正成立：

1. `Logs / History / Settings / Resources` 是否已经进入同一套工作台语言
2. 附属工作区是否不再像独立页面拼接
3. 新用户是否能更快理解这些视图都属于同一产品

本清单默认服务于当前并行 change：

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-frontend-workbench-reframe-20260421/`

不改变主推进状态，不作为 `current-state.md` 的替代口径。

## 检查方式

建议优先按下面顺序执行：

1. 先看 Logs
2. 再看 History
3. 再看 Settings
4. 最后看 Settings 内的 Resources 模块

如果浏览器自动化仍不可用，则允许采用：

1. 代码结构检查
2. 已有截图对照
3. 本地手工 walkthrough

的保守方式记录结论。

## 检查项

### A. 页面头部一致性

#### A1. Hero / Header 语义是否统一

检查点：

1. Logs 是否有明确的 workspace hero
2. History 是否有明确的 page / review header
3. Settings 是否有明确的 workspace hero
4. Resources 是否至少在模块级具备 workspace 说明

通过标准：

1. 各视图都不是裸标题
2. 都能看到 title + description 或等效结构
3. 用户能一眼判断当前处于哪个工作区

当前观察：

1. Logs：**通过**
2. History：**基本通过**
3. Settings：**通过**
4. Resources：**模块级通过**

补证建议：

1. 本轮已补 Logs / History / Settings 相关页面截图，后续仅在需要时补人工复看说明

#### A2. Header action / meta 语义是否统一

检查点：

1. 是否都使用同类 meta / chip 表达
2. 是否都能看到当前视图最重要的数量或状态摘要

通过标准：

1. 不要求完全一致，但要求同一语言体系
2. 不出现某页是工作台 header、另一页退回普通表单标题的情况

当前观察：

1. Logs：**通过**
2. History：**通过**
3. Settings：**通过**
4. Resources：**通过（模块级）**

### B. Panel / Card / Section 一致性

#### B1. 外层 panel 层级是否统一

检查点：

1. Logs 主体是否使用统一 panel 容器
2. History 的 timeline / spotlight / detail rail 是否属于同一套 panel 语言
3. Settings 是否已从传统设置页进入 panel/workspace 语义

通过标准：

1. 各视图都能看出相同的深色 panel/card 层级
2. 不再出现“某页像工作台、某页像旧页面”的割裂感

当前观察：

1. Logs：**通过**
2. History：**通过**
3. Settings：**通过**

#### B2. 区块标题是否统一进入 SectionHeader 语义

检查点：

1. 筛选台、焦点卡、详情栏、设置模块、资源列表是否都使用统一 section header 表达

通过标准：

1. 标题层级清晰
2. 文案风格一致
3. 描述文案不再出现严重混杂

当前观察：

1. Logs：**通过**
2. History：**通过**
3. Settings：**通过**
4. Resources：**基本通过**

### C. 状态表达一致性

#### C1. Status / Pill / Tone 是否统一

检查点：

1. Logs 时间线状态表达是否统一
2. History timeline / spotlight / detail rail 是否共享 tone 语义
3. Settings status card 和模块 badge 是否进入统一状态语言
4. Resources item 是否沿用统一状态 pill

通过标准：

1. completed / running / failed / awaiting 等状态表达不相互冲突
2. 颜色和标签语义基本一致

当前观察：

1. Logs：**通过**
2. History：**通过**
3. Settings：**通过**
4. Resources：**通过**

#### C2. 空状态是否统一

检查点：

1. Logs 空状态是否清楚说明下一步
2. History detail rail 空状态是否清楚说明先选记录
3. Settings 空状态是否说明正在同步设置
4. Resources 空状态是否说明调整筛选后会出现什么

通过标准：

1. 空状态不只说“暂无数据”
2. 至少能告诉用户下一步该做什么

当前观察：

1. History detail rail：**通过**
2. Settings：**通过**
3. Resources：**通过**
4. Logs：**通过**

### D. 信息密度与认知负担

#### D1. 标签与细节噪音是否受控

检查点：

1. History timeline tag 是否已收紧
2. Event / Logs 细节是否避免无限堆叠
3. Resources 列表是否先给摘要，再按需展开

通过标准：

1. 默认阅读路径清晰
2. 细节不是第一视觉重心

当前观察：

1. History：**通过**
2. Resources：**通过**
3. Logs：**基本通过**

#### D2. 视图之间的学习成本是否下降

检查点：

1. 用户从 Logs 切到 History，是否还认得 header / card / status 语言
2. 用户从 History 切到 Settings，是否仍能识别这是同一产品
3. Resources 是否仍像外接子系统

通过标准：

1. 切视图时不需要重新学习新的页面语法
2. Resources 不应继续显得像额外拼接页

当前观察：

1. Logs → History：**通过**
2. History → Settings：**基本通过**
3. Settings → Resources：**通过（模块级）**

### E. 结构边界与 scope 控制

#### E1. 是否保持“收口而非重写”

检查点：

1. 是否主要调整壳层、标题、card、status、空状态
2. 是否避免重写数据逻辑
3. 是否没有引入大型 UI 框架

通过标准：

1. 本轮变更仍属于渐进收口
2. 风险边界清晰

当前观察：

1. **通过**

#### E2. Resources 的真实边界是否清晰

检查点：

1. 文档是否明确 resources 不是独立页面
2. 当前实现是否尊重其“Settings 内资源模块”的事实

通过标准：

1. 不把 resources 错写为独立 workspace 页面
2. 模块级收口与当前仓库结构一致

当前观察：

1. **通过**

## 当前结论

基于现有代码与文档证据，当前可先给出保守判断：

1. **Wave 3 多视图一致性在代码侧已基本成立**
2. **Logs / History / Settings / Resources 已开始共享同一套工作台语言**
3. **页面级截图与最小 walkthrough 已补齐，当前主要边界是不切主推进、不扩大代码范围**

因此当前建议口径是：

- **“Wave 3 多视图一致性在代码与页面证据层面已基本闭合；本并行 change 可结束（并行），但不切主推进。”**

## 后续补证建议

### 优先保持口径

1. 保持“可结束（并行）”而非“全部完成”
2. 明确 History / Review 不是独立页面
3. 明确 Resources 是 Settings 内模块，不是独立页面

### 文档同步建议

当前建议同步：

1. `verify.md`
2. `status.md`
3. `当前阶段总整理.md`

### 若工具仍受阻

如果浏览器自动化继续受现有 DevTools 会话占用，则建议：

1. 保留本清单作为一致性证据
2. 优先引用现有截图与 walkthrough
3. 不继续扩大实现 diff
