# H-learning-mode-browser-20260415（status）

最近更新时间：2026-04-16
状态：已收口（非当前主推进）
阶段关系：阶段 H / Gate-H（已收口，非当前主推进）

## 当前状态

1. 已完成：
   - 对标 yilan 的能力与迁移边界分析
   - H04 任务拆解初稿
   - H04-01 学习模式协议 v1 已冻结：`/api/v1/learning/extract` 最小输入输出字段已固定
   - H04-02a BestBlogs provider 接入方案已冻结：provider 落点、错误码、回退路径已固定
   - BestBlogs 文章页加载链路验证：已确认公开 API `GET /api/proxy/resources/{id}?language=zh` 可返回结构化正文
   - BestBlogs provider 执行交接简报已落档：`bestblogs-provider-implementation-brief.md`
   - BestBlogs provider 最小实现已落地到 `gateway/internal/providers/bestblogs/`
   - gateway 最小入口已提供：`POST /api/v1/providers/bestblogs/article/read`
   - 学习模式最小采集入口已提供：`POST /api/v1/learning/extract`
   - 学习模式最小 explain 入口已提供：`POST /api/v1/learning/explain`
   - 学习模式最小 translate 入口已提供：`POST /api/v1/learning/translate`
   - 学习模式最小价值判断入口已提供：`POST /api/v1/learning/value-score`
   - 学习模式最小建议入口已提供：`POST /api/v1/learning/recommend`
   - 学习模式最小记忆写入口已提供：`POST /api/v1/learning/memory/write`
   - 学习模式最小审计入口已提供：`POST /api/v1/learning/audit-trace`
   - 学习模式最小回退校验入口已提供：`POST /api/v1/learning/rollback-check`
   - 主样本 `42acaf7d` 已完成 title/summary/html/markdown/images 验证
   - H04-02 页面抽取 MVP 已完成批量验收：`/api/v1/learning/extract` 对 21 个 BestBlogs 样本实测成功 21 个，成功率 `100%`
   - H04-04 价值判断引擎 v1 已完成主样本实证：输出 `score/reason/next_action` 且保留最小规则信号
   - H04-03 解释/翻译卡片已达最小验收：`/api/v1/learning/explain` 与 `/api/v1/learning/translate` 对 21 个 BestBlogs 样本自动结构验证成功率均为 `100%`，人工回看 5/5 通过
   - H04-05 个性化学习建议已完成最小闭环：`/api/v1/learning/recommend` 对 21 个 BestBlogs 样本自动结构验证成功率 `100%`，人工回看 5/5 通过
   - H04-06 记忆路由联动已完成最小闭环：`/api/v1/learning/memory/write` 对 21 个 BestBlogs 样本实测写入/读回/注入预览全部成功，`effective_hit_rate=100%`
   - H04-07 审计与回放已完成最小闭环：`/api/v1/learning/audit-trace` 对 21 个 BestBlogs 样本实测 `trace_id` 全链路串联成功，`trace_link_rate=100%`
   - H04-08 回退开关验证已完成最小闭环：`/api/v1/learning/rollback-check` 对 21 个 BestBlogs 样本实测关闭学习模式后均降级为 `explain_translate_only`
   - 最小证据已落盘：
     - `tmp/stage-h-learning/latest.json`
     - `tmp/stage-h-learning/extract.json`
     - `tmp/stage-h-learning/bestblogs-provider.json`
     - `tmp/stage-h-learning/bestblogs-markdown.json`
     - `tmp/stage-h-learning/bestblogs-fallback.json`
     - `tmp/stage-h-learning/explain-translate.json`
     - `tmp/stage-h-learning/value-score.json`
     - `tmp/stage-h-learning/recommend.json`
     - `tmp/stage-h-learning/memory-routing.json`
     - `tmp/stage-h-learning/audit-trace.json`
     - `tmp/stage-h-learning/rollback.json`
2. 进行中：
   - browser fallback 产品化仍未启用，本轮仅保留占位说明
3. 未开始：
   - 插件/前端联调
   - 多站点 provider 扩展
   - 插件/前端联调

## 阻塞与风险

1. 阻塞：
   - 无
2. 风险：
   - 插件端承担过多逻辑导致后续维护困难
   - 当前 recommend 明确限定为“基于当前文章内容的学习建议”，尚未接入长期用户画像
   - 当前 memory-routing 只覆盖学习文章写入与注入预览，尚未和运行时真实上下文装配联动
   - 当前 audit-trace 只覆盖 gateway 聚合视角的学习链路审计，不等价于 runtime 原生日志总线
   - 当前 rollback-check 只验证 gateway 侧受控降级，不代表插件端 UI 已完成联调
   - 浏览器探索能力可用于发现正文接口，但生产路径若长期依赖 UI/DOM 抽取，稳定性会下降
   - 当前 Markdown 标准化为最小规则版，后续若接更多站点需补更稳健的块级转换口径

## 下一步

1. 当前 H04 最小闭环已齐，可准备阶段性提审或转下一主推进项
2. 当前阶段性提审包已收口，当前主推进已切换到 `H-memory-routing-kb-20260415`
3. 若后续回补 H-04 采集侧，再实现 browser fallback 的真实回退执行与失败样本
4. 保持 `extract.json`、`recommend.json`、`memory-routing.json`、`audit-trace.json`、`rollback.json` 批量验收口径，后续新增样本时同步更新成功率
