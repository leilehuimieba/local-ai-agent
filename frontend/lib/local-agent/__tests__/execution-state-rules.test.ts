import { describe, it, expect } from 'vitest'
import {
  createEmptyExecutionState,
  buildExecutionAfterTimeBudget,
  buildExecutionAfterCoreDone,
  buildExecutionAfterClose,
  hasExecutionRolledOver,
  buildExecutionAfterDayReset,
} from '../execution-state-rules'
import type { MainlineShellState } from '../types'

function makeShell(overrides: Partial<MainlineShellState> = {}): MainlineShellState {
  return {
    currentGoalLabel: '测试目标',
    probabilityValue: null,
    probabilityState: 'unknown',
    riskLevel: 'yellow',
    evidenceExpired: false,
    evidenceSubmitted: false,
    lastCriticalEvidenceAt: null,
    latestAdjustment: '',
    keyEvidenceFeedback: null,
    switchFeedback: null,
    evidencePacket: { didWhat: '', resultSummary: '', blockers: '', nextAdjustment: '' },
    keyEvidenceEntry: {
      sourceType: 'mock_exam',
      completedDate: '',
      durationMinutes: '',
      underExamCondition: false,
      totalScore: '',
      listeningScore: '',
      readingScore: '',
      writingTranslationScore: '',
      notes: '',
    },
    keyEvidenceHistory: [],
    switchForm: { temporaryGoalLabel: '', reason: '', dueDate: '', urgent: false, important: false, insistAfterReject: false },
    temporaryMainline: {
      isActive: false,
      reviewStatus: 'idle',
      reviewFeedback: null,
      currentTemporaryGoal: null,
      originalSnapshot: null,
      currentSwitchStartedAt: null,
      activeRecordId: null,
      history: [],
    },
    personalizedFeedback: null,
    personalizedEntry: { taskType: 'vocabulary', isCoreTask: true, completed: null, hasShortTermGain: null, resultNote: '' },
    personalizedHistory: [],
    personalizedInsight: { updatedAt: null, basedOnCount: 0, preferredTaskType: null, boostTaskType: null, riskTaskType: null, recommendation: '' },
    timeBudgetFeedback: null,
    timeBudgetEntry: { coreTaskLabel: '核心任务A', budgetChangeNote: '', timeBlockDraft: { startTime: '', endTime: '', label: '' } },
    timeBlocks: [],
    timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 0, level: 'steady', recommendation: '' },
    eveningReview: { scheduledLabel: '', status: 'pending', statusText: '', guidance: '', deadlineLabel: null, branchDecision: 'none', branchText: '', lastReviewedAt: null, lastSubmittedForDate: null },
    followthrough: { lastActionLabel: null, nextActionLabel: '', nextActionHelper: '', planUsesLatestEvidence: null },
    execution: createEmptyExecutionState(),
    nextDayPlan: { generatedAt: null, ready: false, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    todayPlanStatusText: '',
    todayPlanReconcileText: null,
    recentPlanHistory: [],
    ...overrides,
  } as MainlineShellState
}

describe('createEmptyExecutionState', () => {
  it('returns idle status with defaults', () => {
    const state = createEmptyExecutionState()
    expect(state.status).toBe('idle')
    expect(state.todayCoreTaskCompleted).toBe(false)
    expect(state.todayClosed).toBe(false)
    expect(state.activeDate).toBeNull()
  })
})

describe('buildExecutionAfterTimeBudget', () => {
  it('returns executing status and sets activeDate', () => {
    const shell = makeShell()
    const now = new Date('2026-05-08T09:00:00')
    const state = buildExecutionAfterTimeBudget(shell, now)
    expect(state.status).toBe('executing')
    expect(state.activeDate).toBe('2026-05-08')
    expect(state.todayCoreTaskCompleted).toBe(false)
    expect(state.todayClosed).toBe(false)
  })

  it('picks current task label from time budget entry', () => {
    const shell = makeShell({ timeBudgetEntry: { coreTaskLabel: '阅读训练', budgetChangeNote: '', timeBlockDraft: { startTime: '', endTime: '', label: '' } } })
    const state = buildExecutionAfterTimeBudget(shell)
    expect(state.currentTaskLabel).toBe('阅读训练')
  })
})

describe('buildExecutionAfterCoreDone', () => {
  it('returns completed with todayCoreTaskCompleted true', () => {
    const shell = makeShell({ execution: { ...createEmptyExecutionState(), activeDate: '2026-05-08' } })
    const now = new Date('2026-05-08T15:00:00')
    const state = buildExecutionAfterCoreDone(shell, now)
    expect(state.status).toBe('completed')
    expect(state.todayCoreTaskCompleted).toBe(true)
    expect(state.todayClosed).toBe(false)
  })
})

describe('buildExecutionAfterClose', () => {
  it('returns closed with todayClosed true', () => {
    const shell = makeShell({ execution: { ...createEmptyExecutionState(), activeDate: '2026-05-08' } })
    const now = new Date('2026-05-08T20:00:00')
    const state = buildExecutionAfterClose(shell, now)
    expect(state.status).toBe('closed')
    expect(state.todayCoreTaskCompleted).toBe(true)
    expect(state.todayClosed).toBe(true)
  })
})

describe('hasExecutionRolledOver', () => {
  it('returns false when activeDate matches today', () => {
    const shell = makeShell({ execution: { ...createEmptyExecutionState(), activeDate: '2026-05-08' } })
    const now = new Date('2026-05-08T10:00:00')
    expect(hasExecutionRolledOver(shell, now)).toBe(false)
  })

  it('returns true when activeDate is yesterday', () => {
    const shell = makeShell({ execution: { ...createEmptyExecutionState(), activeDate: '2026-05-07' } })
    const now = new Date('2026-05-08T10:00:00')
    expect(hasExecutionRolledOver(shell, now)).toBe(true)
  })

  it('returns false when activeDate is null', () => {
    const shell = makeShell({ execution: { ...createEmptyExecutionState(), activeDate: null } })
    expect(hasExecutionRolledOver(shell)).toBe(false)
  })
})

describe('buildExecutionAfterDayReset', () => {
  it('returns reopen message when previous day was closed', () => {
    const shell = makeShell({
      execution: { ...createEmptyExecutionState(), activeDate: '2026-05-07', todayClosed: true },
    })
    const now = new Date('2026-05-08T08:00:00')
    const state = buildExecutionAfterDayReset(shell, now)
    expect(state.status).toBe('idle')
    expect(state.needsReopen).toBe(true)
    expect(state.statusText).toBe('新的一天待重开')
    expect(state.staleFromDate).toBe('2026-05-07')
  })

  it('returns unclosed message when previous day was not closed', () => {
    const shell = makeShell({
      execution: { ...createEmptyExecutionState(), activeDate: '2026-05-07', todayClosed: false },
    })
    const now = new Date('2026-05-08T08:00:00')
    const state = buildExecutionAfterDayReset(shell, now)
    expect(state.status).toBe('idle')
    expect(state.statusText).toBe('昨日未收尾，今天待重开')
  })
})
