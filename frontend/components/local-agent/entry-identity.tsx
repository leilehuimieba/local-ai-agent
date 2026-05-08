"use client"

import { Badge } from "@/components/ui/badge"

const coreCapabilities = ["主目标接管", "关键证据更新", "临时切主线", "自动恢复"]
const auxiliaryCapabilities = ["历史", "知识库", "设置"]

function CapabilityRow(props: { title: string; items: string[]; tone: "default" | "secondary" }) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      <span className="text-xs font-medium text-muted-foreground">{props.title}</span>
      {props.items.map((item) => <Badge key={item} variant={props.tone}>{item}</Badge>)}
    </div>
  )
}

export function EntryIdentity() {
  return (
    <section className="border-b border-border bg-muted/30 px-4 py-3">
      <div className="mx-auto flex max-w-6xl flex-col gap-2">
        <p className="text-sm font-semibold text-foreground">主线总控 Agent：围绕当前主目标运行，先看证据，再改判断。</p>
        <p className="text-xs text-muted-foreground">当前主入口优先服务主目标接管、晚间证据、关键证据和临时切主线；其他能力继续保留，但降级为辅助入口。</p>
        <CapabilityRow title="核心链路" items={coreCapabilities} tone="default" />
        <CapabilityRow title="辅助能力" items={auxiliaryCapabilities} tone="secondary" />
      </div>
    </section>
  )
}
