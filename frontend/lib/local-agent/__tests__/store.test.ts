import { describe, it, expect, beforeEach } from 'vitest'

const storage: Record<string, string> = {}

Object.defineProperty(global, 'localStorage', {
  value: {
    getItem: (k: string) => storage[k] ?? null,
    setItem: (k: string, v: string) => { storage[k] = v },
    removeItem: (k: string) => { delete storage[k] },
  },
  writable: true,
})

async function getStore() {
  const mod = await import('../store')
  return mod.useRuntimeStore
}

describe('useRuntimeStore', () => {
  beforeEach(() => {
    Object.keys(storage).forEach((k) => delete storage[k])
  })

  it('adds a user message', async () => {
    const useStore = await getStore()
    useStore.getState().clearSession()
    useStore.getState().addMessage({ role: 'user', content: 'hello' })
    const { messages } = useStore.getState()
    expect(messages).toHaveLength(1)
    expect(messages[0].role).toBe('user')
    expect(messages[0].content).toBe('hello')
    expect(messages[0].id).toBeDefined()
  })

  it('cancels run and clears streaming flag', async () => {
    const useStore = await getStore()
    useStore.getState().clearSession()
    useStore.getState().addMessage({ role: 'assistant', content: 'typing...', isStreaming: true })
    useStore.getState().cancelRun()
    const { messages, runState } = useStore.getState()
    expect(runState).toBe('idle')
    expect(messages[0].isStreaming).toBeFalsy()
  })

  it('saves and loads session-specific data via resumeSession', async () => {
    const useStore = await getStore()
    useStore.getState().clearSession()
    useStore.getState().addMessage({ role: 'user', content: 'msg A' })
    const sessionA = useStore.getState().sessionId

    // resumeSession saves current session, then loads target session
    // Switching to a new session ID will save session A first
    useStore.getState().resumeSession('other-session-id')

    // Now session A should be persisted
    const raw = storage[`la:session:${sessionA}`]
    expect(raw).toBeDefined()
    const data = JSON.parse(raw)
    expect(data.messages[0].content).toBe('msg A')
    expect(data.sessionId).toBe(sessionA)
  })

  it('loads persisted session on resume', async () => {
    const useStore = await getStore()
    useStore.getState().clearSession()
    useStore.getState().addMessage({ role: 'user', content: 'msg B' })
    const sessionB = useStore.getState().sessionId

    // Save session B by switching away
    useStore.getState().resumeSession('temp-session')

    // Switch back to session B
    useStore.getState().resumeSession(sessionB)
    const { messages, sessionId } = useStore.getState()
    expect(sessionId).toBe(sessionB)
    expect(messages).toHaveLength(1)
    expect(messages[0].content).toBe('msg B')
  })
})
