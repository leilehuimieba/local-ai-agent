# Wave 3 文件级实施计划

## 定位

本文件用于把 Wave 3 从“最小入口”推进到“可开工的文件级计划”。

当前口径：

1. 不切当前主推进
2. 不改 `docs/11-hermes-rebuild/current-state.md`
3. 仅作为本并行 change 的下一波实施准备
4. 保持“收口而非推翻重写”

## 目标

Wave 3 的目标不是重写业务，而是把附属工作区纳入统一工作台语言。

第一阶段以 `Logs + History` 为主，`Settings + Resources` 为次阶段进入。

## 文件级实施顺序

## Phase 1：Logs / History

### 1. `D:/newwork/本地智能体/frontend/src/logs/LogsPanel.tsx`

优先级：**P1 / 第一入口**

#### 当前判断

1. 文件体量较小，适合作为 Wave 3 的首个试点。
2. 与主线程 / 事件流关系最近，最容易复用 Wave 2 已冻结的工作台语言。
3. 风险最低，改完后最容易直观看到“附属工作区已进入统一产品语义”。

#### 改动目标

1. 页面头部改成与任务页一致的 workbench 头部语义。
2. 日志摘要、过滤区、时间线区、详情区统一进入 panel / card / status 语言。
3. 复用既有 `InfoCard / EmptyStateBlock / StatusPill` 与深色 token。
4. 让 Logs 页看起来像 `Workbench Logs / Review`，而不是独立日志页。

#### 先改点

1. 页面头部与区块标题
2. 日志摘要 strip
3. 时间线卡片与详情卡的边框、背景、状态标签
4. 空状态和过滤区的一致性

#### 风险

1. 如果直接重排布局，容易打乱现有复盘效率。
2. 如果把过滤逻辑和展示层一起改，容易扩 scope。

#### 回退点

1. 只收口样式与区块表达，不改日志数据逻辑。
2. 若新布局影响阅读顺序，优先回退布局，只保留视觉语义收口。

---

### 2. `D:/newwork/本地智能体/frontend/src/history/useHistoryReview.ts`

优先级：**P1**

#### 当前判断

1. 文件较大，属于 history 视图的状态与派生逻辑核心。
2. 不建议在 Wave 3 第一轮直接重写逻辑。
3. 更适合作为“只确认边界、不优先改”的支撑文件。

#### 改动目标

1. 第一轮尽量不动业务逻辑。
2. 仅在组件改造需要时，做最小接线或字段映射整理。

#### 风险

1. 直接进入容易把展示层收口任务拖成逻辑重构。

#### 回退点

1. 如无必要，本轮不改。

---

### 3. `D:/newwork/本地智能体/frontend/src/history/viewModel.ts`

优先级：**P1**

#### 当前判断

1. 文件较大，承担 history 页面的大量展示模型组织。
2. 很可能影响 review workspace 的统一表达。
3. 适合作为 history 组件层改造时的轻接线入口，而不是第一刀。

#### 改动目标

1. 只处理为组件统一语义所必需的展示字段整理。
2. 不在本轮做大规模 view model 重构。

#### 回退点

1. 如 history 组件层可以直接复用现有字段，则本轮不改。

---

### 4. `D:/newwork/本地智能体/frontend/src/history/components/*`

优先级：**P1 / Phase 1 主战场**

#### 当前判断

1. 这是 History 真正的页面表达层。
2. 最适合承接 Wave 2 已经形成的 timeline / review / spotlight 语言。

#### 改动目标

1. 统一 history 页头部、spotlight、timeline、detail card 语义。
2. 让 history 与 logs / 当前任务页共享相同的 panel/card/status 体系。
3. 让 review workspace 看起来属于同一产品，而不是另一套页面系统。

#### 先改点

1. 页面头部与区块标题
2. spotlight 与 review card
3. timeline / detail card 的层级与状态表达
4. 空状态与辅助说明

#### 风险

1. history 组件可能较多，容易 diff 扩散。
2. 若一次性处理所有子组件，容易失去最小改造边界。

#### 回退点

1. 先从视觉一致性最明显的组件开始。
2. 每轮只动一组组件，不做全量翻修。

## Phase 2：Settings / Resources

### 5. `D:/newwork/本地智能体/frontend/src/settings/SettingsPanel.tsx`

优先级：**P2 / 第二入口**

#### 当前判断

1. 是 Settings 页最合适的入口文件。
2. 适合作为 settings workspace 的外层壳收口点。

#### 改动目标

1. 统一 settings 页头部和区块容器。
2. 把设置页从传统 form page 收口成 workbench settings workspace。

#### 回退点

1. 只改容器与视觉结构，不改配置逻辑。

---

### 6. `D:/newwork/本地智能体/frontend/src/settings/SettingsSections.tsx`

优先级：**P2**

#### 当前判断

1. 文件体量较大，是 settings 内容表达主战场。
2. 不适合在第一阶段直接深改。

#### 改动目标

1. 第二阶段再统一 section / card / empty / status 语义。
2. 优先做视觉层级收口，不动配置流程。

---

### 7. `D:/newwork/本地智能体/frontend/src/settings/StatusCard.tsx`

优先级：**P2**

#### 当前判断

1. 很适合作为 settings workspace 的统一状态卡试点。
2. 可直接复用主工作台的状态表达。

#### 改动目标

1. 对齐 `StatusPill / InfoCard / summary card` 语言。

---

### 8. `D:/newwork/本地智能体/frontend/src/resources/components/*`

优先级：**P2**

#### 当前判断

1. resources 的核心问题在于“独立功能区感太强”。
2. 最适合通过组件层统一卡片、列表、空状态表达来收口。

#### 改动目标

1. 让 Resources / Memory 进入统一工作台设计系统。
2. 保持独立功能，但视觉上属于同一产品。

#### 回退点

1. 先收口 card/list/empty-state，不动 memory 数据逻辑。

---

### 9. `D:/newwork/本地智能体/frontend/src/resources/useMemories.ts`

优先级：**P2**

#### 当前判断

1. 属于数据 hook，不应在 Wave 3 第一轮优先进入。
2. 仅在组件改造需要时做最小接线。

## 建议实施节奏

### Wave 3 - Round 1

1. `LogsPanel.tsx`
2. `history/components/*`

目标：

- 先证明附属工作区可以进入统一工作台语言
- 先做最有感知的页面收口

### Wave 3 - Round 2

1. `SettingsPanel.tsx`
2. `StatusCard.tsx`
3. `SettingsSections.tsx`
4. `resources/components/*`

目标：

- 完成 settings / resources 的产品语言统一

## 最小验证

Wave 3 文件级实施计划落地后，至少应补：

1. Logs 页页面截图
2. History 页页面截图
3. 多视图一致性检查记录
4. 如继续推进 Settings / Resources，再补第二轮页面截图

## 开工建议

如果下一步进入实现，建议直接从：

1. `D:/newwork/本地智能体/frontend/src/logs/LogsPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/history/components/*`

开始，不建议先动 `settings/*` 或 `resources/*`。
