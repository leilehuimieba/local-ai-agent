# H-remediation-playbook-20260415（status）

最近更新时间：2026-04-23（已执行第二受限验证窗口 `frontend_dist_missing_build_ready_rebuild`）
状态：进行中（并行观察，冻结观察；第二窗口已成功执行，但仍为 warning）
状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`

## 当前状态

1. 已完成：
   - H-02 change 工作区已建立，五件套已补齐。
   - H02-01 已冻结：五类问题范围、自动修复白名单、人工接管模板、证据目录口径已固定。
   - 已明确当前定位：先做“修复与接管”边界冻结，再进入最小样本与实现闭环。
   - 已确认 Gate-H 当前缺 H-02 / H-03 输入，H-02 需要先补样本与实证口径。
   - H02-02 已完成：五类最小样本与 `doctor`/diagnostics 基线映射已落入 `tmp/stage-h-remediation/latest.json`。
   - H02-04 已完成：手动接管指引模板已落盘到 `tmp/stage-h-remediation/manual-guides/`。
   - H02-05 已完成：`tmp/stage-h-remediation/replay-results.json` 已建立回放口径。
   - H02-03 已完成首个真实修复样本：`logs_not_writable` 支持最小自动修复链路（创建 `logs/` 目录并写入探针校验）。
   - 已补第一类真实 replay：
     - `logs_missing_dir` -> 自动修复成功
     - `logs_path_is_file` -> 自动降级人工接管
   - H02-06 已完成第二个真实修复入口：`frontend_dist_missing`。
   - H02-03 已补第二个真实修复样本：`frontend_dist_missing` 在 `package.json + node_modules` 就绪时支持最小自动构建修复。
   - 已补第二类真实 replay：
     - `frontend_dist_missing_build_ready` -> 自动修复成功
     - `frontend_dist_missing_prereq_missing` -> 自动降级人工接管
   - H02-06 已完成第三个真实修复入口：`gateway_unreachable`。
   - H02-03 已补第三个真实修复样本：`gateway_unreachable` 在源码入口与 Go 环境就绪时支持最小启动恢复。
   - 已补第三类真实 replay：
     - `gateway_unreachable_source_ready` -> 自动修复成功
     - `gateway_unreachable_source_missing` -> 自动降级人工接管
   - H02-03 已补配置类只读校验子样本：`config_missing_or_invalid` 仅做只读校验与建议修复，不做自动写入。
   - 已补第四类 replay：
     - `config_valid_required_fields_present` -> 只读校验通过
     - `config_missing_required_fields` -> 自动降级人工接管
   - 已补 H02-09 配置类真实自动写入最小闭环：
     - `config_missing_required_fields` -> 自动补齐 `runtime_port=8898` 与 `default_workspace.workspace_id=main` -> 校验通过
     - 同步保留 `config_missing` / `config_invalid_json` 的人工接管降级路径，不扩大自动写入边界
   - 已补人工接管模板实测评分：4 条人工接管样本 `guide_score_avg=4.8/5`，证据已落 `tmp/stage-h-remediation/manual-guide-eval.json`
   - 已补 H02-08 扩样：人工接管样本扩至 8 条，`guide_score_avg=4.75/5`、`manual_takeover_success_rate=1.0`。
2. 进行中：
   - 第二受限验证窗口 `frontend_dist_missing_build_ready_rebuild` 已于 2026-04-23 在主控授权下在真实项目目录上成功执行。
   - 当前仍保留冻结观察口径，第二窗口成功后不再自动进入第三个窗口。
3. 阻塞点：
   - 配置类已补 1 个低风险自动写入闭环样本，但自动写入仍仅限“缺失字段补安全默认值”这一最小边界。
   - 更高风险配置写入与更广权限类场景虽已完成签收级矩阵冻结与后续路径裁决，但尚未形成可支持签收的新增验证结论。
   - 2026-04-23 前，第二窗口只形成一条 `aborted_manual_takeover` 记录：原有样本目录中 `dist/index.html` 已存在，且部分 `package.json` 仅为 `{}`，不足以构成新的合格受限样本。
   - 2026-04-23 在主控授权下，第二窗口已在真实项目目录上完成正式执行：`frontend/dist/index.html` 被删除后通过 `npm run build` 成功重建，构成新的合格受限样本。
   - 已补 `baijiacms-master` 的主链路恢复证据：当前 80 端口下 `__codex_probe.php` 与 `index.php` 均已返回 200，说明 IIS/PHP/FastCGI 主链路已恢复；`index.php` 当前进入数据库配置报错，剩余问题已从环境承载链路收敛到应用层前置条件。该结果增强了 H-02 的本地环境/接管样本强度，但仍不足以把 H-02 写成 ready。
   - 已进一步把 `baijiacms-master` 收口为“环境已恢复、剩余数据库前置条件缺失”的人工接管样本：后续接手者无需再排查 IIS/FastCGI，只需转向数据库配置或初始化。
   - 已为 `baijiacms-master` 补齐对应人工接管指引：当前可直接引导接手者核对 `config/config.php`、MySQL 服务与 `install.php` 安装状态，而不再回到环境链路排查。
   - 已完成数据库前置条件的最小运行核查：当前直接原因可进一步收紧为“本地 MySQL 未启动”；临时启动 MySQL 后，`baijiacms` 库存在、连接链路可用，页面已从“数据库连接失败”推进到业务层“未找到站点ID”。
   - 已进一步确认“未找到站点ID”的直接原因是 Host 不匹配：库中店铺域名为 `localhost`，因此使用 `127.0.0.1` 访问会丢失站点ID；改用 `http://localhost/baijiacms-master/index.php` 后，页面继续推进到业务层提示“请先在后台进行店铺装修，新建一个店铺首页”。
   - 已进一步确认“请先在后台进行店铺装修，新建一个店铺首页”的直接原因是 `baijiacms_eshop_designer` 中缺少 `uniacid=1` 且 `pagetype=1` 的首页装修记录；当前接管入口已明确推进到业务层首页初始化。
   - 已把 `baijiacms` 正式收口为 H-02 的高质量多层人工接管样本：当前链路已稳定落为“环境恢复 -> MySQL 启动 -> Host 匹配 -> 首页装修初始化”，后续默认只作为 warning 级归档样本保留，不再继续扩成业务初始化执行。
4. 下一步：
   - H-02 当前仍为 `warning`，且当前最保守、与证据一致的口径是：保留已落最小闭环证据，第二窗口成功后不再自动进入第三个窗口。
   - 第二窗口已于 2026-04-23 在主控授权下在真实项目目录上成功执行：`frontend/dist/index.html` 被删除后通过 `npm run build` 成功重建。
   - 当前主推进以 `docs/11-hermes-rebuild/current-state.md` 为准；H-02 本 change 当前只保留并行观察、文档一致性维护与候选样本归档，不再自行回抬主推进。
   - 本轮新增的 `baijiacms` 结论已说明：H-02 可以稳定接住一类本地 PHP 靶场环境阻塞并恢复 IIS 主链路，并把剩余问题继续收口为“先启动 MySQL、再确保 Host 与站点映射匹配，最后处理店铺首页装修初始化”的高质量多层人工接管样本；该样本当前已足够作为 H-02 warning 输入长期保留，但仍不能写成 ready 或 Gate-H 可签收输入。
