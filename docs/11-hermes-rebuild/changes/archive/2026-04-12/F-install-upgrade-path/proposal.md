# 变更提案

## 背景

- 阶段 E 的 Gate-E 已通过，主线切换到阶段 F。
- `F-01` 要求“安装/升级主路径实现”，当前仓库缺少统一安装脚本与对应验收证据。
- 用户明确要求继续后端推进，本轮不改前端实现。

## 目标

- 交付一条可执行的 Windows 安装/升级主路径脚本。
- 交付 `F-01` 验收脚本，产出可复核证据 `tmp/stage-f-install/latest.json`。
- 覆盖最小闭环：安装、启动可达、升级、回退副本保留。

## 非目标

- 本轮不实现 `doctor` 命令（归属 `F-02`）。
- 本轮不推进发布清单与回滚文档收口（归属 `F-03`）。
- 本轮不涉及前端 UI 改造。

## 验收口径

- `install-local-agent.ps1` 支持 `install/upgrade` 两种模式。
- `run-stage-f-install-acceptance.ps1` 执行成功并输出 `status=passed`。
- 证据中可复核：
  - 安装产物齐全；
  - 安装后 gateway/runtime 健康可达；
  - 升级后保留 `backups` 副本；
  - 版本文件与升级版本一致。
