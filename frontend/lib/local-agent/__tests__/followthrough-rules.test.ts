import { describe, it, expect } from 'vitest'
import {
  createEmptyFollowthroughState,
  buildFollowthroughAfterEvidence,
  buildFollowthroughAfterKeyEvidence,
  buildFollowthroughAfterPlan,
  buildFollowthroughAfterTimeBudget,
  buildFollowthroughAfterCoreDone,
  buildFollowthroughAfterClose,
  buildFollowthroughAfterDayReset,
} from '../followthrough-rules'
import type { MainlineShellState } from '../types'

function makeShell(overrides: Partial<MainlineShellState> = {}): MainlineShellState {
  return {
    currentGoalLabel: '测试目标',
    probabilityValue: 70,
    probabilityState: 'known',
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
    timeBudgetEntry: { coreTaskLabel: '核心任务', budgetChangeNote: '', timeBlockDraft: { startTime: '', endTime: '', label: '' } },
    timeBlocks: [],
    timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 0, level: 'steady', recommendation: '' },
    eveningReview: { scheduledLabel: '', status: 'pending', statusText: '', guidance: '', deadlineLabel: null, branchDecision: 'none', branchText: '', lastReviewedAt: null, lastSubmittedForDate: null },
    followthrough: createEmptyFollowthroughState(),
    execution: { status: 'idle', currentTaskLabel: null, todayCoreTaskCompleted: false, todayClosed: false, statusText: '', helperText: '', activeDate: null, needsReopen: false, staleFromDate: null },
    nextDayPlan: { generatedAt: null, ready: false, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    todayPlanStatusText: '',
    todayPlanReconcileText: null,
    recentPlanHistory: [],
    ...overrides,
  } as MainlineShellState
}

describe('createEmptyFollowthroughState', () => {
  it('returns default followthrough state', () => {
    const state = createEmptyFollowthroughState()
    expect(state.nextActionLabel).toBe('先交今晚证据')
    expect(state.lastActionLabel).toBeNull()
    expect(state.planUsesLatestEvidence).toBeNull()
  })
})

describe('buildFollowthroughAfterEvidence', () => {
  it('points to key evidence when probability is unknown', () => {
    const shell = makeShell({ probabilityState: 'unknown' })
    const state = buildFollowthroughAfterEvidence(shell)
    expect(state.lastActionLabel).toBe('已提交晚间证据')
    expect(state.nextActionHelper).toContain('优先补关键证据')
    expect(state.planUsesLatestEvidence).toBe(false)
  })

  it('points to plan when probability is known', () => {
    const shell = makeShell({ probabilityState: 'known' })
    const state = buildFollowthroughAfterEvidence(shell)
    expect(state.lastActionLabel).toBe('已提交晚间证据')
    expect(state.planUsesLatestEvidence).toBe(true)
  })
})

describe('buildFollowthroughAfterKeyEvidence', () => {
  it('suggests protecting today when time is critical', () => {
    const shell = makeShell({
      timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 30, level: 'critical', recommendation: '' },
    })
    const state = buildFollowthroughAfterKeyEvidence(shell)
    expect(state.nextActionLabel).toBe('优先保今天核心任务')
    expect(state.planUsesLatestEvidence).toBe(true)
  })

  it('suggests generating next-day plan when time is not critical and plan not ready', () => {
    const shell = makeShell({
      nextDayPlan: { generatedAt: null, ready: false, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    })
    const state = buildFollowthroughAfterKeyEvidence(shell)
    expect(state.nextActionLabel).toBe('优先生成明日计划')
  })

  it('suggests updating plan when plan is ready', () => {
    const shell = makeShell({
      nextDayPlan: { generatedAt: null, ready: true, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    })
    const state = buildFollowthroughAfterKeyEvidence(shell)
    expect(state.nextActionLabel).toBe('更新明日计划')
  })
})

describe('buildFollowthroughAfterPlan', () => {
  it('marks plan as using latest evidence when conditions met', () => {
    const shell = makeShell({ probabilityState: 'known', evidenceExpired: false })
    const state = buildFollowthroughAfterPlan(shell)
    expect(state.lastActionLabel).toBe('已生成明日计划')
    expect(state.nextActionLabel).toBe('按计划执行')
    expect(state.planUsesLatestEvidence).toBe(true)
  })

  it('marks plan as not using latest evidence when expired', () => {
    const shell = makeShell({ probabilityState: 'known', evidenceExpired: true })
    const state = buildFollowthroughAfterPlan(shell)
    expect(state.planUsesLatestEvidence).toBe(false)
  })
})

describe('buildFollowthroughAfterTimeBudget', () => {
  it('suggests completing core task', () => {
    const shell = makeShell({ execution: { status: 'executing', currentTaskLabel: '阅读训练', todayCoreTaskCompleted: false, todayClosed: false, statusText: '', helperText: '', activeDate: '2026-05-08', needsReopen: false, staleFromDate: null } })
    const state = buildFollowthroughAfterTimeBudget(shell)
    expect(state.lastActionLabel).toBe('已接管今天主线')
    expect(state.nextActionLabel).toBe('标记今天核心任务完成')
    expect(state.nextActionHelper).toContain('阅读训练')
  })
})

describe('buildFollowthroughAfterCoreDone', () => {
  it('suggests closing the day', () => {
    const shell = makeShell()
    const state = buildFollowthroughAfterCoreDone(shell)
    expect(state.lastActionLabel).toBe('已完成今天核心任务')
    expect(state.nextActionLabel).toBe('进入今天收尾')
  })
})

describe('buildFollowthroughAfterClose', () => {
  it('suggests waiting for new evidence', () => {
    const shell = makeShell()
    const state = buildFollowthroughAfterClose(shell)
    expect(state.lastActionLabel).toBe('已进入今天收尾')
    expect(state.nextActionLabel).toBe('等待新的关键证据')
  })
})

describe('buildFollowthroughAfterDayReset', () => {
  it('shows closed message when yesterday was closed', () => {
    const shell = makeShell({ execution: { status: 'closed', currentTaskLabel: null, todayCoreTaskCompleted: true, todayClosed: true, statusText: '', helperText: '', activeDate: '2026-05-07', needsReopen: false, staleFromDate: null } })
    const state = buildFollowthroughAfterDayReset(shell)
    expect(state.lastActionLabel).toBe('昨天已收尾')
    expect(state.nextActionLabel).toBe('重开今天主线')
  })

  it('shows unclosed message when yesterday was not closed', () => {
    const shell = makeShell({ execution: { status: 'executing', currentTaskLabel: null, todayCoreTaskCompleted: false, todayClosed: false, statusText: '', helperText: '', activeDate: '2026-05-07', needsReopen: false, staleFromDate: null } })
    const state = buildFollowthroughAfterDayReset(shell)
    expect(state.lastActionLabel).toBe('昨天未收尾')
    expect(state.nextActionLabel).toBe('重开今天主线')
  })
})
