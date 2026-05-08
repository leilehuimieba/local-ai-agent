import { describe, it, expect } from 'vitest'
import {
  createEmptySwitchForm,
  reviewTemporaryMainline,
  buildMainlineSnapshot,
  buildSwitchHistory,
  buildTemporaryAdjustment,
  buildRestoreAdjustment,
} from '../mainline-switch-rules'
import type { MainlineSnapshot, TemporaryMainlineForm } from '../types'

describe('createEmptySwitchForm', () => {
  it('returns default empty switch form', () => {
    const form = createEmptySwitchForm()
    expect(form.temporaryGoalLabel).toBe('')
    expect(form.reason).toBe('')
    expect(form.dueDate).toBe('')
    expect(form.urgent).toBe(true)
    expect(form.important).toBe(true)
    expect(form.insistAfterReject).toBe(false)
  })
})

describe('reviewTemporaryMainline', () => {
  it('approves when urgent and important', () => {
    const form: TemporaryMainlineForm = {
      temporaryGoalLabel: '临时目标',
      reason: 'reason',
      dueDate: '2024-01-01',
      urgent: true,
      important: true,
      insistAfterReject: false,
    }
    const result = reviewTemporaryMainline(form)
    expect(result.reviewStatus).toBe('approved')
    expect(result.approvedByReview).toBe(true)
    expect(result.userInsisted).toBe(false)
  })

  it('rejects when not urgent and not important', () => {
    const form: TemporaryMainlineForm = {
      temporaryGoalLabel: '临时目标',
      reason: 'reason',
      dueDate: '2024-01-01',
      urgent: false,
      important: false,
      insistAfterReject: false,
    }
    const result = reviewTemporaryMainline(form)
    expect(result.reviewStatus).toBe('rejected')
    expect(result.approvedByReview).toBe(false)
    expect(result.userInsisted).toBe(false)
  })

  it('rejects with userInsisted when insistAfterReject is true', () => {
    const form: TemporaryMainlineForm = {
      temporaryGoalLabel: '临时目标',
      reason: 'reason',
      dueDate: '2024-01-01',
      urgent: false,
      important: false,
      insistAfterReject: true,
    }
    const result = reviewTemporaryMainline(form)
    expect(result.reviewStatus).toBe('rejected')
    expect(result.approvedByReview).toBe(false)
    expect(result.userInsisted).toBe(true)
  })
})

describe('buildMainlineSnapshot', () => {
  it('returns a shallow copy of the input snapshot', () => {
    const input: MainlineSnapshot = {
      currentGoalLabel: '四级冲刺',
      probabilityValue: 70,
      probabilityState: 'known',
      riskLevel: 'yellow',
      evidenceExpired: false,
      lastCriticalEvidenceAt: '2024-01-01T00:00:00.000Z',
      latestAdjustment: '保持节奏',
    }
    const result = buildMainlineSnapshot(input)
    expect(result).toEqual(input)
    expect(result).not.toBe(input)
  })
})

describe('buildSwitchHistory', () => {
  it('returns history record from form and snapshot', () => {
    const form: TemporaryMainlineForm = {
      temporaryGoalLabel: '  临时目标  ',
      reason: '  有急事  ',
      dueDate: '2024-01-15',
      urgent: true,
      important: true,
      insistAfterReject: false,
    }
    const snapshot: MainlineSnapshot = {
      currentGoalLabel: '原目标',
      probabilityValue: 60,
      probabilityState: 'known',
      riskLevel: 'yellow',
      evidenceExpired: false,
      lastCriticalEvidenceAt: null,
      latestAdjustment: '保持',
    }
    const review = reviewTemporaryMainline(form)
    const now = '2024-01-10T08:00:00.000Z'
    const result = buildSwitchHistory(form, snapshot, review, now)

    expect(result.temporaryGoalLabel).toBe('临时目标')
    expect(result.originalGoalLabel).toBe('原目标')
    expect(result.reason).toBe('有急事')
    expect(result.dueDate).toBe('2024-01-15')
    expect(result.approvedByReview).toBe(true)
    expect(result.userInsisted).toBe(false)
    expect(result.switchedAt).toBe(now)
    expect(result.restoredAt).toBeNull()
  })
})

describe('buildTemporaryAdjustment', () => {
  it('returns adjustment text with trimmed temporary goal', () => {
    const form: TemporaryMainlineForm = {
      temporaryGoalLabel: '  紧急任务  ',
      reason: '',
      dueDate: '',
      urgent: true,
      important: true,
      insistAfterReject: false,
    }
    const result = buildTemporaryAdjustment(form)
    expect(result).toContain('紧急任务')
    expect(result).toContain('临时主线已切到')
  })
})

describe('buildRestoreAdjustment', () => {
  it('returns restore text with original goal label', () => {
    const snapshot: MainlineSnapshot = {
      currentGoalLabel: '原主线目标',
      probabilityValue: null,
      probabilityState: 'unknown',
      riskLevel: 'yellow',
      evidenceExpired: true,
      lastCriticalEvidenceAt: null,
      latestAdjustment: '',
    }
    const result = buildRestoreAdjustment(snapshot)
    expect(result).toContain('原主线目标')
    expect(result).toContain('恢复原主线优先级结构')
  })
})
