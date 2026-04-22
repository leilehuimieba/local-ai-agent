import { HistoryStats, ReviewFocusFilter } from "../useHistoryReview";
import { MetricChip, SectionHeader } from "../../ui/primitives";
import { AuditFilter } from "../auditSignals";
import { readReviewTypeLabel, ReviewLogType } from "../logType";

export function HistoryLogsHeader(props: {
  logCount: number;
  sessionCount: number;
}) {
  return (
    <SectionHeader
      kind="page"
      kicker="Review"
      level="h2"
      title="调查记录工作区"
      description="把稳定记录、会话规模与复盘入口收在同一视图里，先看收口状态，再继续下钻。"
      action={
        <div className="page-header-meta">
          <MetricChip label="记录" value={`${props.logCount} 条`} />
          <MetricChip label="会话" value={`${props.sessionCount} 个`} />
          <MetricChip label="模式" value="Logs" />
        </div>
      }
    />
  );
}

export function HistoryLogsSummary(props: {
  total: number;
  stats: HistoryStats;
}) {
  return (
    <section className="logs-summary-strip" aria-label="记录摘要">
      <HistorySummaryCard label="稳定记录" value={`${props.total} 条`} tone="neutral" />
      <HistorySummaryCard label="异常收口" value={`${props.stats.errorCount} 条`} tone="danger" />
      <HistorySummaryCard label="待确认" value={`${props.stats.confirmationCount} 条`} tone="warning" />
      <HistorySummaryCard label="工具触达" value={`${props.stats.toolCount} 个`} tone="calm" />
    </section>
  );
}

export function HistoryFilterToolbar(props: {
  query: string;
  focusFilter: ReviewFocusFilter;
  typeFilter: string;
  auditFilter: AuditFilter;
  levelFilter: string;
  onlyErrors: boolean;
  onlyConfirmations: boolean;
  resultCount: number;
  onQueryChange: (value: string) => void;
  onFocusFilterChange: (value: ReviewFocusFilter) => void;
  onTypeFilterChange: (value: string) => void;
  onAuditFilterChange: (value: AuditFilter) => void;
  onLevelFilterChange: (value: string) => void;
  onOnlyErrorsChange: (value: boolean) => void;
  onOnlyConfirmationsChange: (value: boolean) => void;
}) {
  return (
    <section className="logs-filter-toolbar" aria-label="记录筛选">
      <FilterToolbarHeader resultCount={props.resultCount} />
      <FilterToolbarControls props={props} />
      <FilterToolbarNote />
    </section>
  );
}

function FilterToolbarHeader(props: { resultCount: number }) {
  return (
    <SectionHeader
      kicker="Filter"
      title="复盘筛选台"
      description="先定焦点，再缩小范围，最后进入时间线和详情栏。"
      action={<MetricChip label="结果" value={`${props.resultCount} 条`} />}
    />
  );
}

function FilterToolbarControls(props: {
  props: Parameters<typeof HistoryFilterToolbar>[0];
}) {
  return (
    <div className="filter-toolbar">
      <HistoryFocusGroupBar value={props.props.focusFilter} onChange={props.props.onFocusFilterChange} />
      <HistorySearchBox value={props.props.query} onChange={props.props.onQueryChange} />
      <HistoryFilterSelect fieldName="history_type_filter" label="类型" value={props.props.typeFilter} options={buildTypeOptions()} onChange={props.props.onTypeFilterChange} />
      <HistoryFilterSelect fieldName="history_audit_filter" label="审计" value={props.props.auditFilter} options={buildAuditOptions()} onChange={readAuditChange(props.props.onAuditFilterChange)} />
      <HistoryFilterSelect fieldName="history_level_filter" label="级别" value={props.props.levelFilter} options={buildLevelOptions()} onChange={props.props.onLevelFilterChange} />
      <HistoryFilterToggle name="history_only_errors" checked={props.props.onlyErrors} label="仅错误" note="真实可用" onChange={props.props.onOnlyErrorsChange} />
      <HistoryFilterToggle name="history_only_confirmations" checked={props.props.onlyConfirmations} label="仅确认" note="真实可用" onChange={props.props.onOnlyConfirmationsChange} />
    </div>
  );
}

function FilterToolbarNote() {
  return (
    <div className="logs-filter-note">
      <strong>建议顺序</strong>
      <p>先选焦点分组，再叠加类型、审计和级别过滤，避免时间线噪音回升。</p>
    </div>
  );
}

function HistoryFocusGroupBar(props: {
  value: ReviewFocusFilter;
  onChange: (value: ReviewFocusFilter) => void;
}) {
  return (
    <div className="history-focus-bar" role="group" aria-label="复盘焦点分组">
      {buildFocusOptions().map((item) => (
        <button
          key={item.value}
          type="button"
          className={props.value === item.value ? "focus-chip active" : "focus-chip"}
          onClick={() => props.onChange(item.value)}
        >
          <strong>{item.label}</strong>
          <span>{item.note}</span>
        </button>
      ))}
    </div>
  );
}

function HistoryFilterSelect(props: {
  fieldName: string;
  label: string;
  value: string;
  options: Array<{ value: string; label: string }>;
  onChange: (value: string) => void;
}) {
  return (
    <label className="filter-select">
      <span>{props.label}</span>
      <select id={props.fieldName} name={props.fieldName} value={props.value} onChange={(event) => props.onChange(event.target.value)}>
        {props.options.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
      </select>
    </label>
  );
}

function HistorySearchBox(props: {
  value: string;
  onChange: (value: string) => void;
}) {
  return (
    <label className="search-placeholder logs-search">
      <span>搜索</span>
      <input
        id="history_search_query"
        name="history_search_query"
        value={props.value}
        placeholder="搜索摘要、工具、来源、Run ID"
        onChange={(event) => props.onChange(event.target.value)}
      />
    </label>
  );
}

function HistoryFilterToggle(props: {
  name: string;
  label: string;
  note: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <label className="toggle-tile filter-toggle">
      <input
        id={props.name}
        name={props.name}
        type="checkbox"
        checked={props.checked}
        onChange={(event) => props.onChange(event.target.checked)}
      />
      <div>
        <strong>{props.label}</strong>
        <span className="toggle-state">{props.note}</span>
      </div>
    </label>
  );
}

function HistorySummaryCard(props: {
  label: string;
  value: string;
  tone: "neutral" | "danger" | "warning" | "calm";
}) {
  return (
    <div className={`summary-card tone-${props.tone}`}>
      <span>{props.label}</span>
      <strong>{props.value}</strong>
    </div>
  );
}

function buildLevelOptions() {
  return [
    { value: "all", label: "全部级别" },
    { value: "error", label: "错误" },
    { value: "warn", label: "警告" },
    { value: "info", label: "信息" },
  ];
}

function buildTypeOptions() {
  const types: ReviewLogType[] = ["result", "tool", "memory", "verification", "confirmation", "error", "system"];
  return [{ value: "all", label: "全部类型" }, ...types.map((type) => ({ value: type, label: readReviewTypeLabel(type) }))];
}

function buildAuditOptions() {
  return [
    { value: "all", label: "全部审计" },
    { value: "confirmation_chain", label: "确认链" },
    { value: "tool_elapsed", label: "工具耗时" },
    { value: "governance", label: "治理字段" },
  ];
}

function readAuditChange(onChange: (value: AuditFilter) => void) {
  return (value: string) => onChange(value as AuditFilter);
}

function buildFocusOptions(): Array<{ value: ReviewFocusFilter; label: string; note: string }> {
  return [
    { value: "all", label: "全部", note: "整体复盘" },
    { value: "result", label: "结果", note: "看结论与输出" },
    { value: "risk", label: "风险", note: "看失败与确认" },
    { value: "verification", label: "验证", note: "看验证证据" },
    { value: "governance", label: "治理", note: "看沉淀与治理" },
  ];
}
