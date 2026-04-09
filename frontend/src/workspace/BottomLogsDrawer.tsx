import { useEffect, useRef } from "react";
import { RunEvent } from "../shared/contracts";

type BottomLogsDrawerProps = {
  isOpen: boolean;
  events: RunEvent[];
  onClose: () => void;
};

export function BottomLogsDrawer({ isOpen, events, onClose }: BottomLogsDrawerProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current && isOpen) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [events, isOpen]);

  return (
    <div className={`logs-drawer ${isOpen ? "open" : ""}`} aria-hidden={!isOpen}>
      <div className="logs-drawer-header">
        Terminal Logs
        <button className="logs-drawer-close" onClick={onClose} aria-label="Close Logs">
          ×
        </button>
      </div>
      <div className="logs-drawer-body" ref={containerRef}>
        {events.length === 0 ? (
          <div className="logs-message-line" style={{ color: "#888" }}>No terminal logs to display.</div>
        ) : (
          events.map((event, i) => <LogLine key={event.event_id || i} event={event} />)
        )}
      </div>
    </div>
  );
}

function LogLine({ event }: { event: RunEvent }) {
  const timestamp = safeTimeLabel(event.timestamp);
  const label = event.event_type;
  const detail = event.result_summary || event.summary || event.detail || "";
  
  return (
    <div className="logs-message-line">
      <span className="logs-message-meta">[{timestamp}]</span>
      <strong style={{ color: "#569cd6", marginRight: "8px" }}>{label}</strong>
      <span>{detail}</span>
    </div>
  );
}

function safeTimeLabel(value?: string) {
  if (!value) return "";
  const date = parseTimestamp(value);
  if (Number.isNaN(date.getTime())) return "";
  return date.toLocaleTimeString();
}

function parseTimestamp(value: string) {
  const trimmed = value.trim();
  if (/^\d+$/.test(trimmed)) {
    const millis = Number(trimmed);
    return new Date(millis);
  }
  return new Date(trimmed);
}
