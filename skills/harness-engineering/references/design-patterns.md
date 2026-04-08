# 设计模式

## 1. Progressive Disclosure

先暴露目录、索引、摘要，再按需加载细节。

适用：

1. 文档多
2. 工具多
3. 知识多
4. 长任务

## 2. Plan Then Verify

不要把 verify 当成一句礼貌性提示。
把 verify 设计成主循环的一段。

## 3. Independent Evaluator

条件允许时，让生成和评估分离。
如果不能分离模型，也要分离阶段和输入视角。

## 4. Artifact-First Long Outputs

长输出先外置，再把摘要塞回主链路。
这通常比继续压 prompt 更稳。

## 5. Handoff by State Package

长任务不要强行单线程跑到底。
在关键节点生成状态包，再续跑。

## 6. Permission Before Execution

高风险动作要在执行前被识别，而不是执行后补救。
