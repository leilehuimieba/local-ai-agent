# 主库到 Cortex 字段映射（T12）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`  
适用任务：`T13-T19`

## 1. 映射原则

1. 主库（SQLite）为事实源，Cortex 为外部增强层。
2. 映射以“可检索语义”为主，不做无意义字段搬运。
3. 外部写入失败不影响主链路写入成功判定。

## 2. `knowledge_base` -> `/api/v1/ingest`

| 主库字段 | Cortex 字段 | 映射规则 |
|---|---|---|
| `summary` | `user_message` | 优先使用 `summary`，为空时回退 `title` |
| `content` | `assistant_message` | 使用 `content` 截断后写入（建议 <= 2000 字符） |
| `workspace_id` | `agent_id` | 默认映射为 `default`；后续可扩展为 workspace-agent 命名空间 |
| `source`/`source_type` | `metadata` | 合并为扩展元信息（后续适配器注入） |

## 3. `knowledge_base` -> `/api/v1/memories`（显式记忆写入）

| 主库字段 | Cortex 字段 | 映射规则 |
|---|---|---|
| `knowledge_type` | `category` | 映射到 Cortex 分类：`workflow_pattern -> fact`，`decision -> decision`，其余回退 `context` |
| `summary` | `content` | 作为核心可检索内容 |
| `workspace_id` | `agent_id` | 默认 `default` |
| `priority` | `importance` | 归一化到 `0.0-1.0`（建议 `min(max(priority/10,0),1)`） |
| `source` | `source` | 透传（如 `runtime`、`manual`） |
| 固定值 | `layer` | 统一写 `core` |

## 4. `long_term_memory` -> `/api/v1/memories`

| 主库字段 | Cortex 字段 | 映射规则 |
|---|---|---|
| `memory_type` | `category` | `constraint/policy/goal/fact/...` 按同义映射；未知回退 `context` |
| `summary` + `content` | `content` | 优先 `summary`，不足时拼接 `content` |
| `workspace_id` | `agent_id` | 默认 `default` |
| `priority` | `importance` | 同上归一化 |
| `source` | `source` | 透传 |
| 固定值 | `layer` | `core`（长期记忆默认进入 core） |

## 5. 样例

### 5.1 主库知识记录（源）

```json
{
  "workspace_id": "main",
  "knowledge_type": "workflow_pattern",
  "title": "CET4 听力提分套路",
  "summary": "听力分块跟读每天 10 分钟，周更错题复盘。",
  "content": "场景：四级备考；动作：分块跟读+错题复盘；验证：每周正确率。",
  "priority": 8,
  "source": "runtime",
  "source_type": "agent_resolve"
}
```

### 5.2 映射到 `/api/v1/memories`（目标）

```json
{
  "layer": "core",
  "category": "fact",
  "content": "听力分块跟读每天 10 分钟，周更错题复盘。",
  "agent_id": "default",
  "importance": 0.8,
  "source": "runtime"
}
```

### 5.3 映射到 `/api/v1/ingest`（目标）

```json
{
  "user_message": "听力分块跟读每天 10 分钟，周更错题复盘。",
  "assistant_message": "场景：四级备考；动作：分块跟读+错题复盘；验证：每周正确率。",
  "agent_id": "default"
}
```
