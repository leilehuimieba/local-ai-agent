import { useMemo, useState } from "react";

import { MemoryEntry, SettingsResponse } from "../../shared/contracts";
import { EmptyStateBlock, MetaGrid, MetricChip, SectionHeader, StatusPill } from "../../ui/primitives";

type MemoryFilter = "all" | "workspace" | "knowledge" | "runtime" | "verified";
type MemoryGroupKey = "workspace" | "knowledge" | "runtime" | "other";
type ResourceActionState = {
  tone: "running" | "failed" | "completed";
  message: string;
};

type MemoryResourcesSectionProps = {
  settings: SettingsResponse;
  memories: MemoryEntry[];
  error: string | null;
  actionState: ResourceActionState | null;
  deletingId: string;
  isRunning: boolean;
  isRefreshing: boolean;
  onDeleteMemory: (memoryId: string) => void;
  onRefresh: () => void;
};

const MEMORY_GROUP_ORDER: MemoryGroupKey[] = ["workspace", "knowledge", "runtime", "other"];

export function MemoryResourcesSection(props: MemoryResourcesSectionProps) {
  const [filter, setFilter] = useState<MemoryFilter>("all");
  const [expandedId, setExpandedId] = useState("");
  const model = useMemo(() => buildMemoryModel(props.memories, props.settings.workspace.workspace_id, filter), [filter, props.memories, props.settings.workspace.workspace_id]);
  return (
    <section className="settings-module">
      <MemorySectionHeader enabled={props.settings.memory_policy.enabled} count={model.filtered.length} />
      <div className="settings-control-grid">
        <MemoryStatusCard enabled={props.settings.memory_policy.enabled} />
        <MemoryStrategyCard title="召回策略" body={props.settings.memory_policy.recall_strategy} />
        <MemoryStrategyCard title="写入策略" body={props.settings.memory_policy.write_strategy} />
        <MemoryStrategyCard title="清理策略" body={props.settings.memory_policy.cleanup_strategy} />
        <MemoryStats settings={props.settings} filteredCount={model.filtered.length} />
        <MemoryPathCard settings={props.settings} />
        <MemoryList
          actionState={props.actionState}
          deletingId={props.deletingId}
          error={props.error}
          expandedId={expandedId}
          filter={filter}
          groups={model.groups}
          isRunning={props.isRunning}
          isRefreshing={props.isRefreshing}
          onDeleteMemory={props.onDeleteMemory}
          onExpandedIdChange={setExpandedId}
          onFilterChange={setFilter}
          onRefresh={props.onRefresh}
        />
      </div>
    </section>
  );
}

function MemorySectionHeader(props: { enabled: boolean; count: number }) {
  return <SectionHeader title="Memory Policy" action={<div className="page-header-meta"><MetricChip label="结果" value={`${props.count} 条`} /><StatusPill label={props.enabled ? "已启用" : "未启用"} /></div>} />;
}

function MemoryStatusCard(props: { enabled: boolean }) {
  return (
    <div className="detail-card">
      <strong>记忆状态</strong>
      <p>{props.enabled ? "当前工作区已启用记忆召回与沉淀。" : "当前工作区未启用记忆链路。"}</p>
    </div>
  );
}

function MemoryStats(props: { settings: SettingsResponse; filteredCount: number }) {
  return <MetaGrid items={buildMemoryStats(props.settings, props.filteredCount)} />;
}

function MemoryStrategyCard(props: { title: string; body: string }) {
  return (
    <div className="detail-card muted-card">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}

function MemoryPathCard(props: { settings: SettingsResponse }) {
  return (
    <div className="detail-card muted-card">
      <strong>存储落点</strong>
      <p className="workspace-root">{props.settings.memory_policy.sqlite_path}</p>
      <p className="workspace-root">{props.settings.memory_policy.working_memory_dir}</p>
      <p className="workspace-root">{props.settings.memory_policy.long_term_memory_path}</p>
      <p className="workspace-root">{props.settings.memory_policy.knowledge_base_path}</p>
    </div>
  );
}

function MemoryList(props: {
  actionState: ResourceActionState | null;
  deletingId: string;
  error: string | null;
  expandedId: string;
  filter: MemoryFilter;
  groups: Array<{ key: string; label: string; items: MemoryEntry[] }>;
  isRunning: boolean;
  isRefreshing: boolean;
  onDeleteMemory: (memoryId: string) => void;
  onExpandedIdChange: (memoryId: string) => void;
  onFilterChange: (filter: MemoryFilter) => void;
  onRefresh: () => void;
}) {
  return (
    <div className="settings-subsection">
      <SectionHeader title="记忆入口" action={<MemoryListActions filter={props.filter} isRefreshing={props.isRefreshing} onFilterChange={props.onFilterChange} onRefresh={props.onRefresh} />} />
      {props.actionState ? <MemoryActionNotice state={props.actionState} /> : null}
      {props.error ? <MemoryErrorCard message={props.error} /> : null}
      {!props.error && props.groups.length === 0 ? <EmptyMemoryCard /> : null}
      {!props.error && props.groups.length > 0 ? <div className="memory-list">{props.groups.map((group) => <MemoryGroup key={group.key} deletingId={props.deletingId} expandedId={props.expandedId} group={group} isRunning={props.isRunning} onDeleteMemory={props.onDeleteMemory} onExpandedIdChange={props.onExpandedIdChange} />)}</div> : null}
    </div>
  );
}

function MemoryListActions(props: {
  filter: MemoryFilter;
  isRefreshing: boolean;
  onFilterChange: (filter: MemoryFilter) => void;
  onRefresh: () => void;
}) {
  return (
    <div className="page-header-meta">
      <MemoryFilterSelect filter={props.filter} onChange={props.onFilterChange} />
      <button type="button" className="secondary-button" disabled={props.isRefreshing} onClick={props.onRefresh}>
        {props.isRefreshing ? "刷新中" : "刷新记忆列表"}
      </button>
    </div>
  );
}

function MemoryActionNotice(props: { state: ResourceActionState }) {
  return <p className={`settings-inline-feedback settings-inline-feedback-${props.state.tone}`}>{props.state.message}</p>;
}

function MemoryErrorCard(props: { message: string }) {
  return (
    <div className="detail-card">
      <strong>记忆入口</strong>
      <p>{props.message}</p>
    </div>
  );
}

function EmptyMemoryCard() {
  return <EmptyStateBlock compact title="没有匹配记忆" text="调整筛选条件后，这里会显示可管理的记忆条目。" />;
}

function MemoryFilterSelect(props: { filter: MemoryFilter; onChange: (filter: MemoryFilter) => void }) {
  return (
    <label className="filter-select">
      <span>筛选</span>
      <select value={props.filter} onChange={(event) => props.onChange(event.target.value as MemoryFilter)}>
        <option value="all">全部</option>
        <option value="workspace">当前工作区</option>
        <option value="knowledge">知识沉淀</option>
        <option value="runtime">运行过程</option>
        <option value="verified">仅已验证</option>
      </select>
    </label>
  );
}

function MemoryGroup(props: {
  deletingId: string;
  expandedId: string;
  group: { key: string; label: string; items: MemoryEntry[] };
  isRunning: boolean;
  onDeleteMemory: (memoryId: string) => void;
  onExpandedIdChange: (memoryId: string) => void;
}) {
  return (
    <div className="memory-group">
      <div className="memory-group-head">
        <strong>{props.group.label}</strong>
        <span className="sidebar-chip-muted">{`${props.group.items.length} 条`}</span>
      </div>
      {props.group.items.map((memory) => <MemoryItem key={memory.id} expanded={props.expandedId === memory.id} isDeleting={props.deletingId === memory.id} isRunning={props.isRunning} memory={memory} onDeleteMemory={props.onDeleteMemory} onExpandedIdChange={props.onExpandedIdChange} />)}
    </div>
  );
}

function MemoryItem(props: {
  expanded: boolean;
  isDeleting: boolean;
  isRunning: boolean;
  memory: MemoryEntry;
  onDeleteMemory: (memoryId: string) => void;
  onExpandedIdChange: (memoryId: string) => void;
}) {
  return (
    <div className="memory-item">
      <div className="memory-item-head">
        <div>
          <strong>{props.memory.title || props.memory.summary}</strong>
          <p>{readMemoryLead(props.memory)}</p>
        </div>
        <div className="timeline-chip-row">
          <StatusPill className={props.memory.verified ? "status-completed" : "status-idle"} label={props.memory.verified ? "已验证" : "未验证"} />
          <button type="button" className="secondary-button" onClick={() => props.onExpandedIdChange(props.expanded ? "" : props.memory.id)}>
            {props.expanded ? "收起详情" : "查看详情"}
          </button>
          <button type="button" className="secondary-button" onClick={() => props.onDeleteMemory(props.memory.id)} disabled={props.isRunning || props.isDeleting}>
            {props.isDeleting ? "删除中" : "删除记忆"}
          </button>
        </div>
      </div>
      <MetaGrid items={buildMemoryRows(props.memory)} />
      {props.expanded ? <MemoryDetail memory={props.memory} /> : null}
    </div>
  );
}

function MemoryDetail(props: { memory: MemoryEntry }) {
  return (
    <div className="timeline-detail-group">
      <p className="timeline-detail">{`存在原因：${memoryReason(props.memory)}`}</p>
      <p className="timeline-detail">{`作用范围：${memoryScope(props.memory)}`}</p>
      <p className="timeline-detail">{`来源：${memorySourceLabel(props.memory)}`}</p>
      {props.memory.source_title ? <p className="timeline-detail">{`来源标题：${props.memory.source_title}`}</p> : null}
      {props.memory.source_event_type ? <p className="timeline-detail">{`来源事件类型：${props.memory.source_event_type}`}</p> : null}
      {props.memory.source_artifact_path ? <p className="timeline-detail">{`证据/产物路径：${props.memory.source_artifact_path}`}</p> : null}
      <p className="timeline-detail">{`所属运行：${props.memory.source_run_id || "未记录"}`}</p>
      <p className="timeline-detail">{`所属会话：${props.memory.session_id || "未记录"}`}</p>
      <p className="timeline-detail">{`归档状态：${memoryArchivedLabel(props.memory)}`}</p>
      {props.memory.archived && props.memory.archived_at ? <p className="timeline-detail">{`归档时间：${props.memory.archived_at}`}</p> : null}
      <p className="timeline-detail">{`创建时间：${memoryCreatedAt(props.memory)}`}</p>
      <p className="timeline-detail">{`最近更新：${memoryUpdatedAt(props.memory)}`}</p>
      <p className="timeline-detail">{`原始内容：${props.memory.content || props.memory.summary}`}</p>
    </div>
  );
}

function buildMemoryModel(memories: MemoryEntry[], workspaceId: string, filter: MemoryFilter) {
  const filtered = sortMemories(memories.filter((memory) => matchesMemoryFilter(memory, workspaceId, filter)));
  return {
    filtered,
    groups: buildMemoryGroups(filtered, workspaceId),
  };
}

function matchesMemoryFilter(memory: MemoryEntry, workspaceId: string, filter: MemoryFilter) {
  if (filter === "workspace") return isWorkspaceMemory(memory, workspaceId);
  if (filter === "knowledge") return isKnowledgeMemory(memory);
  if (filter === "runtime") return isRuntimeMemory(memory);
  if (filter === "verified") return memory.verified;
  return true;
}

function buildMemoryGroups(memories: MemoryEntry[], workspaceId: string) {
  const groups = new Map<MemoryGroupKey, MemoryEntry[]>();
  memories.forEach((memory) => {
    const key = readMemoryGroupKey(memory, workspaceId);
    groups.set(key, [...(groups.get(key) || []), memory]);
  });
  return MEMORY_GROUP_ORDER
    .filter((key) => groups.has(key))
    .map((key) => ({ items: groups.get(key) || [], key, label: readMemoryGroupLabel(key) }));
}

function readMemoryGroupKey(memory: MemoryEntry, workspaceId: string): MemoryGroupKey {
  if (isWorkspaceMemory(memory, workspaceId)) return "workspace";
  if (isKnowledgeMemory(memory)) return "knowledge";
  if (isRuntimeMemory(memory)) return "runtime";
  return "other";
}

function readMemoryGroupLabel(key: MemoryGroupKey) {
  if (key === "workspace") return "当前工作区";
  if (key === "knowledge") return "知识沉淀";
  if (key === "runtime") return "运行过程";
  return "其他记录";
}

function sortMemories(memories: MemoryEntry[]) {
  return [...memories].sort((left, right) => readMemoryTime(right) - readMemoryTime(left));
}

function isWorkspaceMemory(memory: MemoryEntry, workspaceId: string) {
  return Boolean(memory.workspace_id && memory.workspace_id === workspaceId);
}

function isKnowledgeMemory(memory: MemoryEntry) {
  return containsKeyword(memory.kind, "knowledge") || containsKeyword(memory.source_type, "knowledge");
}

function isRuntimeMemory(memory: MemoryEntry) {
  return (memory.source || "runtime") === "runtime";
}

function containsKeyword(value: string, keyword: string) {
  return value.toLowerCase().includes(keyword);
}

function readMemoryTime(memory: MemoryEntry) {
  const value = memory.updated_at || memory.timestamp || memory.created_at;
  const numeric = Date.parse(value);
  return Number.isNaN(numeric) ? 0 : numeric;
}

function buildMemoryStats(settings: SettingsResponse, filteredCount: number) {
  return [
    { label: "长期记忆", value: `${settings.memory_policy.long_term_memory_count} 条` },
    { label: "知识条目", value: `${settings.memory_policy.knowledge_count} 条` },
    { label: "工作记忆文件", value: `${settings.memory_policy.working_memory_files} 个` },
    { label: "当前列表", value: `${filteredCount} 条` },
  ];
}

function buildMemoryRows(memory: MemoryEntry) {
  return [
    { label: "类型", value: memory.kind },
    { label: "来源", value: memorySourceLabel(memory) },
    { label: "来源类型", value: memory.source_type || "runtime" },
    memory.source_title ? { label: "来源标题", value: memory.source_title } : null,
    memory.source_event_type ? { label: "来源事件", value: memory.source_event_type } : null,
    memory.source_artifact_path ? { label: "证据/产物路径", value: memory.source_artifact_path } : null,
    { label: "范围", value: memoryScope(memory) },
    { label: "运行", value: memory.source_run_id || "未记录" },
    { label: "会话", value: memory.session_id || "未记录" },
    { label: "归档", value: memoryArchivedLabel(memory) },
    memory.archived && memory.archived_at ? { label: "归档时间", value: memory.archived_at } : null,
    { label: "优先级", value: String(memory.priority) },
    { label: "更新时间", value: memoryUpdatedAt(memory) },
  ].filter(Boolean) as Array<{ label: string; value: string }>;
}

function readMemoryLead(memory: MemoryEntry) {
  return `${memory.kind} / ${memorySourceLabel(memory)} / ${memoryScope(memory)}`;
}

function memoryReason(memory: MemoryEntry) {
  return memory.reason || "按当前工作区持续复用";
}

function memoryScope(memory: MemoryEntry) {
  return memory.scope || memory.workspace_id || "未标记";
}

function memorySourceLabel(memory: MemoryEntry) {
  if (memory.source && memory.source_type) return `${memory.source} / ${memory.source_type}`;
  return memory.source || memory.source_type || "runtime";
}

function memoryArchivedLabel(memory: MemoryEntry) {
  return memory.archived ? "已归档" : "活跃";
}

function memoryCreatedAt(memory: MemoryEntry) {
  return memory.created_at || memory.timestamp || "未记录";
}

function memoryUpdatedAt(memory: MemoryEntry) {
  return memory.updated_at || memory.timestamp || "未记录";
}
