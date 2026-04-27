# H-remediation-playbook-20260415（tasks）

更新时间：2026-04-21
状态：进行中

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| H02-00 | 建立 H-02 change 五件套初稿 | 文档 | done | proposal/design/tasks/status/verify 已齐备，并加入 change 索引 | `proposal.md` |
| H02-01 | 冻结 H-02 修复与接管边界 | 设计 | done | 五类问题、自动修复白名单、人工接管模板、证据目录已固定 | `design.md` |
| H02-02 | 定义最小修复样本集 | 设计 | done | 至少覆盖依赖、PATH、权限、端口、配置五类样本 | `tmp/stage-h-remediation/latest.json` |
| H02-03 | 自动修复最小链路 | 实现 | done | 已完成 `logs_not_writable`、`frontend_dist_missing`、`gateway_unreachable`，并补 `config_missing_or_invalid` 只读校验子样本 | `tmp/stage-h-remediation/latest.json` |
| H02-04 | 人工接管指引模板落地 | 实现 | done | 每步含操作、预期结果、失败分流与下一步 | `tmp/stage-h-remediation/manual-guides/*.md` |
| H02-05 | 回放与提审收口 | 验证 | done | `latest.json` 与 `replay-results.json` 可复现并可提审 | `tmp/stage-h-remediation/replay-results.json` |
| H02-06 | 决定首批真实修复实现入口 | 设计/实现前检查 | done | 已确定 `logs_not_writable`、`frontend_dist_missing` 与 `gateway_unreachable` 为首批真实执行入口 | `status.md` |
| H02-07 | 核验 Gate-H 对 H-02 阻塞描述并回填 | 验证/文档同步 | done | 已核验人工接管模板实测评分证据存在，H-02 阻塞收缩为“更广人机接管样本不足”并同步 Gate-H 聚合文档 | `tmp/stage-h-remediation/manual-guide-eval.json`, `docs/11-hermes-rebuild/changes/H-gate-h-signoff-20260416/status.md` |
| H02-08 | 补一轮更广人机接管扩样证据 | 验证 | done | `manual-guide-eval.json` 扩至 8 条样本，保持 `guide_score_avg>=4.5` 且 `manual_takeover_success_rate>=0.95` | `tmp/stage-h-remediation/manual-guide-eval.json` |
| H02-09 | 配置类真实自动写入修复最小闭环 | 实现/验证 | done | `config_missing_required_fields` 样本可完成“修复前缺字段 -> 自动补默认值 -> 修复后校验通过 -> 失败降级人工接管”闭环并可复跑 | `tmp/stage-h-remediation/latest.json`, `tmp/stage-h-remediation/replay-results.json` |
| H02-10 | 冻结签收级缺口分层与边界 | 文档/设计 | done | design.md 已明确区分已有闭环与签收级缺口，并冻结高风险配置写入、权限类验收、人工接管、停止/回退边界 | `design.md` |
| H02-11 | 冻结高风险配置写入验收矩阵 | 文档/验证设计 | done | design/verify 已明确 C-A ~ C-F 矩阵，区分低风险基线、必须人工接管项与永久排除项 | `design.md`, `verify.md` |
| H02-12 | 冻结权限类最小必测验收集 | 文档/验证设计 | done | design/verify 已明确 P-A/P-B/P-C/P-D 四层，且每层均有人工接管边界与不可自动代执行说明 | `design.md`, `verify.md` |
| H02-13 | 冻结 H-02 再提审最低门槛 | 文档 | done | design/verify/status/review 已一致表达：未补签收级缺口矩阵前不得再提审 ready，不以继续补低风险样本替代 | `design.md`, `verify.md`, `status.md`, `review.md` |
| H02-14 | 冻结停止条件与回退条件 | 文档 | done | design/tasks/review 已一致表达：若触及系统级写入、管理员权限或跨范围扩项，立即停止并回退到文档收口态 | `design.md`, `review.md` |
| H02-15 | 裁决高风险配置写入后续路径 | 文档/执行决策 | done | design/verify/review 已明确：哪些继续禁止自动化、哪些未来可进入更高门槛验证、哪些必须保持人工接管，且阶段 H 当前不进入高风险实现 | `design.md`, `verify.md`, `review.md` |
| H02-16 | 裁决权限类场景后续路径 | 文档/执行决策 | done | design/verify/review 已明确：哪些继续观察、哪些后续可验证、哪些保持人工接管，且与现有 doctor/remediation 证据一致 | `design.md`, `verify.md`, `review.md` |
| H02-17 | 冻结 H-02 再提审最低闭口条件 | 文档/执行决策 | done | verify/status/review 已明确：什么时候值得进入下一轮验证、什么时候仍停留 warning、哪些条件不满足时禁止回刷 Gate-H | `verify.md`, `status.md`, `review.md` |
| H02-18 | 选定 H-02 本轮唯一受限验证窗口 | 文档/受限验证计划 | done | design/status/review 已明确：本轮仅允许 `logs_not_writable_safe_dir_switch` 进入验证计划，且其余场景继续禁止、观察或维持人工接管 | `design.md`, `status.md`, `review.md` |
| H02-19 | 冻结唯一受限验证计划 | 文档/受限验证计划 | done | design/verify 已明确：前置条件、动作范围、成功判据、失败降级、回退方式、立即停止条件均已写清；本轮不进入实现执行 | `design.md`, `verify.md` |
| H02-20 | 执行唯一受限验证窗口 | 验证/受限执行 | done | `logs_not_writable_safe_dir_switch` 已完成一次真实受限验证；`latest.json` 与 `replay-results.json` 已记录执行结果；未扩展到第二个场景 | `tmp/stage-h-remediation/latest.json`, `tmp/stage-h-remediation/replay-results.json` |
| H02-21 | 选定第二个受限验证窗口 | 文档/授权准备 | done | design/status/review 已明确：第二窗口固定为 `frontend_dist_missing_build_ready_rebuild`，且其他候选继续保持禁止、观察或人工接管 | `design.md`, `status.md`, `review.md` |
| H02-22 | 冻结第二窗口授权准备说明 | 文档/授权准备 | done | design/verify 已明确：第二窗口的前置条件、动作范围、成功判据、失败降级、回退方式、停止条件与授权边界；本轮不进入真实执行 | `design.md`, `verify.md` |
| H02-23 | 回填第二个受限验证窗口结果 | 验证/受限执行 | done | `frontend_dist_missing_build_ready_rebuild` 已在受限边界内完成一次前置条件核查并记录为 `aborted_manual_takeover`；该记录不构成新的合格受限样本，当前只允许冻结观察并维持人工接管 | `tmp/stage-h-remediation/latest.json`, `tmp/stage-h-remediation/replay-results.json`, `status.md`, `verify.md`, `review.md` |
| H02-24 | 回填 baijiacms IIS 主链路恢复样本 | 验证/样本增强 | done | `baijiacms-master` 已从“IIS 统一 500”推进到“80 端口下 PHP 探针与 index.php 均 200”，并把剩余问题收敛到应用层数据库配置；该结果仅增强 H-02 样本强度，不得外推为 ready | `tmp/stage-h-remediation/h02-baijiacms-iis-restored-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-25 | 回填 baijiacms 数据库前置条件缺失接管样本 | 验证/人工接管样本 | done | `baijiacms-master` 已明确收口为“环境已恢复、剩余数据库前置条件缺失”的人工接管样本；接手者无需再排查 IIS/FastCGI，只需转向数据库配置或初始化 | `tmp/stage-h-remediation/h02-baijiacms-db-prereq-takeover-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-26 | 补 baijiacms 数据库前置条件缺失接管指引 | 验证/人工接管指引 | done | 已补专用人工接管指引，接手者可直接按 `config/config.php`、MySQL 服务、`install.php` 三步核对，不再回到环境链路排查 | `tmp/stage-h-remediation/manual-guides/baijiacms-db-prereq-missing.md`, `tmp/stage-h-remediation/h02-baijiacms-db-prereq-guide-20260421.json`, `status.md`, `verify.md` |
| H02-27 | 回填 baijiacms 数据库前置条件最小运行核查 | 验证/人工接管样本 | done | 已确认当前直接原因可收紧为“MySQL 未启动”；临时启动后 `baijiacms` 库存在且页面推进到“未找到站点ID”，接手者应先启动 MySQL，再转业务层排查 | `tmp/stage-h-remediation/h02-baijiacms-db-prereq-runtime-check-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-28 | 回填 baijiacms 站点ID/Host 匹配核查 | 验证/人工接管样本 | done | 已确认“未找到站点ID”源于 Host 与 `system_store.website` 不匹配；切换到 `localhost` 后页面推进到“请先在后台进行店铺装修，新建一个店铺首页”，接手者应先修正访问 Host，再转业务初始化 | `tmp/stage-h-remediation/h02-baijiacms-siteid-host-check-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-29 | 回填 baijiacms 首页装修缺失核查 | 验证/人工接管样本 | done | 已确认“请先在后台进行店铺装修，新建一个店铺首页”源于 `baijiacms_eshop_designer` 缺少 `uniacid=1` 且 `pagetype=1` 的首页装修记录；接手者应直接进入后台店铺装修初始化 | `tmp/stage-h-remediation/h02-baijiacms-homepage-check-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-30 | 收口 baijiacms 多层接管样本总结 | 验证/样本归档 | done | 已把 `baijiacms` 收口为“环境恢复 -> MySQL 启动 -> Host 匹配 -> 首页装修初始化”的高质量多层人工接管样本，并明确当前停止点只到 warning 归档，不外推为 ready | `tmp/stage-h-remediation/h02-baijiacms-sample-pass-summary-20260421.json`, `status.md`, `verify.md`, `review.md` |
| H02-31 | 执行第二受限验证窗口 `frontend_dist_missing_build_ready_rebuild` | 验证/受限执行 | done | 在主控授权下构造"dist 缺失 + build ready"前置条件，执行 `npm run build`，验证 `dist/index.html` 重新生成，全程未触发管理员权限或系统级修复 | `tmp/stage-h-remediation/replay-results.json`, `tmp/stage-h-remediation/latest.json`, `verify.md`, `status.md` |

## 执行顺序

1. 已完成主链路：H02-01 -> H02-02 -> H02-03 -> H02-04 -> H02-05 -> H02-06 -> H02-07 -> H02-08 -> H02-09 -> H02-10
2. 已完成签收级缺口收口：H02-11 -> H02-12 -> H02-13 -> H02-14
3. 已完成执行决策收口：H02-15 -> H02-16 -> H02-17
4. 已完成受限验证计划收口：H02-18 -> H02-19
5. 已完成唯一受限验证窗口执行：H02-20
6. 已完成第二窗口授权准备：H02-21 -> H02-22
7. 已完成第二窗口结果回填：H02-23
8. 已完成 `baijiacms` IIS 主链路恢复样本回填：H02-24
9. 已完成 `baijiacms` 多层接管样本总结收口：H02-25 -> H02-26 -> H02-27 -> H02-28 -> H02-29 -> H02-30
10. H02-21/H02-22 负责选定并冻结第二窗口授权准备；H02-23 记录的是一次受限边界内的前置条件核查，当前结果为前置条件不足而中止并降级人工接管。
11. H02-24 ~ H02-30 共同记录的是 `baijiacms` 从环境恢复一路推进到业务初始化缺口的多层接管样本；当前最强结论只到高质量 warning 样本归档，不得外推为 ready。
12. H02-31 记录的是在主控授权下，第二窗口在真实项目目录上的正式执行：`dist/index.html` 被删除后通过 `npm run build` 成功重建；该结果只证明第二受限窗口在真实条件下可闭环，不等于 H-02 ready，不等于 Gate-H 可签收。
13. 当前 H-02 仍应理解为“冻结但可继续观察”的 warning；第二窗口成功后不再自动进入第三个窗口。


