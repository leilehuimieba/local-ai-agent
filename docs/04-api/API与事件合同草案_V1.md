# 本地智能体 - API 与事件合同草案 V1

更新时间：2026-03-31

状态：第一版 API 与事件合同草案已形成，可作为前端、Go 控制面、Rust 运行时之间的通信基线。

关联文档：
- [需求冻结稿_V1](D:/newwork/本地智能体/docs/00-charter/需求冻结稿_V1.md)
- [产品边界与入口定义_V1](D:/newwork/本地智能体/docs/01-boundary/产品边界与入口定义_V1.md)
- [MVP功能清单_V1](D:/newwork/本地智能体/docs/01-boundary/MVP功能清单_V1.md)
- [总体架构草案_V1](D:/newwork/本地智能体/docs/02-architecture/总体架构草案_V1.md)
- [Rust运行时设计草案_V1](D:/newwork/本地智能体/docs/03-runtime/Rust运行时设计草案_V1.md)

---

## 1. 文档目的

本文件用于冻结第一版系统各层之间的通信边界，重点回答：

1. 前端需要调用哪些核心 API
2. Go 控制面与 Rust 运行时之间需要传什么
3. 事件流应该长什么样
4. 风险确认如何来回传递
5. 最终结果如何回流前端
6. 日志和进度怎样统一描述

本文件不是最终 OpenAPI 文档，也不是最终 protobuf/JSON schema 文档。
本文件的作用是先固定“合同的形状和语义”。

---

## 2. 合同设计原则

### 2.1 Contract First

前端、Go、Rust 必须围绕稳定合同交互，不允许依赖隐式内部对象结构。

### 2.2 Event First

第一版必须具备任务过程可见性，因此事件流不是附属功能，而是主合同的一部分。

### 2.3 Stable Envelope First

所有请求、响应、事件、确认、日志都应优先采用统一外层 envelope。

### 2.4 Summary First

合同中优先传递摘要、状态和关键元信息，而不是冗长原始内容。

### 2.5 Frontend Friendly

合同要支持前端低成本消费：

1. 字段语义清晰
2. 状态有限
3. 事件类型有限
4. 风险确认信息直接可显示

### 2.6 Runtime Friendly

合同要支持 Rust 运行时：

1. 状态机清晰
2. 动作和结果有明确对象
3. 确认流可恢复
4. 日志和记忆写入可扩展

---

## 3. 通信边界总览

第一版通信边界建议分为三段：

1. 前端 <-> Go 控制面
2. Go 控制面 <-> Rust 运行时
3. Go 控制面 <-> 前端事件流

说明：

1. 前端不直接调用 Rust 运行时。
2. Rust 运行时不直接向前端推送事件。
3. Go 控制面是唯一正式桥梁。

---

## 4. 合同对象分类

第一版合同建议至少包含以下对象族：

1. 请求对象
2. 响应对象
3. 事件对象
4. 确认对象
5. 日志对象
6. 错误对象
7. 配置对象
8. 模型对象

---

## 5. 统一 Envelope 设计

建议所有主要通信对象共享统一 envelope 思想。

### 5.1 基础 envelope 建议字段

建议所有主要对象优先包含以下公共字段：

1. `request_id`
2. `session_id`
3. `run_id`
4. `trace_id`
5. `timestamp`
6. `source`
7. `kind`
8. `payload`

### 5.2 字段语义

#### `request_id`

表示一次前端发起请求的唯一标识。

#### `session_id`

表示当前对话会话。

#### `run_id`

表示本次智能体运行。

#### `trace_id`

表示跨前端、Go、Rust、日志的统一链路标识。

#### `source`

表示对象来源，例如：

1. `frontend`
2. `gateway`
3. `runtime`
4. `system`

#### `kind`

表示当前对象的种类，例如：

1. `run_request`
2. `run_event`
3. `run_result`
4. `confirmation_request`
5. `confirmation_response`

### 5.3 设计要求

1. 不要求所有字段在第一版每个对象里全部出现。
2. 但 `session_id`、`run_id`、`kind`、`timestamp` 应尽量稳定存在。

---

## 6. 前端到 Go 的核心 API

第一版前端只需要少量核心 API 即可支撑 MVP。

### 6.1 `POST /api/v1/chat/run`

#### 作用

发起一次新的任务运行，或在当前会话中继续一次任务。

#### 请求语义

前端将用户输入、当前模式、当前模型、当前工作区等信息发给 Go。

#### 建议请求内容

1. 用户输入文本
2. 当前会话标识
3. 当前模式
4. 当前模型
5. 当前工作区
6. 可选上下文元信息

#### 建议响应内容

同步响应应尽量轻：

1. `accepted`
2. `run_id`
3. `session_id`
4. 初始状态

说明：

1. 详细过程通过事件流回传，而不是阻塞在同步响应里。

---

### 6.2 `POST /api/v1/chat/confirm`

#### 作用

提交用户对高风险动作的确认决策。

#### 请求语义

前端将确认结果发给 Go，再由 Go 回传给 Rust 运行时。

#### 建议请求内容

1. `session_id`
2. `run_id`
3. `confirmation_id`
4. `decision`
5. 可选备注

#### `decision` 建议取值

1. `approve`
2. `reject`
3. `cancel`

---

### 6.3 `GET /api/v1/events/stream`

#### 作用

建立事件流订阅，用于接收运行过程更新。

#### 传输方式建议

第一版建议：

1. SSE 优先

理由：

1. 实现简单
2. 适合单向事件流
3. 对聊天场景足够

WebSocket 可以后续再评估，但第一版不强制。

---

### 6.4 `GET /api/v1/settings`

#### 作用

获取基础设置。

#### 第一版至少包括

1. 当前模式
2. 当前模型
3. 可用模型列表
4. 当前工作区
5. 目录提示相关设置

---

### 6.5 `POST /api/v1/settings`

#### 作用

更新基础设置。

#### 第一版允许更新

1. 模式
2. 当前模型
3. 当前工作区
4. 目录提醒偏好

---

### 6.6 `GET /api/v1/logs`

#### 作用

获取日志页需要的运行记录。

#### 第一版目标

服务于：

1. 复盘
2. 测试
3. 排障

---

## 7. Go 到 Rust 的运行时合同

Go 与 Rust 之间不应采用“前端对象直传”思路，而应有专门的运行时请求对象。

### 7.1 `RunRequest`

表示发起一次运行所需的最小上下文。

#### 建议字段

1. `run_id`
2. `session_id`
3. `trace_id`
4. `user_input`
5. `mode`
6. `model_ref`
7. `workspace_ref`
8. `context_hints`

### 7.2 `RunResult`

表示运行时最终返回给 Go 的结束结果。

#### 建议字段

1. `run_id`
2. `status`
3. `final_answer`
4. `summary`
5. `error`
6. `memory_write_summary`
7. `final_stage`

### 7.3 `RunEvent`

表示运行时推送给 Go 的中间事件。

#### 建议字段

1. `run_id`
2. `session_id`
3. `event_type`
4. `stage`
5. `sequence`
6. `summary`
7. `detail`
8. `metadata`

### 7.4 `ConfirmationRequest`

表示运行时需要用户确认时发给 Go 的对象。

#### 建议字段

1. `confirmation_id`
2. `run_id`
3. `risk_level`
4. `action_summary`
5. `reason`
6. `impact_scope`
7. `target_paths`
8. `reversible`
9. `hazards`
10. `alternatives`

### 7.5 `ConfirmationDecision`

表示 Go 将用户决策返回给运行时的对象。

#### 建议字段

1. `confirmation_id`
2. `run_id`
3. `decision`
4. 可选备注

---

## 8. 事件流合同

### 8.1 事件流目标

事件流必须同时满足：

1. 前端实时展示
2. 日志面板复盘
3. 问题排查
4. 未来扩展性

### 8.2 第一版建议事件类型

建议第一版事件类型固定在少量核心集合中：

1. `run_started`
2. `analysis_ready`
3. `plan_ready`
4. `action_requested`
5. `action_completed`
6. `confirmation_required`
7. `verification_completed`
8. `memory_written`
9. `run_failed`
10. `run_finished`

### 8.3 事件基础字段

每个事件建议至少包含：

1. `event_id`
2. `event_type`
3. `session_id`
4. `run_id`
5. `sequence`
6. `timestamp`
7. `stage`
8. `summary`

### 8.4 事件附加字段建议

根据类型可附加：

1. `tool_name`
2. `result_summary`
3. `risk_level`
4. `error_code`
5. `memory_kind`
6. `model_ref`

### 8.5 事件内容原则

1. 事件应以摘要为主
2. 避免把长原文直接塞入事件
3. 面向前端显示与日志复盘双重用途

---

## 9. 确认流合同

### 9.1 确认流目标

确认流必须足够明确，让前端无需猜测如何展示高风险动作。

### 9.2 `ConfirmationRequest` 必须可直接渲染

前端拿到确认对象后，应能直接展示：

1. 在做什么
2. 为什么要做
3. 影响范围
4. 目标路径
5. 能否回退
6. 潜在危害
7. 风险等级
8. 可选替代方案

### 9.3 风险等级建议值

1. `low`
2. `medium`
3. `high`
4. `irreversible`

### 9.4 决策建议值

1. `approve`
2. `reject`
3. `cancel`

### 9.5 确认对象的 UI 适配要求

1. 前端不应自行拼接风险说明逻辑
2. 后端应直接返回适合展示的摘要文本

---

## 10. 日志合同

### 10.1 日志目标

日志主要服务于：

1. 复盘
2. 测试
3. 排障

### 10.2 日志与事件的关系

建议第一版采用：

1. 事件是运行时过程对象
2. 日志是事件的稳定落库或聚合视图

换句话说：

1. 不是单独设计一套完全不同的日志对象
2. 而是在事件对象基础上补足日志需要的元信息

### 10.3 日志基础字段建议

1. `log_id`
2. `session_id`
3. `run_id`
4. `timestamp`
5. `level`
6. `category`
7. `summary`
8. `detail`
9. `source`

### 10.4 日志分类建议

1. `runtime`
2. `tool`
3. `risk`
4. `memory`
5. `model`
6. `system`

---

## 11. 错误对象合同

### 11.1 统一错误目标

前端、Go、Rust 都应围绕统一错误语义工作，避免每层都自造错误表达方式。

### 11.2 建议错误对象字段

1. `error_code`
2. `message`
3. `summary`
4. `retryable`
5. `source`
6. `stage`
7. `metadata`

### 11.3 第一版常见错误类别建议

1. `model_unavailable`
2. `provider_auth_failed`
3. `provider_quota_exceeded`
4. `provider_timeout`
5. `tool_execution_failed`
6. `risk_confirmation_required`
7. `workspace_access_denied`
8. `invalid_request`

---

## 12. 模型与设置合同

### 12.1 模型对象建议

建议前后端共享统一的模型引用对象：

1. `provider_id`
2. `model_id`
3. `display_name`
4. `enabled`
5. `available`

### 12.2 第一版不要求的字段

1. 自动测速结果
2. 自动优先级分数
3. fallback 权重

### 12.3 设置对象建议

设置对象第一版至少覆盖：

1. 当前模式
2. 当前模型
3. 当前工作区
4. 新目录提示偏好
5. 可选风险展示偏好

---

## 13. 工作区与目录授权合同

### 13.1 工作区对象建议

1. `workspace_id`
2. `name`
3. `root_path`
4. `is_active`

### 13.2 目录提示对象建议

用于支持“新目录首次接触提示”：

1. `path`
2. `first_seen`
3. `decision`
4. `remembered`

### 13.3 目录提示决策建议值

1. `allow_once`
2. `allow_and_remember`
3. `read_only_once`
4. `deny`

---

## 14. 记忆合同

### 14.1 第一版目标

合同层不要求暴露全部记忆内部实现，但至少要明确“写了什么”和“召回了什么类别”。

### 14.2 记忆写入摘要对象建议

1. `memory_kind`
2. `summary`
3. `scope`
4. `written`

### 14.3 建议记忆类别值

1. `project_knowledge`
2. `preference`
3. `daily_note`
4. `lesson_learned`

### 14.4 运行时事件中与记忆相关的最小信息

1. 是否触发了记忆写入
2. 写入了哪一类
3. 写入摘要是什么

---

## 15. API 响应风格建议

### 15.1 同步响应

同步 API 尽量轻量，只返回：

1. 是否接受
2. 关键标识
3. 初始状态

### 15.2 异步结果

任务过程、工具调用、确认请求、结束结果应尽量通过事件流回传。

### 15.3 最终结果

最终结果仍应存在一个统一对象，方便：

1. 聊天页收口
2. 日志页检索
3. 测试脚本判断

---

## 16. 为什么第一版采用这种合同设计

### 16.1 原因

这样设计的原因是：

1. 前端容易实现
2. Go 控制面易于承接
3. Rust 运行时边界清晰
4. 事件流天然适合聊天与日志页
5. 确认流可以自然插入
6. 后续扩展浏览器、桌面、语音时不必推翻主合同

### 16.2 避免的问题

这种设计试图避免：

1. 前端和 Rust 直接耦合
2. 事件、日志、结果三套对象风格完全不同
3. 确认流没有稳定结构
4. 每种能力都自定义返回格式

---

## 17. 第一版明确不做的合同复杂化

第一版不建议引入：

1. 过重的协议层抽象
2. 多种并行事件协议
3. 多层嵌套到难以理解的 envelope
4. 为未来假设预埋大量未使用字段
5. 复杂双向流控制协议

第一版只要做到：

1. 请求清楚
2. 事件清楚
3. 确认清楚
4. 结果清楚

就足够了。

---

## 18. 后续文档顺序建议

基于本合同草案，建议继续写：

1. `docs/05-migration/源仓库迁移边界_V1.md`
2. `docs/archive/06-development/开发顺序与里程碑_V1.md`
3. `docs/07-test/验收标准_V1.md`

如果希望进一步细化，也可插入一份：

1. `docs/04-api/JSON对象清单_V1.md`

---

## 19. 使用规则

1. 后续接口实现不得绕过本文件定义的合同语义。
2. 若某接口字段不在本文件中，应补充说明其用途与所属对象族。
3. 如果后续实现中出现“前端必须猜测后端语义”的情况，应回到本文件修正合同，而不是在实现层继续堆特殊逻辑。

