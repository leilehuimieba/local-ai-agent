import { describe, it, expect } from 'vitest'
import {
  createEmptyTimeBlockDraft,
  createEmptyTimeBudgetEntry,
  createEmptyTimeBudgetInsight,
  evaluateTimeBudgetEntry,
} from '../time-budget-rules'
import type { TimeBudgetEntry, TimeBlockItem } from '../types'

describe('createEmptyTimeBlockDraft', () => {
  it('returns empty draft', () => {
    const draft = createEmptyTimeBlockDraft()
    expect(draft).toEqual({ startTime: '', endTime: '', label: '' })
  })
})

describe('createEmptyTimeBudgetEntry', () => {
  it('returns default entry with core task label', () => {
    const entry = createEmptyTimeBudgetEntry()
    expect(entry.coreTaskLabel).toBe('今天先保词汇记忆主轴')
    expect(entry.budgetChangeNote).toBe('')
    expect(entry.timeBlockDraft).toEqual({ startTime: '', endTime: '', label: '' })
  })
})

describe('createEmptyTimeBudgetInsight', () => {
  it('returns steady level with zero minutes', () => {
    const insight = createEmptyTimeBudgetInsight()
    expect(insight.level).toBe('steady')
    expect(insight.totalAvailableMinutes).toBe(0)
    expect(insight.updatedAt).toBeNull()
  })
})

describe('evaluateTimeBudgetEntry', () => {
  it('returns feedback asking to complete time when block parsing fails', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '', endTime: '', label: '' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.feedback).toContain('时间块未新增')
    expect(result.nextBlocks).toHaveLength(0)
  })

  it('returns feedback asking to complete time when end <= start', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '14:00', endTime: '14:00', label: '学习' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.feedback).toContain('时间块未新增')
  })

  it('appends valid block to nextBlocks', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '09:00', endTime: '10:00', label: '早读' },
    }
    const existing: TimeBlockItem[] = []
    const result = evaluateTimeBudgetEntry(entry, existing)
    expect(result.nextBlocks).toHaveLength(1)
    expect(result.nextBlocks[0].durationMinutes).toBe(60)
    expect(result.nextBlocks[0].label).toBe('早读')
  })

  it('computes ample level when total >= 240 minutes', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '08:00', endTime: '12:00', label: '上午' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.insight.level).toBe('ample')
    expect(result.insight.totalAvailableMinutes).toBe(240)
  })

  it('computes steady level when total >= 120 minutes', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '08:00', endTime: '10:00', label: '上午' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.insight.level).toBe('steady')
    expect(result.insight.totalAvailableMinutes).toBe(120)
  })

  it('computes tight level when total >= 60 minutes', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '08:00', endTime: '09:00', label: '早读' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.insight.level).toBe('tight')
    expect(result.insight.totalAvailableMinutes).toBe(60)
  })

  it('computes critical level when total < 60 minutes', () => {
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '08:00', endTime: '08:30', label: '早读' },
    }
    const result = evaluateTimeBudgetEntry(entry, [])
    expect(result.insight.level).toBe('critical')
    expect(result.insight.totalAvailableMinutes).toBe(30)
  })

  it('accumulates minutes from existing blocks', () => {
    const existing: TimeBlockItem[] = [
      { startTime: '08:00', endTime: '09:00', label: '早读', durationMinutes: 60 },
    ]
    const entry: TimeBudgetEntry = {
      coreTaskLabel: '阅读',
      budgetChangeNote: '',
      timeBlockDraft: { startTime: '09:00', endTime: '11:00', label: '上午' },
    }
    const result = evaluateTimeBudgetEntry(entry, existing)
    expect(result.insight.totalAvailableMinutes).toBe(180)
    expect(result.insight.level).toBe('steady')
  })
})
