# CET4 验收样例集（T01）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`  
用途：作为“学习4级备考”真实业务固定样例，覆盖写入、召回、可视化三类链路。

## 1. 样例来源与边界

1. 来源目录：`docs/07-test/evidence/20260410-cet4-user-simulation/`
2. 本样例集用于本轮 Gate-E 子门禁，不替代全量回归。
3. 输入文本按真实 run 抽取，预期结果按“结构化验收条件”定义，不绑定某一模型具体措辞。

## 2. 样例清单（9 条）

| id | 场景 | 输入（摘要） | 预期结果（结构化） |
|---|---|---|---|
| `cet4-q1-plan` | 备考总计划 | 8 周里程碑 + 每天 30 分钟安排 | 输出包含“周里程碑”和“日计划”；不能只返回补信息提示 |
| `cet4-q2-method` | 分模块训练 | 听力/阅读/写作/翻译方法与避坑 | 输出覆盖四模块；每模块至少 1 个方法 + 1 个避坑点 |
| `cet4-q3-today30` | 当天执行清单 | 今天可开始的 30 分钟训练 | 输出为分钟级拆分（总时长约 30 分钟） |
| `cet4-q4-tracking-template` | 学习追踪模板 | 字段、频率、进步判断 | 输出包含模板字段、记录频率、进步判据 |
| `cet4-q5-write-log` | 学习日志写入 | `write: tmp/cet4/20260410-study-log.md` | 文件写入成功，返回写入摘要，路径在工作区内 |
| `cet4-q6-read-log` | 学习日志读取 | `read: tmp/cet4/20260410-study-log.md` | 文件读取成功，摘要包含关键学习数据 |
| `cet4-q7-adjust-next` | 次日计划调整 | 基于“背词80、听力62%、阅读58%”出明日安排 | 输出含“问题定位 + 次日动作 + 可量化目标” |
| `cet4-q8-browser-extension` | 能力边界问答 | 四级场景下浏览器扩展能力现状 | 输出含“已完成/未完成/下一步”三段 |
| `cet4-q9-next-line` | 启动语 | 明早可直接执行的一句话 | 输出 1 句可执行指令，不空泛 |

## 3. 验收检查项（每条通用）

1. `completion_status=completed`
2. `verification_code` 为 `verified` 或 `verified_with_recovery`
3. `final_answer` 为中文可执行内容，不包含工具调用标记
4. 若是工具型样例（`write/read`），必须有对应 artifact 且文件操作成功

## 4. 与后续任务的对应关系

1. `T09-T10`：用于 ingest/recall 冒烟输入
2. `T20-T24`：用于导出 Markdown 与图谱可视化验证
3. `T25`：作为固定评测集核心样本

## 5. 配套机器可读文件

1. `docs/11-hermes-rebuild/changes/E-knowledge-base-activation/fixtures/cet4-acceptance-cases.jsonl`
