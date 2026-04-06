# Claude Code 本地智能体框架架构设计文档

## 1. 文档目的

本文档用于总结 Claude Code 相关源码分析与方法论提炼，目标不是复刻某个具体仓库，而是吸收其最有价值的框架思想，设计一套适合本地整体项目接入的智能体运行时架构。

当前执行入口：
- 当前运行时主干开发，统一参考 [智能体框架主干开发任务书_V1](D:/newwork/本地智能体/docs/06-development/智能体框架主干开发任务书_V1.md)。

本文档同时面向两类读者：

- AI：作为系统级设计说明、能力分层说明、运行规则说明与后续提示词输入材料。
- 人：作为本地智能体平台的架构蓝图、实现路线、能力边界与演进方案。

本文档重点回答以下问题：

- Claude Code 这一类系统的整体架构模式是什么。
- 这类框架有哪些运行模式和角色模式。
- 这些模式在本地项目里应该如何实现。
- 技能（skills）应该做成底层能力，还是作为后续可插拔模块。
- 本地模型接入是否需要纳入整体设计。
- 如何在本地构建一套更稳定、更强、更接近工程实战的智能体框架。

---

## 2. 核心结论

Claude Code 值得学习的重点，不是单一模型能力，而是它把智能体做成了一个完整的运行时系统。其核心价值可概括为：

1. 把代码库当作活的工程环境，而不是静态文本。
2. 把工具当作协议化能力，而不是随手调用的函数。
3. 把上下文管理、权限管理、记忆管理、压缩管理前置为系统能力。
4. 把多代理协作、验证、探索、调度做成可控流程，而不是随意分叉。
5. 把运行时能力和上层工作方法论分开。

用一句话总结：

> 一个优秀的本地智能体框架，本质上应该是一个以消息流为中心、以 Agent 主循环为引擎、以工具协议为扩展点、以策略层和记忆层为护栏、以多代理协作为增强层的本地运行时系统。

---

## 3. 需要区分的两层体系

这是整份文档最重要的边界。

### 3.1 底层运行时架构

这部分回答的是“系统怎么造”。

主要包括：

- 实时仓库上下文加载
- Prompt 静态/动态分层与缓存
- Tool 协议与 Tool Registry
- Agent 主循环
- 权限系统与策略层
- 上下文压缩与结果外置
- 结构化记忆
- Fork / 子 Agent / 并发调度
- 模型适配层

### 3.2 上层工作方法论

这部分回答的是“系统怎么用得稳、用得好”。

主要包括：

- Coordinator Orchestrator
- Task Concurrency Patterns
- Adversarial Verification
- Self-Rationalization Guard
- Worker Prompt Craft
- Memory Type System
- Smart Memory Guard
- Lightweight Explorer

因此，正确的理解方式不是“所有内容都做成 skill”，而是：

- 底层运行时能力应该成为框架的内建能力。
- 上层工作规范可以通过内建策略、系统技能、调度模板、角色提示词来实现。

---

## 4. Claude Code 架构中最值得学习的能力

### 4.1 实时仓库上下文加载

这类系统不是只读取用户当前打开的文件，而是在任务启动时主动建立项目全景上下文。

应纳入的上下文来源包括：

- 当前工作目录
- Git 主分支
- 当前分支
- 最近提交记录
- 未提交变更
- 项目级说明文件，例如 `CLAUDE.md`、`README.md`、`AGENTS.md`
- 当前任务相关文件
- 历史工具输出摘要

意义：

- 模型看到的是“正在变化的代码仓库”
- 能感知当前工作树状态，而不是死文件
- 能根据变更历史理解当前任务语境

### 4.2 Prompt 静态/动态分层与缓存

成熟 Agent 系统不会每轮都完整重建提示词。

应拆分为：

- 静态部分
  - 系统角色定义
  - 工具协议说明
  - 安全/权限规则
  - 项目长期说明
  - 技能定义
- 动态部分
  - 当前任务
  - 当前分支/工作区信息
  - 当前对话消息
  - 本轮工具结果
  - 当前文件片段
  - 当前压缩摘要

意义：

- 降低重复 token 开销
- 提高上下文稳定性
- 更容易命中缓存
- 支持增量更新

### 4.3 工具链比聊天上传文件强得多

代码智能体的核心不只是“能聊天”，而是能以工具为媒介理解和操作项目。

基础工具类型建议包括：

- 文件发现工具：`glob`
- 代码检索工具：`grep`、结构化搜索
- 文件读取工具：`read_file`
- 文件编辑工具：`apply_patch`
- 终端执行工具：`shell`
- Git 工具：`git_status`、`git_diff`、`git_log`
- 语言理解工具：LSP 引用查找、定义跳转、调用层级、符号搜索
- 测试工具：运行单测、运行指定验证脚本
- 记忆工具：写入摘要、读取历史任务状态
- 协调工具：启动 worker、读取 worker 结果

这里最值得学习的是：工具不是“附属功能”，而是 Agent 理解和改变世界的主要接口。

### 4.4 极致上下文压缩

长任务必须解决上下文膨胀问题。

建议实现：

- 文件读取去重
- 工具结果去重
- 大输出写磁盘，仅将预览和引用放回上下文
- 自动摘要
- 长消息截断
- 历史任务压缩
- 结构化记忆替代原始长文本

意义：

- 避免多轮任务后上下文失控
- 提升长任务稳定性
- 降低推理成本

### 4.5 结构化会话记忆

Claude Code 类系统非常值得借鉴的一点，是把工作过程外部化。

建议记忆内容不是自由散文，而是结构化项目状态：

- 会话标题
- 当前目标
- 当前状态
- 任务规格
- 关键文件
- 关键函数
- 已尝试方案
- 错误与修正
- 决策记录
- 当前阻塞点
- 下一步计划

这本质上是“外部化工作记忆”，不是普通聊天记录。

### 4.6 Fork 与子 Agent 并行

多代理不是为了炫技，而是为了把不同性质的工作隔离开。

适合拆出去的任务：

- 只读探索
- 宽范围搜索
- 摘要压缩
- 失败路径验证
- 单独实现某个模块
- 回归验证

核心原则：

- 主 Agent 负责拆解、裁决、汇总
- Worker 负责局部执行
- 避免把所有思考和所有结果都塞进主上下文

---

## 5. 框架可以有哪些模式

这里建议分成两类：运行模式和角色模式。

## 5.1 运行模式

### 模式 1：交互式单代理模式

适用场景：

- 本地 CLI
- IDE 插件对话框
- 人工监督的逐轮开发

特点：

- 用户驱动
- 一轮一轮推进
- 工具调用需要展示和可审计

### 模式 2：Headless / SDK 模式

适用场景：

- 后台任务
- 自动化工作流
- 外部系统调用 Agent

特点：

- 无需 REPL 界面
- 统一输出结构化事件
- 可供其他服务嵌入

### 模式 3：自治执行模式

适用场景：

- 长任务自动执行
- 大范围重构
- 定期分析与修复

特点：

- 主循环持续运行
- 需要预算限制、权限规则、失败恢复
- 必须结合压缩和记忆管理

### 模式 4：分叉并行模式

适用场景：

- 宽搜
- 并行验证
- 子任务并发实现

特点：

- 共享父任务关键上下文
- 子任务隔离执行
- 结果按需回收

## 5.2 角色模式

### 角色 1：Coordinator

负责：

- 拆解任务
- 指派 worker
- 合并结果
- 做最终决策

不负责：

- 模糊委托
- 把未消化的中间结果直接转发给下游 worker

### 角色 2：Implementer

负责：

- 局部模块实现
- 精准修复
- 受限区域内编辑

### 角色 3：Explorer

负责：

- 低成本只读搜索
- 快速定位文件、函数、调用关系
- 为主 Agent 降低搜索成本

### 角色 4：Verifier

负责：

- 运行验证命令
- 尝试打破实现
- 发现遗漏与隐患

### 角色 5：Memory Keeper

负责：

- 压缩上下文
- 提取关键记忆
- 删除无效历史
- 维护项目状态摘要

---

## 6. 本地智能体框架推荐的总体分层

建议将本地框架拆成以下 8 层。

### 6.1 Interface Layer

职责：

- CLI
- Web UI
- IDE 插件
- SDK / API

要求：

- 不直接承载核心智能体逻辑
- 只负责输入输出与事件展示

### 6.2 Session & Context Layer

职责：

- 建立会话
- 收集仓库上下文
- 维护消息历史
- 读取项目说明文件
- 维护工作目录与分支信息

关键模块建议：

- `SessionStore`
- `RepoContextBuilder`
- `ConversationState`
- `PromptAssembler`

### 6.3 Query Engine / Agent Loop

职责：

- 执行主循环
- 发送模型请求
- 解析工具调用
- 处理工具结果回填
- 处理中断、超时、取消、预算

这是整个系统的中枢。

### 6.4 Tool Protocol Layer

职责：

- 定义工具协议
- 工具注册
- 工具元信息
- 工具权限策略
- 工具执行与输出规范

工具协议至少应包含：

- `name`
- `description`
- `input_schema`
- `validate_input`
- `check_permission`
- `call`
- `is_enabled`
- `is_readonly`
- `is_destructive`
- `is_concurrency_safe`

### 6.5 Policy & Safety Layer

职责：

- 权限控制
- 危险操作确认
- 可执行目录限制
- 并发限制
- 测试前置条件
- 自我合理化防护规则

### 6.6 Memory & Compaction Layer

职责：

- 长对话压缩
- 会话记忆抽取
- 项目记忆维护
- 大结果外置
- 失效引用清理

### 6.7 Orchestration Layer

职责：

- 子任务拆分
- worker 创建
- 任务依赖调度
- 结果汇总
- 并发策略执行

### 6.8 Model Adapter Layer

职责：

- 对接本地模型或云模型
- 统一不同协议差异
- 统一流式响应
- 统一工具调用格式
- 统一结构化输出支持

---

## 7. 建议的主循环实现方式

### 7.1 核心思想

主循环应围绕“消息流 + 工具调用 + 状态回填”设计，而不是围绕 UI 设计。

### 7.2 参考伪代码

```ts
while (!session.aborted) {
  const prompt = promptAssembler.build({
    staticContext,
    dynamicContext,
    messages,
    memorySummary,
    repoContext,
  })

  const modelResponse = await modelAdapter.generate(prompt, tools)

  streamToUI(modelResponse.events)

  if (modelResponse.stopReason === "end_turn") {
    saveAssistantMessage(modelResponse.message)
    break
  }

  if (modelResponse.stopReason === "tool_use") {
    const toolCalls = extractToolCalls(modelResponse.message)

    for (const toolCall of toolCalls) {
      await policyLayer.check(toolCall, session)
      const result = await toolExecutor.run(toolCall, session)
      const compacted = await compactor.compactIfNeeded(result)
      appendToolResultToMessages(compacted)
    }

    continue
  }

  if (modelResponse.stopReason === "context_limit") {
    await compactor.compactConversation(session)
    continue
  }

  handleUnknownStopReason(modelResponse)
}
```

### 7.3 必须实现的控制点

- 取消信号
- 超时控制
- token 或预算统计
- 工具失败重试策略
- 上下文压缩触发条件
- 大输出落盘策略
- 权限拦截与审批

---

## 8. 实时仓库上下文应如何实现

建议实现一个 `RepoContextBuilder`，在每次任务开始或关键阶段刷新。

### 8.1 最低配置

- 当前工作目录
- Git 当前分支
- 主分支名称
- 最近若干条提交
- 工作区未提交变更摘要
- 项目根目录说明文档

### 8.2 增强配置

- 近期改动文件列表
- 当前任务影响的模块
- 关键入口文件
- 当前语言服务可见的符号信息
- 最近一次测试结果摘要

### 8.3 输出格式建议

```json
{
  "repo_root": "C:/project",
  "branch": "feature/agent-runtime",
  "main_branch": "main",
  "recent_commits": [
    {"hash": "abc123", "message": "refactor query engine"}
  ],
  "working_tree_summary": "...",
  "project_docs": [
    {"path": "CLAUDE.md", "summary": "..."}
  ]
}
```

---

## 9. Prompt 缓存与上下文构造建议

建议将 Prompt 组装器拆成三层。

### 9.1 Static Prompt

内容：

- 系统角色
- 工具协议说明
- 基本执行规则
- 通用安全规则
- 系统级技能

特点：

- 很少变化
- 可强缓存

### 9.2 Project Prompt

内容：

- 项目说明文件
- 仓库规范
- 当前仓库结构摘要
- 项目级记忆

特点：

- 会变化，但频率较低
- 可按仓库级缓存

### 9.3 Dynamic Prompt

内容：

- 当前用户任务
- 当前轮消息
- 本轮工具结果
- 当前文件片段
- 当前工作摘要

特点：

- 高频变化
- 每轮动态更新

### 9.4 推荐缓存键

- `system_version`
- `toolset_hash`
- `project_root`
- `project_docs_hash`
- `memory_digest`
- `task_id`

---

## 10. Tool 协议应如何设计

### 10.1 工具不是函数，而是能力对象

建议每个工具具备以下字段：

```ts
type AgentTool = {
  name: string
  description: string
  inputSchema: JsonSchema
  validateInput(input: unknown): ValidationResult
  checkPermission(ctx: ToolContext, input: unknown): Promise<PermissionDecision>
  call(ctx: ToolContext, input: unknown): Promise<ToolResult>
  isEnabled(ctx: ToolContext): boolean
  isReadonly: boolean
  isDestructive: boolean
  isConcurrencySafe: boolean
}
```

### 10.2 建议的工具分类

#### A. Repository Tools

- `glob`
- `grep`
- `list_dir`
- `read_file`
- `git_status`
- `git_diff`
- `git_log`

#### B. Code Intelligence Tools

- `lsp_find_references`
- `lsp_find_definition`
- `lsp_call_hierarchy`
- `symbol_search`

#### C. Editing Tools

- `apply_patch`
- `write_file`
- `rename_file`

#### D. Execution Tools

- `shell`
- `run_test`
- `run_build`
- `run_lint`

#### E. Memory & Task Tools

- `read_memory`
- `write_memory`
- `spawn_worker`
- `get_worker_result`
- `compact_context`

### 10.3 工具结果如何返回

不要把超长原始输出直接塞回会话。

建议工具返回：

```json
{
  "preview": "前 30 行摘要预览",
  "artifact_path": "C:/project/.agent/artifacts/test-run-001.txt",
  "summary": "测试失败于 user.service.spec.ts 的第 3 组用例",
  "truncated": true
}
```

---

## 11. 上下文压缩与结果外置

这是本地智能体能否长时间稳定运行的关键。

### 11.1 必须实现的压缩手段

- 相同文件内容不重复加入上下文
- 相同工具输出不重复加入上下文
- 大文本输出写入 artifact 文件
- 模型只看到摘要和路径引用
- 历史消息达到阈值后做摘要替换
- 子 Agent 结果先摘要后回收

### 11.2 何时触发压缩

- 总 token 接近阈值
- 单个工具结果过大
- 单个文件过大
- 对话轮数过多
- worker 结果过多

### 11.3 推荐压缩策略

- 文件级摘要
- 模块级摘要
- 会话级摘要
- 任务级状态更新

---

## 12. 结构化记忆系统应该如何设计

建议不要把所有记忆混在一个自由文本文件里。

## 12.1 记忆类型系统

建议至少分为四类：

### `user`

记录与用户偏好和协作方式有关的信息：

- 用户偏好的输出风格
- 用户偏好的工作方式
- 用户明确给出的长期要求

### `feedback`

记录用户纠正和长期有效的修正规则：

- 用户不喜欢过度改动
- 用户要求先跑测试再汇报
- 用户要求修改时保持目录结构不变

### `project`

记录项目运行状态与当前阶段信息：

- 当前任务目标
- 当前里程碑
- 当前未解决问题
- 当前关键文件

### `reference`

记录持久有价值的索引信息：

- 关键设计文档路径
- 关键工具说明路径
- 项目规范路径

## 12.2 不应该长期记忆的内容

以下内容应尽量避免写入长期记忆：

- 代码全文
- Git 历史全文
- 临时调试输出
- 一次性 grep 结果
- 可通过工具快速重新获取的信息
- 很快会过期的中间推理过程

## 12.3 记忆存储建议

建议目录：

```text
.agent/
  memory/
    user.md
    feedback.md
    project.md
    reference.md
  sessions/
    2026-04-01-task-001.md
  artifacts/
  summaries/
```

## 12.4 会话记录建议结构

```md
# 会话标题

## 当前目标
## 当前状态
## 任务规格
## 关键文件与函数
## 已尝试方案
## 错误与修正
## 当前工作流
## 关键结果
## 下一步
## 工作日志
```

---

## 13. 多代理与协调者模式如何落地

这是方法论中最值得学习的一层，但不建议一开始就做得过重。

## 13.1 Coordinator 应遵守的规则

- 主 Agent 负责理解问题与拆解任务
- 主 Agent 负责消化 worker 结果后再下发新任务
- 主 Agent 不应懒委托
- 发给 worker 的任务必须自包含
- 发给 worker 的任务应带文件路径、目标、完成标准

## 13.2 Worker Prompt 应包含的信息

- 任务目标
- 相关背景
- 约束条件
- 文件路径
- 关注代码区域
- 验收标准
- 输出格式

推荐模板：

```md
任务目标：
修复 XXX 问题。

背景：
当前主 Agent 已确认问题集中在 `src/query/engine.ts` 中的上下文压缩逻辑。

工作范围：
- 只分析以下文件：
  - `src/query/engine.ts`
  - `src/context/compactor.ts`

约束：
- 不修改无关文件
- 不重构接口
- 只读分析

交付要求：
- 给出根因
- 给出建议修改点
- 标注具体文件路径和函数名
```

## 13.3 并发规则

建议实现如下规则：

- 只读任务可以自由并行
- 不同文件的写任务可以受控并行
- 同一文件同一区域写任务必须串行
- 验证任务可以与不冲突写任务并行
- 汇总任务必须在依赖完成后执行

## 13.4 推荐调度模型

```ts
type TaskKind =
  | "explore"
  | "implement"
  | "verify"
  | "memory"
  | "summarize"

type Task = {
  id: string
  kind: TaskKind
  readSet: string[]
  writeSet: string[]
  dependencies: string[]
}
```

调度规则：

- `writeSet` 交叉时不可并行
- `readSet` 之间可并行
- `verify` 可与无写冲突任务并行

---

## 14. 对抗性验证为何应做成系统能力

验证不应只是“再看一遍代码”。

### 14.1 Verifier 的目标

不是确认实现看起来正确，而是尽量证明实现可能失败。

### 14.2 必须防止的失败模式

- 只读代码，不跑命令，就给出 PASS
- 看到一组测试通过，就假设全部功能正确
- 用“看起来合理”替代“已验证”
- 只检查 happy path
- 忽略空实现、未触达分支、未覆盖边界条件

### 14.3 建议的验证链

1. 运行最小复现
2. 运行已有测试
3. 运行针对修改区域的专项验证
4. 检查边界条件
5. 检查回归风险

### 14.4 结论格式建议

不要允许 verifier 输出“代码看起来没问题”。

应要求输出：

- 运行了哪些命令
- 看到了什么结果
- 哪些范围已验证
- 哪些范围未验证
- 还剩什么风险

---

## 15. 自我合理化防护应如何实现

这是智能体系统必须具备的纠偏规则。

### 15.1 常见自我合理化语句

- 这段代码看起来应该是对的
- 这个可能太耗时，先不做
- 我先处理简单部分
- 这个大概没问题
- 不需要真的运行

### 15.2 对应纠偏规则

- 如果说“看起来正确”，则必须运行验证命令
- 如果说“太耗时”，则先估算时间，再执行
- 如果说“先做简单的”，则重新评估关键路径
- 如果在写解释而不是执行动作，则优先执行动作

### 15.3 设计建议

不建议把这部分只写在提示词里。

建议同时落地为：

- 系统提示词规则
- 验证阶段检查器
- Coordinator 调度规则
- 最终报告格式约束

---

## 16. Skill 应该怎么放置

这是你当前最关心的问题之一。

## 16.1 不建议的做法

不建议把所有能力都后置成独立 skill。

原因：

- 核心框架能力不应依赖后加载
- 权限、压缩、记忆、协调这些是运行时基础设施
- 如果做成纯 skill，能力会不稳定，难以统一治理

## 16.2 也不建议把所有经验都硬编码到底层

原因：

- 难以迭代
- 难以按项目差异调整
- 难以热更新
- 会让系统非常重

## 16.3 推荐方案：分层放置

建议采用“底层内建能力 + 系统技能 + 项目技能”三层模型。

### A. 底层内建能力

这些必须直接写进框架：

- Agent 主循环
- Tool 协议
- 权限系统
- 上下文压缩
- 结构化记忆
- RepoContextBuilder
- ModelAdapter
- Orchestrator
- 基础并发控制

### B. 系统技能 `skills/system`

这些建议做成系统级技能或策略模板：

- Coordinator Orchestrator
- Worker Prompt Craft
- Adversarial Verification
- Self-Rationalization Guard
- Memory Type System
- Smart Memory Guard
- Lightweight Explorer

原因：

- 它们不是某个业务域专属
- 它们会随着实践逐步升级
- 适合被主 Agent 与 worker 共享复用

### C. 项目技能 `skills/project`

这些与具体项目绑定：

- 项目编码规范
- 模块边界
- 测试命令
- 部署流程
- 领域模型说明
- 特定工作流

## 16.4 对问题 4 的直接回答

### 4(a) 是否在底层设计时就加入这些框架设计，从而不再额外添加 skill

答案：不建议全都吞到底层。

更合理的做法是：

- 底层只内建稳定的框架能力
- 将可演化的工作规则写成系统技能
- 将项目特有知识写成项目技能

### 4(b) 是否后续再添加 skill

答案：是，但要有边界。

后续添加的 skill 应该主要是：

- 项目知识
- 角色提示词模板
- 验收模板
- 领域流程

而不是把基础运行时能力拖到 skill 层去补。

---

## 17. 本地模型搭建是否需要写入整体方案

答案：需要，而且必须写。

原因很简单：

- 本地模型能力会直接影响工具调用格式
- 本地模型能力会影响上下文窗口策略
- 本地模型能力会影响是否能稳定输出结构化结果
- 不同模型服务协议不同，必须有统一适配层

因此，本地模型不是部署细节，而是框架设计的一部分。

## 17.1 推荐的模型接入架构

建议单独做一层 `Model Adapter` 或 `Inference Gateway`。

职责：

- 对接不同模型服务端
- 屏蔽协议差异
- 统一流式输出
- 统一 tool calling 结构
- 统一错误处理
- 统一能力发现

推荐接口：

```ts
type ModelCapabilities = {
  supportsTools: boolean
  supportsStreaming: boolean
  supportsStructuredOutput: boolean
  supportsLongContext: boolean
  supportsVision?: boolean
}

type ModelAdapter = {
  name: string
  capabilities(): Promise<ModelCapabilities>
  generate(req: GenerateRequest): Promise<GenerateResponse>
  stream(req: GenerateRequest): AsyncIterable<GenerateEvent>
}
```

## 17.2 本地模型服务端的角色

本地模型服务端负责：

- 载入模型
- 对外暴露统一 API
- 支持流式响应
- 支持尽可能稳定的工具调用或结构化输出

## 17.3 可以考虑的本地服务端类型

适合作为本地推理后端的路线一般有三类：

### A. 轻量本地开发型

特点：

- 快速启动
- 适合单机试验
- 方便本地开发联调

### B. OpenAI 兼容服务型

特点：

- 更容易接入现有 Agent 框架
- 便于统一模型适配层
- 方便后续替换模型

### C. 高吞吐部署型

特点：

- 更适合并发 worker
- 更适合多任务批量执行
- 更适合后续扩展为局域网服务

## 17.4 能力要求

不论底层用哪种本地服务，至少应验证以下能力：

- 是否支持流式输出
- 是否支持长上下文
- 是否支持结构化输出
- 是否支持稳定的工具调用格式
- 是否支持多轮消息输入
- 是否支持系统提示词

## 17.5 模型能力与 skill 的关系

skill 不属于模型。

正确关系是：

- 模型负责推理
- Runtime 负责调度
- Skill 负责提供规则、模板、领域知识、角色行为约束

所以本地模型搭建要写进文档，但 skill 不应耦合在模型部署层里。

---

## 18. 推荐目录结构

下面给出一个适合本地项目落地的推荐目录。

```text
agent-runtime/
  apps/
    cli/
    web/
    ide/
  core/
    session/
    context/
    query-engine/
    tools/
    policies/
    memory/
    compaction/
    orchestration/
    artifacts/
  adapters/
    models/
      openai/
      anthropic/
      local/
    lsp/
    git/
  skills/
    system/
      coordinator/
      verifier/
      explorer/
      memory-guard/
      self-rationalization-guard/
    project/
      coding-standards/
      domain-knowledge/
      test-workflow/
  prompts/
    system/
    roles/
    workers/
  storage/
    sessions/
    memory/
    artifacts/
  config/
    models/
    tools/
    policies/
```

---

## 19. 建议的实现阶段

不要一开始就做成一个超重系统。建议分阶段推进。

## 19.1 第一阶段：最小可用运行时

目标：

- 单代理可运行
- 有基础工具
- 有基础上下文加载
- 有基础压缩
- 可接本地模型

必须完成：

- CLI 或 API 入口
- QueryEngine
- Tool Registry
- RepoContextBuilder
- Prompt 分层
- 基础 Memory
- 基础 artifact 落盘
- 基础 shell/read/grep/apply_patch

## 19.2 第二阶段：工程可用版

目标：

- 更稳定
- 可长任务执行
- 有验证链
- 有记忆治理

必须完成：

- 结构化记忆
- 大结果外置
- 自动摘要
- 权限系统
- 验证器角色
- Git 工具
- LSP 工具

## 19.3 第三阶段：多代理版

目标：

- 有协调者
- 可只读并发探索
- 可实现与验证分离
- 可任务汇总

必须完成：

- Orchestrator
- Worker Prompt 模板
- 并发冲突控制
- 子任务摘要回收
- 对抗性验证

## 19.4 第四阶段：高阶版

目标：

- 长期项目记忆
- 多角色工作流
- 更强的自治能力

可追加：

- 自动任务分解
- 周期性记忆整理
- 背景任务
- 多模型路由
- 项目级 dashboard

---

## 20. 推荐你当前本地项目的落地选择

如果你的目标是“拥有一个比较良好的、优秀的智能体框架架构，然后写在本地”，建议如下。

## 20.1 架构选择建议

优先做：

- 单一稳定主循环
- 强工具协议
- 强上下文管理
- 强记忆与压缩
- 轻量多代理

不要一开始就做：

- 过重的团队协作系统
- 复杂自治工作流
- 太多角色
- 过多 skill 层级

## 20.2 Skill 放置建议

推荐策略：

- 把运行时能力写到底层
- 把通用工作方法写成系统技能
- 把具体项目知识写成项目技能

换句话说：

- Coordinator 规则、Verifier 规则、Memory Guard 规则可以做成系统技能
- RepoContext、Tool Registry、Prompt Cache、Compaction 必须做成底层模块

## 20.3 本地模型接入建议

推荐做法：

- 建统一 `ModelAdapter`
- 先接一个本地模型服务端
- 先把工具调用与结构化输出跑通
- 再考虑多模型切换

最重要的不是一开始接多少模型，而是保证：

- 流式稳定
- 工具调用稳定
- 消息格式统一
- 错误恢复统一

## 20.4 与当前新项目、旧 Python 项目的对比结论

为了避免后续实现阶段再次混淆“参考框架”“旧项目”“新项目正式主链路”三者的关系，这里给出一次明确对比。

对比对象：

- 本文档总结的 Claude Code 类框架方法论
- 当前新项目 `D:\newwork\本地智能体`
- 旧项目 `D:\newwork\local-agent-control-hub`
- 参考仓库 `https://github.com/sanbuphy/claude-code-source-code/tree/main`

### 20.4.1 三者的关系定位

可以这样理解：

- 本文档解决的是“理想运行时应该长什么样”
- 新项目解决的是“正式产品主链路落在哪里”
- 旧项目解决的是“哪些边界和经验已经被验证过”
- 参考仓库解决的是“成熟代码智能体通常由哪些能力层组成”

因此，正确落地方式不是：

- 直接复刻参考仓库
- 直接把旧 Python runtime 翻译成 Rust
- 让新项目重新长回旧项目的研究型形态

而应该是：

- 用参考仓库帮助校准运行时能力地图
- 用旧项目提炼边界、事件、会话、能力分层经验
- 用新项目承载正式的 Go + Rust + React 主链路

### 20.4.2 新项目与本文档架构的一致处

当前新项目已经与本文档主张的方向形成了较强一致性，主要体现在：

- 已采用 React + Go + Rust 的清晰分层
- Go 已承担控制面、会话、事件回流、确认交互和设置管理
- Rust 已承担运行时核心而不是前端或 HTTP 路由
- 运行时已经出现 `Analyze -> Plan -> Execute -> Observe -> Verify -> Finish` 的最小状态机主线
- 已经把 `planner / execution / risk / session / memory / knowledge / events / contracts` 拆成独立模块
- 前端已经围绕聊天、日志、设置和事件流构建正式入口

这说明新项目在“架构骨架”上，实际上比旧 Python 项目更接近一条可长期维护的正式主链路。

### 20.4.3 新项目与本文档架构的差距

当前新项目仍然处于起步阶段，和本文档描述的成熟运行时相比，还缺少以下关键能力：

- 真正的实时仓库上下文加载
- Prompt 静态层 / 项目层 / 动态层分层构造
- 可扩展的 Tool Registry，而不只是当前较固定的动作分支
- 大输出外置、自动摘要、去重和压缩链路
- 更明确的结构化记忆类型系统
- 统一模型适配层与模型路由层
- 更完整的验证链，而不只是最小执行后收口
- 受控的 worker / sub-agent 并发调度能力

其中最本质的差距不是“功能少”，而是：

- 当前 Rust runtime 还更像一个最小可运行骨架
- 本文档描述的是一个工程化的 Agent Runtime

所以，新项目下一步不应该推翻分层，而应该继续把运行时能力补厚。

### 20.4.4 旧 Python 项目与本文档架构的一致处

旧项目 `D:\newwork\local-agent-control-hub` 有不少值得保留的框架思想。

它与本文档相符的地方主要有：

- 明确区分 React 前端、Go 控制面、Python runtime、worker/MCP 能力层
- 已经把会话、事件、日志、配置、审计看成正式系统概念
- 风险控制、确认流程和执行能力隔离意识较强
- worker / MCP 化方向是正确的，说明“能力层可插拔”这条路是成立的
- 记忆、知识、索引等能力已经有原型积累
- 旧文档中已经反复强调 gateway 不是第二个 runtime，worker 不是第二条主智能体链路

这些内容说明，旧项目最大的价值不在于“代码可直接迁移”，而在于：

- 边界意识
- 分层意识
- 控制面设计经验
- 能力层解耦经验

### 20.4.5 旧 Python 项目与本文档架构的不一致处

旧项目虽然理念先进，但运行时实现已经明显偏向研究型和实验型架构，不适合作为新项目第一版的直接底座。

典型表现包括：

- `loop.py` 及周边模块过于庞大，微策略、评分器、恢复器、验证器数量很多
- 大量 candidate scoring、bias、suppression、freshness、replay、proof、portfolio 类模块更像研究试验场
- 运行时职责虽然被拆散，但整体认知成本很高
- 很多模块服务于“让系统更聪明地裁决”，而不是“让主链路更稳定可维护”
- 对第一版 MVP 来说，复杂度远高于必要水平

换句话说，旧项目的问题不是方向错，而是：

- 正确方向上叠加了过多实验性细节

这也是为什么新项目不能走“Python 旧 runtime 全量 Rust 化”的路线。

### 20.4.6 参考仓库对当前项目最有价值的地方

参考仓库 `https://github.com/sanbuphy/claude-code-source-code/tree/main` 更适合作为“能力地图参考”，不适合作为“直接照搬模板”。

它对当前项目最有价值的启发主要是：

- 入口、查询引擎、工具层、状态层应清晰分开
- Agent Runtime 的核心不是聊天界面，而是消息流和工具调度
- 子 Agent、并发、验证、记忆都是运行时增强能力，而不是 UI 功能
- 工具协议、上下文管理、状态沉淀应优先于零散 heuristics

因此，它最适合帮助新项目回答的问题是：

- 我们的 Rust runtime 后续还缺哪些系统能力
- 哪些能力应是内建模块，哪些能力适合做成上层策略或系统 skill

### 20.4.7 最终架构判断

综合本文档、旧项目和当前新项目，当前最推荐的判断是：

- 保留新项目现在的 Go + Rust + React 总分层
- 继续以 Rust runtime 作为唯一正式智能体主链路
- 从旧 Python 项目中吸收边界、会话、事件、确认、能力解耦经验
- 从参考仓库中吸收成熟 Agent Runtime 的能力地图
- 明确放弃旧项目中研究型微策略的大量直接继承

也就是说：

> 新项目应该继承旧项目的“边界和经验”，借鉴参考仓库的“运行时能力地图”，但正式主链路必须在当前 Go + Rust + React 架构上继续长出来。

### 20.4.8 当前阶段最值得优先补齐的能力

如果以“最小成本、最大收益”的方式继续推进，新项目建议优先补齐：

1. 实时仓库上下文加载
2. Prompt 分层与上下文装配
3. Tool Registry / Tool Contract
4. 结构化记忆 schema 与按需召回
5. 大输出外置与上下文压缩
6. 更正式的验证链与完成判定
7. 统一模型适配层

在这些完成之前，不建议优先投入：

1. 复杂多智能体
2. 大量角色系统
3. 旧项目式海量微策略模块
4. 重型自治工作流

原因很简单：

- 当前新项目最缺的是运行时“主干能力”
- 不是运行时“高级枝叶”

---

## 21. 建议的最终结论

你要学习的，不是 Claude Code 的所有功能，而是它的三层精华。

### 第一层：底层运行时能力

- 实时仓库上下文
- Prompt 缓存与分层
- Tool 协议
- Agent 主循环
- 权限系统
- 上下文压缩
- 结构化记忆
- 模型适配层

### 第二层：上层工作方法论

- 协调者模式
- 并发控制
- 对抗性验证
- 自我合理化防护
- Worker 指令模板
- 记忆类型系统
- 记忆防护
- 轻量探索

### 第三层：本地化落地路径

- 先做单代理和工具层
- 再做记忆和压缩
- 再做验证和协调者
- 最后再做多代理和自治增强

最终推荐方案不是：

- 全部做成硬编码底层
- 或者全部做成后置 skill

而是：

> 用底层运行时承载稳定能力，用系统技能承载方法论，用项目技能承载领域知识。

这才是最适合本地整体项目长期演进的结构。

---

## 22. 附：建议纳入的系统级技能列表

建议作为 `skills/system` 的第一批技能：

- `coordinator-orchestrator`
- `worker-prompt-craft`
- `adversarial-verification`
- `self-rationalization-guard`
- `memory-type-system`
- `smart-memory-guard`
- `lightweight-explorer`

建议作为 `skills/project` 的第一批技能：

- `project-coding-standards`
- `project-test-workflow`
- `project-architecture-map`
- `project-domain-reference`

---

## 23. 外部参考

以下资料适合作为后续继续学习和实现比对的参考入口。

注意：

- 这些外部资料用于辅助理解与交叉验证。
- 本文档的架构建议不等于要求完整复刻某一仓库。
- 本地实现时应优先遵循本文档的分层原则与演进路线。

参考资料：

- Claude Code 源码还原仓库：`https://github.com/sanbuphy/claude-code-source-code/tree/main`
- Claude Code sourcemap 总结仓库：`https://github.com/ChinaSiro/claude-code-sourcemap`
- Ollama 官方文档：`https://docs.ollama.com/`
- vLLM 官方文档：`https://docs.vllm.ai/`
- llama.cpp 官方仓库：`https://github.com/ggml-org/llama.cpp`

---

## 24. 一句话版本

如果只保留一句话来指导本地实现：

> 先做稳定的 Agent Runtime，再把优秀的方法论做成系统技能，最后把项目知识做成项目技能，并通过统一模型适配层接入本地模型。
