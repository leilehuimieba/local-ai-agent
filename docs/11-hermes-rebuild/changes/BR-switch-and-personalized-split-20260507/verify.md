# 验证记录

## 验证方式

- 单元测试：
  - 临时切主线复核 / 恢复原主线 / 个性化反馈与洞察相关链路保持通过
- 人工验证：
  - 临时切主线与个性化记录入口行为保持一致

## 本次执行结果

1. 定向回归：
   - 命令：`npm test -- mainline-shell`
   - 结果：`30 passed (30)`
2. 前端全量回归：
   - 命令：`npm test`
   - 结果：`56 passed (56)`

## 关键结果

1. 新增 `switch-flow-rules.ts`，承接：
   - 临时切主线草稿
   - 复核失败 / 通过后的状态落点
   - 自动恢复原主线
2. 新增 `personalized-flow-rules.ts`，承接：
   - 个性化反馈草稿
   - 提交后的反馈、历史追加与洞察回写
3. `store.ts` 中不再直接内联：
   - `buildSwitchDraftState(...)`
   - `buildTemporarySwitchState(...)`
   - `restoreOriginalMainlineState(...)`
   - `buildPersonalizedDraftState(...)`
   - `buildPersonalizedSubmitState(...)`
4. `store.ts` 行数进一步下降到 `1096`，`switch / personalized` 责任边界已独立。

## 证据位置

1. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/switch-flow-rules.ts`
4. `D:/newwork/本地智能体/frontend/lib/local-agent/personalized-flow-rules.ts`

## Gate 映射

- 对应自由迭代期目标：
  - 持续压缩 `store.ts` 热点，并把 `switch / personalized` 簇独立成可维护规则文件
