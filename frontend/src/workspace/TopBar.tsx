import { getRunTone, RunState } from "../runtime/state";
import { SettingsResponse } from "../shared/contracts";

type TopBarProps = {
  settings: SettingsResponse | null;
  runState: RunState;
  statusLine: string;
  rightPanelOpen: boolean;
  onToggleRightPanel: () => void;
};

export function TopBar(props: TopBarProps) {
  const tone = getRunTone(props.runState);
  return (
    <div className="topbar-minimal">
      <div className="topbar-brand-minimal">
        <span className={`topbar-status-dot ${tone}`} aria-hidden="true" />
        <span>{props.settings?.app_name || "本地智能体"}</span>
      </div>
      <button
        type="button"
        className="topbar-panel-toggle"
        aria-label={props.rightPanelOpen ? "收起右侧面板" : "展开右侧面板"}
        aria-pressed={props.rightPanelOpen}
        onClick={props.onToggleRightPanel}
      >
        <span aria-hidden="true">{props.rightPanelOpen ? "\u2630" : "\u2630"}</span>
      </button>
    </div>
  );
}
