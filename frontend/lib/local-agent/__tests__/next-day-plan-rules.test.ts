import { describe, it, expect } from 'vitest'
import {
  createEmptyNextDayPlan,
  buildNextDayPlan,
  syncNextDayPlan,
} from '../next-day-plan-rules'
import { createDefaultMainlineShell } from '../mainline-default-state'
import type { MainlineShellState } from '../types'

function makeShell(partial: Partial<MainlineShellState> = {}): MainlineShellState {
  return { ...createDefaultMainlineShell(), ...partial }
}

describe('createEmptyNextDayPlan', () => {
  it('returns default empty plan', () => {
    const plan = createEmptyNextDayPlan()
    expect(plan.ready).toBe(false)
    expect(plan.coreTaskLabel).toBe('请先补关键证据后再生成明日计划')
    expect(plan.supportTaskLabel).toBe('暂无补充任务')
    expect(plan.generatedAt).toBeNull()
    expect(plan.targetDate).toBeNull()
    expect(plan.needsRefresh).toBe(false)
    expect(plan.takeoverStatus).toBe('idle')
  })
})

describe('buildNextDayPlan', () => {
  it('returns restore note when temporary mainline is active', () => {
    const shell = makeShell({
      temporaryMainline: {
        isActive: true,
        reviewStatus: 'approved',
        reviewFeedback: null,
        currentTemporaryGoal: '临时目标',
        originalSnapshot: null,
        currentSwitchStartedAt: null,
        activeRecordId: null,
        history: [],
      },
      currentGoalLabel: '原目标',
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.ready).toBe(true)
    expect(plan.coreTaskLabel).toContain('恢复后优先回到')
    expect(plan.coreTaskLabel).toContain('原目标')
    expect(plan.restoreNote).toContain('临时主线')
  })

  it('asks to collect evidence when probability is unknown', () => {
    const shell = makeShell({
      probabilityState: 'unknown',
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
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.coreTaskLabel).toContain('先补 1 份完整关键证据')
  })

  it('uses time budget core task when time budget is critical', () => {
    const shell = makeShell({
      probabilityState: 'known',
      timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 30, level: 'critical', recommendation: '极紧' },
      timeBudgetEntry: { coreTaskLabel: '今天只保听力', budgetChangeNote: '', timeBlockDraft: { startTime: '', endTime: '', label: '' } },
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
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.coreTaskLabel).toBe('今天只保听力')
  })

  it('uses latest adjustment as core task in normal case', () => {
    const shell = makeShell({
      probabilityState: 'known',
      latestAdjustment: '保持词汇记忆主轴',
      timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 180, level: 'steady', recommendation: '正常' },
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
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.coreTaskLabel).toBe('保持词汇记忆主轴')
  })

  it('sets support task based on boost task type', () => {
    const shell = makeShell({
      personalizedInsight: {
        updatedAt: null,
        basedOnCount: 2,
        preferredTaskType: 'reading',
        boostTaskType: 'reading',
        riskTaskType: null,
        recommendation: '继续阅读',
      },
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
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.supportTaskLabel).toContain('阅读')
  })

  it('sets support task to extra goal when time budget is ample', () => {
    const shell = makeShell({
      timeBudgetInsight: { updatedAt: null, totalAvailableMinutes: 300, level: 'ample', recommendation: '宽松' },
      personalizedInsight: {
        updatedAt: null,
        basedOnCount: 0,
        preferredTaskType: null,
        boostTaskType: null,
        riskTaskType: null,
        recommendation: '',
      },
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
    })
    const plan = buildNextDayPlan(shell)
    expect(plan.supportTaskLabel).toContain('再补 1 个次目标')
  })
})

describe('syncNextDayPlan', () => {
  it('returns unchanged plan when not ready', () => {
    const shell = makeShell()
    const result = syncNextDayPlan(shell)
    expect(result).toEqual(shell.nextDayPlan)
  })

  it('marks refresh when target date has passed', () => {
    const now = new Date('2024-01-15T10:00:00.000Z')
    const shell = makeShell({
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: '2024-01-14',
        generatedAt: '2024-01-13T10:00:00.000Z',
      },
    })
    const result = syncNextDayPlan(shell, now)
    expect(result.needsRefresh).toBe(true)
    expect(result.refreshReason).toContain('目标日已过')
  })

  it('marks refresh when evidence is expired', () => {
    const now = new Date('2024-01-15T10:00:00.000Z')
    const shell = makeShell({
      evidenceExpired: true,
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: '2024-01-15',
        generatedAt: '2024-01-14T10:00:00.000Z',
      },
    })
    const result = syncNextDayPlan(shell, now)
    expect(result.needsRefresh).toBe(true)
    expect(result.refreshReason).toContain('过期')
  })

  it('updates takeover status when execution is executing and today matches target date', () => {
    const now = new Date('2024-01-15T10:00:00.000Z')
    const shell = makeShell({
      execution: {
        status: 'executing',
        currentTaskLabel: '听力训练',
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: '',
        helperText: '',
        activeDate: '2024-01-15',
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: '2024-01-15',
        generatedAt: '2024-01-14T10:00:00.000Z',
      },
    })
    const result = syncNextDayPlan(shell, now)
    expect(result.takeoverStatus).toBe('taken_over')
    expect(result.todayTaskLabel).toBe('听力训练')
  })

  it('updates takeover status to completed when core task is done', () => {
    const now = new Date('2024-01-15T10:00:00.000Z')
    const shell = makeShell({
      execution: {
        status: 'completed',
        currentTaskLabel: '听力训练',
        todayCoreTaskCompleted: true,
        todayClosed: false,
        statusText: '',
        helperText: '',
        activeDate: '2024-01-15',
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: '2024-01-15',
        generatedAt: '2024-01-14T10:00:00.000Z',
      },
    })
    const result = syncNextDayPlan(shell, now)
    expect(result.takeoverStatus).toBe('completed')
  })

  it('updates takeover status to closed when today is closed', () => {
    const now = new Date('2024-01-15T10:00:00.000Z')
    const shell = makeShell({
      execution: {
        status: 'closed',
        currentTaskLabel: '听力训练',
        todayCoreTaskCompleted: true,
        todayClosed: true,
        statusText: '',
        helperText: '',
        activeDate: '2024-01-15',
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: '2024-01-15',
        generatedAt: '2024-01-14T10:00:00.000Z',
      },
    })
    const result = syncNextDayPlan(shell, now)
    expect(result.takeoverStatus).toBe('closed')
  })
})
