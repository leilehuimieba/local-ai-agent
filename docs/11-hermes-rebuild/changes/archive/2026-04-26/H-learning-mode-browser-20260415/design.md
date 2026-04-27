# H-learning-mode-browser-20260415（design）

更新时间：2026-04-15
状态：已冻结（H04-01 / H04-02a）

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

## 最小接口（v1 冻结）

1. `POST /api/v1/learning/extract`
2. `POST /api/v1/learning/explain`
3. `POST /api/v1/learning/translate`
4. `POST /api/v1/learning/value-score`
5. `POST /api/v1/learning/recommend`
6. `POST /api/v1/learning/memory/write`
7. `POST /api/v1/learning/audit-trace`
8. `POST /api/v1/learning/rollback-check`

### `POST /api/v1/learning/extract` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs",
  "include_html": true,
  "include_markdown": true,
  "include_images": true
}
```

约束：

1. `article_url` 为主输入，provider 负责自行提取站点内 `article_id`。
2. `language` 默认 `zh`。
3. `provider_hint` 当前允许为空；BestBlogs 样本固定写作 `bestblogs`。
4. `include_html/include_markdown/include_images` 为最小可选输出开关。

### `POST /api/v1/learning/extract` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "meta": {
    "title": "",
    "author": "",
    "publish_time": "",
    "tags": []
  },
  "summary": {
    "one_sentence": "",
    "full": "",
    "main_points": [],
    "key_quotes": []
  },
  "content": {
    "html": "",
    "markdown": "",
    "images": []
  }
}
```

### `POST /api/v1/learning/value-score` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs"
}
```

约束：

1. 当前最小实现复用 `article_url` 作为主输入，不引入额外页面结构协议。
2. `provider_hint` 可为空；BestBlogs 样本固定写作 `bestblogs`。
3. `value-score` 主路径复用 provider 采集结果，不额外依赖 UI/DOM 抽取。

### `POST /api/v1/learning/value-score` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "score": 0,
  "level": "high",
  "reason": "",
  "next_action": "",
  "signals": {
    "main_points": 0,
    "tags": 0,
    "images": 0,
    "markdown_chars": 0,
    "keyword_hits": 0
  }
}
```

约束：

1. `score` 取值范围固定为 `0-100`。
2. `level` 当前固定为 `high | medium | low`。
3. `reason` 与 `next_action` 必须可直接展示给学习模式卡片。
4. `signals` 只暴露最小规则评分依据，用于审计与证据复跑。

### `POST /api/v1/learning/explain` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs"
}
```

约束：

1. 当前最小实现复用 provider 采集结果，不单独接收自由文本。
2. explain 输出面向“先理解再深读”的学习卡片，不承担价值判断。

### `POST /api/v1/learning/explain` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "title": "",
  "explain": "",
  "main_points": [],
  "key_terms": [
    {
      "term": "",
      "explanation": ""
    }
  ]
}
```

约束：

1. `explain` 必须可直接展示给学习模式说明卡片。
2. `main_points` 复用结构化要点，`key_terms` 只保留最小术语解释集合。

### `POST /api/v1/learning/translate` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs",
  "target_language": "en"
}
```

约束：

1. 当前最小实现仅支持学习模式桥接翻译卡，不做逐段精译。
2. `target_language` 缺省时按 `en` 处理。

### `POST /api/v1/learning/translate` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "source_language": "zh",
  "target_language": "en",
  "translation_type": "reader_bridge",
  "title": "",
  "summary": "",
  "main_points": [],
  "notes": ""
}
```

约束：

1. `translation_type=reader_bridge` 明确表示当前是“帮助理解主题”的桥接翻译，不宣称逐句精译。
2. `summary` 与 `main_points` 必须帮助用户快速判断是否回读原文。

### `POST /api/v1/learning/recommend` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs"
}
```

约束：

1. 当前最小实现继续复用 `article_url` 作为主输入，不引入用户画像协议。
2. `recommend` 主路径复用 provider 采集结果与规则评分，不接入模型调用链。

### `POST /api/v1/learning/recommend` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "score": 0,
  "level": "high",
  "recommendation": "",
  "focus_topics": [],
  "why": "",
  "next_step": "",
  "meta": {
    "title": "",
    "author": "",
    "publish_time": "",
    "tags": []
  }
}
```

约束：

1. `recommendation` 明确回答“这篇值不值得学、适合怎么学”。
2. `focus_topics` 只输出当前文章应优先关注的主题点，不宣称长期个性化画像。
3. `why` 与 `next_step` 必须可直接展示给学习模式建议卡片。

### `POST /api/v1/learning/memory/write` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs"
}
```

约束：

1. 当前最小实现只接受学习文章 URL，不额外引入自由文本写入协议。
2. `memory/write` 主路径复用 provider 采集结果与规则评分，不接入独立模型判定。

### `POST /api/v1/learning/memory/write` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "title": "",
  "route": "long_term_memory",
  "write_status": "written",
  "reason": "",
  "memory_id": "",
  "memory_title": "",
  "score": 0,
  "level": "high",
  "recall_count": 1,
  "memory_digest": "",
  "injection_preview": ""
}
```

约束：

1. `route` 当前固定为 `long_term_memory | skip`，明确说明是否进入长期记忆层。
2. `write_status` 当前固定为 `written | duplicate | skipped_low_score`。
3. `memory_digest` 与 `injection_preview` 只提供摘要注入预览，不返回全文记忆。

### `POST /api/v1/learning/audit-trace` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs",
  "trace_id": ""
}
```

约束：

1. 当前最小实现复用学习文章 URL，不单独接入 runtime run_id/session_id。
2. `trace_id` 允许为空；为空时由 gateway 生成最小 trace 标识。

### `POST /api/v1/learning/audit-trace` 输出合同（冻结）

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "title": "",
  "trace_id": "",
  "replay_ready": true,
  "steps": [
    {
      "sequence": 1,
      "trace_id": "",
      "stage": "extract",
      "event_type": "extract_completed",
      "status": "ok",
      "summary": "",
      "artifact_ref": ""
    }
  ],
  "evidence_refs": []
}
```

约束：

1. `steps[*].trace_id` 必须与顶层 `trace_id` 一致，用于证明全链路可串联。
2. `steps` 当前固定覆盖 `extract -> explain -> translate -> value_score -> recommend -> memory_write`。
3. `evidence_refs` 只返回证据文件引用，不引入新的日志存储层。

### `POST /api/v1/learning/rollback-check` 输入合同（冻结）

```json
{
  "article_url": "",
  "language": "zh",
  "provider_hint": "bestblogs",
  "learning_mode_enabled": false
}
```

约束：

1. 当前最小实现只校验学习模式关闭时的降级行为，不引入新的全局配置存储。
2. `learning_mode_enabled=false` 时必须走受控回退路径。

### `POST /api/v1/learning/rollback-check` 输出合同（冻结）

```json
{
  "ok": true,
  "rollback_applied": true,
  "learning_mode_enabled": false,
  "fallback_mode": "explain_translate_only",
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "",
  "title": "",
  "allowed_actions": [],
  "disabled_actions": [],
  "explain_preview": "",
  "translate_preview": "",
  "explain": {},
  "translate": {}
}
```

约束：

1. `fallback_mode` 当前固定为 `learning_mode | explain_translate_only`。
2. 回退时只允许 `extract / explain / translate`，禁止 `value_score / recommend / memory_write / audit_trace`。
3. 回退响应必须直接给出 explain / translate 结果，便于插件端降级展示。

## Provider 接入策略（v1 冻结）

学习模式的页面采集不默认等价于 UI 抽取，优先按“站点 provider -> 公开 API -> 浏览器回退”执行。

### BestBlogs provider

#### 已验证结论

1. 文章页：`https://www.bestblogs.dev/article/{id}`
2. 正文接口：`https://www.bestblogs.dev/api/proxy/resources/{id}?language=zh`
3. 当前验证口径：正文接口可返回 `200`，且包含结构化元数据与 `displayDocument` HTML 正文。
4. 当前策略判定：`Tier 1 / public`

#### 落点建议

1. gateway：
   - 新增目录：`gateway/internal/providers/bestblogs/`
   - 职责：URL -> article_id 提取、上游请求、UTF-8 解码、JSON 解析、字段归一。
2. runtime-core：
   - 落点：`crates/runtime-core/src/executors/`
   - 职责：学习模式下的 provider 路由、能力调用、结果注入，不直接承接站点细节。
3. capability/tool：
   - 落点：`crates/runtime-core/src/tool_registry.rs` 与 `crates/runtime-core/src/capabilities/`
   - 职责：登记学习模式采集能力，不把站点实现细节写入 registry。

#### gateway 侧建议结构

1. `gateway/internal/providers/bestblogs/client.go`
   - 负责请求 `GET /api/proxy/resources/{id}?language={language}`
2. `gateway/internal/providers/bestblogs/types.go`
   - 定义上游响应与内部归一化结构
3. `gateway/internal/providers/bestblogs/normalize.go`
   - 提取 `metaData`、`contentData.displayDocument`、图片 URL、Markdown 转换入口
4. `gateway/internal/providers/bestblogs/errors.go`
   - 定义输入错误、上游错误、解码错误、空内容错误

#### runtime-core 侧建议结构

1. `crates/runtime-core/src/executors/learning_fetch.rs`
   - 负责学习模式文章采集入口，按域名路由 provider
2. `crates/runtime-core/src/tool_registry.rs`
   - 暴露通用能力名，如 `learning_article_fetch`
3. `crates/runtime-core/src/capabilities/spec.rs`
   - 声明输入为 `article_url`，输出为 `meta/summary/content/images`

#### 输入输出约束（冻结）

1. 输入优先接收 `article_url`，必要时提取 `article_id`
2. 输出统一归一为：
   - `meta`
   - `summary`
   - `content.html`
   - `content.markdown`
   - `content.images`
3. `displayDocument` 作为 HTML 正文主来源，Markdown 为标准化派生结果

#### BestBlogs 最小错误码（冻结）

1. `BESTBLOGS_INVALID_INPUT`
2. `BESTBLOGS_UPSTREAM_HTTP_ERROR`
3. `BESTBLOGS_UPSTREAM_NOT_SUCCESS`
4. `BESTBLOGS_EMPTY_CONTENT`
5. `BESTBLOGS_DECODE_ERROR`

#### 回退策略

1. 主路径：BestBlogs 公开 API
2. 回退 1：`opencli web read --url ...`
3. 回退 2：浏览器探索模式重新抓取网络请求
4. 不默认把 UI 自动化作为主路径

#### 与学习模式主链的关系

1. BestBlogs provider 属于学习模式“采集层 provider”，不承担解释、翻译、价值判断和记忆治理。
2. 学习模式主链保持：
   - 采集
   - 解释/翻译
   - 价值判断
   - 推荐
   - 记忆写入
3. provider 层输出先标准化，再交给后续能力消费。

## 回退设计

1. feature flag：`learning_mode_enabled`
2. 降级模式：
   - 仅解释/翻译
   - 禁止写入记忆
   - 全量回退普通模式
