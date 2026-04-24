# H-runtime-strict-e2e-20260424 Proposal

## 背景

Gate-H 已签收，但端到端复核发现 `scripts/run-stage-e-entry1-acceptance.ps1` 当前可在仅 accepted / protocol 字段匹配时通过，不能证明 Gateway → Runtime → Logs → terminal completed 的严格运行时闭环。

## 目标

1. 将 Stage-E Entry1 默认验收恢复为严格 runtime terminal 口径。
2. 保留 accepted fallback 作为显式降级模式，不再作为默认通过条件。
3. 让 doctor 支持在 JSON `status=failed` 时返回非零退出码，避免批量脚本只看 exit code 误判。
4. 记录当前 runtime 严格闭环失败证据，为后续定位 runtime/logs 链路提供可复现入口。

## 非目标

1. 本 change 不放宽 Gate-H 已签收边界。
2. 本 change 不宣称 runtime 完整 E2E 已通过。
3. 本 change 不处理长期校准治理闭合。
