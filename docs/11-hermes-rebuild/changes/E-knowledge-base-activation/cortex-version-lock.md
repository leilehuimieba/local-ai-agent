# Cortex 版本锁定记录（T04）

更新时间：2026-04-13  
所属 change：`E-knowledge-base-activation`  
目的：固定外部依赖版本，避免本地适配过程中被上游变更扰动。

## 1. 锁定目标

1. 上游仓库：`https://github.com/rikouu/cortex`
2. 锁定分支：`main`（仅作为来源说明，不作为运行时浮动依赖）
3. 锁定提交：
   - 全量 SHA：`213916230d1ec4b4b5ddbba83a090400a795941f`
   - 短 SHA：`2139162`
4. 提交信息：`chore: bump versions to v0.10.5 / @cortexmem/openclaw@0.6.6`
5. 提交时间：`2026-03-31 19:38:00 +0900`

## 2. 本地评估副本

1. 评估路径：`D:\newwork\third_party_eval\cortex`
2. 校验命令：
   - `git rev-parse HEAD`
   - `git show -s --format="%ci %s" HEAD`

## 3. 使用约束

1. 本轮 `T04-T11` 统一基于上述锁定提交，不直接跟随上游 `main` 最新状态。
2. 若需升级版本，必须先新增一条任务并补差异评估，不可直接替换。
3. 所有适配脚本与文档默认标注该提交号，避免复现歧义。
