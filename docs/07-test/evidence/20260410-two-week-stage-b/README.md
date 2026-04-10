# 2026-04-10 两周执行清单 Stage B 证据

本目录用于承接“知识沉淀主线两周执行清单”的日级证据。

当前已落地：

1. Day1：输入类型表与沉淀字段表。
2. Day2：真实资料导入最小闭环（A/B 两组 read + write）。

---

## 1. Day1 产物

文件：

1. `day1-input-and-schema-20260410.md`

用途：

1. 固定输入类型分层。
2. 固定沉淀字段口径。
3. 作为 Day2~Day7 的上游输入。

---

## 2. Day2 产物

### 2.1 样本范围

Case A：

1. 读取：`day2-a-read.run-*.json`（`read: docs/README.md`）
2. 写入：`day2-a-write.run-*.json`（写入 `tmp/knowledge-import-day2/20260410-two-week/import-a-summary.md`）

Case B：

1. 读取：`day2-b-read.run-*.json`（`read: docs/06-development/知识沉淀型个人助手产品定义_V1.md`）
2. 写入：`day2-b-write.run-*.json`（写入 `tmp/knowledge-import-day2/20260410-two-week/import-b-summary.md`）

### 2.2 汇总文件

1. `day2-import-run-summary-20260410.json`
2. `day2-import-visibility-diff-20260410.json`
3. `day2-import-a.preview.md`
4. `day2-import-b.preview.md`

### 2.3 Day2 结论

1. 四条 run 均为 `run_finished + completed + answer + verified`。
2. 两个目标文件在写入前不存在，写入后存在（`target_exists_after_write=true`）。
3. 导入闭环可追溯：每个 case 都可由 `read run_id -> write run_id -> 目标文件` 串联。

---

## 3. 下一步默认入口

1. Day3 默认进入检索问句专项（5 条问句，记录命中 / 部分命中 / 未命中）。
2. Day3 产物建议继续写入本目录，命名前缀固定为 `day3-`。

---

## 4. Day3 产物（检索问句专项）

### 4.1 原始样本

1. `day3-q1.run-accepted.json` / `day3-q1.run-events.json` / `day3-q1.run-finished.json`
2. `day3-q2.run-accepted.json` / `day3-q2.run-events.json` / `day3-q2.run-finished.json`
3. `day3-q3.run-accepted.json` / `day3-q3.run-events.json` / `day3-q3.run-finished.json`
4. `day3-q4.run-accepted.json` / `day3-q4.run-events.json` / `day3-q4.run-finished.json`
5. `day3-q5.run-accepted.json` / `day3-q5.run-events.json` / `day3-q5.run-finished.json`

### 4.2 判定文件

1. 自动判定：`day3-retrieval-results-20260410.json`、`day3-retrieval-table-20260410.md`
2. 人工复核：`day3-retrieval-reviewed-20260410.json`、`day3-retrieval-reviewed-20260410.md`

### 4.3 Day3 结论

1. 5 条 run 均为 `run_finished + completed + answer + verified`。
2. 人工复核结果：命中 4 条，部分命中 1 条，未命中 0 条。
3. 当前主要风险：读取大 JSON 时摘要截断，可能导致关键词覆盖不完整（q4）。

---

## 5. Day4 产物（真实问句可用性）

### 5.1 首轮快测（当前口径问句）

文件：

1. `day4-q1.run-*.json` 到 `day4-q5.run-*.json`
2. `day4-answer-quality-results-20260410.json`
3. `day4-answer-quality-table-20260410.md`

结论：

1. 首轮结果为 `0/5` 可用。
2. 主要失败类型为 `result_mode=recovery`（`verified_with_recovery`），以及单条 `system` 失败。
3. 核心原因：当前问句在 provider 波动下易触发 `model_parse_failed`，回退到会话恢复兜底答案。

### 5.2 同题复跑（历史稳定问句）

文件：

1. `day4-rerun-q1.run-*.json` 到 `day4-rerun-q5.run-*.json`
2. `day4-answer-quality-rerun-20260410.json`
3. `day4-answer-quality-rerun-table-20260410.md`

结论：

1. 复跑结果为 `2/5` 可用（q3、q5 命中稳定模板并返回 `answer/verified`）。
2. 其余 3 条仍为 `recovery`，说明当前波动主要在“会话续推模板覆盖不全 + provider 解析不稳定”。

### 5.3 最小修补后复跑（rerun4）

文件：

1. `day4-rerun4-q1.run-*.json` 到 `day4-rerun4-q5.run-*.json`
2. `day4-answer-quality-rerun4-20260410.json`
3. `day4-answer-quality-compare-rerun4-20260410.md`

结论：

1. rerun4 结果为 `5/5` 可用（q1~q5 全部 `answer/verified`）。
2. 可用率轨迹：首轮 `0/5` -> 复跑1 `2/5` -> 复跑4 `5/5`。
3. 本轮仅做最小模板收口与路由误判收口，不扩能力域。

---

## 6. Day5 产物（学习记录回写与追溯）

### 6.1 回写与读回样本

文件：

1. `day5-rerun4-write.run-*.json`
2. `day5-rerun4-readback.run-*.json`
3. `day5-learning-writeback-summary-rerun4-20260410.json`
4. `day5-learning-writeback-summary-rerun4-20260410.md`

### 6.2 Day5 结论

1. write/readback 两条 run 均为 `completed + answer + verified`。
2. 回写文件为 `tmp/knowledge-import-day2/20260410-two-week/day5-learning-records-rerun4.md`。
3. 来源追溯检查通过：3 条学习记录均命中对应 `source_run_id`（q1/q2/q4 的 rerun4 样本）。

---

## 7. Day6 产物（端到端演练）

### 7.1 演练样本

文件：

1. `day6-e2e-read.run-*.json`
2. `day6-e2e-write.run-*.json`
3. `day6-e2e-answer.run-*.json`
4. `day6-e2e-writeback.run-*.json`
5. `day6-e2e-readback.run-*.json`
6. `day6-e2e-summary-20260410.json`
7. `day6-e2e-summary-20260410.md`

### 7.2 Day6 结论

1. 端到端 5 步均为 `run_finished + completed + answer + verified`。
2. 链路闭环成立：`导入新资料 -> 写摘要 -> 续推回答 -> 回写学习记录 -> 读回校验`。
3. 当前轨道可继续进入 Day7 阶段判断，不需要扩能力域。

---

## 8. Day7 产物（阶段判断）

### 8.1 判断样本

文件：

1. `day7-stage-decision.run-*.json`
2. `day7-stage-decision-20260410.md`
3. `day7-stage-decision-summary-20260410.json`

### 8.2 Day7 结论

1. 阶段判断 run 为 `completed + answer + verified`。
2. 阶段结论为 `继续`，且下一周第一动作已固定为“Day1-Day6 证据总表归并”。
3. Day7 之后可切入 Stage C Day8（状态字段与更新规则冻结）。

---

## 9. Day8 产物（Stage C 起步：状态字段与规则）

### 9.1 字段与规则文档

文件：

1. `day8-state-schema-20260410.md`

### 9.2 Day8 结论

1. 已冻结最小状态字段表（7 字段）和更新规则表（5 规则）。
2. 已明确 Day9/Day10 对接口径：先补状态快照，再跑 6 条续推问句专项。
3. Day8 仅做口径冻结，不扩能力域。

---

## 10. Day9 产物（状态快照样本）

### 10.1 快照与更新样本

文件：

1. `day9-state-q1.run-*.json`
2. `day9-state-q2.run-*.json`
3. `day9-state-q3.run-*.json`
4. `day9-state-snapshot-20260410.json`
5. `day9-state-snapshot-20260410.md`

### 10.2 Day9 结论

1. 状态快照字段值已形成并落证。
2. 更新样本显示：`q1` 存在 recovery 回退，`q2/q3` 为 answer/verified。
3. Day10 已可基于该快照执行 6 条续推专项。

---

## 11. Day10 产物（6 条续推专项）

### 11.1 首轮与复跑

文件：

1. 首轮：`day10-continue-q1.run-*.json` 到 `day10-continue-q6.run-*.json`、`day10-continuation-results-20260410.{json,md}`
2. 复跑：`day10-rerun2-q5.run-*.json`、`day10-continuation-results-rerun2-20260410.{json,md}`

### 11.2 Day10 结论

1. 首轮结果：`5/6` 可用，唯一失败类型为 `recovery_fallback`（q5）。  
2. 最小修补后复跑：`6/6` 可用（q1~q6 全部 `answer/verified`）。
3. 本轮仅补“30 分钟 + 下一步计划”稳定模板，不扩能力域。

---

## 12. Day11 产物（下一步建议模板校验）

### 12.1 模板与样本

文件：

1. `day11-next-step-template-20260410.md`
2. `day11-next-step-check-20260410.{json,md}`
3. `day11-next-step-check-rerun2-20260410.{json,md}`
4. `day11-rerun3-next-step-s2.run-*.json`
5. `day11-next-step-check-rerun3-20260410.{json,md}`

### 12.2 Day11 结论

1. 首轮格式检查 `0/3`，rerun2 提升到 `2/3`。
2. 最小补丁后（只补“单动作+原因”模板）rerun3 达到 `3/3` 通过。
3. 当前已形成双口径模板：四段式（s1/s3）+ 单动作原因式（s2）。

---

## 13. Day12 产物（低风险动作桥接）

### 13.1 桥接样本

文件：

1. `day12-bridge-suggest.run-*.json`
2. `day12-bridge-read.run-*.json`
3. `day12-bridge-write.run-*.json`
4. `day12-bridge-readback.run-*.json`
5. `day12-bridge-summary-20260410.{json,md}`

### 13.2 Day12 结论

1. 低风险桥接 4 步全通过（`completed + answer + verified`）。
2. 已闭合“建议 -> 读动作 -> 写动作 -> 读回校验”最小执行桥链路。
3. 可继续进入 Day13（执行结果回写状态字段与失败收口）。

---

## 14. Day13 产物（状态回写与失败收口）

### 14.1 状态回写样本

文件：

1. `day13-state-suggest.run-*.json`
2. `day13-state-write.run-*.json`
3. `day13-state-readback.run-*.json`
4. `day13-state-writeback-20260410.{json,md}`

### 14.2 失败收口样本

文件：

1. `day13-failure-write.run-*.json`
2. `day13-failure-readback.run-*.json`
3. `day13-failure-closeout-20260410.{json,md}`

### 14.3 Day13 结论

1. 状态回写 3 步全通过（`completed + answer + verified`）。
2. 失败收口 2 步全通过，当前为“空失败收口记录”。
3. Day13 后可进入 Day14 两周总演练（全链路收口）。

---

## 15. Day14 产物（两周总演练）

### 15.1 全链路样本

文件：

1. `day14-chain-import-read.run-*.json`
2. `day14-chain-sediment-write.run-*.json`
3. `day14-chain-state-update.run-*.json`
4. `day14-chain-continue-answer.run-*.json`
5. `day14-chain-next-step.run-*.json`
6. `day14-chain-lowrisk-read.run-*.json`
7. `day14-chain-writeback.run-*.json`
8. `day14-chain-readback.run-*.json`
9. `day14-full-chain-summary-20260410.{json,md}`
10. `day14-top3-next-issues-20260410.md`

### 15.2 Day14 结论

1. 两周总演练 8 步全通过（`completed + answer + verified`）。
2. 已形成完整链路：`导入资料 -> 沉淀 -> 状态更新 -> 续推回答 -> 下一步建议 -> 低风险动作 -> 回写结果 -> 读回校验`。
3. 当前两周清单（Stage B/C/D）已完成可复跑收口，可转入下周期最小自动化回归。
