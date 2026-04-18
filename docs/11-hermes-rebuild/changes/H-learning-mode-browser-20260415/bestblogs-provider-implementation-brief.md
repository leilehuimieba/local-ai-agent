# BestBlogs provider 执行交接简报

更新时间：2026-04-15
状态：待执行
适用范围：`H-learning-mode-browser-20260415`

## 1. 任务目标

实现学习模式采集层的 BestBlogs provider 最小闭环，优先走公开 API，不把 UI/DOM 抽取作为主路径。

本轮只做最小能力闭环，不扩展到学习建议、记忆写回或多站点抽象。

## 2. 已确认事实

1. 文章页格式：`https://www.bestblogs.dev/article/{id}`
2. 已验证正文接口：
   - `GET https://www.bestblogs.dev/api/proxy/resources/{id}?language=zh`
3. 当前验证口径：
   - 接口可返回 `200`
   - 返回体包含 `data.metaData`
   - 返回体包含 `data.contentData.displayDocument`
4. 当前策略判定：`Tier 1 / public`

## 3. 本轮范围

### In-Scope

1. URL -> `article_id` 提取
2. 调用 BestBlogs 正文接口
3. UTF-8 解码与 JSON 解析
4. 提取并归一化：
   - `meta`
   - `summary`
   - `content.html`
   - `content.markdown`
   - `content.images`
5. 预留 browser fallback 接口或占位，但不要求本轮把 fallback 做到完整产品化
6. 补最小验证证据

### Out-of-Scope

1. 多站点 provider registry 大重构
2. OpenCLI 深度集成到主运行时
3. 学习建议 / 价值判断 / 记忆写入实现
4. 浏览器插件端大改
5. 前端展示联调

## 4. 落点要求

### gateway

建议新增目录：

- `gateway/internal/providers/bestblogs/`

建议文件职责：

1. `client.go`
   - 负责请求上游接口
2. `types.go`
   - 负责定义上游响应与内部输出结构
3. `normalize.go`
   - 负责字段归一、图片提取、HTML -> Markdown 转换入口
4. `errors.go`
   - 负责定义输入错误、上游错误、解码错误、空内容错误

### gateway API

建议新增一个最小接口供后续学习模式接入：

- `POST /api/v1/providers/bestblogs/article/read`

若当前仓库结构更适合先走内部服务调用，也可先不暴露公共路由，但要保留清晰入口。

### runtime-core

本轮只做最小接入预留，不要求深度实现。若必须登记能力，建议：

- `crates/runtime-core/src/tool_registry.rs`
- `crates/runtime-core/src/capabilities/spec.rs`

能力名建议使用通用名：

- `learning_article_fetch`

不要把 BestBlogs 特化实现直接写死到 registry 的描述逻辑里。

## 5. 输入输出契约

### 输入

最少支持：

```json
{
  "article_url": "https://www.bestblogs.dev/article/42acaf7d?entry=resource_card&from=%2Fexplore%2Fbrief",
  "language": "zh",
  "include_html": true,
  "include_markdown": true,
  "include_images": true
}
```

### 输出

最少返回：

```json
{
  "ok": true,
  "provider": "bestblogs",
  "strategy": "public_api",
  "article_id": "42acaf7d",
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

## 6. 字段来源

1. `meta` 来源：`data.metaData`
2. `summary` 来源：
   - `oneSentenceSummary`
   - `summary`
   - `mainPoints`
   - `keyQuotes`
3. `content.html` 来源：`data.contentData.displayDocument`
4. `content.markdown` 来源：`displayDocument` 转换结果
5. `content.images` 来源：从 `displayDocument` 抽取 `img[src]`

## 7. 错误码建议

最少支持：

1. `BESTBLOGS_INVALID_INPUT`
2. `BESTBLOGS_UPSTREAM_HTTP_ERROR`
3. `BESTBLOGS_UPSTREAM_NOT_SUCCESS`
4. `BESTBLOGS_EMPTY_CONTENT`
5. `BESTBLOGS_DECODE_ERROR`

## 8. 验收标准

### 功能验收

以文章：

- `https://www.bestblogs.dev/article/42acaf7d?entry=resource_card&from=%2Fexplore%2Fbrief`

作为主样本，要求：

1. 能正确提取 `article_id = 42acaf7d`
2. 能返回标题：`浏览器自动化：从 GUI 到 OpenCLI`
3. `summary.main_points` 非空
4. `content.html` 非空
5. `content.markdown` 包含：
   - `为什么我们需要浏览器自动化`
   - `未来软件竞争维度`
6. `content.images` 非空

### 结构验收

1. 返回字段稳定
2. 中文不乱码
3. HTML -> Markdown 不丢主段落

## 9. 证据要求

本轮至少补以下证据：

1. `tmp/stage-h-learning/bestblogs-provider.json`
   - provider 主路径结果样本
2. `tmp/stage-h-learning/bestblogs-markdown.json`
   - Markdown 转换样本与关键字段校验
3. `tmp/stage-h-learning/bestblogs-fallback.json`
   - 如做了 fallback，则补回退证据；若本轮只做占位，也要写明未启用原因

## 10. 回退要求

1. 不破坏现有学习模式草案接口
2. 若 provider 路由方案不稳定，允许回退为仅保留内部 client 与 normalize 层
3. 不要把 OpenCLI 或浏览器依赖写死为主路径

## 11. 实施约束

1. 仅做最小闭环
2. 不要顺手做多站点抽象
3. 不要新增不必要依赖
4. 新增或修改函数遵守仓库规则：单函数不超过 30 行
5. 新增 Go 测试统一使用 `testify`

## 12. 执行顺序建议

1. 先实现 `client.go` + `types.go`
2. 再实现 `normalize.go`
3. 再补最小 API 入口
4. 最后补证据与文档回填
