import type {
  KnowledgeItem,
  LogRun,
  Memory,
  Message,
  RuntimeEvent,
} from "./types"

// Mock Knowledge Items
export const mockKnowledgeItems: KnowledgeItem[] = [
  {
    id: "k1",
    title: "React Server Components Best Practices",
    summary: "Guidelines for implementing React Server Components effectively in Next.js applications.",
    content: `# React Server Components Best Practices

## Overview
React Server Components (RSC) allow you to render components on the server, reducing the JavaScript bundle size sent to the client.

## Key Principles
1. **Use Server Components by default** - Only add "use client" when necessary
2. **Keep data fetching in Server Components** - Avoid client-side data fetching for initial renders
3. **Pass serializable props** - Server Components can only pass serializable data to Client Components

## Example
\`\`\`tsx
// Server Component (default)
async function UserProfile({ userId }: { userId: string }) {
  const user = await fetchUser(userId);
  return <ProfileCard user={user} />;
}
\`\`\``,
    category: "Architecture",
    tags: ["React", "Next.js", "Performance"],
    citationCount: 24,
    source: "internal-docs",
    createdAt: "2024-01-15T10:00:00Z",
    updatedAt: "2024-03-20T14:30:00Z",
  },
  {
    id: "k2",
    title: "API Rate Limiting Strategy",
    summary: "Implementation details for rate limiting across all API endpoints using Redis.",
    content: `# API Rate Limiting Strategy

## Implementation
We use a sliding window algorithm with Redis for rate limiting.

### Configuration
- Standard users: 100 requests/minute
- Premium users: 1000 requests/minute
- Enterprise: Custom limits

## Code Example
\`\`\`typescript
const rateLimiter = new RateLimiter({
  windowMs: 60000,
  max: 100,
  store: redisStore,
});
\`\`\``,
    category: "API",
    tags: ["Security", "Redis", "Performance"],
    citationCount: 18,
    source: "engineering-wiki",
    createdAt: "2024-02-10T09:00:00Z",
    updatedAt: "2024-03-18T11:00:00Z",
  },
  {
    id: "k3",
    title: "Authentication Flow Documentation",
    summary: "Complete authentication flow including OAuth 2.0, JWT tokens, and session management.",
    content: `# Authentication Flow

## Supported Methods
- Email/Password
- OAuth 2.0 (Google, GitHub)
- Magic Links

## Token Structure
JWTs are signed with RS256 and contain user roles and permissions.`,
    category: "Security",
    tags: ["Auth", "OAuth", "JWT"],
    citationCount: 42,
    source: "security-docs",
    createdAt: "2024-01-05T08:00:00Z",
    updatedAt: "2024-03-22T16:00:00Z",
  },
  {
    id: "k4",
    title: "Database Query Optimization Guide",
    summary: "Best practices for optimizing PostgreSQL queries and indexing strategies.",
    content: `# Database Query Optimization

## Index Types
- B-tree (default, most common)
- Hash (equality comparisons)
- GIN (full-text search)
- GiST (geometric data)

## Query Analysis
Always use EXPLAIN ANALYZE to understand query performance.`,
    category: "Performance",
    tags: ["PostgreSQL", "Database", "Optimization"],
    citationCount: 31,
    source: "dba-handbook",
    createdAt: "2024-02-20T11:00:00Z",
    updatedAt: "2024-03-19T09:00:00Z",
  },
  {
    id: "k5",
    title: "E2E Testing with Playwright",
    summary: "Comprehensive guide to end-to-end testing using Playwright framework.",
    content: `# E2E Testing with Playwright

## Setup
\`\`\`bash
npm init playwright@latest
\`\`\`

## Writing Tests
\`\`\`typescript
test('user can login', async ({ page }) => {
  await page.goto('/login');
  await page.fill('[name="email"]', 'test@example.com');
  await page.click('button[type="submit"]');
});
\`\`\``,
    category: "Testing",
    tags: ["Playwright", "E2E", "QA"],
    citationCount: 15,
    source: "qa-docs",
    createdAt: "2024-03-01T14:00:00Z",
    updatedAt: "2024-03-21T10:00:00Z",
  },
  {
    id: "k6",
    title: "API Documentation Standards",
    summary: "Standards and templates for documenting REST and GraphQL APIs.",
    content: `# API Documentation Standards

## OpenAPI Specification
All REST APIs must be documented using OpenAPI 3.0 spec.

## Required Sections
- Authentication
- Endpoints
- Request/Response examples
- Error codes`,
    category: "Documentation",
    tags: ["OpenAPI", "REST", "GraphQL"],
    citationCount: 12,
    source: "style-guide",
    createdAt: "2024-02-28T13:00:00Z",
    updatedAt: "2024-03-15T08:00:00Z",
  },
]

// Mock Log Runs
export const mockLogRuns: LogRun[] = [
  {
    run_id: "run_001",
    session_id: "session_abc",
    title: "Analyze codebase architecture",
    status: "completed",
    started_at: new Date(Date.now() - 1000 * 60 * 30).toISOString(),
    completed_at: new Date(Date.now() - 1000 * 60 * 25).toISOString(),
    duration_ms: 300000,
    event_count: 12,
  },
  {
    run_id: "run_002",
    session_id: "session_abc",
    title: "Generate unit tests for utils",
    status: "completed",
    started_at: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
    completed_at: new Date(Date.now() - 1000 * 60 * 55).toISOString(),
    duration_ms: 300000,
    event_count: 8,
  },
  {
    run_id: "run_003",
    session_id: "session_abc",
    title: "Refactor authentication module",
    status: "failed",
    started_at: new Date(Date.now() - 1000 * 60 * 90).toISOString(),
    completed_at: new Date(Date.now() - 1000 * 60 * 88).toISOString(),
    duration_ms: 120000,
    event_count: 5,
  },
  {
    run_id: "run_004",
    session_id: "session_abc",
    title: "Update API documentation",
    status: "running",
    started_at: new Date(Date.now() - 1000 * 60 * 5).toISOString(),
    duration_ms: 0,
    event_count: 3,
  },
  {
    run_id: "run_005",
    session_id: "session_abc",
    title: "Database migration for user table",
    status: "completed",
    started_at: new Date(Date.now() - 1000 * 60 * 120).toISOString(),
    completed_at: new Date(Date.now() - 1000 * 60 * 118).toISOString(),
    duration_ms: 120000,
    event_count: 6,
  },
]

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

// Mock Messages for demo
export const mockMessages: Message[] = [
  {
    id: "msg_001",
    role: "user",
    content: "Analyze the current authentication implementation and suggest improvements",
    timestamp: new Date(Date.now() - 1000 * 60 * 10).toISOString(),
  },
  {
    id: "msg_002",
    role: "assistant",
    content: "I'll analyze your authentication implementation. Let me examine the relevant files.",
    blocks: [
      {
        type: "text",
        content: "## Analysis Results\n\nI've reviewed your authentication implementation and found the following:",
      },
      {
        type: "list",
        items: [
          "JWT tokens are properly signed with RS256",
          "Refresh token rotation is implemented correctly",
          "Missing CSRF protection on some endpoints",
          "Session timeout is set to 24 hours (consider reducing)",
        ],
      },
      {
        type: "code",
        language: "typescript",
        content: `// Suggested improvement for CSRF protection
import { csrf } from '@/lib/security';

export async function POST(request: Request) {
  await csrf.verify(request);
  // ... rest of handler
}`,
      },
    ],
    timestamp: new Date(Date.now() - 1000 * 60 * 9).toISOString(),
  },
]

// Mock Events for running state
export const mockRunningEvents: RuntimeEvent[] = [
  {
    event_id: "evt_001",
    event_type: "tool_call",
    stage: "end",
    summary: "Reading file: src/lib/auth.ts",
    timestamp: new Date(Date.now() - 1000 * 30).toISOString(),
  },
  {
    event_id: "evt_002",
    event_type: "tool_call",
    stage: "end",
    summary: "Analyzing authentication patterns",
    timestamp: new Date(Date.now() - 1000 * 20).toISOString(),
  },
  {
    event_id: "evt_003",
    event_type: "thinking",
    stage: "progress",
    summary: "Generating security recommendations...",
    timestamp: new Date(Date.now() - 1000 * 10).toISOString(),
  },
]

// Quick start prompts
export const quickStartPrompts = [
  "Analyze the codebase architecture",
  "Find and fix potential bugs",
  "Generate unit tests for utils",
  "Explain this repository structure",
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
