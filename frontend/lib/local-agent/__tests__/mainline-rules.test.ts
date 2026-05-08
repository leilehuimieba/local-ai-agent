import { describe, it, expect } from 'vitest'
import {
  createEmptyKeyEvidenceEntry,
  isCriticalEvidenceExpired,
  evaluateKeyEvidenceEntry,
  buildWeakEvidenceSummary,
} from '../mainline-rules'
import type { KeyEvidenceEntry, KeyEvidenceRecord } from '../types'

const EXPIRE_MS = 48 * 60 * 60 * 1000

function makeEntry(partial: Partial<KeyEvidenceEntry> = {}): KeyEvidenceEntry {
  return { ...createEmptyKeyEvidenceEntry(), ...partial }
}

describe('createEmptyKeyEvidenceEntry', () => {
  it('returns default empty entry', () => {
    const entry = createEmptyKeyEvidenceEntry()
    expect(entry.sourceType).toBe('mock_exam')
    expect(entry.completedDate).toBe('')
    expect(entry.durationMinutes).toBe('')
    expect(entry.underExamCondition).toBe(false)
    expect(entry.totalScore).toBe('')
    expect(entry.listeningScore).toBe('')
    expect(entry.readingScore).toBe('')
    expect(entry.writingTranslationScore).toBe('')
    expect(entry.notes).toBe('')
  })
})

describe('isCriticalEvidenceExpired', () => {
  it('returns true when timestamp is null', () => {
    expect(isCriticalEvidenceExpired(null)).toBe(true)
  })

  it('returns false within 48 hours', () => {
    const now = Date.now()
    const timestamp = new Date(now - EXPIRE_MS + 1000).toISOString()
    expect(isCriticalEvidenceExpired(timestamp, now)).toBe(false)
  })

  it('returns true after 48 hours', () => {
    const now = Date.now()
    const timestamp = new Date(now - EXPIRE_MS - 1000).toISOString()
    expect(isCriticalEvidenceExpired(timestamp, now)).toBe(true)
  })
})

describe('evaluateKeyEvidenceEntry', () => {
  it('returns weak when underExamCondition is not true', () => {
    const entry = makeEntry({
      underExamCondition: false,
      totalScore: '500',
      listeningScore: '150',
      readingScore: '180',
      writingTranslationScore: '170',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('weak')
    expect(result.probabilityValue).toBeNull()
  })

  it('returns weak when scores are incomplete', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '',
      listeningScore: '',
      readingScore: '',
      writingTranslationScore: '',
      durationMinutes: '',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('weak')
    expect(result.probabilityValue).toBeNull()
  })

  it('returns strong with correct probability for score >=520', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '530',
      listeningScore: '180',
      readingScore: '180',
      writingTranslationScore: '170',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(92)
    expect(result.riskLevel).toBe('green')
  })

  it('returns strong with correct probability for score >=480', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '490',
      listeningScore: '160',
      readingScore: '170',
      writingTranslationScore: '160',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(82)
    expect(result.riskLevel).toBe('green')
  })

  it('returns strong with correct probability for score >=450', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '460',
      listeningScore: '150',
      readingScore: '160',
      writingTranslationScore: '150',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(70)
    expect(result.riskLevel).toBe('yellow')
  })

  it('returns strong with correct probability for score >=425', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '430',
      listeningScore: '140',
      readingScore: '150',
      writingTranslationScore: '140',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(58)
    expect(result.riskLevel).toBe('orange')
  })

  it('returns strong with correct probability for score >=390', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '400',
      listeningScore: '130',
      readingScore: '140',
      writingTranslationScore: '130',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(42)
    expect(result.riskLevel).toBe('red')
  })

  it('returns strong with correct probability for score below 390', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '380',
      listeningScore: '120',
      readingScore: '130',
      writingTranslationScore: '130',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.status).toBe('strong')
    expect(result.probabilityValue).toBe(28)
    expect(result.riskLevel).toBe('red')
  })

  it('applies positive trend when score rises by 20+', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '460',
      listeningScore: '150',
      readingScore: '160',
      writingTranslationScore: '150',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const history: KeyEvidenceRecord[] = [
      {
        sourceType: 'mock_exam',
        completedDate: '2024-01-01',
        durationMinutes: 120,
        underExamCondition: true,
        totalScore: 430,
        listeningScore: 140,
        readingScore: 150,
        writingTranslationScore: 140,
        notes: '',
        submittedAt: '2024-01-01T00:00:00.000Z',
        probabilityAfter: 58,
      },
    ]
    const result = evaluateKeyEvidenceEntry(entry, history, null)
    expect(result.probabilityValue).toBe(76)
  })

  it('applies negative trend when score drops by 20+', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '400',
      listeningScore: '130',
      readingScore: '140',
      writingTranslationScore: '130',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const history: KeyEvidenceRecord[] = [
      {
        sourceType: 'mock_exam',
        completedDate: '2024-01-01',
        durationMinutes: 120,
        underExamCondition: true,
        totalScore: 430,
        listeningScore: 140,
        readingScore: 150,
        writingTranslationScore: 140,
        notes: '',
        submittedAt: '2024-01-01T00:00:00.000Z',
        probabilityAfter: 58,
      },
    ]
    const result = evaluateKeyEvidenceEntry(entry, history, null)
    expect(result.probabilityValue).toBe(34)
  })

  it('includes adjustment based on total score and weakest module', () => {
    const entry = makeEntry({
      underExamCondition: true,
      totalScore: '530',
      listeningScore: '150',
      readingScore: '200',
      writingTranslationScore: '180',
      durationMinutes: '120',
      completedDate: '2024-01-01',
    })
    const result = evaluateKeyEvidenceEntry(entry, [], null)
    expect(result.adjustment).toContain('听力')
  })
})

describe('buildWeakEvidenceSummary', () => {
  it('returns mock exam summary', () => {
    const result = buildWeakEvidenceSummary(createEmptyKeyEvidenceEntry(), 'mock_exam')
    expect(result).toBe('模考结果已记录，等待补齐关键证据。')
  })

  it('returns real exam summary', () => {
    const result = buildWeakEvidenceSummary(createEmptyKeyEvidenceEntry(), 'real_exam')
    expect(result).toBe('真题结果已记录，等待补齐关键证据。')
  })
})
