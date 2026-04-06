import { HistoryStats } from "../useHistoryReview";
import { MetricChip, SectionHeader } from "../../ui/primitives";
import { readReviewTypeLabel, ReviewLogType } from "../logType";

export function HistoryLogsHeader(props: {
  logCount: number;
  sessionCount: number;
}) {
  return (
    <SectionHeader
      kind="page"
      kicker="记录"
      level="h2"
      title="调查与复盘"
      action={
        <div className="page-header-meta">
          <MetricChip label="记录" value={`${props.logCount} 条`} />
          <MetricChip label="会话" value={`${props.sessionCount} 个`} />
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
      <HistorySummaryCard label="总记录" value={`${props.total} 条`} tone="neutral" />
      <HistorySummaryCard label="错误" value={`${props.stats.errorCount} 条`} tone="danger" />
      <HistorySummaryCard label="确认" value={`${props.stats.confirmationCount} 条`} tone="warning" />
      <HistorySummaryCard label="工具" value={`${props.stats.toolCount} 个`} tone="calm" />
    </section>
  );
}

export function HistoryFilterToolbar(props: {
  query: string;
  typeFilter: string;
  levelFilter: string;
  onlyErrors: boolean;
  onlyConfirmations: boolean;
  resultCount: number;
  onQueryChange: (value: string) => void;
  onTypeFilterChange: (value: string) => void;
  onLevelFilterChange: (value: string) => void;
  onOnlyErrorsChange: (value: boolean) => void;
  onOnlyConfirmationsChange: (value: boolean) => void;
}) {
  return (
    <section className="logs-filter-toolbar" aria-label="记录筛选">
      <FilterToolbarHeader resultCount={props.resultCount} />
      <FilterToolbarControls props={props} />
    </section>
  );
}

function FilterToolbarHeader(props: { resultCount: number }) {
  return <SectionHeader title="筛选" action={<MetricChip label="结果" value={`${props.resultCount} 条`} />} />;
}

function FilterToolbarControls(props: {
  props: Parameters<typeof HistoryFilterToolbar>[0];
}) {
  return (
    <div className="filter-toolbar">
      <HistorySearchBox value={props.props.query} onChange={props.props.onQueryChange} />
      <HistoryFilterSelect label="类型" value={props.props.typeFilter} options={buildTypeOptions()} onChange={props.props.onTypeFilterChange} />
      <HistoryFilterSelect label="级别" value={props.props.levelFilter} options={buildLevelOptions()} onChange={props.props.onLevelFilterChange} />
      <HistoryFilterToggle checked={props.props.onlyErrors} label="仅错误" note="真实可用" onChange={props.props.onOnlyErrorsChange} />
      <HistoryFilterToggle checked={props.props.onlyConfirmations} label="仅确认" note="真实可用" onChange={props.props.onOnlyConfirmationsChange} />
    </div>
  );
}

function HistoryFilterSelect(props: {
  label: string;
  value: string;
  options: Array<{ value: string; label: string }>;
  onChange: (value: string) => void;
}) {
  return (
    <label className="filter-select">
      <span>{props.label}</span>
      <select value={props.value} onChange={(event) => props.onChange(event.target.value)}>
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
        value={props.value}
        placeholder="搜索摘要、工具、来源、Run ID"
        onChange={(event) => props.onChange(event.target.value)}
      />
    </label>
  );
}

function HistoryFilterToggle(props: {
  label: string;
  note: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <label className="toggle-tile filter-toggle">
      <input
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
