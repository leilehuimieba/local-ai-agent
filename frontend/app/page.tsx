"use client"

import { useEffect } from "react"
import { TopBar } from "@/components/local-agent/top-bar"
import { LeftSidebar } from "@/components/local-agent/left-sidebar"
import { RightDrawer } from "@/components/local-agent/right-drawer"
import { TaskView } from "@/components/local-agent/views/task-view"
import { LogsView } from "@/components/local-agent/views/logs-view"
import { KnowledgeView } from "@/components/local-agent/views/knowledge-view"
import { SettingsView } from "@/components/local-agent/views/settings-view"
import { useUIStore, useSettingsStore } from "@/lib/local-agent/store"

export default function LocalAgentPage() {
  const { activeView } = useUIStore()
  const { loadSettings } = useSettingsStore()

  useEffect(() => {
    loadSettings()
  }, [loadSettings])

  return (
    <div className="flex h-dvh w-full overflow-hidden bg-background" suppressHydrationWarning>
      {/* Left Sidebar */}
      <LeftSidebar />

      {/* Main Content Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top Bar */}
        <TopBar />

        {/* Content + Right Drawer */}
        <div className="flex flex-1 overflow-hidden">
          {/* Main View */}
          <main className="flex-1 overflow-hidden">
            <div className="h-full animate-in fade-in duration-200">
              {activeView === "task" && <TaskView />}
              {activeView === "logs" && <LogsView />}
              {activeView === "knowledge" && <KnowledgeView />}
              {activeView === "settings" && <SettingsView />}
            </div>
          </main>

          {/* Right Drawer - Only visible in Task view */}
          <RightDrawer />
        </div>
      </div>
    </div>
  )
}
