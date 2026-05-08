import { describe, it, expect } from 'vitest'
import {
  buildEvidenceRefreshState,
  buildEvidenceDraftState,
  buildEvidenceSubmitShell,
  buildKeyEvidenceDraftState,
  applyStrongEvidence,
  applyWeakEvidence,
} from '../evidence-flow-rules'
import { createEmptyEveningReviewState } from '../evening-review-rules'
import { createEmptyExecutionState } from '../execution-state-rules'
import type { MainlineShellState, KeyEvidenceResult } from '../types'

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
    execution: createEmptyExecutionState(),
    nextDayPlan: { generatedAt: null, ready: false, coreTaskLabel: '', supportTaskLabel: '', rationale: '', restoreNote: null, targetDate: null, basedOnEvidenceDate: null, needsRefresh: false, statusText: '', refreshReason: null, todayTaskLabel: null, takeoverStatus: 'idle', takeoverHint: null },
    todayPlanStatusText: '',
    todayPlanReconcileText: null,
    recentPlanHistory: [],
    ...overrides,
  } as MainlineShellState
}

describe('buildEvidenceRefreshState', () => {
  it('keeps probability when evidence is not expired', () => {
    const shell = makeShell({
      probabilityValue: 70,
      probabilityState: 'known',
      lastCriticalEvidenceAt: new Date().toISOString(),
    })
    const result = buildEvidenceRefreshState(shell)
    expect(result.probabilityValue).toBe(70)
    expect(result.probabilityState).toBe('known')
    expect(result.evidenceExpired).toBe(false)
  })

  it('nullifies probability when evidence is expired', () => {
    const shell = makeShell({
      probabilityValue: 70,
      probabilityState: 'known',
      lastCriticalEvidenceAt: new Date(Date.now() - 50 * 60 * 60 * 1000).toISOString(),
    })
    const result = buildEvidenceRefreshState(shell)
    expect(result.probabilityValue).toBeNull()
    expect(result.probabilityState).toBe('unknown')
    expect(result.evidenceExpired).toBe(true)
  })

  it('applies day reset when execution has rolled over', () => {
    const shell = makeShell({
      execution: { ...createEmptyExecutionState(), activeDate: '2026-05-07', todayClosed: true },
    })
    const result = buildEvidenceRefreshState({ ...shell, execution: { ...shell.execution, activeDate: '2026-05-07' } })
    expect(result.execution.needsReopen).toBe(true)
    expect(result.execution.staleFromDate).toBe('2026-05-07')
  })
})

describe('buildEvidenceDraftState', () => {
  it('merges patch into evidence packet and resets submitted flag', () => {
    const shell = makeShell({ evidenceSubmitted: true })
    const result = buildEvidenceDraftState(shell, { didWhat: '完成阅读', resultSummary: '良好' })
    expect(result.evidencePacket.didWhat).toBe('完成阅读')
    expect(result.evidencePacket.resultSummary).toBe('良好')
    expect(result.evidenceSubmitted).toBe(false)
  })
})

describe('buildEvidenceSubmitShell', () => {
  it('marks evidence as submitted and updates evening review', () => {
    const shell = makeShell()
    const result = buildEvidenceSubmitShell(shell)
    expect(result.evidenceSubmitted).toBe(true)
    expect(result.eveningReview.status).toBe('submitted')
    expect(result.followthrough.lastActionLabel).toBe('已提交晚间证据')
  })
})

describe('buildKeyEvidenceDraftState', () => {
  it('merges patch into key evidence entry and clears feedback', () => {
    const shell = makeShell({ keyEvidenceFeedback: '旧反馈' })
    const result = buildKeyEvidenceDraftState(shell, { totalScore: '480', notes: '测试笔记' })
    expect(result.keyEvidenceEntry.totalScore).toBe('480')
    expect(result.keyEvidenceEntry.notes).toBe('测试笔记')
    expect(result.keyEvidenceFeedback).toBeNull()
  })
})

describe('applyStrongEvidence', () => {
  it('updates probability and appends record to history', () => {
    const shell = makeShell()
    const result: KeyEvidenceResult = {
      status: 'strong',
      feedback: '已通过',
      probabilityValue: 82,
      riskLevel: 'green',
      adjustment: '保持节奏',
      record: {
        sourceType: 'mock_exam',
        completedDate: '2026-05-08',
        durationMinutes: 120,
        underExamCondition: true,
        totalScore: 500,
        listeningScore: 160,
        readingScore: 180,
        writingTranslationScore: 160,
        notes: '',
        submittedAt: new Date().toISOString(),
        probabilityAfter: 82,
      },
    }
    const next = applyStrongEvidence(shell, result)
    expect(next.probabilityValue).toBe(82)
    expect(next.probabilityState).toBe('known')
    expect(next.riskLevel).toBe('green')
    expect(next.keyEvidenceHistory).toHaveLength(1)
    expect(next.keyEvidenceFeedback).toBe('已通过')
    expect(next.keyEvidenceEntry.totalScore).toBe('')
  })

  it('works without a record', () => {
    const shell = makeShell()
    const result: KeyEvidenceResult = {
      status: 'strong',
      feedback: '已通过',
      probabilityValue: 60,
      adjustment: '调整',
    }
    const next = applyStrongEvidence(shell, result)
    expect(next.keyEvidenceHistory).toHaveLength(0)
    expect(next.probabilityValue).toBe(60)
  })
})

describe('applyWeakEvidence', () => {
  it('sets feedback and resets entry', () => {
    const shell = makeShell({
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
    })
    const next = applyWeakEvidence(shell, '分数不完整')
    expect(next.keyEvidenceFeedback).toContain('分数不完整')
    expect(next.latestAdjustment).toBe('先补齐完整关键证据，再决定是否调整模块押注。')
    expect(next.keyEvidenceEntry.totalScore).toBe('')
  })
})
