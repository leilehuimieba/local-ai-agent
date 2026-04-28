import type { Memory } from "./types"

// Mock Memories
export const mockMemories: Memory[] = [
  {
    id: "mem_001",
    kind: "preference",
    title: "TypeScript 偏好",
    summary: "User prefers TypeScript over JavaScript for all new code",
    content: "User prefers TypeScript over JavaScript for all new code",
    createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 7).toISOString(),
    sourceRunId: "run_001",
  },
  {
    id: "mem_002",
    kind: "fact",
    title: "包管理器",
    summary: "Project uses pnpm as the package manager",
    content: "Project uses pnpm as the package manager",
    createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 5).toISOString(),
  },
  {
    id: "mem_003",
    kind: "fact",
    title: "认证方式",
    summary: "Authentication uses JWT tokens stored in httpOnly cookies",
    content: "Authentication uses JWT tokens stored in httpOnly cookies",
    createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 3).toISOString(),
    sourceRunId: "run_002",
  },
  {
    id: "mem_004",
    kind: "preference",
    title: "样式偏好",
    summary: "User prefers Tailwind CSS for styling",
    content: "User prefers Tailwind CSS for styling",
    createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 2).toISOString(),
  },
]

// Category colors
export const categoryColors: Record<string, string> = {
  Architecture: "#ff6b35",
  API: "#3b82f6",
  Security: "#ef4444",
  Performance: "#10b981",
  Testing: "#8b5cf6",
  Documentation: "#f59e0b",
}
