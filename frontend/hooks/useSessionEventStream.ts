"use client"

import { useEffect, useRef, useState, useCallback } from "react"

export type RunEvent = {
  event_id: string
  event_type: string
  trace_id?: string
  session_id: string
  run_id: string
  sequence: number
  timestamp: string
  stage: string
  summary: string
  detail?: string
  metadata?: Record<string, string>
}

export type ConnectionState = "connecting" | "connected" | "reconnecting" | "disconnected" | "closed"

type SessionEventHandlers = {
  onEvent: (event: RunEvent) => void
  onConnectionChange: (
    connectionState: ConnectionState,
    latestEventAt: string | null,
    canReconnect: boolean,
  ) => void
  onStreamError: (message: string) => void
}

export function useSessionEventStream(sessionId: string, handlers: SessionEventHandlers) {
  const retryRef = useRef(0)
  const [retryToken, setRetryToken] = useState(0)
  const latestEventAtRef = useRef<string | null>(null)
  const closedByEffectRef = useRef(false)
  const sourceRef = useRef<EventSource | null>(null)

  const bumpRetry = useCallback(() => {
    retryRef.current += 1
    setRetryToken((v) => v + 1)
  }, [])

  const handleEvent = useCallback((rawEvent: MessageEvent<string>) => {
    try {
      const payload = JSON.parse(rawEvent.data) as RunEvent
      latestEventAtRef.current = payload.timestamp
      handlers.onConnectionChange("connected", payload.timestamp, false)
      handlers.onEvent(payload)
    } catch {
      // ignore malformed events
    }
  }, [handlers])

  const handleError = useCallback(() => {
    if (closedByEffectRef.current) return
    handlers.onConnectionChange("disconnected", latestEventAtRef.current, Boolean(sessionId))
    handlers.onStreamError("事件流连接已断开")
  }, [sessionId, handlers])

  const reconnect = useCallback(() => {
    if (!sessionId) return
    handlers.onConnectionChange("reconnecting", latestEventAtRef.current, true)
    bumpRetry()
  }, [sessionId, handlers, bumpRetry])

  useEffect(() => {
    if (!sessionId) {
      latestEventAtRef.current = null
      handlers.onConnectionChange("closed", null, false)
      return
    }

    closedByEffectRef.current = false
    const openingState: ConnectionState = retryRef.current === 0 ? "connecting" : "reconnecting"
    handlers.onConnectionChange(openingState, latestEventAtRef.current, true)

    const source = new EventSource(`/api/v1/events/stream?session_id=${encodeURIComponent(sessionId)}`)
    sourceRef.current = source

    source.addEventListener("run_event", handleEvent as EventListener)
    source.onopen = () => handlers.onConnectionChange("connected", latestEventAtRef.current, false)
    source.onerror = () => {
      source.close()
      handleError()
    }

    return () => {
      closedByEffectRef.current = true
      source.removeEventListener("run_event", handleEvent as EventListener)
      source.close()
      handlers.onConnectionChange("closed", latestEventAtRef.current, false)
    }
  }, [sessionId, retryToken, handlers, handleEvent, handleError])

  return { reconnect }
}
