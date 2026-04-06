import { useEffect, useEffectEvent, useMemo, useRef, useState } from "react";

import { RunEvent } from "../shared/contracts";
import { ConnectionState } from "../runtime/state";

type SessionEventHandlers = {
  onEvent: (event: RunEvent) => void;
  onConnectionChange: (
    connectionState: ConnectionState,
    latestEventAt: string | null,
    canReconnect: boolean,
  ) => void;
  onStreamError: (message: string) => void;
};

export function useSessionEventStream(sessionId: string, handlers: SessionEventHandlers) {
  const retry = useRetryToken();
  const refs = useStreamRefs();
  const stableHandlers = useStableHandlers(handlers);
  const callbacks = useStreamCallbacks(sessionId, stableHandlers, refs, retry.bump);
  const params = useMemo(
    () => ({ callbacks, handlers: stableHandlers, refs, retryToken: retry.value, sessionId }),
    [callbacks, refs, retry.value, sessionId, stableHandlers],
  );
  useStreamLifecycle(params);
  return { reconnect: callbacks.reconnect };
}

function useRetryToken() {
  const [value, setValue] = useState(0);
  return { bump: () => setValue((current) => current + 1), value };
}

function useStreamRefs() {
  return {
    closedByEffectRef: useRef(false),
    latestEventAtRef: useRef<string | null>(null),
  };
}

function useStableHandlers(handlers: SessionEventHandlers) {
  const onConnectionChange = useEffectEvent(handlers.onConnectionChange);
  const onEvent = useEffectEvent(handlers.onEvent);
  const onStreamError = useEffectEvent(handlers.onStreamError);
  return useMemo(
    () => ({ onConnectionChange, onEvent, onStreamError }),
    [onConnectionChange, onEvent, onStreamError],
  );
}

function useStreamCallbacks(
  sessionId: string,
  handlers: SessionEventHandlers,
  refs: ReturnType<typeof useStreamRefs>,
  bumpRetry: () => void,
) {
  const handleEvent = useEffectEvent((rawEvent: MessageEvent<string>) => {
    const payload = JSON.parse(rawEvent.data) as RunEvent;
    refs.latestEventAtRef.current = payload.timestamp;
    handlers.onConnectionChange("connected", payload.timestamp, false);
    handlers.onEvent(payload);
  });
  const handleError = useEffectEvent(() => {
    handlers.onConnectionChange("disconnected", refs.latestEventAtRef.current, Boolean(sessionId));
    handlers.onStreamError("事件流连接已断开");
  });
  const reconnect = useEffectEvent(() => {
    if (!sessionId) return;
    handlers.onConnectionChange("reconnecting", refs.latestEventAtRef.current, true);
    bumpRetry();
  });
  return { handleError, handleEvent, reconnect };
}

function useStreamLifecycle(params: {
  sessionId: string;
  handlers: SessionEventHandlers;
  refs: ReturnType<typeof useStreamRefs>;
  retryToken: number;
  callbacks: ReturnType<typeof useStreamCallbacks>;
}) {
  useEffect(() => {
    if (!params.sessionId) {
      params.refs.latestEventAtRef.current = null;
      params.handlers.onConnectionChange("closed", null, false);
      return;
    }
    return bindEventSource(params);
  }, [params.sessionId, params.retryToken]);
}

function bindEventSource(params: {
  sessionId: string;
  handlers: SessionEventHandlers;
  refs: ReturnType<typeof useStreamRefs>;
  retryToken: number;
  callbacks: ReturnType<typeof useStreamCallbacks>;
}) {
  params.refs.closedByEffectRef.current = false;
  params.handlers.onConnectionChange(getOpeningState(params.retryToken), params.refs.latestEventAtRef.current, true);
  const source = createEventSource(params.sessionId);
  bindListeners(source, params.handlers, params.refs, params.callbacks);
  return () => closeStream(source, params.handlers, params.refs, params.callbacks.handleEvent);
}

function getOpeningState(retryToken: number) {
  return retryToken === 0 ? "connecting" : "reconnecting";
}

function createEventSource(sessionId: string) {
  return new EventSource(`/api/v1/events/stream?session_id=${encodeURIComponent(sessionId)}`);
}

function bindListeners(
  source: EventSource,
  handlers: SessionEventHandlers,
  refs: ReturnType<typeof useStreamRefs>,
  callbacks: ReturnType<typeof useStreamCallbacks>,
) {
  source.addEventListener("run_event", callbacks.handleEvent as EventListener);
  source.onopen = () => handlers.onConnectionChange("connected", refs.latestEventAtRef.current, false);
  source.onerror = () => closeBrokenStream(source, refs, callbacks.handleError);
}

function closeBrokenStream(
  source: EventSource,
  refs: ReturnType<typeof useStreamRefs>,
  handleError: () => void,
) {
  source.close();
  if (refs.closedByEffectRef.current) return;
  handleError();
}

function closeStream(
  source: EventSource,
  handlers: SessionEventHandlers,
  refs: ReturnType<typeof useStreamRefs>,
  handleEvent: (rawEvent: MessageEvent<string>) => void,
) {
  refs.closedByEffectRef.current = true;
  source.removeEventListener("run_event", handleEvent as EventListener);
  source.close();
  handlers.onConnectionChange("closed", refs.latestEventAtRef.current, false);
}
