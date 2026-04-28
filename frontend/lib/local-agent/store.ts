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
import { submitChatRun, submitConfirmationDecision, type SubmitChatRunPayload, fetchKnowledgeItems, fetchSettings, fetchLogs, type SettingsResponse } from "./api"

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
  completeRun: () => void
  failRun: (error: string) => void
}

export const useRuntimeStore = create<RuntimeStore>((set, get) => ({
  // Initial state
  sessionId: generateId(),
  currentRunId: "",
  currentTaskTitle: "",
  runState: "idle",
  connectionState: "connected",
  messages: [],
  events: [],
  confirmation: null,
  composeValue: "",
  criticalError: null,
  submitError: null,

  // Actions
  setConnectionState: (connectionState) => set({ connectionState }),
  setRunState: (runState) => set({ runState }),
  
  addMessage: (message) =>
    set((state) => ({
      messages: [
        ...state.messages,
        {
          ...message,
          id: generateId(),
          timestamp: new Date().toISOString(),
        },
      ],
    })),

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
  clearMessages: () => set({ messages: [], events: [] }),

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

  acceptRun: (sessionId, runId) =>
    set({
      sessionId,
      currentRunId: runId,
      runState: "running",
    }),

  applyEvent: (event) =>
    set((state) => {
      const events = [...state.events, event]
      if (event.event_type === "confirmation_required") {
        return {
          events,
          runState: "awaiting_confirmation",
          confirmation: {
            confirmation_id: event.metadata?.confirmation_id || "",
            run_id: event.run_id,
            risk_level: (event.metadata?.risk_level as any) || "medium",
            action_summary: event.metadata?.action_summary || event.summary,
            reason: event.metadata?.reason || event.summary,
            target_paths: event.metadata?.target_paths ? event.metadata.target_paths.split("\n").filter(Boolean) : [],
            hazards: event.metadata?.hazards ? event.metadata.hazards.split("\n").filter(Boolean) : [],
            alternatives: event.metadata?.alternatives ? event.metadata.alternatives.split("\n").filter(Boolean) : [],
          } as Confirmation,
        }
      }
      if (event.event_type === "run_finished" || event.event_type === "completion") {
        return { events, runState: "completed" }
      }
      if (event.event_type === "error") {
        return { events, runState: "failed", criticalError: event.summary }
      }
      return { events }
    }),

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
}))

// UI Store
interface UIStore {
  activeView: ViewType
  leftSidebarExpanded: boolean
  rightDrawerOpen: boolean
  setActiveView: (view: ViewType) => void
  toggleLeftSidebar: () => void
  toggleRightDrawer: () => void
  setRightDrawerOpen: (open: boolean) => void
}

export const useUIStore = create<UIStore>((set) => ({
  activeView: "task",
  leftSidebarExpanded: false,
  rightDrawerOpen: true,

  setActiveView: (activeView) => set({ activeView }),
  toggleLeftSidebar: () =>
    set((state) => ({ leftSidebarExpanded: !state.leftSidebarExpanded })),
  toggleRightDrawer: () =>
    set((state) => ({ rightDrawerOpen: !state.rightDrawerOpen })),
  setRightDrawerOpen: (rightDrawerOpen) => set({ rightDrawerOpen }),
}))

// Settings Store
interface SettingsStore extends Settings {
  setMode: (mode: Settings["mode"]) => void
  setModel: (model: Settings["model"]) => void
  setWorkspace: (workspace: Settings["workspace"]) => void
  setEmbeddingProvider: (providerId: string) => void
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
    id: "default",
    name: "默认工作区",
    root_path: "/",
  },
  embedding_provider_id: "",
  providers: [],

  setMode: (mode) => set({ mode }),
  setModel: (model) => set({ model }),
  setWorkspace: (workspace) => set({ workspace }),
  setEmbeddingProvider: (embedding_provider_id) => set({ embedding_provider_id }),
  
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
      set({
        mode: data.mode as any,
        model: data.model,
        workspace: data.workspace,
        embedding_provider_id: data.embedding_provider_id,
        providers: data.providers.map((p) => ({
          provider_id: p.provider_id,
          display_name: p.display_name,
          base_url: p.base_url,
          api_key: p.api_key,
          embedding_model: p.embedding_model,
          status: p.api_key ? "active" : "inactive",
        })),
      })
    } catch {
      // ignore load errors
    }
  },
}))

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
  removeMemory: (id: string) => void
}

export const useMemoryStore = create<MemoryStore>((set) => ({
  memories: [],

  setMemories: (memories) => set({ memories }),
  
  addMemory: (memory) =>
    set((state) => ({
      memories: [...state.memories, memory],
    })),

  removeMemory: (id) =>
    set((state) => ({
      memories: state.memories.filter((m) => m.id !== id),
    })),
}))
