# H-mcp-skills-quality-20260415（design）

更新时间：2026-04-16
状态：草案（已补 H03-02 架构冻结口径）

## 影响范围

1. 涉及模块：
   - `crates/runtime-core/src/skill_catalog.rs`
   - `crates/runtime-core/src/context_builder.rs`
   - `crates/runtime-core/src/context_policy.rs`
   - `crates/runtime-core/src/memory_router.rs`
   - `crates/runtime-core/src/verify.rs`
   - `tmp/stage-h-mcp-skills/`
2. 涉及文档或 contract：
   - `docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/`

## 方案

1. 核心做法：
   - 先冻结 H-03 的最小质量口径，不直接进入大规模接入开发。
   - 吸收外部参考项目 `rtk-ai/rtk` 中已经被证明有效的执行治理原则，但只借鉴治理结构，不照搬其产品目标与平台默认路径。
   - 以当前仓库已经存在的能力骨架为锚点：
     1. 技能资产入口：`skill_catalog`
     2. 上下文装配：`context_builder + context_policy`
     3. 事实/经验写入：`memory_router`
     4. 执行后验证与证据：`verify`
   - H-03 最小主链路固定为：
     `skill_catalog -> context_policy/context_builder -> execution -> verify/evidence`
   - 本轮只冻结边界、注入口径和安全治理口径，不扩成 runtime 主重构。
2. 状态流转或调用链变化：
   - `skill_catalog`：承担 skill 元数据入口、版本 pin、隔离校验、来源分级。
   - `context_policy`：决定是否注入 skill、注入到什么级别、预算是否允许继续展开。
   - `context_builder`：按固定四层结构组装上下文，不把 skill/memory/evidence 混堆。
   - `memory_router`：仅沉淀事实、偏好、项目结论，不承载 skill SOP 或原始执行证据。
   - `verify`：记录执行结果、命中有效性与证据引用，为后续评测提供统一出口。

### 方案补充：借鉴 `rtk-ai/rtk` 的 4 条治理原则

1. 薄适配层 + 单一裁决核心
   - 接入层只负责把不同入口的 skill / MCP / tool 信号适配成统一输入。
   - 裁决层统一负责 `route / fallback / verify / evidence`，不允许各入口各写一套质量判定。
2. 增强层失败不阻断主链路
   - skill 注入、MCP 补充、上下文整形、证据压缩都属于增强层。
   - 增强层失败时，必须保留主链继续执行或显式降级为 `review / manual / passthrough`，不得把增强失败误写为主链失败。
3. 摘要与原始证据双轨
   - 默认把摘要放回上下文，把原始执行证据单独留在 artifact。
   - 任何 fallback、误命中、guard 降级都应同时保留 `summary_ref` 与 `raw_artifact_ref`。
4. 质量收益必须量化
   - H-03 不只看“能不能命中 skill”，还要看命中是否有效、是否引入噪声、失败是否可定位、降级是否可审计。
   - 量化结果进入 `latest.json` 与各分项 eval，不用聊天印象代替。

## 冻结结论

### 1. H-03 最小能力边界

H-03 当前只覆盖以下三类质量问题：

1. Skills 是否按版本、来源与隔离规则正确装载。
2. Skills 是否按固定四层上下文结构被按需注入，而不是无预算堆入 prompt。
3. 失败场景是否能给出明确 fallback、证据引用和命中效果验证线索。

H-03 当前不覆盖：

1. 新建复杂 MCP 协议桥或第三方插件市场。
2. 大规模 Skills 生态扩张。
3. 远程服务编排或云端调度。
4. runtime 主状态机重写。

### 1.1 架构升级口径（基于 Hermes Skills 闭环）

补充判断见：

- `hermes-skills-architecture-upgrade-notes.md`

当前冻结结论：

1. 当前项目主骨架仍然合适，不做推倒重来。
2. H-03 后续升级重点不是重做 runtime 主循环，而是补齐“Skill 闭环 / 技能资产治理层”。
3. 升级方式是“在现有基础上补层”，而不是“大重构后再回接现有模块”。
4. H-03 需要显式覆盖：
   - Skill / Memory / Evidence 边界
   - system / run / skill / evidence 四层注入
   - Skill 渐进加载
   - Skill Guard
   - Skill 命中有效性评测

### 2. H03-02a：Skill / Memory / Evidence 边界冻结

#### 2.1 三层职责冻结

1. Skill
   - 定义：可复用的任务 SOP、步骤、适用条件、风险点、回退路径。
   - 允许承载：manifest 元数据、激活条件、分层摘要、来源与信任级别。
   - 不允许承载：用户长期偏好、项目事实、原始 provider 输出。
2. Memory
   - 定义：用户偏好、环境事实、项目事实、历史结论。
   - 允许承载：可跨轮复用的事实摘要与治理字段。
   - 不允许承载：操作手册式步骤、完整 skill playbook、原始失败日志全文。
3. Evidence
   - 定义：执行日志、provider 输出、验证结果、artifact 引用、fallback 样本。
   - 允许承载：可复放、可审计、可定位失败的运行时证据。
   - 不允许承载：未经评测直接晋升为 memory 或 skill 的内容。

#### 2.2 路由与模块落点冻结

1. `skill_catalog.rs`
   - 负责 skill 元数据入口、版本 pin、隔离校验、来源分级。
   - 不负责事实沉淀与原始证据持久化。
2. `memory_router.rs`
   - 负责 memory/knowledge 写入治理。
   - 只接收“已归一化事实/结论”，不直接写入 skill SOP。
3. `verify.rs`
   - 负责执行结果验证与证据串联。
   - 不把 evidence 直接视为长期记忆或 skill 命中事实。

#### 2.3 边界约束冻结

1. Evidence -> Memory：必须经过验证与归一化，不能直接搬运原始日志。
2. Evidence -> Skill：必须经过人工评审或独立评测，不允许运行中自动晋升。
3. Memory -> Skill：只有当事实已被抽象为可复用 SOP，且满足 Skill Guard，才允许形成 skill 草案。
4. Skill -> Memory：只允许回写“命中结果摘要/经验结论”，不回写整段 skill 内容。

#### 2.4 当前代码映射冻结（2026-04-16 静态读证）

1. `skill_catalog.rs`
   - 已具备：manifest 读取、version pin、入口路径隔离、workspace scope 过滤。
   - 未具备：`trust_tier`、`guard_action`、`guard_reason` 稳定字段。
2. `memory_router.rs`
   - 已具备：长期记忆、偏好、知识沉淀的分流写入。
   - 已符合：当前未见“把 skill SOP 直接写入 memory”的主链口径。
3. `verify.rs`
   - 已具备：`summary/reasoning/artifact/cache_status` 等验证证据串联。
   - 未具备：`skill_hit_effective`、`guard_degraded`、`evidence_layer_ref` 等 H-03 专属字段。

#### 2.5 H03-02a 最小实现口径冻结

1. 第一批只允许补只读字段或样例输出，不改 runtime 主循环。
2. 推荐字段优先级：
   - `skill_catalog.rs`：`trust_tier`、`guard_action`、`guard_reason`
   - `verify.rs`：`skill_hit_effective`、`skill_hit_reason`
3. 在这些字段未落地前，H03-02a 只按“文档冻结完成”判定，不按“运行时已实现”判定。

### 3. H03-02b：四层上下文注入与渐进加载口径冻结

#### 3.1 四层注入冻结

1. System Layer
   - 内容：稳定角色契约、模式边界、全局规则。
   - 落点：`context_builder.rs` 的 `static_prompt_block`。
   - 特性：最稳定，不承载具体 skill 或运行证据。
2. Run Context Layer
   - 内容：当前任务、阶段、工作区、会话摘要、必要项目摘要。
   - 落点：`context_builder.rs` 的 `project_block` 与 `dynamic_block` 基础字段。
   - 特性：围绕当前运行实例变化，不承载 skill 全文。
3. Skill Injection Layer
   - 内容：命中 skill 的摘要、步骤提要、风险点、回退说明。
   - 落点：后续由 `context_builder.rs` 组装、`context_policy.rs` 决策预算和展开级别。
   - 特性：按命中和预算渐进加载，不默认整包注入。
4. Evidence Layer
   - 内容：验证摘要、失败证据、artifact/path/ref、必要 observation 引用。
   - 落点：`verify.rs` 产出的证据 + `context_builder.rs` 的 observation/evidence 注入位。
   - 特性：只注入当前动作需要的最小证据，不回灌为 system 规则。

#### 3.2 渐进加载冻结

1. Level 1：索引级摘要
   - 默认级别。
   - 只注入 skill 标识、适用条件、1 行摘要、来源等级。
2. Level 2：操作级摘要
   - 触发条件：Level 1 命中且当前动作需要明确操作步骤。
   - 注入内容：步骤提要、风险点、回退提示、所需工具类别。
3. Level 3：完整 skill / playbook / 样例
   - 触发条件：Level 2 仍不足，且 `context_policy` 预算允许。
   - 注入内容：完整 SOP、样例片段、限制说明。

#### 3.3 渐进加载规则冻结

1. 先召回，再命中，再展开，不允许默认整包加载 skill。
2. `context_policy.rs` 负责：
   - 是否允许 skill 注入
   - 当前最大加载级别
   - 加载预算与冲突处理
3. `context_builder.rs` 负责：
   - 输出固定四层结构
   - 记录本次实际注入级别与原因
4. 出现以下任一情况时禁止升级到 Level 3：
   - 当前任务更像事实问答而非执行任务
   - 当前预算已接近上限
   - skill 来源信任等级不足
   - 已有 evidence 足以完成当前动作

#### 3.4 当前代码映射冻结（2026-04-16 静态读证）

1. `context_builder.rs`
   - 已具备：`static_block/project_block/dynamic_block` 三级装配，及 observation 注入预算。
   - 未具备：显式 `system/run/skill/evidence` 四层命名结构与 skill 注入层稳定输出。
2. `context_policy.rs`
   - 已具备：按动作切换 `session/memory/knowledge/tool_preview` 的装配开关。
   - 未具备：`skill_level_cap`、`skill_budget_reason`、`trust_tier_cap` 等渐进加载决策字段。

#### 3.5 H03-02b 最小实现口径冻结

1. 第一批实现只允许补“层标识 + 实际注入级别”可观测字段。
2. 推荐最小字段：
   - `context_policy.rs`：`skill_injection_enabled`、`max_skill_level`
   - `context_builder.rs`：`injected_skill_level`、`injected_skill_ids`、`evidence_refs`
3. 在这些字段未落地前，四层注入只按“设计口径冻结”成立，不按“运行时分层已全量可见”成立。

### 4. H03-02c：Skill Guard 与信任分级口径冻结

#### 4.1 Skill Guard 检查面冻结

Skill Guard 至少覆盖以下检查项：

1. 文件数量与大小约束。
2. 路径越界与符号链接检查。
3. 版本 pin / manifest 完整性检查。
4. 敏感模式扫描（危险命令、越权路径、可疑外部依赖指令）。
5. 外部依赖声明检查。
6. 来源信任等级检查。
7. 注入预算检查（低信任 skill 不得默认高等级展开）。

#### 4.2 信任等级冻结

1. T0：内置技能（builtin）
   - 默认可进入 Level 2。
   - 进入 Level 3 仍受预算控制。
2. T1：项目受信技能（project_trusted）
   - 默认可进入 Level 2。
   - Level 3 需要当前任务强相关。
3. T2：本地生成技能（local_generated）
   - 默认只到 Level 1。
   - 升到 Level 2 需命中验证记录。
4. T3：外部导入技能（external_imported）
   - 默认只到 Level 1。
   - 进入 Level 2 需通过 Guard 且有人工确认或评审记录。
5. T4：社区技能（community）
   - 默认仅索引可见，不直接注入执行上下文。
   - 需先完成隔离校验、敏感扫描和项目级信任提升。

#### 4.3 Guard 动作冻结

1. allow：允许注入到当前预算级别。
2. review：保留索引级摘要，但禁止继续展开。
3. deny：禁止装载或注入，并生成 skip/reason/evidence。

#### 4.4 模块落点冻结

1. `skill_catalog.rs`
   - 增加来源等级、Guard 结果、skip reason 的稳定输出口径。
2. `context_policy.rs`
   - 依据 trust tier 决定最大展开级别。
3. `verify.rs`
   - 增加 skill 命中是否有效、是否因 Guard 被降级的证据字段。

#### 4.5 当前代码映射冻结（2026-04-16 静态读证）

1. `skill_catalog.rs`
   - 已具备：manifest 缺失、pin 不匹配、路径越界等 skip reason。
   - 未具备：trust tier 分层与 `allow/review/deny` Guard 动作输出。
2. `context_policy.rs`
   - 当前尚无依据 skill 来源限制 Level 1~3 的稳定逻辑。
3. `verify.rs`
   - 当前尚无 “因 Guard 被降级/拒绝” 的显式验证字段。

#### 4.6 H03-02c 最小实现口径冻结

1. 第一批实现只允许补 Guard 结果可观测字段，不允许引入新的 skill 生命周期系统。
2. 推荐最小字段：
   - `skill_catalog.rs`：`trust_tier`、`guard_action`、`guard_reason`
   - `context_policy.rs`：`max_skill_level_by_trust`
   - `verify.rs`：`guard_downgraded`、`guard_decision_ref`
3. Guard 字段未落地前，H03-02c 只视为“治理口径冻结”，不视为“Guard 运行时已闭环”。

### 4.7 接入能力分层与双层扩展口径冻结

#### 4.7.1 接入能力分层

1. H-03 不要求所有入口拥有相同的接入能力，而要求所有入口收敛到相同的裁决口径。
2. 当前至少区分以下三层能力：
   - L1：索引/感知层
     - 只能暴露 skill / MCP 存在与基础摘要，不能改写执行链。
   - L2：建议/评审层
     - 可以给出 `review / manual_takeover / fallback_hint`，但不能直接改写主链输入。
   - L3：受控注入层
     - 可以在 `context_policy/context_builder` 的约束下做 skill 注入、evidence 引用和受控展开。
3. 无论入口落在哪一层，最终都必须统一产出：
   - `route`
   - `fallback_reason`
   - `verify_result`
   - `evidence_ref`

#### 4.7.2 代码层 + 规则层双层扩展

1. 复杂判定保留在代码层：
   - trust tier
   - guard action
   - skill hit effectiveness
   - failure route
2. 轻量映射保留在规则层：
   - 样本分类映射
   - fallback 文案模板
   - 证据摘要模板
   - 命中有效性评测桶定义
3. 规则层失效时，必须回退到代码层默认口径，不得阻断主链。
4. 当前只冻结这条扩展原则，不在本轮引入新的 DSL 或配置系统。

### 5. 质量指标口径

1. `mcp_skills_success_rate`
   - 含义：主链样本中成功路由并完成执行的比例。
   - 阈值：`>= 92%`
2. `failure_locatable_rate`
   - 含义：失败样本中能明确给出 `failure_route`、`waiting_reason`、`evidence_ref` 的比例。
   - 阈值：`>= 95%`
3. `critical_skill_eval_pass_rate`
   - 含义：关键技能样本在最小评测包中的通过率。
   - 阈值：`>= 95%`
4. `skill_hit_effective_rate`
   - 含义：命中 skill 后，执行结果被 verify 判定为有效增益的比例。
   - 当前状态：冻结指标口径，暂未进入扩展样本实测。
5. `skill_false_positive_rate`
   - 含义：skill 被命中但 verify 认为对结果无增益或引入噪声的比例。
   - 当前状态：冻结指标口径，暂未进入扩展样本实测。
6. `fallback_trigger_rate`
   - 含义：命中 skill / MCP / 上下文增强后，因 Guard、预算、证据不足或执行失败而触发降级的比例。
   - 目标：可稳定观测，不要求当前轮次直接收阈值。
7. `raw_artifact_link_rate`
   - 含义：需要回退、误命中或 guard 降级的样本中，同时给出摘要引用与原始 artifact 引用的比例。
   - 目标：`= 100%` 作为设计目标，当前先冻结口径。

### 6. 最小评测包冻结

H-03 当前先冻结 5 类评测：

1. Skills 装载评测
   - manifest 缺失
   - pin 不匹配
   - entry 越界
   - workspace scope 匹配成功
2. 边界路由评测
   - Skill / Memory / Evidence 各自落点明确
   - 无“原始证据直接写 memory / skill”的旁路口径
3. 四层注入评测
   - system/run/skill/evidence 四层有稳定定义
   - skill 默认不越级展开
4. Skill Guard / trust 评测
   - trust tier 影响最大展开级别
   - deny/review/allow 结果可给出原因
5. fallback / 可观测评测
   - `confirmation_required -> waiting_reason=confirmation`
   - `run_failed -> failure_route/manual`
   - `artifact_path/raw_output_ref/evidence_ref` 串联

### 7. 证据目录冻结

H-03 证据统一冻结为以下结构：

1. `tmp/stage-h-mcp-skills/latest.json`
   - 聚合报告：覆盖项、主链结果、失败定位率、ready 状态。
2. `tmp/stage-h-mcp-skills/evals/*.json`
   - 分项 eval：skills、tool routing、visibility/fallback。
3. `tmp/stage-h-mcp-skills/fallback-cases.json`
   - 失败回退样本、回退路径、证据引用。
   - 每条样本后续应补齐：`summary_ref / raw_artifact_ref / fallback_reason / verify_result`。
4. `tmp/stage-h-mcp-skills/architecture-freeze-h03.json`
   - H03-02a/H03-02b/H03-02c 的冻结摘要、模块落点、后续待实现项。
5. `tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`
   - H-03 是否进入规模化扩样策略设计的判断、结构性缺口、策略轴与再提审最低门槛建议。

### 8. H-03 规模化扩样策略设计判断

1. 当前判断：H-03 下一步应进入“规模化扩样策略设计”阶段，而不是继续默认小步补样。
2. 原因：
   - 当前 `business-task-chain=20`、`skill-false-positive=16`、`manual-review=12` 已足以证明 H-03 不再只是数量不足问题。
   - 当前更主要的差距已转为结构性问题：
     1. 真实主链分布仍属中小样本
     2. `skill_hit_effective` 的真实分布尚未校准
     3. 长尾语境、恢复链与交叉复核虽已具备最小证据，但尚未形成可签收级分布规则
   - 若继续按对话式小步补样推进，边际收益会下降，并容易进入无限扩样。
3. 后续策略设计至少应覆盖以下轴：
   - 样本来源分层
   - 代表性分布要求
   - 长尾语境覆盖要求
   - 恢复链覆盖要求
   - 命中有效性分布校准要求
   - 人工复核轮次 / 角色多样性要求
   - 再次回刷 Gate-H 前的最小阈值
   - 接入能力分层与“双层扩展”何时落成正式执行规则
4. 下一轮 Gate-H 提审前的最低门槛建议：
   - `business-task-chain >= 30`
   - `skill-false-positive >= 24`
   - `manual-review >= 16`
   - 长尾行业语境至少形成 4 类稳定分层，且每类至少 2 条非换皮样本
   - 恢复链至少覆盖 5 条，其中至少 2 条为三段及以上链路
   - `skill_hit_effective` 需补最小校准集，不能继续只看定向误命中样本
   - 人工复核至少体现 2 轮复核视角或等价角色差异说明
5. 当前不建议直接承诺上述门槛等于最终 ready 门槛；它们只用于“何时值得再次回刷 Gate-H”的最低判断口径。

## 实现边界

1. H03-02a / H03-02b / H03-02c 当前允许的最小增量仅限：
   - 文档冻结。
   - 最小证据占位或样例 JSON。
   - 为后续实现明确模块落点、字段口径与评测入口。
2. H03-02a / H03-02b / H03-02c 当前不允许扩项到：
   - 新增大规模 MCP 网络层实现。
   - 修改 H-02 / H-04 / H-05 主链。
   - 为了评测而重写 runtime 主状态机。
   - 一次性引入新的技能资产生命周期系统。

## 风险与回退

1. 主要风险：
   - 若 Skill / Memory / Evidence 边界不冻结，后续 memory 写入与 skill 注入会互相污染。
   - 若四层注入口径不冻结，skill 很容易被直接塞进 system 或 run 层，导致上下文膨胀。
   - 若 Skill Guard 与 trust tier 不冻结，后续外部 skill 的引入将缺乏统一治理口径。
   - 若把外部参考项目的 WSL-first / token-first 目标直接照搬，会偏离本项目 Windows-first 与质量优先的主目标。
   - 若只记录摘要、不保留原始 artifact 引用，后续 fallback 与误命中样本将难以复盘。
2. 回退方式：
   - 暂停任何扩展实现，只保留当前冻结文档与证据占位。
   - 若后续字段口径不稳，先维持文档冻结，不宣称 H-03 ready。


## 9. H-03 规模化扩样策略设计

### 9.1 设计目标与适用边界

#### 设计目标
本节设计的目标不是继续进行零散补样，而是将 H-03 从“判断已完成 / 待执行”的状态推进到“设计可执行”的状态，形成后续规模化扩样的正式策略基础。
本设计应为下一轮 H-03 执行任务、验证口径与 Gate-H 再提审门槛提供统一依据，避免后续继续按局部问题、单点样本和临时讨论方式推进。

#### 本轮冻结范围（H03-30 ~ H03-32）
本轮正式定向执行只冻结以下三项：

1. H03-30：策略设计总章骨架、适用边界、非目标、风险与停止条件；
2. H03-31：样本来源分层规则与代表性分布检查规则；
3. H03-32：长尾语境覆盖矩阵与恢复链覆盖矩阵。

本轮明确不执行：

1. H03-33：命中有效性校准集定义；
2. H03-34：人工复核轮次与角色差异规则；
3. H03-35：Gate-H 再提审最低门槛冻结；
4. H03-36：执行前提审包形成。

#### 适用边界
本节冻结后的规则只用于：

1. 约束 H-03 后续如何组织扩样；
2. 约束后续样本如何进入代表性、长尾与恢复链统计；
3. 为后续 H03-33 ~ H03-36 提供稳定前置输入。

本节不用于：

1. 直接替代样本执行；
2. 直接替代命中有效性校准；
3. 直接替代多轮人工复核；
4. 直接给出 Gate-H 再提审结论。

#### 非目标
本轮设计明确不承担以下目标：

1. 不直接执行规模化扩样本身；
2. 不直接新增 runtime 实现改动；
3. 不直接将 H-03 改判为 ready；
4. 不直接触发 Gate-H 再提审；
5. 不以“再补几个样本”替代结构性策略设计。

#### 为什么现在进入策略设计
当前 H-03 已完成“是否进入规模化扩样策略设计”的判断。
现有最小有界证据已足以表明：问题已从“样本数量偏少”转为“结构性缺口未建模”。

当前已有基线：

- business-task-chain = 20
- skill-false-positive = 16
- manual-review = 12
- long_tail_context_samples = 5
- multi_round_recovery_samples = 3
- cross_review_disagreement_samples = 2

现阶段继续默认以小步补样推进，边际收益已经下降，且难以支撑 H-03 从 warning 收缩到 ready。因此，下一步必须先把规模化扩样策略写成正式设计，而不是继续无界补样。

#### 与当前最小有界证据的关系
本设计建立在现有多轮 bounded evidence 的基础上。
当前证据已经足以支持以下判断：

- H-03 不是 ready；
- H-03 不再适合继续默认小步补样；
- H-03 的下一步应切换为策略设计执行；
- Gate-H 当前仍不可签收。

因此，本设计不是在证据不足时做空想规划，而是在已有最小覆盖基础上，对后续扩样进行结构化治理设计。

#### 风险与停止条件
本轮 H03-30 的冻结条件为：

1. 设计目标、适用边界、非目标、风险与停止条件已显式成文；
2. 已明确本轮只做到 H03-30 ~ H03-32；
3. 已明确 H03-33 ~ H03-36 仍未执行；
4. 已明确 H-03 仍为 warning，Gate-H 仍不可签收。

若上述任一项缺失，则 H03-30 不应视为完成。

#### 与 Gate-H 的关系
本设计不直接改变 Gate-H 当前结论。
当前 Gate-H 仍不可签收，原因仍包括：

- H-02 仍为 warning；
- H-03 仍为 warning。

本设计的作用是明确：H-03 的 warning 后续应如何被继续收缩，以及在什么条件下才值得回刷 Gate-H。

### 9.2 当前基线与结构性缺口

#### 当前基线
当前 H-03 已形成如下最小基线：

- business-task-chain = 20
- skill-false-positive = 16
- manual-review = 12
- long_tail_context_samples = 5
- multi_round_recovery_samples = 3
- cross_review_disagreement_samples = 2

#### 当前已有能力
当前证据已经证明以下能力已具备最小覆盖：

1. H03-01 ~ H03-02c 边界已冻结；
2. 四层注入、Guard 与信任分级口径已冻结；
3. 最小可观测字段已具备；
4. 已有多轮有界补证；
5. 已覆盖最小长尾样本；
6. 已覆盖最小恢复链样本；
7. 已有最小交叉复核说明。

#### 结构性缺口
当前距离 ready 的核心缺口已不再是“数量是否继续增加”，而是以下结构性缺口：

1. **真实主链分布仍属中小样本**  
   当前 20 条业务链足够支撑 warning 收缩，但不足以支撑 ready 判断。

2. **命中有效性分布尚未校准**  
   当前 skill-false-positive 样本是定向误命中口径；  
   `skill_hit_effective` 的真实分布尚未形成正式校准集。

3. **样本来源分层未正式定义**  
   现有样本尚未按来源层、任务层、业务层与信任层形成正式分层策略。

4. **长尾语境覆盖尚未成体系**  
   目前已有长尾与行业尾部样本，但尚未形成“类别约束 + 非换皮要求 + 分布规则”。

5. **恢复链覆盖仅有最小模式**  
   当前已有最小恢复链证据，但仍缺更长链、更自然分布、更高频模式的覆盖要求。

6. **人工复核仍属最小交叉说明**  
   当前有人工复核与分歧说明，但还未形成轮次要求、角色差异要求与正式治理强度。

#### 结论
H-03 当前已进入结构性阶段。
后续必须通过正式策略设计推进，而不是继续以局部补样方式推进。

### 9.3 策略轴 A：样本来源分层

#### 目标
后续样本不再按零散个案组织，而是按来源层、任务层、业务层与信任层形成稳定分层，以避免样本只在局部来源上堆积。

#### 分层字段
后续样本至少应具备以下分层字段：

- `source_tier`
- `source_origin`
- `domain_family`
- `task_family`
- `trust_tier`
- `import_path_type`
- `case_shape`
- `coverage_role`

#### 最低来源层定义
后续最少分为以下四层：

1. `builtin`
2. `project_trusted`
3. `local_generated`
4. `external_imported`

必要时允许在此基础上继续叠加：

- 行业域
- 任务类型
- 冲突类型
- 恢复链类型

#### 归类规则
每条样本必须能回答以下问题：

1. 样本来自哪个来源层；
2. 样本属于哪个业务域；
3. 样本属于哪个任务族；
4. 样本所依赖的信任等级是什么；
5. 样本在引入路径上属于哪类来源方式；
6. 样本在本轮覆盖中承担什么角色：`mainstream / secondary / tail / recovery / conflict`。

#### 执行时必须检查的规则
H03-31 冻结后，后续组织样本时必须检查以下规则：

1. 每条样本必须完整填写来源字段，字段缺失不得计入正式覆盖；
2. 每一来源层至少要有 1 条主流样本与 1 条非标准路径样本；
3. 若某来源层只出现误命中样本，不得宣称该来源层已完成覆盖；
4. 若样本来源层与信任层长期绑定为单一组合，必须标记为“分层失真”；
5. 任何被作为代表性样本引用的条目，都必须能被追溯到来源层和任务层。

#### 覆盖要求
后续每一来源层都应至少覆盖：

- 主流样本
- 尾部样本
- 至少一类非标准路径样本（如冲突、恢复、降级）

#### 非换皮约束
以下情况不得计为新增有效样本：

1. 仅替换行业名称但执行链与冲突结构完全相同；
2. 仅替换 skill 名称但来源层、任务族、回退路径不变；
3. 仅替换描述文本但没有新增覆盖角色；
4. 仅把同一链路拆成多个表述版本重复计数。

#### 禁止项
以下情况视为分层失效：

- 所有样本集中在单一来源层；
- 所有样本集中在单一信任层；
- 所有样本只来自最容易构造的 project_trusted 或 local_generated 层。

### 9.4 策略轴 B：代表性分布要求

#### 目标
样本后续不仅看总量，还必须看分布。
分布是 H-03 是否值得再提审的重要依据。

#### 建议字段
后续样本至少应标记：

- `distribution_bucket`
- `traffic_shape`
- `complexity_level`
- `fallback_route`
- `conflict_density`
- `hit_outcome_type`

#### 分布桶定义
至少划分以下三档：

1. 主流
2. 次主流
3. 尾部

#### 必须体现的分布维度
后续样本分布至少应覆盖：

- 单步链路 / 多步链路 / 冲突链路
- manual / verify / degrade / recover 路径
- 主流 / 次主流 / 尾部场景
- 正常命中 / 噪声命中 / 误命中 / 人工辅助成立

#### 执行时必须检查的规则
H03-31 冻结后，后续每轮扩样至少应完成以下分布检查：

1. 统计三档分布桶，不得只报总量；
2. 统计链路形态，不得只报主流短链；
3. 统计 fallback 路径，不得把 `manual/verify/degrade/recover` 混为一类；
4. 统计命中结果类型，不得只报 false positive；
5. 若新增样本导致单一桶占比继续上升，必须显式记录“分布失真风险”。

#### 分布目标
每轮扩样必须明确给出：

- 主流样本比例
- 尾部样本比例
- 恢复链比例
- 冲突样本比例
- false positive 比例
- manual-review 介入比例

#### 不计入代表性覆盖的情况
以下情况不能视为“代表性分布已形成”：

- 全部集中在高冲突样本；
- 全部集中在误命中样本；
- 全部集中在单一业务域；
- 总量增加，但分布失真更严重。

### 9.5 策略轴 C：长尾语境覆盖要求

#### 目标
把“已有少量长尾样本”推进为“长尾覆盖有类别约束、有非换皮要求、有观测字段”的正式策略。

#### 建议字段
长尾样本至少应补充：

- `long_tail_reason`
- `industry_tail_type`
- `context_type`
- `domain`
- `non_redundant_justification`
- `tail_category`

#### 长尾定义
长尾不是“少见即可”，而是必须满足下列之一：

1. 行业语境明显偏尾部；
2. 任务上下文与主流样本差异明显；
3. 现有证据链难以直接套用；
4. 对恢复、降级或人工复核具有放大效应。

#### 长尾覆盖矩阵（H03-32 冻结）

| 长尾类别 | 说明 | 每类最小要求 | 必填字段 | 不计入覆盖的情况 |
|---|---|---|---|---|
| 行业监管尾部 | 如食品安全、贸易融资等行业尾部语境 | 每类至少 2 条非换皮样本 | `industry_tail_type`, `domain`, `non_redundant_justification` | 仅替换行业名但链路结构不变 |
| 工具/入口尾部 | 非主流 skill 来源、私有 CLI、导入路径重叠 | 每类至少 2 条非换皮样本 | `source_tier`, `import_path_type`, `long_tail_reason` | 仅换 skill 名称或入口名 |
| 自然复杂冲突尾部 | 多候选冲突、上下文重叠、证据不足导致的自然复杂度 | 每类至少 2 条非换皮样本 | `context_type`, `conflict_density`, `non_redundant_justification` | 重复同一冲突模板 |
| 恢复放大型尾部 | 对 manual / degrade / recheck 有放大效应的尾部任务 | 每类至少 2 条非换皮样本 | `recovery_chain`, `long_tail_reason`, `tail_category` | 只是一段普通失败，无恢复放大效应 |

#### 最低类别要求
下一轮至少形成：

- 长尾行业类别不少于 4 类；
- 每类不少于 2 条非换皮样本。

#### 非换皮判定
样本不能仅换标题、换轻微上下文或换表述方式。
每条长尾样本必须说明：

1. 为什么属于尾部；
2. 与现有样本差异在哪里；
3. 为什么不能视为换皮复用。

#### 结论要求
长尾覆盖的目标不是证明“尾部存在”，而是证明：

- 尾部类别已可分层；
- 尾部覆盖有稳定最低要求；
- 尾部样本可进入正式复核与命中有效性分析。

### 9.6 策略轴 D：恢复链覆盖要求

#### 目标
后续不只看失败样本，还要看恢复链路是否成体系。

#### 建议字段
恢复链样本至少应具备：

- `recovery_chain`
- `recovery_stage_count`
- `recovery_terminal_state`
- `degrade_reason`
- `recheck_required`
- `recovery_pattern`

#### 恢复链覆盖矩阵（H03-32 冻结）

| 恢复模式 | 模式说明 | 最低要求 | 链长要求 | 不计入覆盖的情况 |
|---|---|---|---|---|
| `guard_review -> manual -> verify_recheck` | 因 Guard 降级进入人工确认，再回到 verify 重检 | 至少 1 条 | 至少 3 段 | 只有 review 标记，没有后续 manual/recheck |
| `verify_failed -> manual_retry -> recover` | 首轮 verify 失败后，经人工重试恢复 | 至少 1 条 | 至少 3 段 | 只有失败与重试，没有恢复终态 |
| `multi_candidate_conflict -> degrade -> final_clear_decision` | 候选冲突后经 degrade 收敛到清晰裁决 | 至少 1 条 | 至少 3 段 | 单次冲突即结束，未出现 degrade |
| `tail_context -> manual -> partial_recover / unrecoverable` | 长尾尾部任务经人工介入后仅部分恢复或仍不可恢复 | 至少 1 条 | 至少 3 段 | 把普通失败样本误算为恢复链 |
| `tool_or_skill_noise -> fallback -> recover` | 工具或 skill 噪声触发 fallback 后完成恢复 | 至少 1 条 | 至少 3 段 | 只有 fallback，没有恢复结果 |

#### 最低链长要求
恢复链总量应不少于 5 条，其中：

- 三段及以上恢复链不少于 2 条

#### 恢复结果分类
恢复链终态至少区分：

- 成功恢复
- 部分恢复
- 需人工清决
- 不可恢复

#### 禁止项
以下情况不能计入恢复链覆盖：

- 单点失败即算恢复链；
- 无明显转折过程的重复动作；
- 不含 degrade / manual / recheck 的短平快样本被误算为恢复链。

### 9.7 策略轴 E：命中有效性分布校准要求

#### 目标
这是 H-03 当前最关键的结构性缺口之一。
后续不能继续只看 `skill-false-positive`，而必须正式建立 `skill_hit_effective` 的校准集。

#### 建议字段
命中有效性校准至少应补充：

- `skill_hit`
- `skill_hit_effective`
- `effective_type`
- `evidence_gain_level`
- `manual_assist_required`
- `calibration_bucket`
- `calibration_reason`
- `blocking_uncertainty`

#### 命中有效性校准矩阵（H03-33 冻结）

| 校准桶 | 判定标准 | 必填字段 | 必须排除的误判 | 可否直接外推 |
|---|---|---|---|---|
| `true_positive_effective` | 命中 skill 后对结果形成明确有效增益，且无需靠额外人工兜底才能成立 | `skill_hit=true`, `skill_hit_effective=true`, `effective_type`, `evidence_gain_level` | 把“只是命中但无增益”误算为有效 | 否，只能进入已校准样本集 |
| `false_positive_noise` | 命中 skill 但未带来有效增益，且引入噪声、误导或额外成本 | `skill_hit=true`, `skill_hit_effective=false`, `calibration_reason` | 把所有失败都归因于 skill 命中 | 否，只能证明噪声模式存在 |
| `degraded_but_salvageable` | 首轮命中未直接成功，但经 degrade / fallback 后仍可挽救 | `skill_hit=true`, `calibration_bucket`, `fallback_reason`, `verify_result` | 把完全失败样本误算为可挽救 | 否，只能说明需受控降级 |
| `manual_assisted_effective` | 命中结果需人工辅助后才成立，不能当作纯自动有效命中 | `manual_assist_required=true`, `effective_type`, `calibration_reason` | 把人工辅助成功误写成纯自动有效 | 否，只能归入人工辅助成立 |
| `inconclusive` | 当前证据不足以判断命中是否有效，或摘要/原始证据冲突 | `blocking_uncertainty`, `calibration_reason` | 在证据不足时强行归类 | 否，必须后续复核 |

#### 边界定义
H03-33 冻结后，后续执行必须遵守以下边界：

1. `true_positive_effective` 只适用于“命中带来明确增益且证据闭环成立”的样本；
2. `false_positive_noise` 只适用于“命中无增益或引入噪声”的样本，不能拿主链失败一概代替；
3. `degraded_but_salvageable` 必须出现明确的降级或回退转折；
4. `manual_assisted_effective` 必须显式标注人工介入，不得与纯自动有效命中混算；
5. `inconclusive` 必须保留不可判定原因，不得被强行并入其它桶。

#### 不可判定口径
以下情况统一进入 `inconclusive`：

1. 摘要证据与原始 artifact 引用互相冲突；
2. 无法判断增益是否来自 skill，而非其它上下文或人工补救；
3. 仅有结论，没有可复核的 `fallback_reason / verify_result / evidence_ref`；
4. 样本结构过于局部，无法支持最小校准判断。

#### 执行时必须检查的规则
H03-33 冻结后，后续校准执行必须检查：

1. 每条进入校准集的样本都必须落入 5 个桶中的恰好 1 个；
2. 不得只扩 `false_positive_noise`，必须同步补 `true_positive_effective` 或 `manual_assisted_effective`；
3. `degraded_but_salvageable` 样本必须同时给出降级原因与恢复终态；
4. `inconclusive` 样本必须保留阻塞原因，不得静默丢弃；
5. 命中有效性统计不得把人工辅助样本与纯自动样本直接合并成同一有效率。

#### 结论要求
`skill_hit_effective` 后续必须成为可用于 Gate-H 再提审判断的指标，而不能继续只作为占位描述。

### 9.8 策略轴 F：人工复核轮次 / 角色多样性要求

#### 目标
将当前最小交叉复核说明，升级为正式治理要求。

#### 建议字段
后续复核至少应补充：

- `review_round`
- `review_role`
- `review_disagreement`
- `cautious_reason`
- `ready_blocker_flag`
- `review_scope`
- `review_decision_basis`

#### 最低复核要求
下一轮至少体现：

- 2 轮复核视角  
  或
- 等价的角色差异说明

#### 必须进入复核的样本
以下样本必须进入至少 1 轮复核：

1. 长尾行业样本
2. 恢复链样本
3. 命中有效性边界样本
4. 对 ready 判断具有阻塞意义的样本
5. 被标记为 `inconclusive` 的校准样本

#### 必须双轮或等价差异复核的样本
以下样本必须满足“双轮复核”或“等价角色差异复核”：

1. `manual_assisted_effective` 样本；
2. `degraded_but_salvageable` 样本；
3. 存在 `review_disagreement=true` 的样本；
4. 任何 `ready_blocker_flag=true` 的样本；
5. 任何拟被用于 Gate-H 再提审代表性引用的样本。

#### 角色差异规则
后续复核角色至少应形成以下差异之一：

1. 执行链保守审查视角；
2. 证据/验证一致性视角；
3. 分布代表性与外推风险视角。

若无法提供不同人员，也必须给出等价的差异角色说明，不得把同一视角重复计为双轮复核。

#### 分歧记录要求
复核输出不能只写“accepted_as_noise”。
必须能表达：

- 为什么仍判 warning；
- 为什么该样本不能外推为 ready；
- 为什么该样本只能收缩 warning，不能消除 warning。

#### ready blocker 规则
满足以下任一条件的样本，应标记 `ready_blocker_flag=true`：

1. 校准桶仍无法稳定判定；
2. 长尾类别虽命中但代表性不足；
3. 恢复链只体现局部成功，无法外推；
4. 人工辅助成分过高，不能证明自动链稳定成立。

#### 执行时必须检查的规则
H03-34 冻结后，后续复核执行必须检查：

1. 进入复核的样本必须写明 `review_round` 与 `review_role`；
2. 双轮复核样本必须能展示差异视角，不得同义重复；
3. 存在分歧时必须写明 `review_disagreement` 与 `cautious_reason`；
4. 对 ready 有阻塞意义的样本必须显式写 `ready_blocker_flag`；
5. 复核结论不得把“局部成立”写成“总体 ready”。

### 9.9 策略轴 G：再提审 Gate-H 前的最低门槛

#### 目标
把下一轮 Gate-H 回刷前的最低门槛正式固定，避免后续继续用“再补一些样本”这种模糊口径推进。

#### 最低门槛冻结表（H03-35）

| 门槛类型 | 维度 | 最低门槛 |
|---|---|---:|
| 数量门槛 | business-task-chain | >= 30 |
| 数量门槛 | skill-false-positive | >= 24 |
| 数量门槛 | manual-review | >= 16 |
| 分布门槛 | 长尾行业类别 | >= 4 类，且每类 >= 2 条非换皮样本 |
| 分布门槛 | 恢复链样本 | >= 5 条 |
| 分布门槛 | 三段及以上恢复链 | >= 2 条 |
| 校准门槛 | 命中有效性校准集 | 5 个校准桶均有定义，且至少形成最小可用集 |
| 复核门槛 | 复核轮次 / 角色差异 | >= 2 轮或等价说明 |

#### 阻断规则
即使数量达到，也不能直接视为 ready，也不得直接回刷 Gate-H。
出现以下任一情况，均应阻断再提审：

1. 数量达标但来源分层仍失真；
2. 数量达标但长尾类别未达到 4 类稳定分层；
3. 数量达标但恢复链仍是点状样本，不成矩阵；
4. 数量达标但 `skill_hit_effective` 校准集仍未形成；
5. 数量达标但人工复核仅停留在单轮或同视角复核。

#### 判定要求
即使数量达到，也不能直接视为 ready。
仍需同时满足：

1. 样本来源分层已形成稳定口径；
2. 长尾语境覆盖已具备类别约束；
3. 恢复链分布已不是最小点状覆盖；
4. 命中有效性校准集已可用于判断；
5. 人工复核强度已具备治理意义。

#### H03-35 完成定义
只有当以下内容同时成立时，H03-35 才可视为完成：

1. 数量门槛、分布门槛、校准门槛、复核门槛四类门槛均已成文；
2. 已明确“数量达标但结构不达标时，不得回刷 Gate-H”；
3. 已明确这些门槛只是“最低再提审门槛”，不等于 ready 门槛。

### 9.10 风险、停止条件与后置项

#### 继续零散补样的风险
若不先做策略设计，而继续零散补样，主要风险包括：

1. 无限扩样风险；
2. 定向样本堆数量风险；
3. 分布失真风险；
4. Gate-H 长期无法进入清晰裁决风险。

#### 本轮停止条件
本轮策略设计任务可停止的条件是：

1. 七个策略轴已完整落入 design；
2. 最低门槛表已冻结；
3. verify 与 review 已升级；
4. 下一步已切换为执行任务，而不是继续判断。

#### 本轮做什么
本轮只做：

- 冻结策略设计章节；
- 冻结任务拆分；
- 冻结验证口径；
- 冻结评审口径；
- 冻结再提审最低门槛。

#### 明确后置什么
以下事项后置到下一轮：

1. 真正执行规模化扩样；
2. 新一轮样本补充；
3. 命中有效性校准集实施；
4. 2 轮以上复核执行；
5. H-03 再提审；
6. Gate-H 回刷。


### 9.11 策略设计执行顺序与完成定义

#### 执行顺序冻结
H-03 规模化扩样策略设计后续按以下顺序推进，不允许跳步，也不允许把后项产出倒填为前项已完成：

1. H03-30：冻结规模化扩样策略文档骨架
2. H03-31：定义样本来源分层与代表性分布规则
3. H03-32：定义长尾语境与恢复链覆盖矩阵
4. H03-33：定义命中有效性校准集
5. H03-34：定义人工复核轮次与角色差异规则
6. H03-35：冻结 Gate-H 再提审最低门槛
7. H03-36：形成规模化扩样执行前提审包

#### 每步完成定义
1. H03-30 完成后：
   - 设计总章、七轴结构、In/Out of scope、风险与停止条件齐备；
   - 只能说明“策略设计骨架已冻结”，不能启动样本执行，也不能代表后续各轴细则已完成。
2. H03-31 完成后：
   - 必须形成来源层、分布桶、复杂度/冲突/恢复路径等基础分层规则；
   - 只能说明“后续扩样的取样框架已明确”，不代表长尾、恢复链、命中有效性、复核门槛已完成。
3. H03-32 完成后：
   - 必须形成长尾类别矩阵、恢复链模式矩阵、非换皮要求、最小链长要求；
   - 只能说明“长尾与恢复链的覆盖约束已明确”，不代表样本已经补齐。
4. H03-33 完成后：
   - 必须形成 `skill_hit_effective` 校准桶、字段、边界定义与不可判定口径；
   - 只能说明“后续能按统一标准判断命中是否有效”，不代表校准集已经执行完成。
5. H03-34 完成后：
   - 必须形成复核轮次、角色差异、分歧记录、ready blocker 标识规则；
   - 只能说明“后续复核治理方式已冻结”，不代表已经具备签收级复核强度。
6. H03-35 完成后：
   - 必须形成数量门槛、分布门槛、校准门槛、复核门槛四类最低门槛；
   - 必须能回答“达到什么条件才值得再次回刷 Gate-H”；
   - 不得把“值得再提审”误写成“达到 ready”。
7. H03-36 完成后：
   - 必须形成“策略设计完成，可进入后续执行”的 verify/review 口径；
   - 只能说明“可进入后续执行”，不能说明“H-03 ready”或“Gate-H 可签收”；
   - 也不能说明 active change 已切换或 H-03 已开跑。

#### 每步不代表什么
1. H03-30 ~ H03-34 任一步完成，都不代表后续扩样已开始。
2. H03-31 ~ H03-34 任一步完成，都不代表 H-03 warning 已消除。
3. H03-35 完成，不代表 Gate-H 已具备再提审资格，只代表最低门槛已冻结。
4. H03-36 完成，不代表 ready，不代表 Gate-H 可签收，不代表主推进已切换。

#### 什么叫“策略设计完成，可进入后续执行”
仅当以下条件同时满足时，才可判定为“策略设计完成，可进入后续执行”：

1. H03-30 ~ H03-36 的顺序、边界、验收标准已稳定；
2. 七个策略轴都已从原则描述收口为带字段、分层/分桶、最低要求、禁止项的正式设计；
3. Gate-H 再提审最低门槛已冻结为统一口径；
4. `verify.md` 已能验证 H03-30 ~ H03-36 全部完成，`review.md` 已能评审“策略设计已闭环”；
5. 文档整体已经明确：当前仍未开跑实际扩样执行，仍不是 ready，仍不是 Gate-H 可签收依据，也不等于 active change 切换。

#### 为什么这不等于 Gate-H 可签收
1. 当前完成的是“执行策略的设计包”，不是“按策略补足证据”的执行结果。
2. Gate-H 看的是 H-03 warning 是否被证据性收缩到可签收，而不是策略文档是否写完整。
3. H-03 当前仍缺：
   - 更大规模真实主链分布证据
   - `skill_hit_effective` 校准集执行结果
   - 达到门槛后的正式多轮复核证据
4. 因此，策略设计完成与 Gate-H 可签收之间，仍隔着“按设计执行并补足证据”这一阶段。

#### 与 Gate-H 再提审的关系
1. H03-30 ~ H03-36 的目标是把 H-03 从“草稿可复用”收口成“策略设计已完成、可进入后续执行”的状态。
2. H03-30 ~ H03-36 全部完成，只能作为下一轮执行前提，不直接触发 Gate-H 再提审。
3. Gate-H 再提审至少还需满足：
   - `business-task-chain >= 30`
   - `skill-false-positive >= 24`
   - `manual-review >= 16`
   - 长尾行业类别、恢复链、命中有效性校准集、人工复核轮次达到已冻结门槛
4. 因此，策略设计完成与 Gate-H 可签收之间，仍隔着“按设计执行并补足证据”这一阶段。
