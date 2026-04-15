# H-learning-mode-browser-20260415（design）

更新时间：2026-04-15
状态：草案

## 设计概览

学习模式采用“插件轻端 + 本地网关重端”：

- 插件端负责采集与展示
- 本地网关负责策略、记忆、评估、审计

## 参考对标

1. 参考仓库：`https://github.com/mutuyihao/yilan`
2. 参考点：
   - 页面抽取与侧栏交互
   - 本地优先与历史沉淀
   - provider 适配层思路
3. 不直接复用点：
   - 插件端长期密钥持久化
   - 插件端承担核心决策与记忆治理

## 架构边界

### 插件端职责

1. 页面采集：标题、正文块、选区、元信息
2. 交互展示：解释、翻译、要点、行动建议卡片
3. 短期缓存：当前页面会话态
4. 调用本地网关 API（trace_id 贯通）

### 本地网关职责

1. 用户画像关联与学习建议生成
2. 内容价值评估（score + reason + next_action）
3. 记忆路由（记什么/何时记/何时读/何时注入）
4. 审计与回放（证据路径/trace）

## 最小接口（v1 草案）

1. `POST /api/v1/learning/extract`
2. `POST /api/v1/learning/explain`
3. `POST /api/v1/learning/translate`
4. `POST /api/v1/learning/value-score`
5. `POST /api/v1/learning/recommend`
6. `POST /api/v1/learning/memory/write`

## 回退设计

1. feature flag：`learning_mode_enabled`
2. 降级模式：
   - 仅解释/翻译
   - 禁止写入记忆
   - 全量回退普通模式
