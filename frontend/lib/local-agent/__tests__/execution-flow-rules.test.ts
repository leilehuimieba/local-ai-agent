import { describe, it, expect } from 'vitest'
import {
  buildCoreTaskDoneShell,
  buildTodayClosedShell,
} from '../execution-flow-rules'
import { createDefaultMainlineShell } from '../mainline-default-state'
import type { MainlineShellState } from '../types'

function makeShell(partial: Partial<MainlineShellState> = {}): MainlineShellState {
  return { ...createDefaultMainlineShell(), ...partial }
}

describe('buildCoreTaskDoneShell', () => {
  it('marks execution as completed and updates followthrough', () => {
    const shell = makeShell()
    const result = buildCoreTaskDoneShell(shell)
    expect(result.execution.todayCoreTaskCompleted).toBe(true)
    expect(result.execution.status).toBe('completed')
    expect(result.execution.todayClosed).toBe(false)
    expect(result.followthrough.lastActionLabel).toBe('已完成今天核心任务')
    expect(result.followthrough.nextActionLabel).toBe('进入今天收尾')
  })

  it('preserves current task label from execution', () => {
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
    })
    const result = buildCoreTaskDoneShell(shell)
    expect(result.execution.currentTaskLabel).toBe('听力训练')
  })
})

describe('buildTodayClosedShell', () => {
  it('marks execution as closed and updates followthrough', () => {
    const shell = makeShell()
    const result = buildTodayClosedShell(shell)
    expect(result.execution.todayClosed).toBe(true)
    expect(result.execution.status).toBe('closed')
    expect(result.execution.todayCoreTaskCompleted).toBe(true)
    expect(result.followthrough.lastActionLabel).toBe('已进入今天收尾')
    expect(result.followthrough.nextActionLabel).toBe('等待新的关键证据')
  })

  it('preserves current task label from execution', () => {
    const shell = makeShell({
      execution: {
        status: 'completed',
        currentTaskLabel: '阅读训练',
        todayCoreTaskCompleted: true,
        todayClosed: false,
        statusText: '',
        helperText: '',
        activeDate: '2024-01-15',
        needsReopen: false,
        staleFromDate: null,
      },
    })
    const result = buildTodayClosedShell(shell)
    expect(result.execution.currentTaskLabel).toBe('阅读训练')
  })
})
