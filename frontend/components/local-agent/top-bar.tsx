"use client"

import { useTheme } from "next-themes"
import { Button } from "@/components/ui/button"
import { Moon, Sun, PanelRightClose, PanelRight, Wifi, WifiOff, Loader2 } from "lucide-react"
import { useRuntimeStore, useUIStore } from "@/lib/local-agent/store"
import type { ViewType, ConnectionState } from "@/lib/local-agent/types"
import { cn } from "@/lib/utils"

const viewTabs: { id: ViewType; label: string }[] = [
  { id: "task", label: "任务" },
  { id: "logs", label: "历史" },
  { id: "knowledge", label: "知识库" },
  { id: "settings", label: "设置" },
]

const connectionConfig: Record<ConnectionState, { color: string; label: string; icon?: React.ReactNode }> = {
  connected: { color: "bg-success", label: "已连接" },
  connecting: { color: "bg-warning", label: "连接中..." },
  closed: { color: "bg-muted-foreground", label: "已关闭" },
  disconnected: { color: "bg-destructive", label: "已断开" },
}

export function TopBar() {
  const { theme, setTheme } = useTheme()
  const { connectionState } = useRuntimeStore()
  const { activeView, setActiveView, rightDrawerOpen, toggleRightDrawer } = useUIStore()

  const connection = connectionConfig[connectionState]

  return (
    <header className="flex h-14 shrink-0 items-center justify-between border-b border-border bg-card px-4">
      {/* Logo */}
      <div className="flex items-center gap-2">
        <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary">
          <span className="text-sm font-bold text-primary-foreground">LA</span>
        </div>
        <span className="text-lg font-semibold text-foreground hidden sm:inline">Local Agent</span>
      </div>

      {/* View Tabs */}
      <nav className="hidden md:flex items-center gap-1 rounded-lg bg-muted p-1">
        {viewTabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveView(tab.id)}
            className={cn(
              "px-4 py-1.5 text-sm font-medium rounded-md transition-all duration-200",
              activeView === tab.id
                ? "bg-card text-foreground shadow-sm"
                : "text-muted-foreground hover:text-foreground"
            )}
          >
            {tab.label}
          </button>
        ))}
      </nav>

      {/* Right Controls */}
      <div className="flex items-center gap-3">
        {/* Connection Status */}
        <div className="flex items-center gap-2">
          <div className={cn(
            "h-2.5 w-2.5 rounded-full transition-colors",
            connection.color,
            connectionState === "connected" && "animate-pulse"
          )} />
          <span className="hidden sm:inline text-xs text-muted-foreground">{connection.label}</span>
        </div>

        {/* Theme Toggle */}
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          className="h-8 w-8"
        >
          <Sun className="h-4 w-4 rotate-0 scale-100 transition-transform dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-transform dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>

        {/* Right Drawer Toggle - Only show in Task view */}
        {activeView === "task" && (
          <Button
            variant="ghost"
            size="icon"
            onClick={toggleRightDrawer}
            className="h-8 w-8"
          >
            {rightDrawerOpen ? (
              <PanelRightClose className="h-4 w-4" />
            ) : (
              <PanelRight className="h-4 w-4" />
            )}
            <span className="sr-only">Toggle drawer</span>
          </Button>
        )}
      </div>
    </header>
  )
}
