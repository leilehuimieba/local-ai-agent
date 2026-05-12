import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useSessionEventStream, type RunEvent } from "../useSessionEventStream";

let instances: MockEventSource[] = [];

class MockEventSource {
  url: string;
  onopen: (() => void) | null = null;
  onerror: (() => void) | null = null;
  private listeners: Map<string, Array<(event: MessageEvent<string>) => void>> = new Map();

  constructor(url: string) {
    this.url = url;
    instances.push(this);
  }

  addEventListener(type: string, handler: (event: MessageEvent<string>) => void) {
    if (!this.listeners.has(type)) this.listeners.set(type, []);
    this.listeners.get(type)!.push(handler);
  }

  removeEventListener(type: string, handler: (event: MessageEvent<string>) => void) {
    const list = this.listeners.get(type) || [];
    const idx = list.indexOf(handler);
    if (idx > -1) list.splice(idx, 1);
  }

  emit(type: string, data: string) {
    const list = this.listeners.get(type) || [];
    list.forEach((h) => h(new MessageEvent(type, { data })));
  }

  close() {
    // noop
  }
}

describe("useSessionEventStream", () => {
  let handlers: {
    onEvent: ReturnType<typeof vi.fn>;
    onConnectionChange: ReturnType<typeof vi.fn>;
    onStreamError: ReturnType<typeof vi.fn>;
  };

  beforeEach(() => {
    instances = [];
    handlers = {
      onEvent: vi.fn(),
      onConnectionChange: vi.fn(),
      onStreamError: vi.fn(),
    };
    global.EventSource = MockEventSource as unknown as typeof EventSource;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("initializes with connecting state", () => {
    renderHook(() => useSessionEventStream("s1", handlers));
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "connecting",
      null,
      true,
    );
  });

  it("transitions to connected on open", () => {
    renderHook(() => useSessionEventStream("s1", handlers));
    const es = instances[0];
    act(() => {
      es.onopen?.();
    });
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "connected",
      null,
      false,
    );
  });

  it("parses run_event and calls onEvent", () => {
    renderHook(() => useSessionEventStream("s1", handlers));
    const es = instances[0];
    const event: RunEvent = {
      event_id: "e1",
      event_type: "action_completed",
      session_id: "s1",
      run_id: "r1",
      sequence: 1,
      timestamp: "2024-01-01T00:00:00Z",
      stage: "action",
      summary: "done",
    };
    act(() => {
      es.emit("run_event", JSON.stringify(event));
    });
    expect(handlers.onEvent).toHaveBeenCalledWith(expect.objectContaining({ event_id: "e1" }));
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "connected",
      "2024-01-01T00:00:00Z",
      false,
    );
  });

  it("handles malformed events gracefully", () => {
    renderHook(() => useSessionEventStream("s1", handlers));
    const es = instances[0];
    act(() => {
      es.emit("run_event", "not-json");
    });
    expect(handlers.onEvent).not.toHaveBeenCalled();
  });

  it("calls onStreamError and disconnected on error", () => {
    renderHook(() => useSessionEventStream("s1", handlers));
    const es = instances[0];
    act(() => {
      es.onerror?.();
    });
    expect(handlers.onStreamError).toHaveBeenCalledWith("事件流连接已断开");
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "disconnected",
      null,
      true,
    );
  });

  it("closes and notifies closed on unmount", () => {
    const { unmount } = renderHook(() => useSessionEventStream("s1", handlers));
    act(() => {
      unmount();
    });
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "closed",
      null,
      false,
    );
  });

  it("reconnect triggers reconnecting state", () => {
    const { result } = renderHook(() => useSessionEventStream("s1", handlers));
    act(() => {
      result.current.reconnect();
    });
    expect(handlers.onConnectionChange).toHaveBeenCalledWith(
      "reconnecting",
      null,
      true,
    );
  });

  it("does not connect when sessionId is empty", () => {
    renderHook(() => useSessionEventStream("", handlers));
    expect(instances).toHaveLength(0);
    expect(handlers.onConnectionChange).toHaveBeenCalledWith("closed", null, false);
  });

  it("uses encoded session_id in URL", () => {
    renderHook(() => useSessionEventStream("s/1", handlers));
    expect(instances[0].url).toContain(encodeURIComponent("s/1"));
  });
});
