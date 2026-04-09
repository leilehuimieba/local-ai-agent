import { RunState, getRunTone } from "../runtime/state";
import { SettingsResponse } from "../shared/contracts";

type ViewId = "home" | "task" | "logs" | "settings";

type TopBarProps = {
  settings: SettingsResponse | null;
  currentView: ViewId;
  statusLine: string;
  runState: RunState;
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  homeStateHint: string;
  onOpenHomeStart: () => void;
  onViewChange: (view: ViewId) => void;
};

const NAV_ITEMS = [
  { id: "home", label: "首页" },
  { id: "task", label: "任务" },
  { id: "logs", label: "记录" },
  { id: "settings", label: "设置" },
] as const;

export function TopBar(props: TopBarProps) {
  return (
    <header className="topbar topbar-compact">
      <BrandBlock currentView={props.currentView} settings={props.settings} runState={props.runState} statusLine={props.statusLine} />
      <ConfigSummary
        currentView={props.currentView}
        homeStateHint={props.homeStateHint}
        onOpenHomeStart={props.onOpenHomeStart}
        onViewChange={props.onViewChange}
        settings={props.settings}
      />
      <UtilityBlock
        connectionLabel={props.connectionLabel}
        currentRunId={props.currentRunId}
        currentView={props.currentView}
        onViewChange={props.onViewChange}
        sessionId={props.sessionId}
      />
    </header>
  );
}

function BrandBlock(props: {
  currentView: ViewId;
  settings: SettingsResponse | null;
  runState: RunState;
  statusLine: string;
}) {
  const tone = getRunTone(props.runState);
  return (
    <section className="topbar-group topbar-brand">
      <span className="topbar-brand-mark" aria-hidden="true" />
      <div className="topbar-copy">
        <strong>{props.settings?.app_name || "本地智能体"}</strong>
        <span>{`${props.settings?.workspace.name || "等待工作区"} · ${readViewHint(props.currentView)}`}</span>
      </div>
      <span className={`status-badge status-${tone}`}>{props.statusLine}</span>
    </section>
  );
}

function ConfigSummary(props: {
  settings: SettingsResponse | null;
  currentView: ViewId;
  homeStateHint: string;
  onOpenHomeStart: () => void;
  onViewChange: (view: ViewId) => void;
}) {
  return (
    <section className="topbar-group topbar-summary">
      <ContextItem label="模型" value={props.settings?.model.display_name || "未加载"} />
      <ContextItem label="模式" value={readModeLabel(props.settings?.mode)} />
      <ContextItem label="首页" value={props.homeStateHint} />
      {props.currentView !== "home" ? <button type="button" className="secondary-button" onClick={props.onOpenHomeStart}>新建任务</button> : null}
      {props.currentView !== "settings" ? <button type="button" className="secondary-button" onClick={() => props.onViewChange("settings")}>前往设置调整</button> : null}
    </section>
  );
}

function UtilityBlock(props: {
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  currentView: ViewId;
  onViewChange: (view: ViewId) => void;
}) {
  return (
    <section className="topbar-group topbar-utility">
      <ContextItem label="连接" value={props.connectionLabel} />
      <ContextItem label="会话" value={shortId(props.sessionId, "未创建")} />
      <ContextItem label="运行" value={shortId(props.currentRunId, "空闲")} />
      <nav className="topbar-nav topbar-nav-compact" aria-label="主导航">
        {NAV_ITEMS.map((item) => (
          <button key={item.id} type="button" aria-current={item.id === props.currentView ? "page" : undefined} className={item.id === props.currentView ? "nav-button active" : "nav-button"} onClick={() => props.onViewChange(item.id)}>
            {item.label}
          </button>
        ))}
      </nav>
    </section>
  );
}

function ContextItem(props: { label: string; value: string }) {
  return (
    <div className="topbar-metric">
      <span className="topbar-kicker">{props.label}</span>
      <strong title={props.value}>{props.value}</strong>
    </div>
  );
}

function shortId(value: string, fallback: string) {
  if (!value) return fallback;
  if (value.length <= 12) return value;
  return `${value.slice(0, 6)}...${value.slice(-4)}`;
}

function readModeLabel(mode?: string) {
  if (mode === "observe") return "观察模式";
  if (mode === "full_access") return "全权限模式";
  if (mode === "standard") return "标准模式";
  return "未加载";
}

function readViewHint(view: ViewId) {
  if (view === "home") return "首页开始与恢复";
  if (view === "task") return "任务页主工作区";
  if (view === "logs") return "记录页复盘";
  return "设置页控制入口";
}
