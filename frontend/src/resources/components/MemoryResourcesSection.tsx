import { useMemo, useState } from "react";

import { countMemoryFacets, readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceClass, readMemoryGovernanceLabel } from "../../history/logType";
import { MemoryEntry, SettingsResponse } from "../../shared/contracts";
import { EmptyStateBlock, MetaGrid, MetricChip, SectionHeader, StatusPill } from "../../ui/primitives";

type MemoryFilter = "all" | "preference" | "lesson" | "governance" | "archived" | "verified";
type MemoryGroupKey = "preference" | "lesson" | "knowledge" | "runtime" | "other";
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

const MEMORY_GROUP_ORDER: MemoryGroupKey[] = ["preference", "lesson", "knowledge", "runtime", "other"];

export function MemoryResourcesSection(props: MemoryResourcesSectionProps) {
  const [filter, setFilter] = useState<MemoryFilter>("all");
  const [expandedId, setExpandedId] = useState("");
  const model = useMemo(() => buildMemoryModel(props.memories, filter), [filter, props.memories]);
  return (
    <section className="settings-module">
      <MemorySectionHeader count={model.filtered.length} enabled={props.settings.memory_policy.enabled} />
      <MemoryOverviewGrid filteredCount={model.filtered.length} latestMemory={model.latestMemory} settings={props.settings} memories={props.memories} />
      <MemoryList actionState={props.actionState} deletingId={props.deletingId} error={props.error} expandedId={expandedId} filter={filter} groups={model.groups} isRunning={props.isRunning} isRefreshing={props.isRefreshing} onDeleteMemory={props.onDeleteMemory} onExpandedIdChange={setExpandedId} onFilterChange={setFilter} onRefresh={props.onRefresh} />
    </section>
  );
}

function MemoryOverviewGrid(props: {
  filteredCount: number;
  latestMemory?: MemoryEntry;
  settings: SettingsResponse;
  memories: MemoryEntry[];
}) {
  return (
    <div className="settings-control-grid">
      <MemoryStatusCard enabled={props.settings.memory_policy.enabled} latestMemory={props.latestMemory} />
      <MemoryGovernanceCard memories={props.memories} />
      <MemoryStrategyCard body={props.settings.memory_policy.recall_strategy} title="召回策略" />
      <MemoryStrategyCard body={props.settings.memory_policy.write_strategy} title="写入策略" />
      <MemoryStrategyCard body={props.settings.memory_policy.cleanup_strategy} title="清理策略" />
      <MemoryStats filteredCount={props.filteredCount} settings={props.settings} />
      <MemoryPathCard settings={props.settings} />
    </div>
  );
}

function MemorySectionHeader(props: { enabled: boolean; count: number }) {
  return <SectionHeader title="Memory Policy" action={<div className="page-header-meta"><MetricChip label="结果" value={`${props.count} 条`} /><StatusPill label={props.enabled ? "已启用" : "未启用"} /></div>} />;
}

function MemoryStatusCard(props: { enabled: boolean; latestMemory?: MemoryEntry }) {
  return (
    <div className="detail-card">
      <strong>记忆现场</strong>
      <p>{props.enabled ? "当前工作区会继续召回、沉淀和筛选记忆。" : "当前工作区未启用记忆链路。"}</p>
      <p>{props.latestMemory ? `最近动作：${readMemoryActivityLabel(props.latestMemory)} / ${readMemoryFacetLabel(props.latestMemory)}` : "最近还没有新的记忆动作。"}</p>
      <p>{props.latestMemory ? `治理状态：${readMemoryGovernanceLabel(props.latestMemory)}` : "治理状态会随召回、写入和归档变化。"}</p>
      <p>{props.latestMemory ? `治理版本：${memoryGovernanceVersion(props.latestMemory)}` : "治理版本会随记忆治理策略更新。"}</p>
    </div>
  );
}

function MemoryGovernanceCard(props: { memories: MemoryEntry[] }) {
  return (
    <div className="detail-card muted-card">
      <strong>治理摘要</strong>
      <MetaGrid items={buildGovernanceRows(props.memories)} />
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
  return <EmptyStateBlock compact title="没有匹配记忆" text="调整筛选条件后，这里会显示偏好、失败教训和治理状态。" />;
}

function MemoryFilterSelect(props: { filter: MemoryFilter; onChange: (filter: MemoryFilter) => void }) {
  return (
    <label className="filter-select">
      <span>筛选</span>
      <select name="memory_filter" value={props.filter} onChange={(event) => props.onChange(event.target.value as MemoryFilter)}>
        <option value="all">全部记忆</option>
        <option value="preference">仅看偏好</option>
        <option value="lesson">仅看失败教训</option>
        <option value="governance">仅看待治理</option>
        <option value="archived">仅看已归档</option>
        <option value="verified">仅看已验证</option>
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
    <div className="settings-subsection">
      <div className="thread-record-head">
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
        <MemoryItemActions expanded={props.expanded} isDeleting={props.isDeleting} isRunning={props.isRunning} memory={props.memory} onDeleteMemory={props.onDeleteMemory} onExpandedIdChange={props.onExpandedIdChange} />
      </div>
      <MetaGrid items={buildMemoryRows(props.memory)} />
      {props.expanded ? <MemoryDetail memory={props.memory} /> : null}
    </div>
  );
}

function MemoryItemActions(props: {
  expanded: boolean;
  isDeleting: boolean;
  isRunning: boolean;
  memory: MemoryEntry;
  onDeleteMemory: (memoryId: string) => void;
  onExpandedIdChange: (memoryId: string) => void;
}) {
  return (
    <div className="timeline-chip-row">
      <StatusPill className={readMemoryGovernanceClass(props.memory)} label={readMemoryGovernanceLabel(props.memory)} />
      <StatusPill className="status-idle" label={readMemoryFacetLabel(props.memory)} />
      <span className="sidebar-chip-muted">{readMemoryActivityLabel(props.memory)}</span>
      <button type="button" className="secondary-button" onClick={() => props.onExpandedIdChange(props.expanded ? "" : props.memory.id)}>
        {props.expanded ? "收起详情" : "查看详情"}
      </button>
      <button type="button" className="secondary-button" disabled={props.isRunning || props.isDeleting} onClick={() => props.onDeleteMemory(props.memory.id)}>
        {props.isDeleting ? "删除中" : "删除记忆"}
      </button>
    </div>
  );
}

function MemoryDetail(props: { memory: MemoryEntry }) {
  return (
    <div className="timeline-detail-group">
      <p className="timeline-detail">{`治理状态：${readMemoryGovernanceLabel(props.memory)}`}</p>
      <p className="timeline-detail">{`最近动作：${readMemoryActivityLabel(props.memory)}`}</p>
      <p className="timeline-detail">{`治理版本：${memoryGovernanceVersion(props.memory)}`}</p>
      <p className="timeline-detail">{`治理来源：${memoryGovernanceSource(props.memory)}`}</p>
      <p className="timeline-detail">{`治理时间：${memoryGovernanceAt(props.memory)}`}</p>
      <p className="timeline-detail">{`存在原因：${memoryReason(props.memory)}`}</p>
      <p className="timeline-detail">{`作用范围：${memoryScope(props.memory)}`}</p>
      <p className="timeline-detail">{`来源：${memorySourceLabel(props.memory)}`}</p>
      {props.memory.source_title ? <p className="timeline-detail">{`来源标题：${props.memory.source_title}`}</p> : null}
      {props.memory.source_event_type ? <p className="timeline-detail">{`来源事件：${props.memory.source_event_type}`}</p> : null}
      {props.memory.source_artifact_path ? <p className="timeline-detail">{`证据路径：${props.memory.source_artifact_path}`}</p> : null}
      {props.memory.governance_reason ? <p className="timeline-detail">{`治理依据：${props.memory.governance_reason}`}</p> : null}
      {props.memory.archive_reason ? <p className="timeline-detail">{`归档原因：${props.memory.archive_reason}`}</p> : null}
      <p className="timeline-detail">{`所属运行：${props.memory.source_run_id || "未记录"}`}</p>
      <p className="timeline-detail">{`所属会话：${props.memory.session_id || "未记录"}`}</p>
      <p className="timeline-detail">{`原始内容：${props.memory.content || props.memory.summary}`}</p>
    </div>
  );
}

function buildMemoryModel(memories: MemoryEntry[], filter: MemoryFilter) {
  const filtered = sortMemories(memories.filter((memory) => matchesMemoryFilter(memory, filter)));
  return {
    filtered,
    groups: buildMemoryGroups(filtered),
    latestMemory: filtered[0],
  };
}

function matchesMemoryFilter(memory: MemoryEntry, filter: MemoryFilter) {
  if (filter === "preference") return readMemoryFacetLabel(memory) === "用户偏好";
  if (filter === "lesson") return readMemoryFacetLabel(memory) === "失败教训";
  if (filter === "governance") return readMemoryGovernanceLabel(memory) === "待治理";
  if (filter === "archived") return memory.archived;
  if (filter === "verified") return memory.verified;
  return true;
}

function buildMemoryGroups(memories: MemoryEntry[]) {
  const groups = new Map<MemoryGroupKey, MemoryEntry[]>();
  memories.forEach((memory) => appendMemoryGroup(groups, memory));
  return MEMORY_GROUP_ORDER.filter((key) => groups.has(key)).map((key) => ({ items: groups.get(key) || [], key, label: readMemoryGroupLabel(key) }));
}

function appendMemoryGroup(groups: Map<MemoryGroupKey, MemoryEntry[]>, memory: MemoryEntry) {
  const key = readMemoryGroupKey(memory);
  groups.set(key, [...(groups.get(key) || []), memory]);
}

function readMemoryGroupKey(memory: MemoryEntry): MemoryGroupKey {
  if (readMemoryFacetLabel(memory) === "用户偏好") return "preference";
  if (readMemoryFacetLabel(memory) === "失败教训") return "lesson";
  if (readMemoryFacetLabel(memory) === "知识沉淀") return "knowledge";
  if ((memory.source || "runtime") === "runtime") return "runtime";
  return "other";
}

function readMemoryGroupLabel(key: MemoryGroupKey) {
  if (key === "preference") return "用户偏好";
  if (key === "lesson") return "失败教训";
  if (key === "knowledge") return "知识沉淀";
  if (key === "runtime") return "运行过程";
  return "其他记录";
}

function sortMemories(memories: MemoryEntry[]) {
  return [...memories].sort((left, right) => readMemoryTime(right) - readMemoryTime(left));
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

function buildGovernanceRows(memories: MemoryEntry[]) {
  const summary = countMemoryFacets(memories);
  return [
    { label: "偏好", value: `${summary.preferences} 条` },
    { label: "失败教训", value: `${summary.lessons} 条` },
    { label: "待治理", value: `${summary.pending} 条` },
    { label: "已归档", value: `${summary.archived} 条` },
    { label: "审计覆盖", value: `${countAuditedMemories(memories)} 条` },
    { label: "治理版本", value: latestGovernanceVersion(memories) },
  ];
}

function buildMemoryRows(memory: MemoryEntry) {
  return [
    { label: "类型层", value: readMemoryFacetLabel(memory) },
    { label: "治理状态", value: readMemoryGovernanceLabel(memory) },
    { label: "最近动作", value: readMemoryActivityLabel(memory) },
    { label: "来源", value: memorySourceLabel(memory) },
    { label: "来源事件", value: memory.source_event_type || "未记录" },
    { label: "证据路径", value: memory.source_artifact_path || "未记录" },
    { label: "治理版本", value: memoryGovernanceVersion(memory) },
    { label: "治理来源", value: memoryGovernanceSource(memory) },
    { label: "治理时间", value: memoryGovernanceAt(memory) },
    { label: "范围", value: memoryScope(memory) },
    { label: "运行", value: memory.source_run_id || "未记录" },
    { label: "会话", value: memory.session_id || "未记录" },
    { label: "优先级", value: String(memory.priority) },
    { label: "更新时间", value: memoryUpdatedAt(memory) },
  ];
}

function readMemoryLead(memory: MemoryEntry) {
  return `${readMemoryFacetLabel(memory)} / ${readMemoryActivityLabel(memory)} / ${memoryScope(memory)}`;
}

function memoryReason(memory: MemoryEntry) {
  return memory.reason || "当前记忆没有附带额外原因。";
}

function memoryScope(memory: MemoryEntry) {
  return memory.scope || memory.workspace_id || "未标记";
}

function memorySourceLabel(memory: MemoryEntry) {
  if (memory.source && memory.source_type) return `${memory.source} / ${memory.source_type}`;
  return memory.source || memory.source_type || "runtime";
}

function memoryUpdatedAt(memory: MemoryEntry) {
  return memory.updated_at || memory.timestamp || "未记录";
}

function countAuditedMemories(memories: MemoryEntry[]) {
  return memories.filter((memory) => !!memory.governance_version).length;
}

function latestGovernanceVersion(memories: MemoryEntry[]) {
  return memories.find((memory) => memory.governance_version)?.governance_version || "未记录";
}

function memoryGovernanceVersion(memory: MemoryEntry) {
  return memory.governance_version || "未记录";
}

function memoryGovernanceSource(memory: MemoryEntry) {
  return memory.governance_source || "未记录";
}

function memoryGovernanceAt(memory: MemoryEntry) {
  return memory.governance_at || "未记录";
}
