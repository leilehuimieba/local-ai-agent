import { LogEntry } from "../../shared/contracts";
import { buildLogResult, getFocusLogDetails, getHistoryNextSteps, getLearningContinuation, getReplaySummary } from "../viewModel";
import { EmptyStateBlock, MetaGrid, SectionHeader } from "../../ui/primitives";
import { readLogType, readMemoryActivityLabel, readMemoryFacetLabel, readMemoryGovernanceLabel, readReviewTypeLabel } from "../logType";

const DETAIL_SECTION_ORDER = ["basic", "summary", "learning", "replay", "risk", "metadata", "context", "verification"] as const;

export function HistoryDetailRail(props: { focusLog: LogEntry | null }) {
  return (
    <section className="page-section detail-rail logs-detail-rail">
      <SectionHeader title="详情栏" />
      {buildDetailSections(props.focusLog).map((section) => section.node)}
    </section>
  );
}

function buildDetailSections(focusLog: LogEntry | null) {
  return DETAIL_SECTION_ORDER.map((key) => createDetailSection(key, focusLog));
}

function createDetailSection(key: typeof DETAIL_SECTION_ORDER[number], focusLog: LogEntry | null) {
  if (key === "basic") return { key, node: <BasicInfoSection key={key} focusLog={focusLog} /> };
  if (key === "summary") return { key, node: <SummarySection key={key} focusLog={focusLog} /> };
  if (key === "learning") return { key, node: <LearningSection key={key} focusLog={focusLog} /> };
  if (key === "replay") return { key, node: <ReplaySection key={key} focusLog={focusLog} /> };
  if (key === "risk") return { key, node: <RiskSection key={key} focusLog={focusLog} /> };
  if (key === "metadata") return { key, node: <MetadataSection key={key} focusLog={focusLog} /> };
  if (key === "context") return { key, node: <ContextSection key={key} focusLog={focusLog} /> };
  return { key, node: <VerificationSection key={key} focusLog={focusLog} /> };
}

function BasicInfoSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return <EmptyFocusCard />;
  return (
    <section className="detail-card">
      <strong>基本信息</strong>
      <MetaGrid items={buildBasicInfoRows(props.focusLog)} />
    </section>
  );
}

function SummarySection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  const result = buildLogResult(props.focusLog);
  return (
    <section className="detail-card">
      <strong>{readSummaryTitle(props.focusLog)}</strong>
      <p>{result.summary}</p>
      {result.sections.map((item) => <p key={`${item.kind}-${item.text}`} className="timeline-detail">{`${item.title}：${item.text}`}</p>)}
    </section>
  );
}

function LearningSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  const continuation = getLearningContinuation(props.focusLog);
  if (!continuation) return null;
  return (
    <section className="detail-card learning-continuation-card">
      <strong>学习续接</strong>
      <div className="detail-list learning-continuation-list">
        <LearningRow label="当前学习主题" value={continuation.topic} />
        <LearningRow label="当前掌握情况" value={continuation.grasp} />
        <LearningRow label="待巩固 / 待补" value={continuation.review} />
        <LearningRow label="下一步学习建议" value={continuation.nextStep} />
      </div>
    </section>
  );
}

function ReplaySection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  const replay = getReplaySummary(props.focusLog);
  return (
    <section className="detail-card">
      <strong>复盘拆解</strong>
      <p className="timeline-detail">{`当前结论：${replay.conclusion}`}</p>
      <p className="timeline-detail">{`执行依据：${replay.evidence}`}</p>
      <p className="timeline-detail">{`验证结果：${replay.verification}`}</p>
      <p className="timeline-detail">{`下一步：${replay.nextStep}`}</p>
    </section>
  );
}

function RiskSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  return (
    <section className="detail-card muted-card">
      <strong>风险与错误</strong>
      <MetaGrid items={buildRiskRows(props.focusLog)} />
      {buildRiskParagraphs(props.focusLog).map((item) => <p key={item} className="timeline-detail">{item}</p>)}
    </section>
  );
}

function MetadataSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  return (
    <section className="detail-card muted-card">
      <strong>关键 Metadata</strong>
      <MetaGrid items={buildMetadataRows(props.focusLog)} />
      {buildMetadataParagraphs(props.focusLog).map((item) => <p key={item} className="timeline-detail">{item}</p>)}
    </section>
  );
}

function ContextSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  const details = getFocusLogDetails(props.focusLog);
  return (
    <section className="detail-card muted-card">
      <strong>上下文快照</strong>
      <MetaGrid items={buildContextMetaRows(details)} />
      {buildContextRows(details).map((item) => <p key={item} className="timeline-detail">{item}</p>)}
    </section>
  );
}

function VerificationSection(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return null;
  const details = getFocusLogDetails(props.focusLog);
  return (
    <section className="detail-card muted-card">
      <strong>验证与后续</strong>
      <MetaGrid items={buildVerificationRows(props.focusLog, details)} />
      <ul>{getHistoryNextSteps(props.focusLog).map((item) => <li key={item}>{item}</li>)}</ul>
    </section>
  );
}

function LearningRow(props: { label: string; value: string }) {
  return (
    <div className="sidebar-row">
      <strong>{props.label}</strong>
      <span title={props.value}>{props.value}</span>
    </div>
  );
}

function EmptyFocusCard() {
  return (
    <section className="detail-card">
      <EmptyStateBlock title="基本信息" text="选择左侧记录后，这里会显示详情和后续建议。" />
    </section>
  );
}

function buildBasicInfoRows(log: LogEntry) {
  const details = getFocusLogDetails(log);
  return [
    { label: "记录类型", value: readReviewTypeLabel(readLogType(log)) },
    { label: "正式分类", value: details.category },
    { label: "记录来源", value: readLogSource(log) },
    { label: "事件类型", value: details.eventType },
    { label: "阶段", value: log.stage || "无阶段" },
    { label: "工具名", value: readToolName(log) },
    { label: "错误码", value: log.error?.error_code || "无" },
  ];
}

function buildRiskRows(log: LogEntry) {
  return [
    { label: "级别", value: log.level },
    { label: "风险等级", value: log.risk_level || "无" },
    { label: "确认", value: log.confirmation_id || "无" },
    { label: "来源", value: log.source || "runtime" },
  ];
}

function buildRiskParagraphs(log: LogEntry) {
  return [
    log.error ? `错误：${log.error.error_code} / ${log.error.message}` : "",
    log.metadata?.failure_recovery_hint ? `恢复建议：${log.metadata.failure_recovery_hint}` : "",
  ].filter(Boolean) as string[];
}

function buildMetadataRows(log: LogEntry) {
  const details = getFocusLogDetails(log);
  return [
    { label: "Run ID", value: log.run_id },
    { label: "Session", value: log.session_id },
    { label: "分类", value: log.record_type || details.category },
    { label: "记忆类型", value: readDetailMemoryFacet(log) },
    { label: "治理状态", value: readDetailMemoryGovernance(log) },
    { label: "最近动作", value: readDetailMemoryActivity(log) },
    { label: "来源事件", value: log.event_type || "未附带" },
    { label: "证据路径", value: log.artifact_path || "未附带" },
    { label: "确认链步骤", value: readMetadataValue(log, "confirmation_chain_step") },
    { label: "确认决策", value: readMetadataValue(log, "confirmation_decision") },
    { label: "恢复策略", value: readMetadataValue(log, "confirmation_resume_strategy") },
    { label: "Checkpoint", value: readMetadataValue(log, "checkpoint_id") },
    { label: "工具耗时(ms)", value: readMetadataValue(log, "tool_elapsed_ms") },
    { label: "治理版本", value: readMetadataValue(log, "governance_version") },
    { label: "治理来源", value: readMetadataValue(log, "governance_source") },
    { label: "治理状态", value: readMetadataValue(log, "governance_status") },
    { label: "治理时间", value: readMetadataValue(log, "governance_at") },
    { label: "记录来源", value: readLogSource(log) },
    { label: "记录时间", value: log.timestamp },
  ];
}

function buildMetadataParagraphs(log: LogEntry) {
  return [
    readAuditChainSummary(log),
    readMetadataValue(log, "governance_reason", false),
    readMetadataValue(log, "archive_reason", false),
  ].filter(Boolean) as string[];
}

function buildContextMetaRows(details: ReturnType<typeof getFocusLogDetails>) {
  return [
    { label: "工作区", value: details.workspace || "未附带" },
    { label: "来源类型", value: details.sourceType || "未附带" },
    { label: "缓存", value: details.cache || "未附带" },
  ];
}

function buildVerificationRows(log: LogEntry, details: ReturnType<typeof getFocusLogDetails>) {
  return [
    { label: "验证摘要", value: details.verification || "未附带" },
    { label: "完成状态", value: log.completion_status || "未附带" },
    { label: "完成判定", value: details.completion || "未附带" },
    { label: "失败教训", value: readLessonHint(log) },
  ];
}

function buildContextRows(details: ReturnType<typeof getFocusLogDetails>) {
  return [
    details.memoryDigest ? `记忆摘要：${details.memoryDigest}` : "",
    details.knowledgeDigest ? `知识摘要：${details.knowledgeDigest}` : "",
    details.reasoning ? `思考摘要：${details.reasoning}` : "",
  ].filter(Boolean) as string[];
}

function logLikeMemory(log: LogEntry) {
  return {
    archived: log.record_type === "archived_memory",
    event_type: log.event_type,
    governance_status: log.metadata?.governance_status,
    kind: log.record_type || log.category,
    memory_action: log.metadata?.memory_action,
    metadata: log.metadata,
    reason: log.metadata?.reason || log.detail || log.summary,
    source_type: log.source_type,
    summary: log.summary,
    title: log.metadata?.task_title || log.summary,
    verified: log.verification_snapshot?.passed,
  };
}

function readToolName(log: LogEntry) {
  return log.tool_call_snapshot?.tool_name || log.tool_name || "无";
}

function readSummaryTitle(log: LogEntry) {
  const type = readLogType(log);
  if (type === "result") return "结果摘要";
  if (type === "error") return "失败摘要";
  if (type === "confirmation") return "确认摘要";
  if (type === "memory") return "记忆摘要";
  return "摘要与说明";
}

function readLogSource(log: LogEntry) {
  return log.source_type || log.metadata?.source_type || log.source || "runtime";
}

function readMetadataValue(log: LogEntry, key: string, withFallback = true) {
  const value = log.metadata?.[key] || "";
  return value || (withFallback ? "未附带" : "");
}

function readAuditChainSummary(log: LogEntry) {
  const step = readMetadataValue(log, "confirmation_chain_step", false);
  if (!step) return "";
  const decision = readMetadataValue(log, "confirmation_decision", false);
  const strategy = readMetadataValue(log, "confirmation_resume_strategy", false);
  const checkpoint = readMetadataValue(log, "checkpoint_id", false);
  return `确认链：步骤=${step}；决策=${decision || "未附带"}；策略=${strategy || "未附带"}；checkpoint=${checkpoint || "未附带"}`;
}

function readLessonHint(log: LogEntry) {
  if (readDetailMemoryFacet(log) !== "失败教训") return "无";
  return log.metadata?.reason || log.detail || log.summary;
}

function readDetailMemoryFacet(log: LogEntry) {
  if (readLogType(log) !== "memory") return "无";
  return readMemoryFacetLabel(logLikeMemory(log));
}

function readDetailMemoryGovernance(log: LogEntry) {
  if (readLogType(log) !== "memory") return "无";
  return readMemoryGovernanceLabel(logLikeMemory(log));
}

function readDetailMemoryActivity(log: LogEntry) {
  if (readLogType(log) !== "memory") return "无";
  return readMemoryActivityLabel(logLikeMemory(log));
}
