# Design: 产品化封装上线向导

## 页面形态

新增独立视图 `release`，通过左侧 Rail 进入。页面名称为“上线向导”。

## 向导结构

每一步包含：

- 用户目标
- 执行动作
- 对应脚本
- 关键产物
- 通过标准
- 失败时建议

## 步骤

1. 上线前检查：`scripts/run-full-regression.ps1`
2. 安装包构建：`scripts/install-local-agent.ps1`
3. Doctor 诊断：`scripts/doctor.ps1`
4. 发布候选验证：`scripts/run-stage-f-rc-acceptance.ps1`
5. 发布建议：汇总证据、风险、回退路径

## 风险与回退

- 如果 UI 入口影响导航测试，更新测试断言。
- 如果脚本名称后续调整，只需更新向导配置。
- 回退方式：移除 `release` 视图和导航入口。
