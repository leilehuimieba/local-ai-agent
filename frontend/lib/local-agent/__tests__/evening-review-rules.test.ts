import { describe, it, expect } from 'vitest'
import {
  createEmptyEveningReviewState,
  refreshEveningReview,
  submitEveningReview,
} from '../evening-review-rules'
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
    eveningReview: createEmptyEveningReviewState(),
    followthrough: { lastActionLabel: null, nextActionLabel: '先交今晚证据', nextActionHelper: '', planUsesLatestEvidence: null },
    execution: { status: 'idle', currentTaskLabel: null, todayCoreTaskCompleted: false, todayClosed: false, statusText: '', helperText: '', activeDate: null, needsReopen: false, staleFromDate: null },
    nextDayPlan: { generatedAt: null, ready: false, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    todayPlanStatusText: '',
    todayPlanReconcileText: null,
    recentPlanHistory: [],
    ...overrides,
  } as MainlineShellState
}

describe('createEmptyEveningReviewState', () => {
  it('returns pending with default text', () => {
    const state = createEmptyEveningReviewState()
    expect(state.status).toBe('pending')
    expect(state.statusText).toBe('今晚待查看')
    expect(state.scheduledLabel).toBe('晚上固定查看')
    expect(state.branchDecision).toBe('none')
  })
})

describe('refreshEveningReview', () => {
  it('returns submitted when already submitted today', () => {
    const today = '2026-05-08'
    const shell = makeShell({
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastSubmittedForDate: today,
        lastReviewedAt: new Date().toISOString(),
      },
    })
    const result = refreshEveningReview(shell, new Date('2026-05-08T10:00:00'))
    expect(result.status).toBe('submitted')
    expect(result.statusText).toBe('今晚证据已提交')
  })

  it('returns pending when no history exists', () => {
    const shell = makeShell()
    const result = refreshEveningReview(shell, new Date('2026-05-08T10:00:00'))
    expect(result.status).toBe('pending')
    expect(result.statusText).toBe('今晚待查看')
  })

  it('returns pending with in-window text after 19:00 when no history', () => {
    const shell = makeShell()
    const result = refreshEveningReview(shell, new Date('2026-05-08T20:00:00'))
    expect(result.status).toBe('pending')
    expect(result.statusText).toBe('现在进入固定查看窗口')
  })

  it('returns late_allowed before noon when yesterday was missed', () => {
    const shell = makeShell({
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: '2026-05-06T20:00:00Z',
        lastSubmittedForDate: '2026-05-06',
      },
    })
    const result = refreshEveningReview(shell, new Date('2026-05-08T10:00:00'))
    expect(result.status).toBe('late_allowed')
    expect(result.statusText).toBe('昨晚证据可补交')
    expect(result.branchDecision).toBe('make_up_yesterday')
  })

  it('returns reroute after noon when yesterday was missed', () => {
    const shell = makeShell({
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: '2026-05-06T20:00:00Z',
        lastSubmittedForDate: '2026-05-06',
      },
    })
    const result = refreshEveningReview(shell, new Date('2026-05-08T14:00:00'))
    expect(result.status).toBe('reroute')
    expect(result.statusText).toBe('补交窗口已过')
  })
})

describe('submitEveningReview', () => {
  it('returns "今晚证据已提交" for normal same-day submission', () => {
    const shell = makeShell()
    const now = new Date('2026-05-08T20:00:00')
    const result = submitEveningReview(shell, now)
    expect(result.status).toBe('submitted')
    expect(result.statusText).toBe('今晚证据已提交')
    expect(result.lastSubmittedForDate).toBe('2026-05-08')
  })

  it('returns "昨晚证据已补交" for making up yesterday before noon', () => {
    const shell = makeShell({
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: '2026-05-06T20:00:00Z',
        lastSubmittedForDate: '2026-05-06',
      },
    })
    const now = new Date('2026-05-08T10:00:00')
    const result = submitEveningReview(shell, now)
    expect(result.status).toBe('submitted')
    expect(result.statusText).toBe('昨晚证据已补交')
    expect(result.lastSubmittedForDate).toBe('2026-05-07')
  })
})
