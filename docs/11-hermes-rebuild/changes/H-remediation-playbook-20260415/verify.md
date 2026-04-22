# H-remediation-playbook-20260415（verify）

更新时间：2026-04-21（已补 baijiacms 多层接管样本总结）
状态：部分已验证（边界冻结 + 三类真实修复样本 + 一类配置自动写入最小闭环样本已落证据，并新增 baijiacms 的多层接管样本总结；当前无新的合格受限样本）

## 验证方式

1. 文档验证：
   - H-02 的目标、边界、样本矩阵、回退和证据口径是否齐备。
2. 一致性验证：
   - `current-state.md` 与 `changes/INDEX.md` 是否同步切换到 H-02。
3. 后续实现预留验证：
   - 低风险自动修复样本回放。
   - 手动接管指引逐步复现。
4. 基线复用验证：
   - `doctor` 与 diagnostics 是否能覆盖 H-02 样本识别所需字段。

## 证据位置

1. 当前文档证据：
   - `docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/`
2. 当前运行证据：
   - `tmp/stage-h-remediation/latest.json`（含 `config_missing_required_fields` 自动写入修复前后差异）
   - `tmp/stage-h-remediation/manual-guides/`
   - `tmp/stage-h-remediation/replay-results.json`（`config_missing_required_fields` 已更新为 `auto_fix`）
   - `tmp/stage-h-remediation/manual-guide-eval.json`（当前 8 条扩样，`guide_score_avg=4.75`）
3. 新增样本证据：
   - `tmp/stage-h-remediation/h02-baijiacms-usable-sample-20260420.json`
   - `tmp/stage-h-remediation/h02-baijiacms-iis-restored-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-db-prereq-takeover-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-db-prereq-guide-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-db-prereq-runtime-check-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-siteid-host-check-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-homepage-check-20260421.json`
   - `tmp/stage-h-remediation/h02-baijiacms-sample-pass-summary-20260421.json`
   - `tmp/stage-h-remediation/manual-guides/baijiacms-db-prereq-missing.md`
   - `tmp/stage-h-remediation/baijiacms-phpstudy-8088-final-check.json`
   - `tmp/stage-h-remediation/h02-baijiacms-iis-php-blockers-20260420.json`
4. 聚合回填证据：
   - `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/status.md`
   - `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/verify.md`
5. 复用基线证据：
   - `tmp/stage-f-doctor/latest.json`
   - `scripts/doctor.ps1`
   - `scripts/run-stage-f-doctor-acceptance.ps1`
6. 本轮测试记录：
   - `go test ./internal/api -run 'TestRemediate|TestGenerateH02RemediationEvidence'`（工作目录：`D:/newwork/本地智能体/gateway`）
   - 人工接管评分证据已随 `TestGenerateH02RemediationEvidence` 刷新

## 当前证据支持范围

1. 已支持证明的内容：
   - H-02 已具备最小闭环：诊断、分类、自动修复/人工接管分流、结果校验、回放。
   - 低风险自动修复已覆盖日志目录、前端构建、gateway 启动恢复与一个配置缺失字段补默认值子样本。
   - 人工接管模板可执行性已有 8 条样本评分证据，说明当前手动指引不是空文档。
   - 当前已有足够文档与证据，把剩余 warning 从“缺口已定义”推进到“后续路径已裁决”。
   - `baijiacms` 已额外证明一类本地 PHP 靶场环境问题可从“IIS 500 阻塞”收敛到“IIS 主链路恢复、剩余问题降到应用层数据库前置条件”。
   - `baijiacms` 现已可进一步作为“环境已恢复、剩余数据库前置条件缺失”的人工接管样本保留，后续接手无需再回到环境链路排查。
   - `baijiacms` 的数据库前置条件缺失样本已补齐专用人工接管指引，当前接手动作已能直接落到数据库配置、MySQL 服务与安装状态核对。
   - `baijiacms` 已完成最小运行核查：当前直接阻塞可进一步收紧为“MySQL 未启动”；数据库启动后页面进入“未找到站点ID”，说明接管入口已继续向业务层前移。
   - `baijiacms` 已进一步确认站点ID问题源于 Host 不匹配；切换到 `localhost` 后页面继续推进到“请先在后台进行店铺装修，新建一个店铺首页”，说明接管入口已继续前移到业务初始化。
   - `baijiacms` 已进一步确认店铺首页提示的直接原因是 `baijiacms_eshop_designer` 缺少商城首页装修记录，当前接管入口已具体落到首页装修数据初始化。
   - `baijiacms` 现已可正式收口为“环境恢复 -> MySQL 启动 -> Host 匹配 -> 首页装修初始化”的高质量多层人工接管样本，并可在 H-02 冻结观察期内稳定归档引用。
2. 已支持的结论边界：
   - 可以证明 H-02 warning 已明显收缩。
   - 可以证明 H-02 已进入“签收级缺口收口阶段”，且当前后续路径已裁决。
   - 不能据此证明 H-02 已 ready，更不能据此视为 Gate-H 可签收输入。
   - `baijiacms` 的 IIS 主链路恢复只能增强 H-02 的环境/接管样本强度，不等于阶段签收。
   - `baijiacms` 的数据库前置条件缺失样本只证明人工接管边界已更清楚，不等于获得应用层自动修复结论。
   - `baijiacms` 的最小运行核查只证明接管链路已进一步收紧，不等于业务层初始化已完成。
   - `baijiacms` 的 Host/站点ID 核查只证明当前访问入口应改用 `localhost` 或等价 Host 映射，不等于店铺首页初始化已完成。
   - `baijiacms` 的首页装修缺失核查只证明业务层缺少默认首页数据，不等于后台装修流程已完成。
   - `baijiacms` 的样本总结只证明该样本已可作为高质量多层人工接管样本归档，不等于 H-02 ready 或 Gate-H 可签收。

## 当前仍不足以签收的原因

1. 高风险配置写入与权限类最小必测集虽然已冻结成签收级验收矩阵，但当前冻结的是边界与门槛，不是新增签收结论。
2. 当前仅覆盖“缺失字段补安全默认值”这一低风险基线，不能外推到覆盖已有值、系统级持久配置或跨文件联动写入。
3. 权限类虽已按层级明确哪些必须人工、哪些未来才可考虑自动化，但尚未形成新的签收级验证结论。
4. 现有证据仍偏向“已能接住问题”，不足以回答“再往前一步是否会触发高风险自动写入或越权执行”。
5. 因此，继续补更多低风险样本不能替代签收级缺口闭合。

## 受限验证窗口验证口径

本轮唯一允许进入并已执行的受限验证场景为：

1. `logs_not_writable_safe_dir_switch`
2. 场景归属：`H02-S04 / logs_not_writable`，且限定在 `P-A / 当前用户目录可切换`。

本轮实际执行情况：

1. 已核查前置条件：
   - 原日志路径为不可写目标（以文件占位方式模拟不可写日志目录）；
   - 替代日志目录位于当前用户工作区内；
   - 全程无需管理员权限；
   - 未触发系统级持久配置写入。
2. 已执行动作：
   - 选择当前用户工作区内的安全替代日志目录；
   - 对替代目录执行写入探针校验。
3. 本轮结果：
   - `latest.json` 已记录 `limited_validation_window_result=success`；
   - `replay-results.json` 已新增 `logs_not_writable_safe_dir_switch / limited_validation / passed`。

成功意味着：

1. H-02 已不再停留于纯文档裁决态，而是完成了一次唯一受限验证窗口的真实执行。
2. `logs_not_writable` 可在非管理员、可回退、可降级条件下完成最小安全目录切换验证。
3. 但这仍只证明单一最小场景成立，不等于 H-02 ready，更不等于 Gate-H 可签收。

失败意味着：

1. 该场景在当前窗口下仍只能回到人工接管；
2. 失败不应触发权限提升、系统级路径修改或配置扩项；
3. 失败后只允许保留人工接管与回退记录，不扩大到其他场景。

## 第二受限验证窗口结果与冻结口径

当前被保留为第二受限验证窗口观察对象的场景为：

1. `frontend_dist_missing_build_ready_rebuild`
2. 场景归属：`H02-S03 / frontend_dist_missing`
3. 执行边界仍保持不变：
   - 只允许在 `package.json + node_modules` 已存在时进入；
   - 只允许执行工作区内的前端构建补齐；
   - 不允许扩展到依赖安装、系统级 Node/npm 修复或管理员权限。

当前证据核查情况：

1. 已核查 `node_modules` 目录存在，且当前环境 `node` / `npm` 可用。
2. 已核查现有 success 样本目录中 `dist/index.html` 已存在，因此不再满足“dist 缺失后重建”的正式执行前置条件。
3. 已核查 `package.json` 当前内容仅为 `{}`，未提供可直接核验的构建脚本定义；在不扩展到依赖安装、脚本补写或环境修复的前提下，不能把这次窗口继续外推为成功重建执行。
4. 因此前置条件不足，本次窗口按受限边界记为 `aborted_manual_takeover`：当前只形成一次前置条件核查记录，而未形成新的合格受限样本或成功闭环。

成功意味着：

1. 仅当存在新的 build-ready 且 `dist/index.html` 确实缺失的样本，并能在当前用户工作区内完成构建补齐时，才可记为该窗口成功。
2. 即使未来成功，也仍只代表第二个受限窗口成立，不等于 H-02 ready，更不等于 Gate-H 可签收。

失败或中止意味着：

1. 当前工作区下尚无可直接支撑该窗口成功闭环的合格样本。
2. 构建补齐路径在本次窗口下只能退回人工接管；不得触发依赖安装、系统级环境修复或更高风险配置联动。
3. 本次中止并不推翻既有 `frontend_dist_missing_build_ready` replay 成功证据，但旧 replay 不能替代这次正式窗口执行结果。
4. 失败或中止后仅保留人工接管与回退记录，不外推到其他场景。
5. 在出现新的合格 build-ready 缺失样本前，该窗口只保留候选定义、前置条件与冻结观察口径，不再继续进入新的正式窗口。

## 再提审前最低闭口条件

H-02 只有在同时满足以下条件时，才值得进入下一轮实现或验证：

1. 下一轮目标仅落在已裁决为“后续可验证”的受限范围内。
   - 配置类仅限 `C-B` 或 `C-E` 的文档化高门槛候选，且未进入真实实现。
   - 权限类仅限 `P-A` 或 `P-B` 的非管理员、可降级样本。
2. 已明确该轮验证不会触及系统级持久配置、管理员权限、敏感凭据写入。
3. 已有写前差异、影响面、失败分流、回退提示四项最小口径。
4. 项目推进官已重新分配窗口，允许 H-02 从文档收口进入受限验证。

H-02 在以下情况下仍只适合停留在 warning：

1. 拟推进场景仍属于 `C-C`、`C-D`、`C-F`，或 `P-C`、`P-D`。
2. 只能证明“风险高”，但还不能给出受限验证边界与回退链。
3. 计划以继续补低风险样本替代签收级缺口闭口。
4. 主线尚未重新分配 H-02 的执行窗口。

再提审前最低需要补齐的条件：

1. 已冻结高风险配置写入三类路径：继续禁止 / 后续可验证 / 保持人工接管。
2. 已冻结权限类三类路径：继续观察 / 可后续验证 / 保持人工接管。
3. 已明确哪些路径在阶段 H 内不进入实现验证。
4. 已在 design/tasks/status/verify/review 中一致表达：当前仍是 warning，且当前不建议直接进入高风险实现。
5. 已明确哪些条件不满足时，禁止回刷 Gate-H。

禁止回刷 Gate-H 的条件：

1. 未获得新的签收级验证结论，仅完成边界冻结或路径裁决。
2. 拟补证场景触及系统级持久配置、管理员权限或敏感凭据写入。
3. 权限类或配置类样本无法稳定降级到人工接管。
4. H-02 仍只能保守表达为“后续路径已裁决，但缺口未闭合”。

## 已冻结的签收级验收矩阵

1. 高风险配置写入：
   - 已按 `C-A` ~ `C-F` 冻结。
   - 其中 `C-A` 仅表示低风险已落基线；`C-B` ~ `C-E` 为当前必须人工接管的签收级缺口；`C-F` 为永久排除项。
2. 权限类最小必测集：
   - 已按 `P-A` ~ `P-D` 冻结。
   - 其中 `P-C`、`P-D` 当前均视为必须人工接管且触发自动修复停止扩展的边界。
3. 当前验证目标：
   - 不是证明高风险场景可自动修复。
   - 而是证明这些场景已被明确归类、不会被误纳入自动执行，并且再提审门槛已冻结。

## Gate 映射

1. 对应阶段 Gate：
   - `Gate-H`（子项 H-02）
2. 当前覆盖情况：
   - 已完成 H-02 工作区与主推进切换。
   - 已完成 H02-01/H02-02 设计冻结。
   - 已完成最小样本、接管指引模板与证据结构冻结。
   - 已完成 `logs_not_writable`、`frontend_dist_missing`、`gateway_unreachable` 三类真实修复样本的成功/降级证据。
   - 已完成 `config_missing_or_invalid` 的最小自动写入修复子样本证据（仅补齐 `runtime_port` 与 `default_workspace` 安全默认值）。
   - 已完成 8 条人工接管样本的实测评分证据，`guide_score_avg=4.75/5`。
   - 当前已把剩余 warning 收口为“高风险配置写入边界 + 权限类最小必测验收集”两类签收级缺口。
3. 当前未覆盖且仍阻塞签收的内容：
   - 更高风险配置写入的签收级验证。
   - 更广权限类真实验收的签收级分层验证。

