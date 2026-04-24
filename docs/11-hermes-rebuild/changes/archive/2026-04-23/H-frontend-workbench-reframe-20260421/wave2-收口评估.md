# Wave 2 收口评估

## 结论

当前 Wave 2 在**代码实现层面已基本具备收口条件**，但在 change 工作区口径上，仍建议先补页面级 walkthrough 证据，再正式将 Wave 2 标记为结束。

换句话说：

1. **实现侧**：可以认为 `ChatPanel.tsx + EventTimeline.tsx + ui/primitives/*` 的内容块收口已达到本轮目标。
2. **验证侧**：仍缺少页面级任务 walkthrough 截图与调查层展开态补证，因此不建议直接写成“Wave 2 全部完成”。

## 当前已满足的收口条件

### 1. 主线程语义已形成稳定边界

已形成以下稳定表达：

1. 用户输入块
2. assistant 正式回答块
3. recovery / system 结果块
4. 状态更新块
5. 待确认块
6. 空状态块
7. composer meta 与线程 meta

这说明主线程已经从“消息集合”推进为“任务流”。

### 2. 调查层语义已形成稳定边界

`EventTimeline.tsx` 当前已具备：

1. latest / selected / tone 的统一状态表达
2. summary / primary detail / detail row 的层级表达
3. tag row 的裁剪与减噪策略
4. 事件卡片与主线程共享同一套 panel / card / status 语言

这说明调查层已经从“信息列表”推进为“辅助轨道”。

### 3. Wave 2 的渐进波次已形成完整链条

当前已经完成：

1. 第一轮：shared primitives 接入
2. 第二轮：主线程 meta / composer meta / timeline detail / tag 收口
3. 第三轮：状态块 / 确认块 / detail 行层级收口
4. 第四轮：answer / recovery / system 结果语义与事件 tone 收口
5. 第五轮：主线程 / 调查层减噪
6. 第六轮：最后一轮微调与收口评估

从实现节奏看，Wave 2 已不再缺少关键收口动作。

## 当前仍未闭合的缺口

### 1. 页面级 walkthrough 证据不足

当前已有：

1. Wave 1 首页截图
2. Wave 1 任务页截图
3. Wave 1 调查层展开态截图
4. 真实 `POST /api/v1/chat/run` + `GET /api/v1/logs` 的最小链路证据

当前缺少：

1. **Wave 2 完成后** 的任务页主线程截图
2. **Wave 2 完成后** 的调查层展开截图
3. 主线程与调查层联动的一次页面级 walkthrough 证据

### 2. 浏览器自动化环境仍受阻

当前 DevTools 浏览器会话占用，导致页面自动化抓图仍未恢复。这不是代码阻塞，但会影响验证闭环。

## 建议的收口判定

### 建议口径

当前建议使用：

- **“Wave 2 代码侧已基本收口，验证侧待补页面证据。”**

不建议立即写成：

- “Wave 2 已全部完成”

### 满足以下任一组合后，可正式关闭 Wave 2

#### 方案 A：标准收口

1. 补一组 Wave 2 任务页主线程截图
2. 补一组 Wave 2 调查层展开截图
3. 更新 `verify.md`
4. 在 `status.md` 中将 Wave 2 改为可结束/已结束

#### 方案 B：受工具阻塞下的保守收口

如果浏览器自动化仍不可用，则至少补：

1. 明确记录“浏览器自动化会话受阻”
2. 保留现有真实接口链路 + 日志证据
3. 在 `verify.md` 中说明页面截图待工具恢复后补录
4. 将 Wave 2 标记为“代码侧收口完成，页面证据待补”

## 对 Wave 3 的影响

Wave 2 当前状态已经足够支持进入 Wave 3 的入口整理。

也就是说，下一步可以开始准备：

1. `LogsPanel.tsx`
2. `history/*`
3. `settings/*`
4. `resources/*`

但前提仍然是：

- 不把 Wave 3 的进入表述成“主推进已切换”
- 不改 `current-state.md`
- 仅作为本并行 change 的下一波入口规划
