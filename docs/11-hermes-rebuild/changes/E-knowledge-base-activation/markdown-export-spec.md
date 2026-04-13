# Markdown 导出规范（T20）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`  
适用任务：`T21-T24`

## 1. 目标

1. 固定主库导出到 Markdown 的结构，避免字段漂移。
2. 让 Obsidian 与 graphify 对同一份文件可直接消费。
3. 规范优先于实现，导出脚本必须严格遵守本文件。

## 2. 目录结构

1. 导出根目录：`data/exports/knowledge-markdown/`
2. 批次目录：`data/exports/knowledge-markdown/<yyyyMMdd-HHmmss>/`
3. 文件命名：`<knowledge_type>__<record_id>.md`
4. 索引文件：`index.jsonl`（每行一条导出记录，含源记录与目标路径）

## 3. Frontmatter 规范

每个 Markdown 文件头部必须包含以下字段：

```yaml
---
id: "<knowledge_record.id>"
workspace_id: "<knowledge_record.workspace_id>"
knowledge_type: "<knowledge_record.knowledge_type>"
source: "<knowledge_record.source>"
source_type: "<knowledge_record.source_type>"
verified: true
priority: 0
created_at: "<knowledge_record.created_at>"
updated_at: "<knowledge_record.updated_at>"
exported_at: "<yyyy-MM-ddTHH:mm:ssZ>"
trace_id: "<run_trace_id_or_empty>"
tags:
  - "kb/<knowledge_type>"
  - "source/<source_type>"
  - "verified/<true|false>"
---
```

约束：

1. `id/workspace_id/knowledge_type/source/source_type` 必填。
2. `tags` 至少 3 条，且使用小写与 `/` 分层风格。
3. `trace_id` 允许为空，但字段必须存在。

## 4. 正文模板

正文块按固定顺序输出：

1. `# <title>`
2. `## Summary`：写入 `summary`
3. `## Content`：写入 `content`
4. `## Context`：补充来源与验证信息
5. `## Links`：写入 wikilink 关系

示例：

```markdown
# CET4 听力提分套路

## Summary
听力分块跟读每天 10 分钟，周更错题复盘。

## Content
场景：四级备考；动作：分块跟读+错题复盘；验证：每周正确率。

## Context
- Source: runtime
- Source Type: agent_resolve
- Verified: true
- Updated At: 2026-04-13T10:00:00Z

## Links
- [[topic/cet4]]
- [[workflow/weekly-review]]
- [[source/runtime]]
```

## 5. Wikilink 规则

1. 主题链接：`[[topic/<slug>]]`，从 `title` 与 `summary` 抽取主主题。
2. 流程链接：`[[workflow/<slug>]]`，从 `knowledge_type` 或 `tags` 抽取动作类节点。
3. 来源链接：`[[source/<source_type>]]`，固定保留。
4. 单文件最少 2 条链接（主题 + 来源），建议 3 条以上。
5. slug 规则：小写、空格转 `-`、仅允许 `[a-z0-9-_]`。

## 6. Tag 规则

1. 固定层级：`kb/*`、`source/*`、`verified/*`。
2. 可选层级：`topic/*`、`workflow/*`。
3. 不允许中文空格与大写字母。
4. 去重后写入，保留首次出现顺序。

## 7. 质量门禁

导出任务必须同时满足：

1. Frontmatter 必填字段完整率 `100%`。
2. 每条导出记录至少 2 个 wikilink。
3. `index.jsonl` 记录数与导出文件数一致。
4. 任意导出批次可被 Obsidian 直接打开，无 YAML 解析错误。

## 8. 回退策略

1. 发现字段漂移时，回退到上一版导出模板并重跑当前批次。
2. 图谱异常时，先保留 Markdown 导出，暂时关闭 graphify 构图步骤。
3. 回退后必须更新 `verify.md`，记录原因与恢复时间。
