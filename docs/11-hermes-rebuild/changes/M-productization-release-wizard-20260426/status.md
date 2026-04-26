# Status

更新时间：2026-04-26

## 当前状态

上线向导已从静态说明升级为可点击执行版本。用户可以在页面中点击四个步骤按钮，前端调用后端白名单接口执行对应脚本，并显示执行中、通过、失败、耗时和产物路径。

## 已完成

- M-01：新增上线向导页面组件 `ReleaseWizardPanel`。
- M-02：左侧 Rail 增加“上线”入口。
- M-03：首页“准备上线前检查”入口指向上线向导。
- M-04：向导覆盖上线前检查、安装包构建、Doctor 诊断、发布候选验证与发布建议。
- M-05：补充 `ReleaseWizardPanel` 测试，并更新首页测试。
- M-06：前端测试、TS 编译、Cargo check、全量回归通过；安装包构建、Doctor 诊断、RC 验证均已实跑。
- M-07：已输出用户使用模拟：`user-simulation.md`。
- M-08：新增 `POST /api/v1/release/run`，仅允许 `prelaunch`、`package`、`doctor`、`rc` 四个白名单 step。
- M-09：上线向导按钮接入真实 API；每步按钮可触发对应脚本，并展示结果状态。
- M-10：补充后端白名单测试和前端按钮交互测试。

## 待完成

- 无。后续可另开 change 做“读取 JSON 可视化 / 导出发布报告 / 版本回退”。

## 验证证据

- `cd gateway; go test ./internal/api ./internal/service`：通过。
- `cd frontend; npm test -- --run`：25 文件 / 72 测试通过。
- `cd frontend; npx tsc --noEmit`：0 错误。
- `scripts/run-full-regression.ps1 -OutFile tmp/m-release-wizard-buttonized-regression-20260426.json`：6 项全绿。
- 静态向导上一轮安装、Doctor、RC 实跑证据仍保留：`tmp/m-release-wizard-install.json`、`tmp/m-release-wizard-doctor.json`、`tmp/stage-f-rc/latest.json`。