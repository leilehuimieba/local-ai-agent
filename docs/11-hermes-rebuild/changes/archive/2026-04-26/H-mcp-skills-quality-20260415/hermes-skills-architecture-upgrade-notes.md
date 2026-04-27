# 基于 Hermes Skills 闭环的架构升级建议

更新时间：2026-04-16
状态：草案
适用范围：`H-mcp-skills-quality-20260415`

## 1. 判断结论

### 1.1 当前架构是否合适

当前项目主骨架仍然合适，不建议推倒重来。

现有主干：

1. `runtime-core`
2. `gateway`
3. `frontend`
4. `tool_registry`
5. `memory_router`
6. `verify`
7. `risk`
8. `run_resume / checkpoint`

已经具备“可执行、可恢复、可验证、可治理”的基础，不属于需要重做主循环或重写运行时的状态。

### 1.2 是否需要升级

需要升级，但重点不是重做主循环，而是补齐“可持续成长”的能力层。

核心判断：

1. 现阶段最该升级的是 `Skills 闭环`，不是 `Runtime 主状态机`。
2. 架构方向应从“执行架构”升级为“执行 + 成长架构”。
3. 升级方式应为“在现有基础上增量补层”，而不是“大重构后再接回现有能力”。

## 2. 从 Hermes Skills 闭环学到的关键点

### 2.1 Skill 与 Memory 必须硬分层

建议明确区分：

1. Memory：
   - 用户偏好
   - 环境事实
   - 项目事实
   - 历史结论
2. Skill：
   - 可复用任务 SOP
   - 操作步骤
   - 常见失败与规避方式
   - 适用条件与回退路径
3. Evidence / Artifacts：
   - 回放样本
   - provider 输出
   - 验证结果
   - 页面/API 证据

当前项目不应继续把“事实、方法、证据”混为一种长期知识。

### 2.2 需要的是 Skill 闭环，而不是更多 Skill 文件

建议补齐以下链路：

`任务执行 -> 经验提取 -> Skill 草案 -> 安全检查 -> 索引 -> 条件激活 -> 渐进加载 -> 执行验证 -> patch 改进`

重点不是“手工新增更多 skill”，而是“让系统把经验沉淀成方法资产并持续修订”。

### 2.3 上下文注入必须分层

建议在现有 `context_builder / context_policy` 基础上固定四层注入：

1. Stable System Layer
   - 最稳定的系统契约与风格规则
2. Run Context Layer
   - 当前任务、阶段、工作区、change 信息
3. Skill Injection Layer
   - 命中 skill 的摘要、步骤、风险点、回退说明
4. Evidence Layer
   - provider 结果、失败证据、验证样本

不建议把 skill 或学习结果直接堆入 system prompt。

### 2.4 渐进式加载应成为默认机制

建议 skill 采用三级披露：

1. Level 1：索引级摘要
2. Level 2：操作级摘要
3. Level 3：完整 skill / playbook / 样例

原则：

1. 先召回，再命中，再展开
2. 不默认整包加载 skill
3. 保证上下文成本可控、冲突可追踪

### 2.5 技能资产需要独立安全治理

当前项目已具备运行时风险治理，但后续需要补 `Skill Guard`。

建议 Skill Guard 覆盖：

1. 文件数量与大小约束
2. 路径与符号链接检查
3. 敏感模式扫描
4. 外部依赖声明检查
5. 权限越界与危险命令模式
6. 按来源做信任分级

建议信任等级至少区分：

1. 内置技能
2. 项目受信技能
3. 本地生成技能
4. 外部导入技能
5. 社区技能

### 2.6 需要“经验资产评测”，不只是任务验收

在现有 `verify + tmp/stage-* + evidence` 基础上，建议补四类评测：

1. Skill 命中有效率
2. Skill 误命中率
3. 渐进加载收益
4. patch 改进收益

H-03 不应只验证“能否加载 skill”，还应验证“skill 是否真的提升执行质量”。

## 3. 对当前项目的升级建议

### 3.1 保留项（不建议重做）

以下主骨架建议保留：

1. `runtime-core / gateway / frontend` 三层主结构
2. `tool_registry` 作为能力枚举入口
3. `verify` 作为执行后验证主链
4. `risk` 作为运行时风险控制主入口
5. `run_resume / checkpoint` 作为恢复主链

### 3.2 需要新增的一层

建议在现有基础上新增“经验资产层 / Skill Loop Layer”。

逻辑职责包括：

1. Skill 元数据
2. Skill 索引
3. Skill 激活
4. Skill 渐进加载
5. Skill patch 改进
6. Skill Guard

该层不替代 memory，也不替代 tool registry，而是连接：

1. 学习结果
2. 经验沉淀
3. 上下文装配
4. 执行验证

### 3.3 模块落点建议

#### runtime-core

重点扩展：

1. `skill_catalog.rs`
   - 从“技能清单”提升为“技能资产入口”
2. `context_builder.rs`
   - 明确 skill 注入层级与渐进加载
3. `context_policy.rs`
   - 增加 skill 加载预算、优先级与冲突规则
4. `memory_router.rs`
   - 明确 facts 与 skills 的路由边界
5. `verify.rs`
   - 增加 skill 命中后效果验证口径

#### gateway

主要承担：

1. provider 输出标准化
2. fallback 信息透传
3. trace/evidence 聚合

不建议 gateway 承担 skill 生命周期本体。

#### frontend

主要承担：

1. 命中 skill 可视化
2. 加载级别可视化
3. fallback 与失败定位展示

不建议让前端承担技能决策逻辑。

## 4. 当前最值得优先推进的升级项

### P0

1. Skill / Memory / Evidence 边界冻结
2. Skill 注入分层冻结
3. Skill 元数据与激活规则冻结
4. Skill 安全治理口径冻结

### P1

1. 渐进式加载实现
2. Skill 命中与误命中评测
3. patch 改进最小闭环

### P2

1. 社区技能导入
2. 更复杂的 MCP 生态扩展
3. 更大规模多站点 / 多来源技能生态

## 5. 不建议当前推进的方向

1. 不建议为了对齐 Hermes 而重做 runtime 状态机
2. 不建议一开始就做多智能体重编排
3. 不建议先做大规模抽象层重写
4. 不建议在没有评测前扩大量技能来源

## 6. 对 H-03 的直接启发

H-03 不应停留在：

1. MCP 能接进来
2. Skills 能加载
3. 失败能定位

还应补到：

1. Skill 是否命中正确
2. Skill 是否真的提升成功率
3. Skill 是否被安全治理
4. Skill 是否按预算渐进加载
5. Skill 与 memory 是否边界清晰

## 7. 建议的后续 change / 子任务方向

可在 H-03 后续细化时拆出：

1. `H03-skill-boundary-freeze`
2. `H03-skill-injection-layer`
3. `H03-skill-guard`
4. `H03-skill-eval-pack`
5. `H03-skill-patch-loop`

当前不要求立即拆目录，但这些方向应进入 H-03 的后续任务口径。
