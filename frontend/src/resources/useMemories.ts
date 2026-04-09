import { useEffect, useState } from "react";

import { MemoryEntry } from "../shared/contracts";
import { deleteMemory, fetchMemories } from "./api";

type SyncMemoryCount = (count: number) => void;
type MemoryActionKind = "refresh" | "delete";
type MemoryActionFeedback = {
  action: MemoryActionKind;
  message: string;
};
type MemoryStateSync = {
  setMemories: (items: MemoryEntry[]) => void;
  setMemoryError: (value: string | null) => void;
  syncMemoryCount?: SyncMemoryCount;
};

export function useMemories(syncMemoryCount?: SyncMemoryCount) {
  const state = useMemoryState(syncMemoryCount);
  useMemoryBootstrap(state, syncMemoryCount);
  return {
    memories: state.memories,
    memoryError: state.memoryError,
    memoryPendingAction: state.pendingAction,
    memoryActionError: state.actionError,
    memoryActionSuccess: state.actionSuccess,
    deletingId: state.deletingId,
    refreshMemories: () => refreshMemories(state, syncMemoryCount),
    removeMemory: (memoryId: string) => removeMemory(memoryId, state, syncMemoryCount),
  };
}

function useMemoryState(syncMemoryCount?: SyncMemoryCount) {
  const [memories, setMemories] = useState<MemoryEntry[]>([]);
  const [memoryError, setMemoryError] = useState<string | null>(null);
  const [pendingAction, setPendingAction] = useState<MemoryActionFeedback | null>(null);
  const [actionError, setActionError] = useState<MemoryActionFeedback | null>(null);
  const [actionSuccess, setActionSuccess] = useState<MemoryActionFeedback | null>(null);
  const [deletingId, setDeletingId] = useState("");
  return {
    actionError,
    actionSuccess,
    deletingId,
    memories,
    memoryError,
    pendingAction,
    setActionError,
    setActionSuccess,
    setDeletingId,
    setMemories,
    setMemoryError,
    setPendingAction,
    syncMemoryCount,
  };
}

function useMemoryBootstrap(
  state: ReturnType<typeof useMemoryState>,
  syncMemoryCount?: SyncMemoryCount,
) {
  useEffect(() => {
    const controller = new AbortController();
    void loadInitialMemories(controller.signal, state, syncMemoryCount);
    return () => controller.abort();
  }, []);
}

async function loadInitialMemories(
  signal: AbortSignal,
  state: ReturnType<typeof useMemoryState>,
  syncMemoryCount?: SyncMemoryCount,
) {
  try {
    const items = await fetchMemories(signal);
    applyMemories(items, createMemoryStateSync(state, syncMemoryCount));
  } catch (error) {
    if (!signal.aborted) state.setMemoryError(readMemoryError(error));
  }
}

async function refreshMemories(
  state: ReturnType<typeof useMemoryState>,
  syncMemoryCount?: SyncMemoryCount,
) {
  return runMemoryAction({
    action: createMemoryFeedback("refresh", "正在刷新记忆列表。"),
    execute: async () => fetchMemories(),
    onSuccess: (items) => {
      applyMemories(items, createMemoryStateSync(state, syncMemoryCount));
      return createMemoryFeedback("refresh", `记忆列表已刷新，当前共 ${items.length} 条。`);
    },
    state,
  });
}

async function removeMemory(
  memoryId: string,
  state: ReturnType<typeof useMemoryState>,
  syncMemoryCount?: SyncMemoryCount,
) {
  return runMemoryAction({
    action: createMemoryFeedback("delete", "正在删除记忆条目。"),
    execute: async () => deleteMemory(memoryId),
    onStart: () => state.setDeletingId(memoryId),
    onSuccess: (items) => {
      applyMemories(items, createMemoryStateSync(state, syncMemoryCount));
      return createMemoryFeedback("delete", `删除成功，当前列表剩余 ${items.length} 条。`);
    },
    state,
  });
}

async function runMemoryAction(args: {
  action: MemoryActionFeedback;
  execute: () => Promise<MemoryEntry[]>;
  onStart?: () => void;
  onSuccess: (items: MemoryEntry[]) => MemoryActionFeedback;
  state: ReturnType<typeof useMemoryState>;
}) {
  setMemoryActionStarted(args.state, args.action);
  args.onStart?.();
  try {
    const items = await args.execute();
    args.state.setActionSuccess(args.onSuccess(items));
    return items;
  } catch (error) {
    const message = readMemoryError(error);
    args.state.setMemoryError(message);
    args.state.setActionError(createMemoryFeedback(args.action.action, message));
    throw error;
  } finally {
    clearMemoryAction(args.state);
  }
}

function setMemoryActionStarted(
  state: ReturnType<typeof useMemoryState>,
  action: MemoryActionFeedback,
) {
  state.setPendingAction(action);
  state.setActionError(null);
  state.setActionSuccess(null);
  state.setMemoryError(null);
}

function clearMemoryAction(state: ReturnType<typeof useMemoryState>) {
  state.setPendingAction(null);
  state.setDeletingId("");
}

function applyMemories(items: MemoryEntry[], state: MemoryStateSync) {
  const normalized = normalizeMemories(items);
  state.setMemories(normalized);
  state.setMemoryError(null);
  state.syncMemoryCount?.(normalized.length);
}

function readMemoryError(error: unknown) {
  return error instanceof Error ? error.message : "加载记忆失败";
}

function createMemoryStateSync(
  state: ReturnType<typeof useMemoryState>,
  syncMemoryCount?: SyncMemoryCount,
) {
  return {
    setMemories: state.setMemories,
    setMemoryError: state.setMemoryError,
    syncMemoryCount,
  };
}

function createMemoryFeedback(action: MemoryActionKind, message: string) {
  return { action, message };
}

export function readMemoriesErrorMessage(error: unknown) {
  return readMemoryError(error);
}

function normalizeMemories(items: MemoryEntry[]) {
  return [...items].map(normalizeMemory).sort(compareMemories);
}

function normalizeMemory(memory: MemoryEntry) {
  return {
    ...memory,
    content: memory.content?.trim() || "",
    reason: memory.reason?.trim() || "",
    summary: memory.summary?.trim() || memory.title?.trim() || "未提供摘要",
    title: memory.title?.trim() || memory.summary?.trim() || "未命名记忆",
  };
}

function compareMemories(left: MemoryEntry, right: MemoryEntry) {
  return readMemoryTime(right) - readMemoryTime(left);
}

function readMemoryTime(memory: MemoryEntry) {
  const value = memory.updated_at || memory.timestamp || memory.created_at;
  const time = Date.parse(value);
  return Number.isNaN(time) ? 0 : time;
}
