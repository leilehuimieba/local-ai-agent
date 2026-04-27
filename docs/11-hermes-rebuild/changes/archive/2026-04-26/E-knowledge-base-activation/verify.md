# 验证记录

## 验证方式

- 单元测试：
  1. `cargo test -p runtime-core knowledge_type_accepts_agent_resolve_when_verified -- --nocapture`
  2. `cargo test -p runtime-core knowledge_summary_falls_back_to_final_answer_when_short -- --nocapture`
- 集成测试：本刀未新增接口，仅做运行态只读核查（settings 中知识库路径与计数）。
- 人工验证：源码走查 `memory_router` 写入链路（`knowledge_type` 与 `knowledge_summary`）。

## 证据位置

- 测试记录：
  1. `knowledge_type_accepts_agent_resolve_when_verified` 通过
  2. `knowledge_summary_falls_back_to_final_answer_when_short` 通过
- 日志或截图：
  1. 运行态核查：`/api/v1/settings` 返回 `knowledge_base_path=D:\\newwork\\本地智能体\\data\\knowledge_base\\main.jsonl`、`knowledge_base_exists=True`
  2. 运行态核查：`/api/v1/settings` 返回 `knowledge_count=0`（说明基础路径可用，当前主要缺口在有效写入量）
  3. 代码证据：`crates/runtime-core/src/memory_router.rs` 已扩展 `agent_resolve -> workflow_pattern`
  4. 代码证据：`crates/runtime-core/src/memory_router.rs` 已新增短摘要回退逻辑（短 `summary` 回退 `final_answer`）

## Gate 映射

- 对应阶段 Gate：Gate-E（执行中）
- 当前覆盖情况：
  1. 本 change 仅覆盖知识库放量与质量拦截策略，不做 Gate-E 完成声明。

## 规划轮验证（2026-04-13）

- 本轮动作：
  1. 仅完成执行计划收口与最小 task 拆分（`T00-T32`），未进行新增代码实现。
  2. 已在 `proposal.md`、`design.md`、`tasks.md`、`status.md` 写入统一约束、门禁与回退口径。
- 运行态基线（用于后续对比）：
  1. `/api/v1/settings` 返回 `knowledge_count=0`。
  2. `/api/v1/settings` 返回 `long_term_memory_count=77`。
  3. `/api/v1/settings` 返回 `knowledge_base_path=D:\\newwork\\本地智能体\\data\\knowledge_base\\main.jsonl`。
- 结论：
  1. 规划信息已从对话迁移到 change 文档，后续执行以 `tasks.md` 为唯一任务列表。

## T01 验证记录（学习4级备考样例集）

- 执行动作：
  1. 从 `docs/07-test/evidence/20260410-cet4-user-simulation/*.run-finished.json` 抽取真实输入与运行结果。
  2. 形成人可读样例说明：`cet4-acceptance-sample-pack.md`。
  3. 形成机器可读样例集：`fixtures/cet4-acceptance-cases.jsonl`。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/cet4-acceptance-sample-pack.md`
  2. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/fixtures/cet4-acceptance-cases.jsonl`
- 验证结论：
  1. 已满足 `T01` 完成判据：形成可复用样例输入与预期输出清单。
  2. 样例覆盖规划、方法、分钟级计划、追踪模板、写读文件、次日调整、能力边界、启动语共 9 条。

## T02 验证记录（风险与回退表）

- 执行动作：
  1. 新增 `risk-rollback-register.md`，按 `G1-G5` 建立风险与回退矩阵。
  2. 每个子门禁至少配置 1 条可执行回退动作，并补触发条件、预警信号与回退后核验项。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/risk-rollback-register.md`
- 验证结论：
  1. 已满足 `T02` 完成判据：`G1-G5` 子门禁均有可执行回退动作。
  2. 回退动作遵循“先止损、再恢复主链路、最后补证据”的固定顺序，可直接执行。

## T03 验证记录（指标口径）

- 执行动作：
  1. 新增 `metrics-spec.md`，统一定义写入成功率、召回 Top-5 命中率、P95 延迟、污染率、回退可用率。
  2. 固化指标公式、统计窗口、数据源、阈值与异常处理规则。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/metrics-spec.md`
- 验证结论：
  1. 已满足 `T03` 完成判据：命中率、写入成功率、延迟、污染率口径已统一。
  2. 后续 `T09/T10/T25/T26/T28` 可直接复用该口径，避免跨任务口径漂移。

## T04 验证记录（cortex 版本锁定）

- 执行动作：
  1. 复核本地评估副本 `D:\\newwork\\third_party_eval\\cortex` 当前提交。
  2. 新增 `cortex-version-lock.md` 固化仓库地址、提交号、提交时间与使用约束。
- 校验命令：
  1. `git rev-parse HEAD`
  2. `git show -s --format=\"%ci %s\" HEAD`
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/cortex-version-lock.md`
- 验证结论：
  1. 已满足 `T04` 完成判据：`cortex` 版本已锁定为 `213916230d1ec4b4b5ddbba83a090400a795941f`。
  2. 后续 `T05-T11` 可基于固定版本实施，避免上游漂移。

## T05 验证记录（Windows compose 模板）

- 执行动作：
  1. 新增 Windows 本地化模板：`templates/cortex/docker-compose.windows.yml`。
  2. 去除原仓库中的作者私有挂载路径（`/opt/...`、`/home/...`），仅保留本地必需配置与命名卷。
- 校验命令：
  1. `docker compose -f docs/11-hermes-rebuild/changes/E-knowledge-base-activation/templates/cortex/docker-compose.windows.yml config`
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/templates/cortex/docker-compose.windows.yml`
- 验证结论：
  1. 已满足 `T05` 完成判据：模板可通过 `docker compose config`。
  2. 模板未包含作者私有路径挂载，适配本地 Windows 场景。

## T06 验证记录（`.env.example` 安全模板）

- 执行动作：
  1. 新增 `templates/cortex/.env.example`，补齐 `OPENAI_API_KEY`、`CORTEX_AUTH_TOKEN`、`NEO4J_PASSWORD`、端口与时区模板值。
  2. 在模板头部明确“未设置 `CORTEX_AUTH_TOKEN` 不得对外暴露端口”。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/templates/cortex/.env.example`
- 验证结论：
  1. 已满足 `T06` 完成判据：模板包含 token/provider/端口/路径说明。
  2. 安全默认口径已写入模板注释，可作为后续 `T07-T08` 的执行前置约束。

## T07 预检记录（阻塞）

- 预检动作：
  1. 使用本地模板执行 `docker compose up -d`，验证 `cortex + neo4j` 启动。
  2. 检查 Docker 服务状态并尝试拉起 `com.docker.service`。
- 预检命令：
  1. `docker compose -f docs/11-hermes-rebuild/changes/E-knowledge-base-activation/templates/cortex/docker-compose.windows.yml --env-file D:\\newwork\\tmp\\cortex-local.env up -d`
  2. `Get-Service *docker*`
  3. `Start-Service -Name com.docker.service`
- 结果：
  1. 阻塞：Docker 引擎不可用，报错 `open //./pipe/dockerDesktopLinuxEngine: The system cannot find the file specified`。
  2. 当前权限下无法直接启动 `com.docker.service`。
- 结论：
  1. `T07` 暂不打勾，等待 Docker 引擎可用后重试。

## T07 验收记录（已解阻）

- 解阻动作：
  1. 启动 Docker Desktop 客户端，待引擎恢复可用。
  2. 受镜像站 403 影响，临时使用 `CORTEX_IMAGE=ghcr.io/rikouu/cortex:latest` 做启动连通性验证。
  3. `neo4j:5-community` 单独拉取成功后，按模板重新执行 `docker compose up -d`。
- 验证命令：
  1. `docker compose ... up -d`
  2. `docker compose ... ps`
  3. `Invoke-WebRequest http://127.0.0.1:21100`
- 结果：
  1. `cortex-local` 与 `cortex-neo4j` 均为 `healthy`。
  2. `http://127.0.0.1:21100` 返回 `200`。
- 结论：
  1. 已满足 `T07` 完成判据：最小实例已启动并可访问本地管理入口。
  2. 当前镜像来源为 `ghcr` 临时兜底；后续如网络恢复，按 `T04` 锁定版本回归到固定提交构建镜像。

## T08 验证记录（API 鉴权）

- 执行动作：
  1. 对同一接口 `/api/v1/stats` 分别发送“无 token”和“有 token”请求。
  2. 比较状态码，验证鉴权链是否生效。
- 验证命令：
  1. `curl -s -o NUL -w "%{http_code}" http://127.0.0.1:21100/api/v1/stats`
  2. `curl -s -o NUL -w "%{http_code}" -H "Authorization: Bearer local-dev-token-20260413" http://127.0.0.1:21100/api/v1/stats`
- 结果：
  1. `without_token=401`
  2. `with_token=200`
- 结论：
  1. 已满足 `T08` 完成判据：无 token 被拒绝，有 token 可通过。

## T09 验证记录（ingest 冒烟）

- 执行动作：
  1. 使用带 token 的 `/api/v1/ingest` 连续写入 20 条样例（`agent_id=default`）。
  2. 统计每条请求状态码，计算写入成功率。
- 结果：
  1. `success=20/20`
  2. `success_rate=100%`
  3. 全部状态码为 `201`
- 结论：
  1. 已满足 `T09` 完成判据：20 条样例写入成功率 `>=95%`。

## T10 验证记录（recall 冒烟）

- 执行动作：
  1. 先通过 `/api/v1/memories` 显式写入 1 条可检索记忆（`category=fact`）。
  2. 对固定 query `listening chunks shadowing` 连续执行 3 轮 `/api/v1/recall`。
- 结果：
  1. `create_status=201`
  2. 三轮 recall 均返回 `count=1`
  3. 三轮首条内容一致：`CET4 recall probe: listening chunks improve score by daily 10-minute shadowing`
- 结论：
  1. 已满足 `T10` 完成判据：固定 query 可返回稳定结果。

## T11 验证记录（feature flag 回退开关）

- 执行动作：
  1. 新增开关脚本：`scripts/cortex/set-external-memory-flag.ps1`。
  2. 连续执行 `-Enabled $true` 与 `-Enabled $false`，确认开关可写入。
  3. 开关关闭后读取 `/api/v1/settings`，核查知识库路径是否仍为本地主路径。
- 验证结果：
  1. `data/settings/external-memory-cortex.json` 最终状态：`enabled=false`。
  2. `/api/v1/settings` 返回：
     - `knowledge_base_path=D:\\newwork\\本地智能体\\data\\knowledge_base\\main.jsonl`
     - `runtime_ok=True`
- 证据文件：
  1. `scripts/cortex/set-external-memory-flag.ps1`
  2. `data/settings/external-memory-cortex.json`
- 结论：
  1. 已满足 `T11` 完成判据：关闭开关后系统保持本地原始知识路径。

## T12 验证记录（字段映射）

- 执行动作：
  1. 新增字段映射文档 `mapping-main-to-cortex.md`。
  2. 覆盖 `knowledge_base`、`long_term_memory` 到 Cortex `/api/v1/ingest` 与 `/api/v1/memories` 的映射规则。
  3. 补齐源记录与目标请求样例，供 `T13/T14` 直接复用。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/mapping-main-to-cortex.md`
- 结论：
  1. 已满足 `T12` 完成判据：主库记录到外部服务字段映射已有文档与样例。

## T13 验证记录（ingest 适配器）

- 执行动作：
  1. 在 `knowledge_store.rs` 为知识写入链路新增 cortex ingest 旁路同步：
     - 本地写入成功后尝试外部同步
     - 受 `external-memory-cortex.json` 开关与 `CORTEX_AUTH_TOKEN` 控制
     - 外部失败不阻断本地主链路
  2. 新增 `should_sync_to_cortex` 规则，覆盖 `workflow_pattern`、`agent_resolve` 标签与 `result_mode` 标签来源。
  3. 新增 2 条单测验证同步触发条件。
- 代码证据：
  1. `crates/runtime-core/src/knowledge_store.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
- 验证结果：
  1. `sync_enabled_for_agent_resolve_workflow_pattern` 通过
  2. `sync_disabled_for_non_runtime_records` 通过
- 结论：
  1. 已满足 `T13` 完成判据：`agent_resolve/result_mode` 来源可进入外部 ingest 适配链路。

## T14 验证记录（recall 适配器）

- 执行动作：
  1. 在 `knowledge.rs` 将检索链路拆分为 `search_local_knowledge + search_external_knowledge`。
  2. 外部 recall 采用 feature flag + `CORTEX_AUTH_TOKEN` 守卫，仅在本地结果不足时补充召回。
  3. 外部调用失败时降级为本地结果，不阻断主流程。
  4. 新增单测 `maps_cortex_recall_payload_to_hits`，验证外部响应到 `KnowledgeHit` 的映射。
- 代码证据：
  1. `crates/runtime-core/src/knowledge.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
  2. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
  3. `curl.exe -s -X POST -H "Content-Type: application/json" -H "Authorization: Bearer <token>" --data-binary @<json> http://127.0.0.1:21100/api/v1/recall`
- 验证结果：
  1. `knowledge::tests::maps_cortex_recall_payload_to_hits` 通过。
  2. `knowledge_store::tests::*` 2 条均通过（回归未破坏 ingest 适配）。
  3. Cortex `/api/v1/recall` 返回 `memories[]` 结构，适配器按该结构完成映射。
- 结论：
  1. 已满足 `T14` 完成判据：任务查询链路支持“本地优先 + 外部补充召回”。

## T15 验证记录（去重策略）

- 执行动作：
  1. 在 `knowledge.rs` 新增 `dedupe_hits`，在本地与外部 recall 合并后执行去重。
  2. 去重锚点采用 `knowledge_type + 标准化文本片段`，本地结果在前时优先保留本地命中。
  3. 新增单测覆盖“重复命中保留本地”“非重复命中全部保留”。
- 代码证据：
  1. `crates/runtime-core/src/knowledge.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
  2. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
- 验证结果：
  1. `dedupe_prefers_local_hit_when_external_is_duplicate` 通过。
  2. `dedupe_keeps_distinct_hits` 通过。
  3. `knowledge_store::tests::*` 回归 2 条均通过。
- 结论：
  1. 已满足 `T15` 完成判据：主链路合并阶段可剔除同源重复条目，降低重复噪声。

## T16 验证记录（超时和重试策略）

- 执行动作：
  1. 在 `knowledge_store.rs` 的 ingest 外部调用中新增有界重试（3 次）与超时参数（connect=2s, max=6s）。
  2. 在 `knowledge.rs` 的 recall 外部调用中新增有界重试（3 次）与超时参数（connect=2s, max=6s）。
  3. 失败信息统一携带 `attempt=当前/总次数`，保证重试行为可追踪；主链路仍保持降级不阻断。
- 代码证据：
  1. `crates/runtime-core/src/knowledge_store.rs`
  2. `crates/runtime-core/src/knowledge.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
  2. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
- 验证结果：
  1. `knowledge::tests` 3 条均通过。
  2. `knowledge_store::tests` 2 条均通过。
  3. 外部调用路径已加入超时参数与重试耗尽错误信息（`retry exhausted` + `attempt`）。
- 结论：
  1. 已满足 `T16` 完成判据：外部超时不阻断主流程，重试行为可追踪。

## T17 验证记录（审计字段补齐）

- 执行动作：
  1. 新增外部记忆审计落盘路径 `data/logs/external-memory-cortex.jsonl`。
  2. 在 ingest 外部调用链路写入审计记录，覆盖 `trace_id`、`agent_id`、`source`、`duration_ms`。
  3. 在 recall 外部调用链路写入审计记录，覆盖同口径字段并补充 `attempts/status/error`。
  4. 新增 2 条单测，分别验证 recall/ingest 审计记录包含必需字段。
- 代码证据：
  1. `crates/runtime-core/src/paths.rs`
  2. `crates/runtime-core/src/knowledge.rs`
  3. `crates/runtime-core/src/knowledge_store.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
  2. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
- 验证结果：
  1. `knowledge::tests::recall_audit_contains_required_fields` 通过。
  2. `knowledge_store::tests::ingest_audit_contains_required_fields` 通过。
  3. 两条链路审计均可追溯 `trace_id/agent_id/source/duration_ms`。
- 结论：
  1. 已满足 `T17` 完成判据：审计关键字段已补齐并可验证。

## T18 验证记录（ingest 单测补齐）

- 执行动作：
  1. 提取 ingest 重试核心为可注入函数 `retry_cortex_ingest_operation`，支持测试覆盖成功/失败/超时分支。
  2. 新增 fallback 测试，验证外部同步失败时本地知识写入仍成功。
  3. 保留既有同步触发与审计字段测试，形成 ingest 侧最小闭环。
- 代码证据：
  1. `crates/runtime-core/src/knowledge_store.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
- 验证结果：
  1. `retry_ingest_operation_returns_success_attempt` 通过（成功路径）。
  2. `retry_ingest_operation_keeps_timeout_error_trace` 通过（失败/超时路径）。
  3. `append_knowledge_record_keeps_local_data_when_external_sync_fails` 通过（回退路径）。
  4. `sync_enabled_for_agent_resolve_workflow_pattern`、`sync_disabled_for_non_runtime_records`、`ingest_audit_contains_required_fields` 均通过（规则与审计回归）。
- 结论：
  1. 已满足 `T18` 完成判据：ingest 成功、失败、超时、回退路径均被覆盖。

## T19 验证记录（recall 单测补齐）

- 执行动作：
  1. 提取 recall 结果合并函数 `merge_knowledge_hits`，固定“本地优先、外部补充”顺序。
  2. 提取降级函数 `cortex_result_or_empty`，统一外部失败时回退空结果。
  3. 新增 recall 路径单测，覆盖本地优先、外部补充、外部失败降级三类场景。
- 代码证据：
  1. `crates/runtime-core/src/knowledge.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge::tests -- --nocapture`
- 验证结果：
  1. `recall_merge_prefers_local_when_limit_is_full` 通过（本地优先）。
  2. `recall_merge_adds_external_when_local_not_enough` 通过（外部补充）。
  3. `recall_degrades_to_empty_when_external_fails` 通过（降级路径）。
  4. recall 既有映射/去重/审计测试均通过（合计 7 条）。
- 结论：
  1. 已满足 `T19` 完成判据：本地优先、外部补充、降级路径均被覆盖。

## T20 验证记录（Markdown 导出规范）

- 执行动作：
  1. 新增导出规范文档 `markdown-export-spec.md`。
  2. 固化导出目录结构、文件命名、frontmatter 必填字段、tag 规则与 wikilink 规则。
  3. 补齐导出质量门禁与回退策略，作为 `T21-T24` 的唯一实现口径。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/markdown-export-spec.md`
- 验证结果：
  1. frontmatter 字段清单、正文模板、wikilink 规则已明确且可执行。
  2. 导出质量门禁与回退动作已固定，可直接用于后续验收。
- 结论：
  1. 已满足 `T20` 完成判据：frontmatter、tag、wikilink 规则固定。

## T21 验证记录（主库到 Markdown 导出器）

- 执行动作：
  1. 新增导出脚本 `scripts/export-knowledge-markdown.ps1`，实现按 `WorkspaceId + 时间窗` 批量导出。
  2. 数据读取优先 SQLite（`data/storage/main.db`），为空时回退 `data/knowledge_base/<workspace>.jsonl`。
  3. 导出结果按 `data/exports/knowledge-markdown/<batch>/` 组织，生成 `*.md` 与 `index.jsonl`。
  4. 脚本输出批次摘要 JSON（总记录数、导出数、目录路径）。
- 代码证据：
  1. `scripts/export-knowledge-markdown.ps1`
  2. `scripts/README.md`
- 验证命令：
  1. `powershell -NoProfile -File scripts/export-knowledge-markdown.ps1 -WorkspaceId main`
- 验证结果：
  1. 生成批次目录：`data/exports/knowledge-markdown/20260413-185636/`。
  2. 批次内导出 `6` 条 Markdown，`index.jsonl` 记录数与导出文件数一致。
  3. 抽样文件 frontmatter、tags、wikilink 与 `markdown-export-spec.md` 口径一致。
- 结论：
  1. 已满足 `T21` 完成判据：可按时间窗批量导出知识条目。

## T22 验证记录（关系链接规则）

- 执行动作：
  1. 在导出器脚本中扩展 `Build-Links`，新增 `task` 与 `conclusion` 关系节点。
  2. 新增 `Task-LinkSlug` 与 `Conclusion-LinkSlug` 自动抽取规则，并补齐短锚点兜底策略。
  3. 关系链接最终固定为 `topic/task/conclusion/workflow/source` 五类。
- 代码证据：
  1. `scripts/export-knowledge-markdown.ps1`
- 验证命令：
  1. `powershell -NoProfile -File scripts/export-knowledge-markdown.ps1 -WorkspaceId main`
- 验证结果：
  1. 生成批次目录：`data/exports/knowledge-markdown/20260413-185903/`。
  2. 抽样导出文件 `## Links` 区块包含 `[[topic/*]]`、`[[task/*]]`、`[[conclusion/*]]`、`[[workflow/*]]`、`[[source/*]]`。
- 结论：
  1. 已满足 `T22` 完成判据：主题、任务、结论关系链接可自动生成。

## T23 验证记录（graphify 图谱构建脚本）

- 执行动作：
  1. 新增脚本 `scripts/build-graphify-input.ps1`，从 Markdown 批次抽取 wikilink 并构建节点/边。
  2. 输出 `graphify-input.json`、`graphify-nodes.jsonl`、`graphify-edges.jsonl` 三类文件。
  3. 将脚本登记到 `scripts/README.md`，明确入口参数与默认行为。
- 代码证据：
  1. `scripts/build-graphify-input.ps1`
  2. `scripts/README.md`
- 验证命令：
  1. `powershell -NoProfile -File scripts/build-graphify-input.ps1 -BatchDir data/exports/knowledge-markdown/20260413-185903`
- 验证结果：
  1. 产出目录：`data/exports/knowledge-markdown/20260413-185903/graphify/`。
  2. 输出文件完整：`graphify-input.json`、`graphify-nodes.jsonl`、`graphify-edges.jsonl`。
  3. 批次统计：`markdown_files=6`、`nodes=22`、`edges=30`。
- 结论：
  1. 已满足 `T23` 完成判据：导出内容可一键生成关系图输入。

## T24 验证记录（图谱可视化验收）

- 执行动作：
  1. 基于 `fixtures/cet4-acceptance-cases.jsonl` 生成 CET4 业务样例导出批次（9 条 Markdown）。
  2. 使用 `build-graphify-input.ps1` 对业务样例批次构建图谱输入。
  3. 输出验收快照 `tmp/stage-e-knowledge-graph/latest.json`，检查“多跳关系 + 可解释节点”。
- 证据文件：
  1. `data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/index.jsonl`
  2. `data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/graphify/graphify-input.json`
  3. `data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/graphify/graphify-nodes.jsonl`
  4. `data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/graphify/graphify-edges.jsonl`
  5. `tmp/stage-e-knowledge-graph/latest.json`
- 验证命令：
  1. `powershell -NoProfile -File scripts/build-graphify-input.ps1 -BatchDir data/exports/knowledge-markdown/20260413-190819-cet4-acceptance`
  2. `Get-Content tmp/stage-e-knowledge-graph/latest.json`
- 验证结果：
  1. 业务样例批次统计：`markdown_files=9`、`nodes=30`、`edges=45`。
  2. 多跳关系样例：`doc/knowledge_recall__cet4-q1-plan -> topic/cet4 -> doc/knowledge_recall__cet4-q7-adjust-next -> conclusion/cet4-q7-adjust-next`（3 跳）。
  3. 可解释节点校验通过：`pass_explainable_nodes=true`，示例节点 `topic/cet4` 关联 9 条业务文档，`task/planning` 与 `conclusion/cet4-q7-adjust-next` 均可回溯到对应文档。
- 结论：
  1. 已满足 `T24` 完成判据：学习 4 级备考业务样例可看到多跳关系和可解释节点。

## T25 验证记录（固定召回评测集批跑）

- 执行动作：
  1. 新增批跑脚本 `scripts/run-stage-e-knowledge-recall-eval.ps1`，固定读取 CET4 样例集并输出评测报告。
  2. 评测前使用隔离 `agent_id=eval-cet4-t25` 清理旧数据并重灌样例，避免历史记忆污染本轮结果。
  3. 逐条执行 recall，统计 Top-5 命中率（M02）与 P95 延迟（M03），并输出失败原因分布。
- 代码证据：
  1. `scripts/run-stage-e-knowledge-recall-eval.ps1`
  2. `scripts/README.md`
- 证据文件：
  1. `tmp/stage-e-knowledge-recall-eval/latest.json`
  2. `tmp/stage-e-knowledge-recall-eval/t25-20260413-191946.json`
- 验证命令：
  1. `powershell -NoProfile -File scripts/run-stage-e-knowledge-recall-eval.ps1`
  2. `Get-Content tmp/stage-e-knowledge-recall-eval/latest.json`
- 验证结果：
  1. `seeded_cases=9`，`total_cases=9`，`hit_cases=9`，`top5_hit_rate=100%`（`>=70%`，M02 通过）。
  2. `recall_p95_latency_ms=46`（`<=1500ms`，M03 通过）。
  3. `primary_cases=5`，`fallback_cases=4`（已在报告保留 `query_mode` 字段，便于后续跟踪中文 query 直召回稳定性）。
  4. 报告包含逐样例 `failure_reason` 字段；本批次 `failed_cases=0`，`failure_reason_top3=[]`。
- 结论：
  1. 已满足 `T25` 完成判据：固定评测集 Top-5 命中率达标，且失败原因标注机制已落地可复用。

## T26 验证记录（低质量条目拦截/清洗规则）

- 执行动作：
  1. 新增清洗脚本 `scripts/cortex/cleanup-low-quality-memories.ps1`，统一识别三类低质量条目：
     - `failed_result`（失败结果）
     - `short_noise`（短噪声）
     - `no_value`（无价值记录）
  2. 在隔离评测 `agent_id=eval-cet4-t26` 注入 5 条混合样例（2 条有效 + 3 条低质量）进行真实清洗验证。
  3. 执行清洗后再次回读该 agent 记忆列表，确认仅低质量条目被删除。
- 代码证据：
  1. `scripts/cortex/cleanup-low-quality-memories.ps1`
  2. `scripts/README.md`
- 证据文件：
  1. `tmp/stage-e-knowledge-cleanup/latest.json`
- 验证命令：
  1. `powershell -NoProfile -File scripts/cortex/cleanup-low-quality-memories.ps1 -AgentId eval-cet4-t26 -OutputPath tmp/stage-e-knowledge-cleanup/latest.json`
  2. `Get-Content tmp/stage-e-knowledge-cleanup/latest.json`
  3. `Invoke-RestMethod GET /api/v1/memories?agent_id=eval-cet4-t26&limit=500`（核对剩余条目）
- 验证结果：
  1. 清洗前 `total_memories=5`，候选 `candidate_count=3`，删除 `deleted_count=3`。
  2. 分类命中：`failed_result=1`、`short_noise=1`、`no_value=1`。
  3. 清洗后该 agent 剩余 `2` 条有效记忆，未被误删。
- 结论：
  1. 已满足 `T26` 完成判据：失败结果、短噪声、无价值记录已可识别并完成清理。

## T27 验证记录（敏感信息清洗规则）

- 执行动作：
  1. 在 `knowledge_store.rs` 增加敏感信息拦截：
     - 新增 `contains_sensitive_marker` 与 `record_contains_sensitive_data`
     - 命中 `authorization: bearer`、`api_key`、`token=`、`password=`、`secret=`、`sk-`、`ak-` 等标记时拒绝入库
  2. 在 `knowledge.rs` 的 recall 审计来源生成逻辑增加敏感查询脱敏：
     - 新增 `sensitive_query_marker`
     - 命中敏感标记时 `source` 固定写为 `knowledge_search:[REDACTED]`
  3. 新增单测覆盖“敏感记录拦截”和“审计脱敏”路径。
- 代码证据：
  1. `crates/runtime-core/src/knowledge_store.rs`
  2. `crates/runtime-core/src/knowledge.rs`
- 验证命令：
  1. `cargo test -p runtime-core knowledge_store::tests -- --nocapture`
  2. `cargo test -p runtime-core knowledge::tests -- --nocapture`
- 验证结果：
  1. `knowledge_store::tests::sensitive_record_is_skipped` 通过（敏感内容不入库）。
  2. `knowledge_store::tests::normal_record_not_skipped` 通过（正常内容不误杀）。
  3. `knowledge::tests::recall_source_redacts_sensitive_query` 通过（审计源字段脱敏）。
  4. 两组回归测试均通过（各 8 条）：未破坏既有 ingest/recall 路径。
- 结论：
  1. 已满足 `T27` 完成判据：密钥/令牌类敏感信息已被入库拦截，且敏感查询不以明文写入审计日志。

## T28 验证记录（回退演练）

- 执行动作：
  1. 新增回退演练脚本 `scripts/cortex/run-external-memory-rollback-drill.ps1`，执行“先开启再关闭 feature flag”的完整回退过程。
  2. 回退完成后自动执行两条关键回归测试：
     - 本地写入在外部不可用时保持可用
     - 外部 recall 失败时本地链路降级为空结果而不崩溃
  3. 输出演练报告到 `tmp/stage-e-rollback-drill/latest.json`。
- 代码证据：
  1. `scripts/cortex/run-external-memory-rollback-drill.ps1`
  2. `scripts/cortex/set-external-memory-flag.ps1`
  3. `scripts/README.md`
- 证据文件：
  1. `tmp/stage-e-rollback-drill/latest.json`
  2. `data/settings/external-memory-cortex.json`
- 验证命令：
  1. `powershell -NoProfile -File scripts/cortex/run-external-memory-rollback-drill.ps1 -OutputPath tmp/stage-e-rollback-drill/latest.json`
  2. `Get-Content tmp/stage-e-rollback-drill/latest.json`
- 验证结果：
  1. 回退状态：`rollback_enabled_false=true`（最终 `enabled=false`）。
  2. 测试 1 通过：`knowledge_store::tests::append_knowledge_record_keeps_local_data_when_external_sync_fails`。
  3. 测试 2 通过：`knowledge::tests::recall_degrades_to_empty_when_external_fails`。
  4. 演练总判定：`passed=true`。
- 结论：
  1. 已满足 `T28` 完成判据：关闭 feature flag 后主业务链路保持连续可用。

## T29 验证记录（状态与证据收口）

- 执行动作：
  1. 同步更新 `tasks.md`、`status.md`，将 `T24-T29` 的完成状态与下一步推进项统一到当前口径。
  2. 在 `verify.md` 补齐 `T24-T28` 的命令、结果、证据路径与结论，形成可追溯链路。
  3. 对照 `G1-G5` 子门禁检查证据覆盖完整性，确认每个已完成 task 都有对应验证段落。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/tasks.md`
  2. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/status.md`
  3. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/verify.md`
- 验证结果：
  1. `G1` 覆盖 `T00-T11`，`G2` 覆盖 `T12-T19`，`G3` 覆盖 `T20-T24`，`G4` 覆盖 `T25-T28`，均可定位到对应证据段。
  2. 新增脚本证据已登记到 `scripts/README.md`，并在 `verify.md` 关联到具体输出路径。
  3. 当前推进状态已收口到 `T30`，不存在“已完成但无证据”的任务项。
- 结论：
  1. 已满足 `T29` 完成判据：每个已完成子门禁任务均具备可追溯证据链接。

## T30 验证记录（端到端业务演示记录）

- 执行动作：
  1. 新增业务演示记录文档 `cet4-e2e-demo-record.md`，固化 CET4 场景完整复现路径。
  2. 汇总召回评测、Markdown 导出、图谱构建与多跳验证证据，形成统一演示入口。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/cet4-e2e-demo-record.md`
  2. `tmp/stage-e-knowledge-recall-eval/latest.json`
  3. `data/exports/knowledge-markdown/20260413-190819-cet4-acceptance/graphify/graphify-input.json`
  4. `tmp/stage-e-knowledge-graph/latest.json`
- 验证结果：
  1. CET4 固定样例 `9` 条已完成“召回评测 + 图谱可视化”双链路复现。
  2. 关键指标：`top5_hit_rate=100%`、`recall_p95_latency_ms=46`、`nodes=30`、`edges=45`。
  3. 多跳关系与可解释节点证据可直接回溯到对应导出批次与报告文件。
- 结论：
  1. 已满足 `T30` 完成判据：学习4级备考场景端到端链路复现成功。

## T31 验证记录（阶段评审材料）

- 执行动作：
  1. 新增评审材料 `stage-review-pack.md`。
  2. 汇总达标项、未达标项、风险项、回退项，并给出评审建议。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/stage-review-pack.md`
- 验证结果：
  1. 评审材料已包含 `达标项/未达标项/风险项/回退项` 四类必需信息。
  2. 内容与 `tasks.md` 当前完成状态一致（`T32` 仍未完成）。
- 结论：
  1. 已满足 `T31` 完成判据：阶段评审材料齐全可用于提审。

## T32 验证记录（冻结下一阶段 backlog）

- 执行动作：
  1. 新增 `next-stage-backlog.md`，将本轮范围外事项统一冻结到下一阶段待办池。
  2. 明确“不纳入本轮交付”的约束，避免后续执行跨范围混入。
- 证据文件：
  1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/next-stage-backlog.md`
- 验证结果：
  1. 已登记至少 5 项范围外事项，并标注进入条件。
  2. 当前 `tasks.md` 全量任务 `T00-T32` 已全部收口，不再混入新增范围外开发项。
- 结论：
  1. 已满足 `T32` 完成判据：下一阶段 backlog 已冻结并与本轮交付边界隔离。
