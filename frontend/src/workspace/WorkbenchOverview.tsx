import { ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";
import { MetricChip, SectionHeader } from "../ui/primitives";

type WorkbenchOverviewProps = {
  settings: SettingsResponse | null;
  statusLine: string;
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  currentTaskTitle: string;
  latestEvent?: RunEvent;
  confirmation: ConfirmationRequest | null;
  eventCount: number;
  logCount: number;
  onOpenAgent: () => void;
  onOpenLogsPage: () => void;
};

export function WorkbenchOverview(props: WorkbenchOverviewProps) {
  const model = buildOverviewModel(props);
  return (
    <article className="panel workbench-overview overview-page">
      <OverviewStatusSection model={model} />
      <OverviewTaskSection model={model} />
      <OverviewResumeSection model={model} />
      <OverviewHealthSection model={model} />
    </article>
  );
}

function buildOverviewModel(props: WorkbenchOverviewProps) {
  return {
    ...props,
    blockState: readBlockState(props),
    latestSummary: props.latestEvent?.summary || "当前还没有任务记录。",
    recentTaskTitle: props.eventCount > 0 ? props.currentTaskTitle : "等待第一条任务",
    recentStage: props.latestEvent?.stage || "等待运行",
    readyState: readReadyState(props),
    resumeLabel: props.eventCount > 0 ? "继续最近任务" : "进入任务页",
  };
}

function OverviewStatusSection(props: { model: ReturnType<typeof buildOverviewModel> }) {
  return (
    <section className="overview-status-section">
      <SectionHeader className="overview-copy" kicker="概览" kind="head" level="h1" title="恢复上下文" description={props.model.readyState} />
      <div className="status-ribbon">
        <MetricChip className="metric-chip" label="系统" value={props.model.statusLine} />
        <MetricChip className="metric-chip" label="工作区" value={props.model.settings?.workspace.name || "未加载"} />
      </div>
    </section>
  );
}

function OverviewTaskSection(props: { model: ReturnType<typeof buildOverviewModel> }) {
  return (
    <section className="overview-task-section">
      <SectionHeader kicker="最近任务" kind="head" title="最近任务" action={<span className="sidebar-chip-muted">{props.model.recentStage}</span>} />
      <p className="sidebar-title">{props.model.recentTaskTitle}</p>
      <p>{props.model.latestSummary}</p>
      <div className="detail-list">
        <Row label="运行" value={props.model.currentRunId || "等待生成"} />
        <Row label="会话" value={props.model.sessionId || "尚未创建"} />
      </div>
    </section>
  );
}

function OverviewResumeSection(props: { model: ReturnType<typeof buildOverviewModel> }) {
  return (
    <section className="overview-resume-section">
      <SectionHeader kicker="继续" kind="head" title="从这里继续" />
      <div className="overview-resume-actions">
        <button type="button" className="primary-action" onClick={props.model.onOpenAgent}>
          {props.model.resumeLabel}
        </button>
        <button type="button" className="secondary-button" onClick={props.model.onOpenLogsPage}>
          查看记录
        </button>
      </div>
      <p>{props.model.eventCount > 0 ? "继续主线程，或进入记录页复盘。" : "先进入任务页提交明确目标。"}</p>
    </section>
  );
}

function OverviewHealthSection(props: { model: ReturnType<typeof buildOverviewModel> }) {
  return (
    <section className="overview-health-section">
      <SectionHeader kicker="状态" kind="head" title="系统与风险" action={<span className="sidebar-chip">{props.model.blockState.label}</span>} />
      <div className="detail-list">
        <Row label="当前判断" value={props.model.blockState.text} />
        <Row label="连接" value={props.model.connectionLabel} />
        <Row label="记录留痕" value={`${props.model.logCount} 条`} />
      </div>
    </section>
  );
}

function Row(props: { label: string; value: string }) {
  return (
    <div className="sidebar-row">
      <strong>{props.label}</strong>
      <span title={props.value}>{props.value}</span>
    </div>
  );
}

function readReadyState(props: WorkbenchOverviewProps) {
  if (!props.settings) return "正在读取配置。";
  if (!props.settings.runtime_status.ok) return "Runtime 不可达。";
  if (props.confirmation) return "当前有待处理确认。";
  if (props.eventCount === 0) return "系统已就绪，可以开始任务。";
  return "系统已就绪，可以继续任务。";
}

function readBlockState(props: WorkbenchOverviewProps) {
  if (!props.settings) return { label: "等待配置", text: "基础配置尚未完成加载。" };
  if (!props.settings.runtime_status.ok) return { label: "运行阻塞", text: "Runtime 不可达，新的任务执行会失败。" };
  if (props.confirmation) return { label: "待确认", text: props.confirmation.action_summary };
  return { label: "无阻塞", text: "当前没有新的风险或阻塞。" };
}
