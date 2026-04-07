# V1 新增能力验收样本（2026-04-07）

本目录用于留存“启动本地程序 / 可控安装与验证能力”的真实验收样本。

## 样本清单

1. `launch-local-program-sample.json`
- 场景：启动本地程序（`powershell.exe`）
- 验证：进程 PID、进程路径、启动时间、受控结束

2. `controlled-install-verify-sample.json`
- 场景：隔离虚拟环境内命令行安装（`ruff==0.6.9`）
- 验证：安装命令输出、版本号、可执行文件路径与版本

3. `winget-system-install-verify-sample.json`
- 场景：通过 `winget` 进行系统级包管理安装（`BurntSushi.ripgrep.MSVC`）
- 验证：安装输出、安装列表、包实际安装路径、命令版本

4. `choco-system-install-verify-sample.json`
- 场景：通过 `choco` 进行系统级包管理安装（`fzf`）
- 结果：失败留证
- 失败原因：当前用户对 `C:\ProgramData\chocolatey` 无写权限，样本用于记录系统级安装边界与环境限制

## 说明

1. `controlled-install-verify-sample.json` 采用隔离虚拟环境 `tmp/v1-capability-acceptance/install-env`，避免污染系统全局环境。
2. `winget-system-install-verify-sample.json` 为成功样本。
3. `choco-system-install-verify-sample.json` 为真实失败样本，反映当前环境权限限制，不代表产品能力链路缺失。
4. 所有样本均由 2026-04-07 当天执行并落盘。
