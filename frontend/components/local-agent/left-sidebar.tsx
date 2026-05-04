"use client"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import {
  Menu,
  Target,
  List,
  BookOpen,
  Settings,
  Plus,
} from "lucide-react"
import { useUIStore, useRuntimeStore } from "@/lib/local-agent/store"
import type { ViewType } from "@/lib/local-agent/types"
import { useIsMobile } from "@/hooks/use-mobile"

const navItems: { id: ViewType; icon: React.ElementType; label: string }[] = [
  { id: "task", icon: Target, label: "任务" },
  { id: "logs", icon: List, label: "历史" },
  { id: "knowledge", icon: BookOpen, label: "知识" },
  { id: "settings", icon: Settings, label: "设置" },
]

export function LeftSidebar() {
  const { activeView, setActiveView, leftSidebarExpanded, toggleLeftSidebar } = useUIStore()
  const { clearMessages } = useRuntimeStore()
  const isMobile = useIsMobile()

  const handleNewTask = () => {
    setActiveView("task")
    clearMessages()
  }

  return (
    <>
    <TooltipProvider delayDuration={0}>
      <aside
        className={cn(
          "hidden md:flex flex-col border-r border-border bg-card transition-all duration-200",
          leftSidebarExpanded ? "w-56" : "w-16"
        )}
      >
        {/* Collapse Button */}
        <div className="flex h-14 items-center justify-center border-b border-border">
          <Button
            variant="ghost"
            size="icon"
            onClick={toggleLeftSidebar}
            className="h-9 w-9"
          >
            <Menu className="h-5 w-5" />
          </Button>
        </div>

        {/* Navigation */}
        <nav className="flex flex-1 flex-col gap-1 p-2">
          {navItems.map((item) => {
            const Icon = item.icon
            const isActive = activeView === item.id

            if (leftSidebarExpanded) {
              return (
                <button
                  key={item.id}
                  onClick={() => setActiveView(item.id)}
                  className={cn(
                    "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors duration-200",
                    isActive
                      ? "bg-primary/10 text-primary"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground"
                  )}
                >
                  <Icon className="h-5 w-5 shrink-0" />
                  <span>{item.label}</span>
                </button>
              )
            }

            return (
              <Tooltip key={item.id}>
                <TooltipTrigger asChild>
                  <button
                    onClick={() => setActiveView(item.id)}
                    className={cn(
                      "flex h-10 w-full items-center justify-center rounded-lg transition-colors duration-200",
                      isActive
                        ? "bg-primary/10 text-primary"
                        : "text-muted-foreground hover:bg-muted hover:text-foreground"
                    )}
                  >
                    <Icon className="h-5 w-5" />
                  </button>
                </TooltipTrigger>
                <TooltipContent side="right" sideOffset={8}>
                  {item.label}
                </TooltipContent>
              </Tooltip>
            )
          })}
        </nav>

        {/* New Task Button */}
        <div className="p-2 border-t border-border">
          {leftSidebarExpanded ? (
            <Button
              onClick={handleNewTask}
              className="w-full gap-2 bg-primary hover:bg-primary/90"
            >
              <Plus className="h-4 w-4" />
              新任务
            </Button>
          ) : (
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  onClick={handleNewTask}
                  size="icon"
                  className="w-full bg-primary hover:bg-primary/90"
                >
                  <Plus className="h-5 w-5" />
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right" sideOffset={8}>
                新任务
              </TooltipContent>
            </Tooltip>
          )}
        </div>
      </aside>
    </TooltipProvider>

    {/* Mobile Bottom Navigation */}
    {isMobile && (
      <nav className="fixed bottom-0 left-0 right-0 z-50 flex items-center justify-around border-t border-border bg-card/95 backdrop-blur-sm h-14 md:hidden">
        {navItems.map((item) => {
          const Icon = item.icon
          const isActive = activeView === item.id
          return (
            <button
              key={item.id}
              onClick={() => setActiveView(item.id)}
              className={cn(
                "flex flex-col items-center justify-center gap-0.5 h-full w-full transition-colors",
                isActive ? "text-primary" : "text-muted-foreground"
              )}
            >
              <Icon className="h-5 w-5" />
              <span className="text-[10px]">{item.label}</span>
            </button>
          )
        })}
        <button
          onClick={handleNewTask}
          className="flex flex-col items-center justify-center gap-0.5 h-full w-full text-primary"
        >
          <Plus className="h-5 w-5" />
          <span className="text-[10px]">新任务</span>
        </button>
      </nav>
    )}
    </>
  )
}
