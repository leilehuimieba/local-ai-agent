import { SettingsResponse } from "../shared/contracts";
import { EmptyStateBlock, MetricChip, SectionHeader, StatusPill } from "../ui/primitives";
import { SettingsActionFeedback } from "./useSettings";

type StatusCardProps = {
  settings: SettingsResponse | null;
  bootstrapError: string | null;
  pendingAction: SettingsActionFeedback | null;
  actionError: SettingsActionFeedback | null;
  lastSuccess: SettingsActionFeedback | null;
};

export function StatusCard(props: StatusCardProps) {
  const model = buildStatusModel(props);
  return (
    <section className="status-card settings-status-card">
      <StatusHeader status={model.status} statusClass={model.statusClass} />
      {model.error ? <ErrorBlock title={model.error.title} body={model.error.body} advice={model.error.advice} /> : null}
      {props.pendingAction ? <ActionBlock tone="running" title={props.pendingAction.title} detail={props.pendingAction.detail} /> : null}
      {props.actionError ? <ActionBlock tone="failed" title={`${props.actionError.title}失败`} detail={props.actionError.detail} /> : null}
      {props.lastSuccess ? <ActionBlock tone="completed" title={props.lastSuccess.title} detail={props.lastSuccess.detail} /> : null}
      {!props.settings && !props.bootstrapError ? <EmptyHint text="正在读取基础设置。" /> : null}
      {props.settings ? <StatusContent settings={props.settings} /> : null}
    </section>
  );
}

function StatusHeader(props: { status: string; statusClass: string }) {
  return <SectionHeader className="status-card-header" kicker="环境" level="h2" title="环境状态" action={<StatusPill className={props.statusClass} label={props.status} />} />;
}

function StatusContent(props: { settings: SettingsResponse }) {
  return (
    <>
      <div className="status-ribbon">
        <RibbonItem label="运行时" value={readRuntimeLabel(props.settings)} />
        <RibbonItem label="模型" value={props.settings.model.display_name} />
        <RibbonItem label="模式" value={props.settings.mode} />
        <RibbonItem label="工作区" value={props.settings.workspace.name} />
      </div>
      <dl className="status-grid">
        <StatusItem label="应用" value={props.settings.app_name || "未提供"} />
        <StatusItem label="服务方" value={props.settings.model.provider_id || "未提供"} />
        <StatusItem label="工作区根" value={props.settings.workspace.root_path || "未提供"} />
        <StatusItem label="已授权目录" value={`${props.settings.approved_directories.length} 个`} />
        <StatusItem label="记忆策略" value={props.settings.memory_policy.enabled ? "已启用" : "未启用"} />
        <StatusItem label="运行时版本" value={props.settings.runtime_status.version || "未提供"} />
      </dl>
    </>
  );
}

function RibbonItem(props: { label: string; value: string }) {
  return <MetricChip label={props.label} value={props.value} />;
}

function StatusItem(props: { label: string; value: string }) {
  return (
    <div>
      <dt>{props.label}</dt>
      <dd>{props.value}</dd>
    </div>
  );
}

function ErrorBlock(props: { title: string; body: string; advice: string }) {
  return (
    <div className="error-block">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
      <p>{props.advice}</p>
    </div>
  );
}

function EmptyHint(props: { text: string }) {
  return <EmptyStateBlock compact title="正在读取设置" text={props.text} />;
}

function ActionBlock(props: { tone: "running" | "failed" | "completed"; title: string; detail: string }) {
  return (
    <div className={`settings-feedback settings-feedback-${props.tone}`} role="status" aria-live="polite">
      <strong>{props.title}</strong>
      <p>{props.detail}</p>
    </div>
  );
}

function buildStatusModel(props: StatusCardProps) {
  return {
    error: readStatusError(props.settings, props.bootstrapError),
    statusClass: readOverallStatusClass(props),
    status: readOverallStatus(props),
  };
}

function readRuntimeLabel(settings: SettingsResponse) {
  return settings.runtime_status.ok ? "可达" : "不可达";
}

function readOverallStatus(props: StatusCardProps) {
  if (props.bootstrapError || props.actionError) return "失败";
  if (props.pendingAction) return "处理中";
  if (props.lastSuccess) return "已完成";
  if (!props.settings) return "空闲";
  return props.settings.runtime_status.ok ? "已完成" : "已断开";
}

function readOverallStatusClass(props: StatusCardProps) {
  if (props.bootstrapError || props.actionError) return "status-failed";
  if (props.pendingAction) return "status-running";
  if (props.lastSuccess) return "status-completed";
  if (!props.settings) return "status-idle";
  return props.settings.runtime_status.ok ? "status-completed" : "status-disconnected";
}

function readStatusError(settings: SettingsResponse | null, bootstrapError: string | null) {
  if (bootstrapError) {
    return {
      advice: "检查运行时与本地配置后重新进入设置页。",
      body: bootstrapError,
      title: "设置加载失败",
    };
  }
  if (!settings || settings.runtime_status.ok) return null;
  return {
    advice: "先恢复运行时，再执行模型切换、导出或诊断动作。",
    body: "运行时当前不可达，依赖运行时的动作会失败。",
    title: "运行时连接异常",
  };
}
