import { create } from "zustand"
import type {
  RuntimeState,
  Message,
  RuntimeEvent,
  Confirmation,
  ConnectionState,
  RunState,
  Settings,
  KnowledgeItem,
  LogRun,
  Memory,
  ViewType,
} from "./types"
import { submitChatRun, submitChatCancel, submitConfirmationDecision, type SubmitChatRunPayload, fetchKnowledgeItems, fetchSettings, fetchLogs, fetchMemories, deleteMemory, fetchProviderSettings, updateSettings, type ProviderSettingsItem, type SettingsResponse, fetchSessionMessages, addSessionMessage } from "./api"

// Generate unique IDs
const generateId = () => Math.random().toString(36).substring(2, 15)

// Runtime Store
interface RuntimeStore extends RuntimeState {
  // Actions
  setConnectionState: (state: ConnectionState) => void
  setRunState: (state: RunState) => void
  addMessage: (message: Omit<Message, "id" | "timestamp">) => void
  addEvent: (event: Omit<RuntimeEvent, "event_id" | "timestamp">) => void
  setConfirmation: (confirmation: Confirmation | null) => void
  setComposeValue: (value: string) => void
  setCriticalError: (error: string | null) => void
  setSubmitError: (error: string | null) => void
  clearMessages: () => void
  startNewRun: (taskTitle: string) => void
  acceptRun: (sessionId: string, runId: string) => void
  applyEvent: (event: RuntimeEvent) => void
  completeRun: () => void
  failRun: (error: string) => void
  cancelRun: () => void
  editAndResend: (messageId: string) => void
  resumeSession: (sessionId: string) => void
  clearSession: () => void
}

const LEGACY_KEY = "la:session"

function getStorageKey(sessionId: string): string {
  return `la:session:${sessionId}`
}

function loadPersistedSession(sessionId?: string): { sessionId: string; messages: Message[]; runState?: RunState; criticalError?: string | null; currentTaskTitle?: string } | null {
  if (typeof window === "undefined") return null
  try {
    if (sessionId) {
      const raw = localStorage.getItem(getStorageKey(sessionId))
      if (raw) return JSON.parse(raw)
      return null
    }
    const raw = localStorage.getItem(LEGACY_KEY)
    if (raw) return JSON.parse(raw)
  } catch { /* ignore */ }
  return null
}

const persisted = loadPersistedSession()

export const useRuntimeStore = create<RuntimeStore>((set, get) => ({
  // Initial state
  sessionId: persisted?.sessionId || generateId(),
  currentRunId: "",
  currentTaskTitle: persisted?.currentTaskTitle || "",
  runState: persisted?.runState === "running" ? "idle" : (persisted?.runState || "idle"),
  connectionState: "connected",
  messages: persisted?.messages || [],
  events: [],
  confirmation: null,
  composeValue: "",
  criticalError: persisted?.criticalError || null,
  submitError: null,

  // Actions
  setConnectionState: (connectionState) => set({ connectionState }),
  setRunState: (runState) => set({ runState }),
  
  addMessage: (message) => {
    const newMessage: Message = {
      ...message,
      id: generateId(),
      timestamp: new Date().toISOString(),
    }
    set((state) => ({
      messages: [...state.messages, newMessage],
    }))
    // Sync to backend asynchronously
    const { sessionId } = get()
    addSessionMessage(sessionId, {
      role: newMessage.role,
      content: newMessage.content,
      timestamp: newMessage.timestamp,
    }).catch(() => { /* ignore network errors */ })
  },

  addEvent: (event) =>
    set((state) => ({
      events: [
        ...state.events,
        {
          ...event,
          event_id: generateId(),
          timestamp: new Date().toISOString(),
        },
      ],
    })),

  setConfirmation: (confirmation) => set({ confirmation }),
  setComposeValue: (composeValue) => set({ composeValue }),
  setCriticalError: (criticalError) => set({ criticalError }),
  setSubmitError: (submitError) => set({ submitError }),
  clearMessages: () => {
    set({ messages: [], events: [] })
    savePersistedSession(get())
  },

  editAndResend: (messageId: string) =>
    set((state) => {
      const idx = state.messages.findIndex((m) => m.id === messageId)
      if (idx < 0) return {}
      const targetMsg = state.messages[idx]
      if (targetMsg.role !== "user") return {}
      return {
        messages: state.messages.slice(0, idx),
        composeValue: targetMsg.content,
        events: [],
        runState: "idle",
        currentRunId: "",
        criticalError: null,
        submitError: null,
      }
    }),

  startNewRun: (taskTitle) =>
    set({
      currentRunId: generateId(),
      currentTaskTitle: taskTitle,
      runState: "running",
      events: [],
      confirmation: null,
      criticalError: null,
      submitError: null,
    }),

  acceptRun: (sessionId: string, runId: string) =>
    set({
      sessionId,
      currentRunId: runId,
      runState: "running",
    }),

  applyEvent: (event: RuntimeEvent) => {
    let assistantContent: string | null = null

    set((state) => {
      const events = [...state.events, event]
      const metadata = event.metadata ?? {}

      function findStreamingIndex(): number {
        for (let i = state.messages.length - 1; i >= 0; i--) {
          if (state.messages[i].role === "assistant" && state.messages[i].isStreaming) {
            return i
          }
        }
        return -1
      }

      function makeStreamingMessage(content: string): Message {
        return {
          id: generateId(),
          role: "assistant",
          content,
          timestamp: new Date().toISOString(),
          isStreaming: true,
        }
      }

      function finalizeStreamingMessage(content: string, messages: Message[]): Message[] {
        const idx = findStreamingIndex()
        if (idx >= 0) {
          const updated = [...messages]
          updated[idx] = { ...updated[idx], content, isStreaming: false }
          return updated
        }
        return [...messages, {
          id: generateId(),
          role: "assistant",
          content,
          timestamp: new Date().toISOString(),
        }]
      }

      function updateStreamingMessage(content: string, messages: Message[]): Message[] {
        const idx = findStreamingIndex()
        if (idx >= 0) {
          const updated = [...messages]
          updated[idx] = { ...updated[idx], content }
          return updated
        }
        return [...messages, makeStreamingMessage(content)]
      }

      if (event.event_type === "confirmation_required") {
        return {
          events,
          runState: "awaiting_confirmation",
          confirmation: {
            confirmation_id: metadataText(metadata, "confirmation_id"),
            run_id: event.run_id,
            risk_level: (metadataText(metadata, "risk_level") as Confirmation["risk_level"]) || "medium",
            action_summary: metadataText(metadata, "action_summary") || event.summary,
            reason: metadataText(metadata, "reason") || event.summary,
            target_paths: metadataList(metadata, "target_paths"),
            hazards: metadataList(metadata, "hazards"),
            alternatives: metadataList(metadata, "alternatives"),
            tool_name: metadataText(metadata, "tool_name"),
            tool_arguments_json: metadataText(metadata, "tool_arguments_json"),
            patch_preview_report_json: metadataText(metadata, "patch_preview_report_json"),
          } as Confirmation,
        }
      }

      // Stream partial answer as soon as action completes (only if final_answer is present)
      if (event.event_type === "action_completed") {
        const answer = metadataText(metadata, "final_answer")
        if (answer) {
          return {
            events,
            messages: updateStreamingMessage(answer, state.messages),
          }
        }
        return { events }
      }

      // Update with verified answer
      if (event.event_type === "verification_completed") {
        const answer = metadataText(metadata, "final_answer") || metadataText(metadata, "result_summary")
        if (answer) {
          return {
            events,
            messages: updateStreamingMessage(answer, state.messages),
          }
        }
        return { events }
      }

      if (event.event_type === "run_finished" || event.event_type === "completion") {
        if (state.runState !== "running" && state.runState !== "awaiting_confirmation") {
          return { events }
        }
        const answer = metadataText(metadata, "final_answer") || metadataText(metadata, "result_summary") || event.summary || "任务结束"
        assistantContent = answer
        return {
          events,
          runState: "completed",
          messages: finalizeStreamingMessage(answer, state.messages),
        }
      }

      if (event.event_type === "run_failed" || event.event_type === "error") {
        if (state.runState !== "running" && state.runState !== "awaiting_confirmation") {
          return { events }
        }
        const answer = metadataText(metadata, "final_answer") || metadataText(metadata, "result_summary") || event.summary || "任务失败"
        assistantContent = answer
        return {
          events,
          runState: "failed",
          criticalError: event.summary,
          messages: finalizeStreamingMessage(answer, state.messages),
        }
      }

      return { events }
    })

    if (assistantContent) {
      const { sessionId } = get()
      addSessionMessage(sessionId, {
        role: "assistant",
        content: assistantContent,
      }).catch(() => { /* ignore network errors */ })
    }
  },

  completeRun: () =>
    set({
      runState: "completed",
      currentRunId: "",
    }),

  failRun: (error) =>
    set({
      runState: "failed",
      criticalError: error,
    }),

  cancelRun: () => {
    let assistantContent: string | null = null

    set((state) => {
      const idx = state.messages.length - 1
      const updatedMessages = [...state.messages]
      if (idx >= 0 && updatedMessages[idx].role === "assistant" && updatedMessages[idx].isStreaming) {
        assistantContent = updatedMessages[idx].content
        updatedMessages[idx] = { ...updatedMessages[idx], isStreaming: false }
      }
      return {
        runState: "idle",
        events: [],
        criticalError: null,
        messages: updatedMessages,
      }
    })

    if (assistantContent) {
      const { sessionId } = get()
      addSessionMessage(sessionId, {
        role: "assistant",
        content: assistantContent,
      }).catch(() => { /* ignore network errors */ })
    }
  },

  resumeSession: (sessionId: string) => {
    savePersistedSession(get())
    // Try backend first, fallback to localStorage
    fetchSessionMessages(sessionId)
      .then((resp) => {
        if (resp.items.length > 0) {
          const messages: Message[] = resp.items.map((m) => ({
            id: m.id,
            role: m.role as "user" | "assistant",
            content: m.content,
            timestamp: m.timestamp,
          }))
          set({
            sessionId,
            messages,
            events: [],
            currentRunId: "",
            currentTaskTitle: messages[0]?.content.slice(0, 50) || "",
            runState: "idle",
            confirmation: null,
            criticalError: null,
            submitError: null,
          })
          savePersistedSession(get())
        } else {
          throw new Error("no backend messages")
        }
      })
      .catch(() => {
        const data = loadPersistedSession(sessionId)
        set({
          sessionId,
          messages: data?.messages || [],
          events: [],
          currentRunId: "",
          currentTaskTitle: data?.currentTaskTitle || "",
          runState: data?.runState === "running" ? "idle" : (data?.runState || "idle"),
          confirmation: null,
          criticalError: data?.criticalError || null,
          submitError: null,
        })
        savePersistedSession(get())
      })
  },

  clearSession: () => {
    const { sessionId } = get()
    const newSessionId = generateId()
    set({
      sessionId: newSessionId,
      messages: [],
      events: [],
      currentRunId: "",
      currentTaskTitle: "",
      runState: "idle",
      confirmation: null,
      criticalError: null,
      submitError: null,
    })
    if (typeof window !== "undefined") {
      localStorage.removeItem(LEGACY_KEY)
      localStorage.removeItem(getStorageKey(sessionId))
    }
  },
}))

function savePersistedSession(state: Pick<RuntimeStore, "sessionId" | "messages" | "runState" | "criticalError" | "currentTaskTitle">) {
  if (typeof window === "undefined") return
  try {
    const messages = state.messages.slice(-200).map((m) => ({
      ...m,
      isStreaming: false,
    }))
    const payload = {
      sessionId: state.sessionId,
      messages,
      runState: state.runState,
      criticalError: state.criticalError,
      currentTaskTitle: state.currentTaskTitle,
    }
    localStorage.setItem(LEGACY_KEY, JSON.stringify(payload))
    localStorage.setItem(getStorageKey(state.sessionId), JSON.stringify(payload))
  } catch { /* ignore */ }
}

if (typeof window !== "undefined") {
  let prevKey = ""
  useRuntimeStore.subscribe((state) => {
    const key = state.sessionId + "|" + state.messages.length + "|" + state.runState + "|" + state.criticalError + "|" + state.currentTaskTitle
    if (key === prevKey) return
    prevKey = key
    savePersistedSession(state)
  })
}

function metadataText(metadata: Record<string, unknown>, key: string) {
  const value = metadata[key]
  return typeof value === "string" ? value : ""
}

function metadataList(metadata: Record<string, unknown>, key: string) {
  return metadataText(metadata, key).split("\n").filter(Boolean)
}

// UI Store
interface UIStore {
  activeView: ViewType
  leftSidebarExpanded: boolean
  rightDrawerOpen: boolean
  mobileMenuOpen: boolean
  mobileDrawerOpen: boolean
  setActiveView: (view: ViewType) => void
  toggleLeftSidebar: () => void
  toggleRightDrawer: () => void
  setRightDrawerOpen: (open: boolean) => void
  setMobileMenuOpen: (open: boolean) => void
  setMobileDrawerOpen: (open: boolean) => void
}

export const useUIStore = create<UIStore>((set) => ({
  activeView: "task",
  leftSidebarExpanded: false,
  rightDrawerOpen: true,
  mobileMenuOpen: false,
  mobileDrawerOpen: false,

  setActiveView: (activeView) => set({ activeView }),
  toggleLeftSidebar: () =>
    set((state) => ({ leftSidebarExpanded: !state.leftSidebarExpanded })),
  toggleRightDrawer: () =>
    set((state) => ({ rightDrawerOpen: !state.rightDrawerOpen })),
  setRightDrawerOpen: (rightDrawerOpen) => set({ rightDrawerOpen }),
  setMobileMenuOpen: (mobileMenuOpen) => set({ mobileMenuOpen }),
  setMobileDrawerOpen: (mobileDrawerOpen) => set({ mobileDrawerOpen }),
}))

// Settings Store
interface SettingsStore extends Settings {
  setMode: (mode: Settings["mode"]) => void
  setModel: (model: Settings["model"]) => void
  setWorkspace: (workspace: Settings["workspace"]) => void
  setEmbeddingProvider: (providerId: string) => void
  setDirectoryPromptEnabled: (enabled: boolean) => void
  setShowRiskLevel: (enabled: boolean) => void
  addDirectory: (name: string, path: string) => Promise<void>
  removeDirectory: (rootPath: string) => Promise<void>
  addProvider: (provider: Settings["providers"][0]) => void
  removeProvider: (providerId: string) => void
  updateProvider: (providerId: string, updates: Partial<Settings["providers"][0]>) => void
  loadSettings: () => Promise<void>
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  mode: "standard",
  model: {
    model_id: "",
    display_name: "",
    provider_id: "",
  },
  workspace: {
    workspace_id: "default",
    name: "默认工作区",
    root_path: "/",
  },
  embedding_provider_id: "",
  providers: [],
  available_models: [],
  available_workspaces: [],
  approved_directories: [],
  directory_prompt_enabled: true,
  show_risk_level: true,
  ports: {},

  setMode: (mode) => {
    set({ mode })
    void updateSettings({ mode })
  },
  setModel: (model) => {
    set({ model })
    void updateSettings({ model })
  },
  setWorkspace: (workspace) => {
    set({ workspace })
    void updateSettings({ workspace_id: workspace.workspace_id })
  },
  setEmbeddingProvider: (embedding_provider_id) => {
    set({ embedding_provider_id })
    void updateSettings({ embedding_provider_id })
  },
  setDirectoryPromptEnabled: (directory_prompt_enabled) => {
    set({ directory_prompt_enabled })
    void updateSettings({ directory_prompt_enabled })
  },
  setShowRiskLevel: (show_risk_level) => {
    set({ show_risk_level })
    void updateSettings({ show_risk_level })
  },
  addDirectory: async (name, path) => {
    await updateSettings({ add_directory_name: name, add_directory_path: path })
    const data = await fetchSettings()
    set({ approved_directories: data.approved_directories })
  },
  removeDirectory: async (rootPath) => {
    await updateSettings({ revoke_directory_root: rootPath })
    set((state) => ({
      approved_directories: state.approved_directories.filter((d) => d.root_path !== rootPath),
    }))
  },
  addProvider: (provider) =>
    set((state) => ({
      providers: [...state.providers, provider],
    })),
  
  removeProvider: (providerId) =>
    set((state) => ({
      providers: state.providers.filter((p) => p.provider_id !== providerId),
    })),
  
  updateProvider: (providerId, updates) =>
    set((state) => ({
      providers: state.providers.map((p) =>
        p.provider_id === providerId ? { ...p, ...updates } : p
      ),
    })),

  loadSettings: async () => {
    try {
      const data = await fetchSettings()
      const providerData = await fetchProviderSettings().catch(() => null)
      const providers = providerData?.providers || data.providers.map(toProviderSettingsItem)
      set({
        mode: data.mode as AgentMode,
        model: data.model,
        workspace: data.workspace,
        embedding_provider_id: data.embedding?.provider_id || "",
        active_provider_id: providerData?.active_provider_id,
        available_models: data.available_models,
        available_workspaces: data.available_workspaces,
        approved_directories: data.approved_directories,
        directory_prompt_enabled: data.directory_prompt_enabled,
        show_risk_level: data.show_risk_level,
        ports: data.ports,
        runtime_status: data.runtime_status,
        embedding: data.embedding,
        mcp: data.mcp,
        providers: providers.map((p) => ({
          provider_id: p.provider_id,
          display_name: p.display_name,
          base_url: p.base_url,
          chat_completions_path: p.chat_completions_path,
          models_path: p.models_path,
          embedding_model: p.embedding_model,
          credential_kind: p.credential_kind,
          supports_test: p.supports_test,
          editable: p.editable,
          credential_status: p.credential_status,
          status: providerStatus(p.credential_status),
        })),
      })
    } catch {
      // ignore load errors
    }
  },
}))

function toProviderSettingsItem(provider: SettingsResponse["providers"][0]): ProviderSettingsItem {
  return {
    ...provider,
    credential_status: {
      has_credential: false,
      last_test_status: "idle",
      apply_status: "not_configured",
      pending_reload: false,
    },
  }
}

function providerStatus(status?: Settings["providers"][0]["credential_status"]) {
  if (!status?.has_credential) return "inactive"
  if (status.last_test_status === "failed") return "error"
  return "active"
}

// Knowledge Store
interface KnowledgeStore {
  items: KnowledgeItem[]
  categories: string[]
  allTags: string[]
  selectedItem: KnowledgeItem | null
  searchQuery: string
  selectedCategory: string | null
  selectedTags: string[]
  setItems: (items: KnowledgeItem[]) => void
  setSelectedItem: (item: KnowledgeItem | null) => void
  setSearchQuery: (query: string) => void
  setSelectedCategory: (category: string | null) => void
  toggleTag: (tag: string) => void
  addItem: (item: KnowledgeItem) => void
  updateItem: (id: string, updates: Partial<KnowledgeItem>) => void
  removeItem: (id: string) => void
  loadItems: () => Promise<void>
}

export const useKnowledgeStore = create<KnowledgeStore>((set) => ({
  items: [],
  categories: [],
  allTags: [],
  selectedItem: null,
  searchQuery: "",
  selectedCategory: null,
  selectedTags: [],

  setItems: (items) => set({ items }),
  setSelectedItem: (selectedItem) => set({ selectedItem }),
  setSearchQuery: (searchQuery) => set({ searchQuery }),
  setSelectedCategory: (selectedCategory) => set({ selectedCategory }),
  
  toggleTag: (tag) =>
    set((state) => ({
      selectedTags: state.selectedTags.includes(tag)
        ? state.selectedTags.filter((t) => t !== tag)
        : [...state.selectedTags, tag],
    })),

  addItem: (item) =>
    set((state) => ({
      items: [...state.items, item],
    })),

  updateItem: (id, updates) =>
    set((state) => ({
      items: state.items.map((item) =>
        item.id === id ? { ...item, ...updates } : item
      ),
    })),

  removeItem: (id) =>
    set((state) => ({
      items: state.items.filter((item) => item.id !== id),
      selectedItem: state.selectedItem?.id === id ? null : state.selectedItem,
    })),

  loadItems: async () => {
    try {
      const data = await fetchKnowledgeItems()
      set({ items: data.items, categories: data.categories, allTags: data.tags })
    } catch {
      // ignore load errors
    }
  },
}))

// Logs Store
interface LogsStore {
  runs: LogRun[]
  selectedRun: LogRun | null
  statusFilter: "all" | "completed" | "failed" | "running"
  timeFilter: "today" | "7days" | "30days"
  searchQuery: string
  setRuns: (runs: LogRun[]) => void
  setSelectedRun: (run: LogRun | null) => void
  setStatusFilter: (filter: LogsStore["statusFilter"]) => void
  setTimeFilter: (filter: LogsStore["timeFilter"]) => void
  setSearchQuery: (query: string) => void
  loadLogs: () => Promise<void>
}

export const useLogsStore = create<LogsStore>((set) => ({
  runs: [],
  selectedRun: null,
  statusFilter: "all",
  timeFilter: "7days",
  searchQuery: "",

  setRuns: (runs) => set({ runs }),
  setSelectedRun: (selectedRun) => set({ selectedRun }),
  setStatusFilter: (statusFilter) => set({ statusFilter }),
  setTimeFilter: (timeFilter) => set({ timeFilter }),
  setSearchQuery: (searchQuery) => set({ searchQuery }),

  loadLogs: async () => {
    try {
      const data = await fetchLogs("runs", { limit: 100 })
      set({ runs: data.runs || [] })
    } catch {
      // ignore load errors
    }
  },
}))

// Memory Store
interface MemoryStore {
  memories: Memory[]
  setMemories: (memories: Memory[]) => void
  addMemory: (memory: Memory) => void
  removeMemory: (id: string) => Promise<void>
  loadMemories: () => Promise<void>
}

export const useMemoryStore = create<MemoryStore>((set) => ({
  memories: [],

  setMemories: (memories) => set({ memories }),

  addMemory: (memory) =>
    set((state) => ({
      memories: [...state.memories, memory],
    })),

  removeMemory: async (id) => {
    const data = await deleteMemory(id)
    set({
      memories: data.items.map((item) => ({
        id: item.id,
        kind: item.kind,
        title: item.title,
        summary: item.summary,
        content: item.content,
        createdAt: item.created_at,
        sourceRunId: item.source_run_id,
      })),
    })
  },

  loadMemories: async () => {
    try {
      const data = await fetchMemories()
      set({
        memories: data.items.map((item) => ({
          id: item.id,
          kind: item.kind,
          title: item.title,
          summary: item.summary,
          content: item.content,
          createdAt: item.created_at,
          sourceRunId: item.source_run_id,
        })),
      })
    } catch {
      // ignore load errors
    }
  },
}))
