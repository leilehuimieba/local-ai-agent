# 结果包装层 + 前端消费层闭环证据

本目录只用于证明“结果包装层 + 前端消费层”的真实链路闭环，不扩接口，不改共享合同，不改主循环。

## 本轮结论

1. 当前源码中，被模式阻止的 `verification_completed` 路径已经写入 `metadata.result_mode = "system"`，不是现行源码缺口。
2. 上一轮之所以出现“阻止样本被页面误显示成恢复结果”，根因是取证时前后台仍跑着旧进程，运行日志里 `result_mode` 为 `null`。
3. 本轮在重启当前代码对应进程后，重新采集了 `answer / recovery / system` 三类真实样本，已证明：
   - runtime 确实发出了 `event.metadata.result_mode`
   - 该字段穿过了现有传输层进入前端事件流
   - 前端优先消费这个运行时信号完成页面分层
   - 页面分层不是只靠前端启发式兜底

## 源码口径

运行时写入点：

- [crates/runtime-core/src/lib.rs](/d:/newwork/本地智能体/crates/runtime-core/src/lib.rs) `135-186`：`RiskOutcome::Blocked` 分支会给 `verification_completed` 和 `run_finished` 写入 `result_mode = "system"`。
- [crates/runtime-core/src/lib.rs](/d:/newwork/本地智能体/crates/runtime-core/src/lib.rs) `739-761`：`make_run_failed_event()` 会给 `run_failed` 写入 `result_mode = "system"`。
- [crates/runtime-core/src/lib.rs](/d:/newwork/本地智能体/crates/runtime-core/src/lib.rs) `448-449`、`570-571`、`655-662`：一般完成路径通过 `read_result_mode()` 推导并写入 `answer / recovery / system`。

前端消费点：

- [frontend/src/chat/chatResultModel.ts](/d:/newwork/本地智能体/frontend/src/chat/chatResultModel.ts) `200-212`：`readAssistantMode()` 先读 `event.metadata.result_mode`，只有缺失时才退回 `verification_snapshot.code`、`completion_status`、`final_answer` 等兜底信号。
- [frontend/src/chat/ChatPanel.tsx](/d:/newwork/本地智能体/frontend/src/chat/ChatPanel.tsx) `126`、`410-414`：assistant 卡片绑定最终 `run_finished/run_failed` 事件，再交给结果模型做分层。
- [frontend/src/runtime/state.ts](/d:/newwork/本地智能体/frontend/src/runtime/state.ts) `238-250`：assistant 消息正文仍主要取自 `final_answer` / `metadata.final_answer`。

这意味着：

1. “显示什么内容”与“按什么结果类型分层”是两件事。
2. 正文内容仍可能来自 `final_answer`。
3. 页面上的“正式回答 / 恢复结果 / 系统说明”标签，优先依据的是 runtime 发出的 `metadata.result_mode`。

## 真实样本

### 1. answer 样本

- 运行 ID：`run-1775728598098-7`
- 输入：`继续推进`
- 事件证据：
  - [answer.run-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/answer.run-events.json)
  - [answer.terminal-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/answer.terminal-events.json)
- 页面证据：
  - [answer.page.png](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/answer.page.png)
- 关键观察：
  - 终态事件中可见 `metadata.result_mode = "answer"`
  - 同一终态事件中可见 `metadata.verification_code = "verified"`
  - 页面主结果区显示“正式回答”

### 2. recovery 样本

- 运行 ID：`run-1775728824872-15`
- 输入：`根据 docs 和当前代码，说明这个项目现在做到什么程度，作为 recovery 样本 20260409B`
- 事件证据：
  - [recovery.run-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/recovery.run-events.json)
  - [recovery.terminal-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/recovery.terminal-events.json)
- 页面证据：
  - [recovery.page.png](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/recovery.page.png)
- 关键观察：
  - 终态事件中可见 `metadata.result_mode = "recovery"`
  - 同一终态事件中可见 `metadata.verification_code = "verified_with_recovery"`
  - `verification_summary` 与 `result_summary` 明确记录了 `model_transport_failed: curl: (7) Failed to connect to 127.0.0.1 port 39081...`
  - 页面主结果区显示“恢复结果”

### 3. system 样本

- 运行 ID：`run-1775728474305-3`
- 输入：`帮我打开计算器`
- 事件证据：
  - [system.run-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/system.run-events.json)
  - [system.terminal-events.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/system.terminal-events.json)
- 页面证据：
  - [system.page.png](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/system.page.png)
- 关键观察：
  - `verification_completed` 中可见 `metadata.result_mode = "system"`
  - `run_failed` 中可见 `metadata.result_mode = "system"`
  - `run_finished` 中仍带有 `final_answer`，同时 `metadata.result_mode = "system"`
  - 页面主结果区显示“系统说明”

## 为什么 system 样本最关键

`system` 样本不是只证明 runtime 发了字段，更证明前端确实吃到了这个字段。

原因如下：

1. [frontend/src/chat/chatResultModel.ts](/d:/newwork/本地智能体/frontend/src/chat/chatResultModel.ts) `203-207` 的兜底逻辑里，只要没有 `metadata.result_mode`，但事件带着 `final_answer`，最终就会落成 `answer`。
2. `system` 样本的 `run_finished` 事件确实带着 `final_answer`。
3. 页面最终却显示“系统说明”，而不是“正式回答”。

因此，这个页面结果不可能只靠 `final_answer` 或其他前端启发式得到，必须依赖 runtime 发来的 `metadata.result_mode = "system"`。

## 页面映射关系

本轮样本对应关系如下：

- `answer` 样本：runtime 发出 `metadata.result_mode = "answer"`，页面显示“正式回答”。
- `recovery` 样本：runtime 发出 `metadata.result_mode = "recovery"`，页面显示“恢复结果”。
- `system` 样本：runtime 发出 `metadata.result_mode = "system"`，页面显示“系统说明”。

这三条样本共同证明了页面分层与 runtime 信号一一对应，而不是前端把所有带正文的结果都当作正式答复。

## 仍属于前端兜底的部分

本轮不应夸大为“前端完全不依赖启发式”。当前真实口径应是：

1. 前端优先消费 `event.metadata.result_mode`。
2. 当该字段缺失时，前端仍会退回 `verification_snapshot.code`、`completion_status`、`hasRecoverySignal()`、`final_answer` 等启发式规则。
3. 因此，本轮闭合的是“runtime 已稳定发出结果模式，前端已优先消费该信号”，而不是“前端兜底逻辑已被删除”。

这部分在本轮验收里不再视为残余缺口，而视为兼容旧事件的保底边界。只要对外交付口径保持为“优先消费 runtime `result_mode`，缺失时才回退兜底”，就不会影响本轮“结果包装层 + 前端消费层最小收口”的成立。

## 旧进程失真说明

上一轮目录 [20260409-result-mode-runtime-ui](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui) 不应再作为本轮最终证据。

原因是：

1. 取证时运行的不是当前源码对应进程。
2. 旧进程导出的终态事件中 `metadata.result_mode` 为 `null`。
3. 这会导致 observe 阻止样本在页面上落入前端兜底，出现错误分层。

本轮在重启当前进程后重新采集，因此本目录才是可信证据。

## 运行环境留证

- 当前恢复后的设置快照：
  - [settings-restored.json](/d:/newwork/本地智能体/docs/07-test/evidence/20260409-result-mode-runtime-ui-closure/settings-restored.json)
- 恢复结果：
  - `mode = observe`
  - `model.provider_id = scnet`
  - `model.model_id = MiniMax-M2.5`
