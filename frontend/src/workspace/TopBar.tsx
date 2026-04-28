import { useState } from "react";
import { getRunTone, RunState } from "../runtime/state";
import { SettingsResponse } from "../shared/contracts";

type TopBarProps = {
  settings: SettingsResponse | null;
  runState: RunState;
  statusLine: string;
  rightPanelOpen: boolean;
  onToggleRightPanel: () => void;
};

function readStoredTheme(): "dark" | "light" {
  const stored = localStorage.getItem("app-theme");
  if (stored === "light") return "light";
  return "dark";
}

function applyTheme(theme: "dark" | "light") {
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("app-theme", theme);
}

export function TopBar(props: TopBarProps) {
  const tone = getRunTone(props.runState);
  const [theme, setTheme] = useState<"dark" | "light">(readStoredTheme);

  function toggleTheme() {
    const next = theme === "dark" ? "light" : "dark";
    applyTheme(next);
    setTheme(next);
  }

  return (
    <div className="topbar-minimal">
      <div className="topbar-brand-minimal">
        <span className={`topbar-status-dot ${tone}`} aria-hidden="true" />
        <span>{props.settings?.app_name || "本地智能体"}</span>
      </div>
      <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
        <button
          type="button"
          className="topbar-panel-toggle"
          aria-label={theme === "dark" ? "切换到亮色模式" : "切换到暗色模式"}
          onClick={toggleTheme}
        >
          <span aria-hidden="true">{theme === "dark" ? "☀" : "☽"}</span>
        </button>
        <button
          type="button"
          className="topbar-panel-toggle"
          aria-label={props.rightPanelOpen ? "收起右侧面板" : "展开右侧面板"}
          aria-pressed={props.rightPanelOpen}
          onClick={props.onToggleRightPanel}
        >
          <span aria-hidden="true">{"☰"}</span>
        </button>
      </div>
    </div>
  );
}
