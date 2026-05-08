import { describe, it, expect } from 'vitest'
import {
  createEmptyPersonalizedEntry,
  createEmptyPersonalizedInsight,
  evaluatePersonalizedEntry,
  getTaskTypeLabel,
} from '../personalized-followup-rules'
import type { PersonalizedFollowupEntry, PersonalizedFollowupRecord } from '../types'

describe('createEmptyPersonalizedEntry', () => {
  it('returns default empty entry', () => {
    const entry = createEmptyPersonalizedEntry()
    expect(entry.taskType).toBe('vocabulary')
    expect(entry.isCoreTask).toBe(true)
    expect(entry.completed).toBeNull()
    expect(entry.hasShortTermGain).toBeNull()
    expect(entry.resultNote).toBe('')
  })
})

describe('createEmptyPersonalizedInsight', () => {
  it('returns default empty insight', () => {
    const insight = createEmptyPersonalizedInsight()
    expect(insight.updatedAt).toBeNull()
    expect(insight.basedOnCount).toBe(0)
    expect(insight.preferredTaskType).toBeNull()
    expect(insight.boostTaskType).toBeNull()
    expect(insight.riskTaskType).toBeNull()
    expect(insight.recommendation).toContain('先通用')
  })
})

describe('evaluatePersonalizedEntry', () => {
  it('returns null record when entry is incomplete', () => {
    const entry: PersonalizedFollowupEntry = {
      taskType: 'reading',
      isCoreTask: false,
      completed: null,
      hasShortTermGain: null,
      resultNote: '',
    }
    const result = evaluatePersonalizedEntry(entry, [])
    expect(result.record).toBeNull()
    expect(result.feedback).toContain('草稿')
  })

  it('returns insight with basedOnCount from history+1 when entry is complete', () => {
    const entry: PersonalizedFollowupEntry = {
      taskType: 'reading',
      isCoreTask: false,
      completed: true,
      hasShortTermGain: true,
      resultNote: '感觉不错',
    }
    const history: PersonalizedFollowupRecord[] = [
      {
        taskType: 'vocabulary',
        isCoreTask: true,
        completed: true,
        hasShortTermGain: false,
        resultNote: '',
        submittedAt: '2024-01-01T00:00:00.000Z',
      },
    ]
    const result = evaluatePersonalizedEntry(entry, history)
    expect(result.record).not.toBeNull()
    expect(result.insight.basedOnCount).toBe(2)
    expect(result.record?.taskType).toBe('reading')
    expect(result.record?.completed).toBe(true)
    expect(result.record?.hasShortTermGain).toBe(true)
  })

  it('picks preferred task type from completed records', () => {
    const entry: PersonalizedFollowupEntry = {
      taskType: 'listening',
      isCoreTask: true,
      completed: true,
      hasShortTermGain: true,
      resultNote: '',
    }
    const history: PersonalizedFollowupRecord[] = [
      { taskType: 'reading', isCoreTask: false, completed: true, hasShortTermGain: true, resultNote: '', submittedAt: '1' },
      { taskType: 'reading', isCoreTask: false, completed: true, hasShortTermGain: false, resultNote: '', submittedAt: '2' },
      { taskType: 'listening', isCoreTask: true, completed: true, hasShortTermGain: true, resultNote: '', submittedAt: '3' },
    ]
    const result = evaluatePersonalizedEntry(entry, history)
    expect(result.insight.preferredTaskType).toBe('reading')
    expect(result.insight.boostTaskType).toBe('listening')
  })

  it('picks risk task type from incomplete records', () => {
    const entry: PersonalizedFollowupEntry = {
      taskType: 'writing_translation',
      isCoreTask: true,
      completed: false,
      hasShortTermGain: false,
      resultNote: '',
    }
    const history: PersonalizedFollowupRecord[] = [
      { taskType: 'writing_translation', isCoreTask: true, completed: false, hasShortTermGain: false, resultNote: '', submittedAt: '1' },
    ]
    const result = evaluatePersonalizedEntry(entry, history)
    expect(result.insight.riskTaskType).toBe('writing_translation')
  })
})

describe('getTaskTypeLabel', () => {
  it('returns correct labels for task types', () => {
    expect(getTaskTypeLabel('vocabulary')).toBe('词汇记忆')
    expect(getTaskTypeLabel('reading')).toBe('阅读')
    expect(getTaskTypeLabel('listening')).toBe('听力')
    expect(getTaskTypeLabel('writing_translation')).toBe('写作与翻译')
    expect(getTaskTypeLabel('mock_exam')).toBe('完整模考')
    expect(getTaskTypeLabel('professional_skill')).toBe('专业技能')
    expect(getTaskTypeLabel('algorithm')).toBe('算法复习')
    expect(getTaskTypeLabel('other')).toBe('其他任务')
  })
})
