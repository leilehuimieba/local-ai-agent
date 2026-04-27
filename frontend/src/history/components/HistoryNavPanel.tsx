import { HistoryStatusFilter, HistoryTimeFilter } from "../../logs/useLogs";

export type HistoryNavPanelProps = {
  logCount: number;
  search: string;
  statusFilter: HistoryStatusFilter;
  timeFilter: HistoryTimeFilter;
  onSearchChange: (value: string) => void;
  onStatusFilterChange: (value: HistoryStatusFilter) => void;
  onTimeFilterChange: (value: HistoryTimeFilter) => void;
};

const STATUS_OPTIONS: { value: HistoryStatusFilter; label: string }[] = [
  { value: "all", label: "全部" },
  { value: "completed", label: "完成" },
  { value: "failed", label: "失败" },
  { value: "running", label: "进行中" },
];

const TIME_OPTIONS: { value: HistoryTimeFilter; label: string }[] = [
  { value: "all", label: "全部时间" },
  { value: "today", label: "今日" },
  { value: "week", label: "近7天" },
  { value: "month", label: "近30天" },
];

export function HistoryNavPanel(props: HistoryNavPanelProps) {
  return (
    <section className="task-nav-panel history-nav-panel">
      <header className="task-nav-panel-head">
        <strong>工作历史</strong>
        <span>全局筛选 · 共 {props.logCount} 条</span>
      </header>
      <input
        className="task-nav-search"
        type="search"
        value={props.search}
        placeholder="搜索任务与历史"
        aria-label="搜索任务与历史"
        autoComplete="off"
        onChange={(event) => props.onSearchChange(event.target.value)}
      />
      <div className="history-filter-group">
        <span className="history-filter-label">状态</span>
        <div className="history-filter-chips">
          {STATUS_OPTIONS.map((opt) => (
            <button
              key={opt.value}
              type="button"
              className={props.statusFilter === opt.value ? "history-filter-chip active" : "history-filter-chip"}
              onClick={() => props.onStatusFilterChange(opt.value)}
            >
              {opt.label}
            </button>
          ))}
        </div>
      </div>
      <div className="history-filter-group">
        <span className="history-filter-label">时间</span>
        <div className="history-filter-chips">
          {TIME_OPTIONS.map((opt) => (
            <button
              key={opt.value}
              type="button"
              className={props.timeFilter === opt.value ? "history-filter-chip active" : "history-filter-chip"}
              onClick={() => props.onTimeFilterChange(opt.value)}
            >
              {opt.label}
            </button>
          ))}
        </div>
      </div>
    </section>
  );
}
