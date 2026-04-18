# H 阶段主控交接（2026-04-17）

更新时间：2026-04-17  
适用范围：当前对话主控交接 / 新主控 agent 接续  
状态：当前有效（直到主控再次改写）

## 1. 当前权威口径

1. 当前阶段：`阶段 H`
2. 当前 Gate：`Gate-H（执行中，未签收）`
3. 当前活跃 change：`H-mcp-skills-quality-20260415`
4. 单一事实源：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`

## 2. 本轮主控已完成的裁决

1. 已将主推进从 `H-remediation-playbook-20260415` 切到 `H-mcp-skills-quality-20260415`。
2. 已将 H-02 改为“并行观察 / 冻结观察”，不再作为当前主推进。
3. 已确认 Gate-H 当前仍不得回刷，不得写成可签收。
4. 已确认 H-03 当前虽为主推进，但仍然是 `warning`，不能写成 `ready`。

## 3. 当前三个方向的定位

### 3.1 H-03（当前主推进）

1. 当前定位：`正式执行入口已明确，已切为当前主推进`
2. 当前状态：`warning`
3. 当前最小推进顺序：
   - `H03-37`：正式执行起跑确认
   - `H03-38`：第一批正式执行批次（未授权前不得开跑）
   - `H03-39`：正式执行后复核与交接
4. 当前不允许：
   - 把 H-03 写成 `ready`
   - 把 H-03 写成 `Gate-H 可签收`
   - 把“主推进”误写成“已完成”
   - 继续无边界补样

### 3.2 H-02（并行观察）

1. 当前定位：`冻结观察`
2. 当前状态：`warning`
3. 当前结论：
   - 第二窗口只形成 `aborted_manual_takeover`
   - 当前没有新的合格受限样本
   - 不再新增正式窗口
4. 当前只允许：
   - 文档一致性维护
   - 新候选样本出现后的“候选准备口径”
5. 当前不允许：
   - 把旧 replay 外推成新成功结论
   - 主动新增正式窗口
   - 回刷 Gate-H

### 3.3 Gate-H（聚合尾线）

1. 当前定位：`挂起，仅聚合，不主推`
2. 当前状态：不可签收
3. 当前不应主动更新结论，除非 H-03 形成新的签收级证据

## 4. 本轮实际改动文件

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`
5. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
6. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/tasks.md`
7. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
8. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/review.md`
9. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`

## 5. 新主控接手后先读哪些文件

按这个顺序读取即可：

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/formal-execution-entry.md`
5. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/tasks.md`
6. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/verify.md`
7. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/latest.json`
8. `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/scale-out-strategy-h03.json`
9. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/status.md`
10. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-remediation-playbook-20260415/verify.md`

## 6. 新主控的第一任务

新主控接手后，第一任务不是改 Gate-H，也不是回头再裁 H-02，而是：

1. 组织一个只在 H-03 工作区内运行的执行 agent
2. 只做 `H03-37`：正式执行起跑确认
3. 让该 agent 输出：
   - `H03-37` 是否完成
   - 当前是否允许交由主控判断是否启动 `H03-38`
   - 当前仍为 `warning` 的明确边界

## 7. 进入 H03-38 前必须坚持的边界

1. 不能把“允许主控判断是否启动 H03-38”写成“已经启动 H03-38”
2. 不能把“已经启动 H03-38”写成“已经形成结果”
3. 不能把数量门槛 `30 / 24 / 16` 当成唯一门槛
4. 必须同时坚持四类结构门槛：
   - 长尾
   - 恢复链
   - 命中有效性校准
   - 双轮 / 角色差异复核
5. 任一结构门槛未达标时，不得回刷 Gate-H

## 8. 当前已知风险点

1. H-03 已切为主推进，但很容易被误写成“已经 ready”
2. H-03 的 formal execution entry 很容易被误读成“已经授权 H03-38”
3. H-02 虽已冻结观察，但若有人把旧 replay 当成新窗口成功，会把口径重新写乱
4. Gate-H 目前只能挂起聚合，不能因为主推进切换就改成可签收

## 9. 交接给新主控时的固定提醒

1. 先读文档，再决定是否开 agent
2. 只允许 H-03 agent 写：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/H-mcp-skills-quality-20260415/`
   - `D:/newwork/本地智能体/tmp/stage-h-mcp-skills/`
3. 不允许 H-03 agent 改：
   - `current-state.md`
   - `changes/INDEX.md`
   - `H-remediation-playbook-20260415/`
   - `H-gate-h-signoff-20260416/`
4. 新主控在拿到 H03-37 结果前，不要再动 Gate-H。
