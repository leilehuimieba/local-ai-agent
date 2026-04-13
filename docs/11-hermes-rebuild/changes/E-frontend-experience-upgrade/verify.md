# 验证记录

## 验证方式

- 构建验证：在 `frontend/` 目录执行 `npm run build`。
- 人工验证：按 `docs/frontend-acceptance.md` 的主工作流闭环与调查层闭环条目逐项核对。
- 人工验证：按 Wave 1 主题 1 范围核对主线程与调查层状态表达，不跨页面扩 scope。
- 接口样本复核：提取阶段 C/D 样本中的审计字段并落盘 `tmp/stage-e-audit-consumption/latest.json`。

## 本轮验证点（Wave 1 - 主题 1）

1. 提交后等待态
   - 提交任务后到首个事件到来前，主线程显示“等待首个事件”状态记录。
2. 事件到来后的调查层模式区分
   - 调查层显示“自动跟随最新 / 手动查看历史”模式提示，且手动查看时可回到最新事件。
3. 失败且无事件恢复态
   - 失败且本轮未形成事件时，调查层不回落空态，展示失败说明与恢复建议。
4. 完成态与失败态可扫读性
   - 主线程状态记录区分运行中 / 失败 / 完成三态，文案与视觉层级可快速识别。

## 本轮验证点（Wave 1 - 主题 2 / E-03）

1. 记录页审计筛选可用
   - 筛选栏新增 `确认链 / 工具耗时 / 治理字段` 维度，可按审计信号收敛时间线结果。
2. 时间线审计字段可见
   - 时间线标签可显示确认链步骤、工具耗时、治理状态或归档标记。
3. 详情栏确认链闭环可复核
   - 详情栏可直接查看 `confirmation_chain_step / confirmation_decision / confirmation_resume_strategy / checkpoint_id`。
4. 详情栏治理字段可复核
   - 详情栏可直接查看 `governance_version / governance_source / governance_status / governance_reason / archive_reason`。
5. 接口样本字段落盘
   - 已生成 `tmp/stage-e-audit-consumption/latest.json`，包含阶段 C 风险确认链与阶段 D 治理字段样本值。

## 本轮验证点（Wave 1 - 主题 3）

1. 任务页调查层真实挂载恢复
   - 任务页渲染路径恢复 `BottomPanel`，`renderBottomPanel` 不再固定返回 `null`。
2. 调查层状态接线可用
   - `BottomPanel` 接收 `isOpen/onOpenChange/events/runState/submitError`，与现有任务态联动一致。
3. 任务页关键场景自动展开可用
   - `openTaskPage/openTaskPageForConfirmation` 在确认、失败或已有事件场景下自动展开调查层。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 4）

1. 文案映射单一来源可用
   - `chatResultModel` 提供 `readPendingHeadline/readPendingBody/readPendingAdvice` 三个统一映射函数。
2. 主线程文案复用映射
   - `ChatPanel` 的等待首事件记录改为调用统一映射函数，不再保留本地同义映射。
3. 调查层文案复用映射
   - `BottomPanel` 的等待首事件摘要与建议改为调用统一映射函数，不再保留本地同义映射。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 5）

1. 首页恢复区关键字段降噪可见
   - 恢复上下文主区保留“最近任务/当前阶段/最近摘要”，不再展示“当前运行/当前会话”两行。
2. 上下文摘要聚合可见
   - 原会话/记忆/知识/思考四张卡改为“会话与记忆”“知识与思考”两张聚合卡。
3. 证据摘要聚合可见
   - 原“结果收口/验证依据”改为单卡“结果与验证”。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 6）

1. 主线程头部文案调整可见
   - 任务页头部从“聊天”调整为“任务主线程”。
2. 主线程状态徽标样式复用可见
   - 主线程头部状态显示为统一 `status-badge`，并复用 `readThreadStatusClass` 映射。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 7）

1. 结果块去重可见
   - 助手结果区块不再重复展示与摘要相同文本，也不重复展示相同区块文案。
2. 默认回答标签降噪可见
   - 正式回答默认不再额外显示“正式”标签，头部信息噪音降低。
3. 事件态回显替代空白态可见
   - 在 `home_preview=resume` 场景进入任务页时，主线程显示“状态摘要卡”，不再显示“开始一个任务”空白态。
4. 状态卡头部语义统一可见
   - 状态卡头部显示“状态更新 + 状态徽标”，避免重复状态词。
5. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 8）

1. 任务页容器视觉统一可见
   - 任务页外层容器采用统一 panel 边框、背景与阴影，不再呈现透明裸面风格。
2. 主线程消息卡层级统一可见
   - `thread-record / result-block / state-record` 在任务页恢复卡片层级，与首页/记录页同语义边框与底色。
3. 左侧导航视觉统一可见
   - 任务左侧栏与导航面板背景、分隔线采用统一 token；移动端保持同样视觉基线，不回落透明背景。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 9）

1. 任务工具条可见性
   - 任务页主线程上方可见“任务标题 + 下一步线索 + 状态徽标 + 运行/会话/工作区”摘要。
2. 快捷动作可用性
   - 工具条可直接触发“查看记录页”和“新建任务”动作，不必切换到其他区域操作。
3. 调查层开关文案动态
   - 调查层关闭时按钮为“展开调查层”，打开后切换为“收起调查层”，且点击可正确切换展开状态。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 10）

1. 任务态检查器卡片数量收敛
   - 任务页右侧检查器仅保留“关键动作与下一步 / 状态沉淀 / 风险与记忆”三张卡，不再重复“当前任务”卡。
2. 动作卡字段动态收敛
   - 动作卡默认显示“当前状态/当前动作/下一步线索”，仅在存在验证摘要时显示“最近验证”。
3. 风险卡字段动态收敛
   - 风险卡默认显示“当前阻塞/系统”，仅在存在记忆事件时显示“最近记忆”。
4. 紧凑样式生效
   - 任务态检查器卡片间距与内边距较首页态更紧凑，主线程可见面积增加。
5. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 11）

1. 主线程间距节奏增强
   - 主线程消息列表与消息卡内边距较上一版更宽松，连续阅读时不再拥挤。
2. 标题层级可见增强
   - 主线程标题、消息头、状态卡标题在字号与字重上形成层次，不再“同一重量”。
3. 结果块权重可见增强
   - 摘要块在主线程中优先级更高；action/risk/next 区块通过边框与标题样式可快速区分。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 12）

1. 状态卡文案段落数收敛
   - 主线程状态卡由两段正文收敛为一段合并文案，避免“状态说明 + 下一步”重复堆叠。
2. 状态卡文案去重可见
   - 当正文与建议重复或包含关系成立时，仅保留一份文案。
3. 状态卡正文样式压紧
   - 状态卡正文在字体与行高上更紧凑，主线程纵向占用减少。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 13）

1. 状态卡标题短词统一
   - 主线程状态卡标题统一为“完成 / 失败 / 进行中”，不再显示“任务已完成”等长句标题。
2. 状态卡徽标短词统一
   - 主线程状态卡运行态徽标由“运行中”统一为“进行中”。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 14）

1. 输入区状态提示可见
   - 任务输入区根据状态显示对应提示：未配置提示、执行中提示、可发送提示、草稿提示。
2. 清空草稿入口可用
   - 输入区存在草稿且任务未执行时显示“清空”按钮，点击后输入内容清空。
3. Esc 清空快捷键可用
   - 输入框聚焦时按 `Esc` 可清空草稿，不影响回车提交行为。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 15）

1. 统一状态词典可复用
   - `runtime/state.ts` 提供统一状态短词词典与 `runState` 映射，统一口径为“进行中 / 待确认 / 完成 / 失败”。
2. 任务页与调查层状态词汇一致
   - 任务页主线程状态卡、任务页工具条标签、调查层摘要状态均消费统一词典，不再混用“运行中/已完成/执行失败”等词。
3. 调查层事件流与记录页时间线状态词汇一致
   - `EventTimeline` 与 `HistoryTimelineSection` 状态徽标统一消费同一词典，跨页状态词汇一致。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 16）

1. 主线程状态样式映射单一来源可用
   - `chatResultModel` 的 `readThreadStatusClass` 改为消费 `runtime/state` 的统一映射，不再本地维护重复 runState 分支。
2. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 17）

1. 检查器当前状态短词统一
   - 任务页与首页态检查器“当前状态”字段统一显示短词“进行中 / 待确认 / 完成 / 失败”。
2. 调查层摘要标题短词统一
   - 调查层摘要主标题状态改为统一短词，和任务页与记录页状态词汇一致。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 18）

1. 主线程状态标题短词化可见
   - `chatResultModel` 的 `readRunStateHeadline/readPendingHeadline` 已统一输出短词状态，不再返回“任务运行中/任务恢复中/任务已提交”等长句标题。
2. 失败标题兼容错误码
   - 失败态存在 `error_code` 时标题显示错误码；无错误码时显示“失败”，详细原因由正文承载。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 19）

1. Pending 文案短句化可见
   - `readPendingBody/readPendingAdvice` 已改为短句，仍保留“等待首个事件”和“自动切回最新焦点”核心语义。
2. 运行态正文与下一步短句化可见
   - `readRunStateBody/readRunStateNextStep` 在 submitting/streaming/awaiting_confirmation/resuming/completed 场景下已收短，状态卡阅读密度下降。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 20）

1. 状态卡标题来源单一可见
   - `ChatPanel` 的 `StateRecord` 不再接收未生效的 `title` 入参，标题完全由状态映射函数生成。
2. 未消费标题函数已移除
   - `chatResultModel` 中未被渲染层消费的 `readRunStateHeadline` 已删除，降低后续维护漂移风险。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 21）

1. `statusLine` 短词口径可见
   - `runtime/state.ts` 的 `getRunStateLabel` 已收口到短词状态输出，不再返回“等待首次任务/空闲/提交中”等并行词汇。
2. 顶栏与检查器状态徽标自动对齐
   - 依赖 `statusLine` 的 TopBar 与检查器状态显示自动对齐到统一短词，无需新增页面内分支映射。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 22）

1. 状态线接口签名收口可见
   - `getRunStateLabel` 已移除未使用 `eventCount` 参数，接口与实际行为一致。
2. 状态线调用同步可见
   - `App.tsx` 中 `statusLine` 计算已切到单参数调用，无行为回归。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 23）

1. TopBar 色词一致性可见
   - `submitting/streaming/resuming` 场景下，TopBar 状态徽标与“进行中”短词保持一致，不再出现空闲色。
2. Tone 映射单一来源可见
   - `getRunTone` 已基于统一状态映射计算，避免与 `getRunStateLabel` 语义分叉。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 24）

1. TopBar 运行占位词汇一致
   - 无 `runId` 时 TopBar “运行”字段显示“等待中”，与全局状态短词口径保持一致。
2. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 25）

1. 等待态词汇统一可见
   - 主线程等待标题、检查器动作状态、调查层摘要标题已统一显示“等待中”。
2. 兼容映射可见
   - 检查器状态样式映射保留“等待任务”兼容分支，历史状态词仍能映射到 `status-idle`。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 26）

1. TopBar tone 命名一致性可见
   - `getRunTone` 确认态已输出 `awaiting`，TopBar 徽标直接命中 `status-awaiting` 样式命名。
2. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 27）

1. TopBar tone 输出与状态类名一致
   - `getRunTone` 输出已统一到 `idle/running/awaiting/completed/failed`，TopBar 徽标不再依赖 `done/error` 别名类。
2. 状态样式别名去冗余可见
   - 样式中 `status-waiting/status-error/status-done` 已移除，统一使用主状态类名。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 28）

1. 检查器空闲词汇统一可见
   - Repo 与 Context 状态在无事件时均显示“等待中”，不再显示“空闲”。
2. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 29）

1. 主线程状态播报短词化可见
   - `getStreamLiveLabel` 在运行中/待确认/完成/失败场景统一播报“状态更新：短词”。
2. 归档态播报可见
   - `archived` 场景播报“状态更新：已归档”。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 30）

1. 检查器低风险状态样式映射正确
   - `ContextSidebar` 中风险状态为 `low` 时，检查器样式命中 `status-awaiting`，不再回落 `status-idle`。
2. 待确认语义色一致
   - 低风险待确认场景与 `high/medium` 场景保持同一待确认语义色，避免风险等级显示成空闲态。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 31）

1. 记录页治理状态 class 映射收口可见
   - `history/logType.ts` 的 `readMemoryGovernanceClass` 已改为复用统一状态词典返回 class，不再本地硬编码 `status-*`。
2. 治理状态语义一致可见
   - “已归档/已验证/已写入/已召回/生效中”命中完成态；“待治理/已跳过”命中待确认态；其余回落等待中。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 32）

1. 检查器状态 class 映射单源化可见
   - `ContextSidebar` 的 `readInspectorStatusClass` 已改为消费 `runtime/state` 的统一状态词典映射。
2. 状态同义词兼容可见
   - `失败/降级`、`待确认/high/medium/low`、`完成/已归档`、`等待中/空闲` 等标签仍可正确命中对应语义色。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 33）

1. 设置页状态 class 映射单源化可见
   - `SettingsSections` 的 `readModuleStatusClass` 已改为消费 `runtime/state` 统一状态词典。
2. 设置页状态别名映射一致可见
   - `已锁定/未启用/已启用` 等状态在设置页仍命中原有语义色，不因映射收口出现视觉偏差。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 34）

1. Provider 状态 class 映射单源化可见
   - `ProviderCredentialsSection` 的 `readProviderModuleBadge/readProviderPill` 已改为消费 `runtime/state` 统一词典映射。
2. Provider 状态别名映射一致可见
   - `已应用/待应用/已保存未应用/未配置` 等 Provider 状态标签仍命中原有语义色，不出现样式回归。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 35）

1. 状态总览卡 class 映射单源化可见
   - `StatusCard` 的 `readOverallStatusClass` 已改为消费 `runtime/state` 统一词典映射。
2. 断连特例语义保持可见
   - 状态为 `已断开` 时仍命中 `status-disconnected`，未因收口回落到默认状态色。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 36）

1. Provider “就绪”语义色修正可见
   - `ProviderCredentialsSection` 在“就绪”标签场景已回归空闲色，不再误命中完成色。
2. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 37）

1. Provider 状态键映射显式化可见
   - `ProviderCredentialsSection` 的状态 class 已改为按明确状态键映射，不再依赖文案反推。
2. 语义色稳定性可见
   - `失败/待应用/已应用/未配置/就绪` 场景分别命中 `failed/awaiting_confirmation/completed/idle` 预期语义色。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 38）

1. Provider 反馈态样式映射收口可见
   - `ProviderInlineFeedback` 已改为消费统一词典状态 class，不再依赖 `settings-inline-feedback-${tone}` 的局部隐式映射。
2. Provider 反馈态语义一致可见
   - `pending/error/success` 场景反馈仍分别命中进行中/失败/完成语义色，视觉语义不回归。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 39）

1. 设置页状态映射消费点巡检完成
   - 已完成设置页 `StatusPill` 与状态 class 消费点巡检，并形成“已收口/待收口”清单。
2. 已收口清单
   - `StatusCard.readOverallStatusClass`：已收口到统一词典（保留 `status-disconnected` 特例）。
   - `SettingsSections.readModuleStatusClass`：已收口到统一词典（保留 `status-disconnected` 特例）。
   - `ProviderCredentialsSection.readProviderModuleBadge/readProviderPill`：已收口为状态键显式映射。
3. 待收口清单
   - `SettingsSections.readExternalConnectionModel`：仍存在 `status-completed/status-awaiting/status-idle/status-disconnected` 直写分支，需改为统一词典消费。

## 本轮验证点（Wave 1 - 主题 40）

1. ExternalConnection 状态映射收口可见
   - `SettingsSections.readExternalConnectionModel` 已移除 `status-*` 直写分支，改为按状态键映射语义色。
2. ExternalConnection 语义色一致可见
   - `已可用/当前受限/已预留未接入/未绑定工具` 场景分别命中 `completed/awaiting_confirmation/idle/disconnected` 预期语义色。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 41）

1. 调查层事件状态别名清理可见
   - `EventTimeline.readEventStatusKey` 已改为先消费失败/待确认/完成明确信号，再回退默认分类，减少按事件类型的同义漂移。
2. 调查层事件语义色一致可见
   - 失败、待确认、完成事件在时间线徽标和卡片 tone 上保持一致语义（`failed/awaiting_confirmation/completed`）。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 42）

1. 记录页状态徽标映射收口补齐可见
   - `HistoryTimelineSection.readHistoryStatusKey` 已改为先消费失败/待确认/完成明确信号，再回退默认分类。
2. 记录页徽标与 tone 语义一致可见
   - 时间线 `StatusPill` 与条目 `tone-*` 同步基于统一状态键，不再按类型与状态并行推断。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 43）

1. 主线程状态卡密度二次压缩可见
   - `chat-panel-simple` 下状态卡 padding、标题字号、正文间距与行高已收紧，状态卡纵向占高下降。
2. 状态语义保持可见
   - 状态卡仍保留“状态更新 + 状态徽标 + 正文”结构，完成/失败/进行中语义不回归。
3. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 本轮验证点（Wave 1 - 主题 44）

1. 任务工具条移动端换行修复可见
   - `<=960px` 下任务工具条元信息与动作区改为网格布局，按钮可稳定换行，不再出现拥挤堆叠。
2. 调查层切换按钮点击区修复可见
   - `panel-toggle` 在移动端最小高度提升到 42px，并在窄屏下扩展为整行可点区域。
3. 状态卡移动端挤压修复可见
   - `chat-panel-simple` 状态卡在移动端进一步收紧间距并维持结构完整，不出现文案挤压重叠。
4. 最小构建回归
   - 在 `frontend/` 目录执行 `npm run build`，`tsc -b && vite build` 通过。

## 证据位置

- 测试记录：
  - `frontend/src/chat/ChatPanel.tsx`
  - `frontend/src/chat/chatResultModel.ts`
  - `frontend/src/runtime/state.ts`
  - `frontend/src/workspace/BottomPanel.tsx`
  - `frontend/src/events/EventTimeline.tsx`
  - `frontend/src/shell/workspaceViewModel.tsx`
  - `frontend/src/index.css`
  - `frontend/src/workspace/ContextSidebar.tsx`
  - `frontend/src/App.tsx`
  - `frontend/src/workspace/WorkbenchOverview.tsx`
  - `frontend/src/history/auditSignals.ts`
  - `frontend/src/history/logType.ts`
  - `frontend/src/history/useHistoryReview.ts`
  - `frontend/src/history/components/HistoryPageSections.tsx`
  - `frontend/src/history/components/HistoryTimelineSection.tsx`
  - `frontend/src/history/components/HistoryDetailRail.tsx`
  - `frontend/src/settings/SettingsSections.tsx`
  - `frontend/src/settings/ProviderCredentialsSection.tsx`
  - `frontend/src/settings/StatusCard.tsx`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/tasks.md`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/status.md`
  - `docs/11-hermes-rebuild/changes/E-frontend-experience-upgrade/verify.md`
- 日志或截图：
  - 构建命令：`frontend/` 目录执行 `npm run build`。
  - 构建结果：`tsc -b && vite build` 执行成功，产物已生成到 `frontend/dist/`。
  - 接口样本提取：`tmp/stage-e-audit-consumption/latest.json`（来源：`tmp/stage-c-risk-audit-acceptance/latest.json`、`tmp/stage-d-day1/latest.json`）。

## Gate 映射

- 对应阶段 Gate：Gate-E（交互收口执行中，不做 Gate-E 完成声明）。
- 当前覆盖情况：
  - 已完成 Wave 1 主题 1「任务页状态闭环优化」代码落地与构建验证。
  - 已完成 E-03「前端确认链字段消费改造」代码落地与构建验证。
  - 已完成阶段 C/D 样本审计字段提取，确认前端消费字段与后端口径一致。
  - 已补齐本轮任务与状态文档同步，证据路径可复查。
  - Wave 1 主题 15「任务页/调查层/记录页状态词汇统一」已完成代码落地与构建验证。
  - Wave 1 主题 16「主线程状态样式映射收口」已完成代码落地与构建验证。
  - Wave 1 主题 17「检查器与调查层状态短词补齐」已完成代码落地与构建验证。
  - Wave 1 主题 18「主线程状态标题短词化」已完成代码落地与构建验证。
  - Wave 1 主题 19「主线程状态副文案压缩」已完成代码落地与构建验证。
  - Wave 1 主题 20「主线程状态标题来源收口」已完成代码落地与构建验证。
  - Wave 1 主题 21「全局状态徽标短词统一」已完成代码落地与构建验证。
  - Wave 1 主题 22「状态线接口去冗余」已完成代码落地与构建验证。
  - Wave 1 主题 23「TopBar 状态色与文案对齐」已完成代码落地与构建验证。
  - Wave 1 主题 24「TopBar 运行占位词汇统一」已完成代码落地与构建验证。
  - Wave 1 主题 25「等待态词汇口径统一」已完成代码落地与构建验证。
  - Wave 1 主题 26「TopBar tone 命名收口」已完成代码落地与构建验证。
  - Wave 1 主题 27「状态样式别名去冗余」已完成代码落地与构建验证。
  - Wave 1 主题 28「检查器空闲词汇统一」已完成代码落地与构建验证。
  - Wave 1 主题 29「主线程播报状态短词化」已完成代码落地与构建验证。
  - Wave 1 主题 30「检查器低风险状态映射修复」已完成代码落地与构建验证。
  - Wave 1 主题 31「记录页治理状态样式收口」已完成代码落地与构建验证。
  - Wave 1 主题 32「检查器状态 class 映射单源化」已完成代码落地与构建验证。
  - Wave 1 主题 33「设置页状态样式映射收口」已完成代码落地与构建验证。
  - Wave 1 主题 34「Provider 状态样式映射收口」已完成代码落地与构建验证。
  - Wave 1 主题 35「状态总览卡样式映射收口」已完成代码落地与构建验证。
  - Wave 1 主题 36「Provider 就绪语义色修正」已完成代码落地与构建验证。
  - Wave 1 主题 37「Provider 状态键映射显式化」已完成代码落地与构建验证。
  - Wave 1 主题 38「设置页 Provider 反馈态样式收口」已完成代码落地与构建验证。
  - Wave 1 主题 39「设置页状态映射消费口径巡检」已完成清单巡检与待收口定位。
  - Wave 1 主题 40「设置页 ExternalConnection 状态映射收口」已完成代码落地与构建验证。
  - Wave 1 主题 41「调查层事件状态别名清理」已完成代码落地与构建验证。
  - Wave 1 主题 42「记录页状态徽标映射收口补齐」已完成代码落地与构建验证。
  - Wave 1 主题 43「主线程状态卡密度二次压缩」已完成代码落地与构建验证。
  - Wave 1 主题 44「任务页移动端布局压测」已完成代码落地与构建验证。
  - Gate-E 仍处于执行中，后续按主题继续收口，不做整体完成声明。
