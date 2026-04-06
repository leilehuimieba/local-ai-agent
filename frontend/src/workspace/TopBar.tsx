import { useEffect, useRef, useState } from "react";

import { RunState, getRunTone } from "../runtime/state";
import { SettingsResponse } from "../shared/contracts";

type ViewId = "home" | "agent" | "logs" | "settings";

type TopBarProps = {
  settings: SettingsResponse | null;
  currentView: ViewId;
  statusLine: string;
  runState: RunState;
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  onModelChange: (modelId: string) => void;
  onViewChange: (view: ViewId) => void;
  onToggleLogsDrawer: () => void;
};

const NAV_ITEMS = [
  { id: "home", label: "概览" },
  { id: "agent", label: "任务" },
  { id: "logs", label: "记录" },
  { id: "settings", label: "设置" },
] as const;

export function TopBar(props: TopBarProps) {
  return (
    <header className="topbar topbar-compact">
      <BrandBlock settings={props.settings} runState={props.runState} statusLine={props.statusLine} />
      <ModelSwitcher settings={props.settings} onModelChange={props.onModelChange} />
      <UtilityBlock
        connectionLabel={props.connectionLabel}
        currentRunId={props.currentRunId}
        currentView={props.currentView}
        onViewChange={props.onViewChange}
        sessionId={props.sessionId}
        onToggleLogsDrawer={props.onToggleLogsDrawer}
      />
    </header>
  );
}

function BrandBlock(props: {
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
        <span>{props.settings?.workspace.name || "等待工作区"} · {props.statusLine}</span>
      </div>
      <span className={`status-badge status-${tone}`}>{props.statusLine}</span>
    </section>
  );
}

function ModelSwitcher(props: {
  settings: SettingsResponse | null;
  onModelChange: (modelId: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement | null>(null);
  useDropdownDismiss(rootRef, () => setOpen(false));
  const items = readModelItems(props.settings);
  const current = props.settings?.model.display_name || "未加载模型";
  const busy = !props.settings;
  return (
    <div className="topbar-model-switcher" ref={rootRef}>
      <button type="button" className={open ? "model-pill open" : "model-pill"} disabled={busy} aria-expanded={open} aria-haspopup="menu" onClick={() => setOpen((value) => !value)}>
        <span className="model-pill-status" aria-hidden="true" />
        <span className="model-pill-label">{current}</span>
        <span className="model-pill-chevron" aria-hidden="true">⌄</span>
      </button>
      {open ? <ModelMenu items={items} onClose={() => setOpen(false)} onModelChange={props.onModelChange} /> : null}
    </div>
  );
}

function ModelMenu(props: {
  items: Array<{ id: string; label: string; meta: string; disabled: boolean; active: boolean }>;
  onClose: () => void;
  onModelChange: (modelId: string) => void;
}) {
  return (
    <div className="model-dropdown" role="menu" aria-label="切换模型">
      {props.items.map((item) => (
        <button
          key={item.id}
          type="button"
          role="menuitemradio"
          aria-checked={item.active}
          className={readMenuItemClass(item)}
          disabled={item.disabled}
          onClick={() => handleModelSelect(item, props.onModelChange, props.onClose)}
        >
          <span className="model-option-copy">
            <strong>{item.label}</strong>
            <span>{item.meta}</span>
          </span>
          {item.active ? <span className="model-option-check">当前</span> : null}
        </button>
      ))}
    </div>
  );
}

function UtilityBlock(props: {
  connectionLabel: string;
  sessionId: string;
  currentRunId: string;
  currentView: ViewId;
  onViewChange: (view: ViewId) => void;
  onToggleLogsDrawer: () => void;
}) {
  return (
    <section className="topbar-group topbar-utility">
      <ContextItem label="连接" value={props.connectionLabel} />
      <ContextItem label="会话" value={shortId(props.sessionId, "未创建")} />
      <ContextItem label="运行" value={shortId(props.currentRunId, "空闲")} />
      <button type="button" className="nav-button" style={{ marginLeft: "8px", fontWeight: "bold" }} onClick={props.onToggleLogsDrawer}>
        &gt;_ Logs
      </button>
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

function useDropdownDismiss(
  rootRef: React.RefObject<HTMLDivElement | null>,
  onClose: () => void,
) {
  useEffect(() => {
    const handlePointerDown = (event: PointerEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) onClose();
    };
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("pointerdown", handlePointerDown);
    window.addEventListener("keydown", handleEscape);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown);
      window.removeEventListener("keydown", handleEscape);
    };
  }, [onClose, rootRef]);
}

function readModelItems(settings: SettingsResponse | null) {
  if (!settings) return [];
  return settings.available_models.map((item) => ({
    id: `${item.provider_id}:${item.model_id}`,
    label: item.display_name,
    meta: item.provider_id || "未标注 Provider",
    disabled: !item.available || !item.enabled,
    active: isCurrentModel(settings, item),
  }));
}

function handleModelSelect(
  item: { id: string; disabled: boolean; active: boolean },
  onModelChange: (modelId: string) => void,
  onClose: () => void,
) {
  if (!item.disabled && !item.active) onModelChange(item.id);
  onClose();
}

function readMenuItemClass(item: { disabled: boolean; active: boolean }) {
  if (item.disabled) return "model-option disabled";
  return item.active ? "model-option active" : "model-option";
}

function isCurrentModel(
  settings: SettingsResponse,
  item: SettingsResponse["available_models"][number],
) {
  return settings.model.model_id === item.model_id
    && settings.model.provider_id === item.provider_id;
}

function shortId(value: string, fallback: string) {
  if (!value) return fallback;
  if (value.length <= 12) return value;
  return `${value.slice(0, 6)}...${value.slice(-4)}`;
}
