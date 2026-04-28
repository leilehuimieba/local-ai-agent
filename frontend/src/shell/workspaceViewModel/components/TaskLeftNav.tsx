import { useMemo, useState } from "react";
import { HistoryNavPanel } from "../../../history/components";
import { KnowledgeNavPanel } from "../../../knowledge-base/KnowledgeNavPanel";
import { SettingsNavPanel } from "../../../settings/SettingsNavPanel";
import { isBusyRunState, readUnifiedStatusFromRunState, readUnifiedStatusMeta, useRuntimeStore } from "../../../runtime/state";
import { HOME_EXAMPLES, type AppModel, type AppView, type TaskNavEntry } from "../types";

export function TaskLeftNav(props: { app: AppModel }) {
  const [expanded, setExpanded] = useState(true);
  const [taskSearch, setTaskSearch] = useState("");
  const groups = useMemo(() => buildTaskNavGroups(props.app), [props.app]);
  const pinnedItems = useMemo(
    () => filterTaskNavEntries(groups.pinned, taskSearch),
    [groups.pinned, taskSearch],
  );
  const recentItems = useMemo(
    () => filterTaskNavEntries(groups.recent, taskSearch),
    [groups.recent, taskSearch],
  );
  const isTaskView = props.app.view.currentView === "task";
  const isLogsView = props.app.view.currentView === "logs";
  const isKnowledgeView = props.app.view.currentView === "knowledge";
  const isSettingsView = props.app.view.currentView === "settings";
  const logs = props.app.logs;
  return (
    <aside className={readTaskLeftNavClass(expanded)} aria-label="任务页导航">
      <TaskNavRail app={props.app} expanded={expanded} onToggleExpand={() => setExpanded((value) => !value)} />
      {expanded && isTaskView ? (
        <TaskNavPanel
          app={props.app}
          search={taskSearch}
          pinnedItems={pinnedItems}
          recentItems={recentItems}
          onSearchChange={setTaskSearch}
        />
      ) : null}
      {expanded && isLogsView ? (
        <HistoryNavPanel
          logCount={logs.filteredLogs.length}
          search={logs.search}
          statusFilter={logs.statusFilter}
          timeFilter={logs.timeFilter}
          onSearchChange={logs.setSearch}
          onStatusFilterChange={logs.setStatusFilter}
          onTimeFilterChange={logs.setTimeFilter}
        />
      ) : null}
      {expanded && isKnowledgeView ? <KnowledgeNavPanel /> : null}
      {expanded && isSettingsView ? <SettingsNavPanel /> : null}
    </aside>
  );
}

function readTaskLeftNavClass(expanded: boolean) {
  return expanded ? "task-left-nav expanded" : "task-left-nav";
}

function TaskNavRail(props: { app: AppModel; expanded: boolean; onToggleExpand: () => void }) {
  return (
    <nav className="task-nav-rail" aria-label="任务快捷导航">
      <button type="button" className="task-nav-button icon-only" title={props.expanded ? "收起侧栏" : "展开侧栏"} aria-label={props.expanded ? "收起侧栏" : "展开侧栏"} onClick={props.onToggleExpand}><span className="task-nav-icon" aria-hidden="true">☰</span></button>
      <NavIconButton app={props.app} nav="task" icon="◉" label="任务" />
      <NavIconButton app={props.app} nav="logs" icon="≡" label="历史" />
      <NavIconButton app={props.app} nav="knowledge" icon="📚" label="知识库" />
      <NavIconButton app={props.app} nav="settings" icon="⚙" label="设置" />
      <button type="button" className="task-nav-button icon-only task-nav-action" title="新任务" aria-label="新任务" onClick={props.app.actions.openHomeStart}><span className="task-nav-icon" aria-hidden="true">＋</span></button>
    </nav>
  );
}

function NavIconButton(props: { app: AppModel; nav: AppView; icon: string; label: string }) {
  return (
    <button
      type="button"
      className={readTaskNavClass(props.app.view.currentView, props.nav)}
      title={props.label}
      aria-label={props.label}
      onClick={() => props.app.view.setCurrentView(props.nav)}
    >
      <span className="task-nav-icon" aria-hidden="true">{props.icon}</span>
    </button>
  );
}

function readTaskNavClass(currentView: AppView, nav: AppView) {
  return currentView === nav ? "task-nav-button icon-only active" : "task-nav-button icon-only";
}

function TaskNavPanel(props: {
  app: AppModel;
  search: string;
  pinnedItems: TaskNavEntry[];
  recentItems: TaskNavEntry[];
  onSearchChange: (value: string) => void;
}) {
  return (
    <section className="task-nav-panel">
      <header className="task-nav-panel-head"><strong>任务</strong><span>会话导航</span></header>
      <button type="button" className="task-nav-primary" onClick={props.app.actions.openHomeStart}>新建任务</button>
      <input id="task_nav_search" name="task_nav_search" className="task-nav-search" type="search" value={props.search} placeholder="搜索任务与历史" aria-label="搜索任务与历史" autoComplete="off" onChange={(event) => props.onSearchChange(event.target.value)} />
      <TaskNavGroup title="置顶" empty="暂无置顶任务" items={props.pinnedItems} onPick={(value) => props.app.actions.openTaskPageWithDraft(value)} />
      <TaskNavGroup title="最近" empty="暂无历史记录" items={props.recentItems} onPick={(value) => props.app.actions.openTaskPageWithDraft(value)} />
    </section>
  );
}

function TaskNavGroup(props: { title: string; empty: string; items: TaskNavEntry[]; onPick: (value: string) => void }) {
  return (
    <section className="task-nav-group">
      <header>{props.title}</header>
      {props.items.length === 0 ? <p className="task-nav-empty">{props.empty}</p> : props.items.map((item) => (
        <button key={item.id} type="button" className="task-history-item" onClick={() => props.onPick(item.title)}>
          <span>{item.title}</span>
          <em>{item.tag}</em>
        </button>
      ))}
    </section>
  );
}

function buildTaskNavGroups(app: AppModel) {
  return {
    pinned: buildPinnedTaskEntries(app),
    recent: buildRecentTaskEntries(app),
  };
}

function hasUsefulTaskTitle(title: string) {
  return Boolean(title && title !== "等待第一条任务");
}

function buildPinnedTaskEntries(app: AppModel) {
  const items: TaskNavEntry[] = [];
  if (hasUsefulTaskTitle(app.runtime.currentTaskTitle)) items.push({ id: "current", title: app.runtime.currentTaskTitle, tag: readRunStateTag(app.runtime.runState) });
  if (app.runtime.confirmation) items.push({ id: `confirm-${app.runtime.confirmation.confirmation_id || "pending"}`, title: app.runtime.confirmation.action_summary || "待确认操作", tag: "待确认" });
  if (app.runtime.runState === "failed") items.push({ id: "failed", title: "上次执行失败，建议重试或调整描述", tag: "恢复建议" });
  return dedupeTaskNavEntries(items).slice(0, 4);
}

function buildRecentTaskEntries(app: AppModel) {
  const userItems = app.runtime.messages.filter((item) => item.role === "user").map((item) => ({
    id: item.id,
    tag: "历史",
    title: readTaskNavTitle(item.content),
  }));
  const suggestions = HOME_EXAMPLES.map((item) => ({ id: `example-${item.id}`, tag: "模板", title: item.prompt }));
  const fallback = hasUsefulTaskTitle(app.runtime.currentTaskTitle)
    ? [{ id: "recent-current", title: app.runtime.currentTaskTitle, tag: "当前" }]
    : suggestions;
  return dedupeTaskNavEntries([...userItems, ...fallback]).slice(0, 12);
}

function filterTaskNavEntries(items: TaskNavEntry[], search: string) {
  const keyword = search.trim().toLowerCase();
  if (!keyword) return items;
  return items.filter((item) => item.title.toLowerCase().includes(keyword) || item.tag.toLowerCase().includes(keyword));
}

function dedupeTaskNavEntries(items: TaskNavEntry[]) {
  const map = new Map<string, TaskNavEntry>();
  items.forEach((item) => {
    const key = item.title.trim().toLowerCase();
    if (!key || map.has(key)) return;
    map.set(key, item);
  });
  return [...map.values()];
}

function readTaskNavTitle(value: string) {
  const text = value.replace(/\s+/g, " ").trim();
  if (!text) return "未命名任务";
  return text.length > 46 ? `${text.slice(0, 46)}…` : text;
}

function readRunStateTag(runState: ReturnType<typeof useRuntimeStore.getState>["runState"]) {
  if (runState === "archived") return "最新";
  if (runState === "idle") return "最新";
  return readUnifiedStatusMeta(readUnifiedStatusFromRunState(runState)).label;
}
