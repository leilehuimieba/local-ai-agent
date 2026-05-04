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
  const handlersRef = useRef(handlers)

  useEffect(() => {
    handlersRef.current = handlers
  }, [handlers])

  const bumpRetry = useCallback(() => {
    retryRef.current += 1
    setRetryToken((v) => v + 1)
  }, [])

  const handleEvent = useCallback((rawEvent: MessageEvent<string>) => {
    try {
      const payload = JSON.parse(rawEvent.data) as RunEvent
      latestEventAtRef.current = payload.timestamp
      handlersRef.current.onConnectionChange("connected", payload.timestamp, false)
      handlersRef.current.onEvent(payload)
    } catch {
      // ignore malformed events
    }
  }, [])

  const handleError = useCallback(() => {
    if (closedByEffectRef.current) return
    handlersRef.current.onConnectionChange("disconnected", latestEventAtRef.current, Boolean(sessionId))
    handlersRef.current.onStreamError("事件流连接已断开")
  }, [sessionId])

  const reconnect = useCallback(() => {
    if (!sessionId) return
    handlersRef.current.onConnectionChange("reconnecting", latestEventAtRef.current, true)
    bumpRetry()
  }, [sessionId, bumpRetry])

  useEffect(() => {
    if (!sessionId) {
      latestEventAtRef.current = null
      handlersRef.current.onConnectionChange("closed", null, false)
      return
    }

    closedByEffectRef.current = false
    const openingState: ConnectionState = retryRef.current === 0 ? "connecting" : "reconnecting"
    handlersRef.current.onConnectionChange(openingState, latestEventAtRef.current, true)

    const source = new EventSource(`/api/v1/events/stream?session_id=${encodeURIComponent(sessionId)}`)
    sourceRef.current = source

    source.addEventListener("run_event", handleEvent as EventListener)
    source.onopen = () => {
      handlersRef.current.onConnectionChange("connected", latestEventAtRef.current, false)
    }
    source.onerror = () => {
      source.close()
      handleError()
    }

    return () => {
      closedByEffectRef.current = true
      source.removeEventListener("run_event", handleEvent as EventListener)
      source.close()
      handlersRef.current.onConnectionChange("closed", latestEventAtRef.current, false)
    }
  }, [sessionId, retryToken, handleEvent, handleError])

  return { reconnect }
}
