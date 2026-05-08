import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import { fireEvent, render, screen } from "@testing-library/react"
import { MainlineShell } from "../mainline-shell"
import { useUIStore } from "@/lib/local-agent/store"
import { createEmptyEveningReviewState } from "@/lib/local-agent/evening-review-rules"
import { createEmptyExecutionState } from "@/lib/local-agent/execution-state-rules"
import { createEmptyFollowthroughState } from "@/lib/local-agent/followthrough-rules"
import { createEmptyKeyEvidenceEntry } from "@/lib/local-agent/mainline-rules"
import { createEmptyPersonalizedEntry, createEmptyPersonalizedInsight } from "@/lib/local-agent/personalized-followup-rules"
import { createEmptySwitchForm } from "@/lib/local-agent/mainline-switch-rules"
import { createEmptyNextDayPlan } from "@/lib/local-agent/next-day-plan-rules"
import { createEmptyTimeBudgetEntry, createEmptyTimeBudgetInsight } from "@/lib/local-agent/time-budget-rules"
import type { MainlineShellState } from "@/lib/local-agent/types"

beforeEach(() => {
  vi.useFakeTimers()
  vi.setSystemTime(new Date("2026-05-07T20:30:00+08:00"))
})

afterEach(() => {
  vi.useRealTimers()
})

function buildTemporaryMainline() {
  return {
    isActive: false,
    reviewStatus: "idle" as const,
    reviewFeedback: null,
    currentTemporaryGoal: null,
    originalSnapshot: null,
    currentSwitchStartedAt: null,
    activeRecordId: null,
    history: [],
  }
}

function buildMainlineShell(overrides: Partial<MainlineShellState> = {}): MainlineShellState {
  return {
    currentGoalLabel: "四级冲刺",
    probabilityValue: null,
    probabilityState: "unknown",
    riskLevel: "yellow",
    evidenceExpired: true,
    evidenceSubmitted: false,
    lastCriticalEvidenceAt: null,
    latestAdjustment: "请先补充一份完整模考 / 真题结果，再决定模块级调整。",
    keyEvidenceFeedback: null,
    switchFeedback: null,
    evidencePacket: { didWhat: "", resultSummary: "", blockers: "", nextAdjustment: "" },
    keyEvidenceEntry: createEmptyKeyEvidenceEntry(),
    keyEvidenceHistory: [],
    switchForm: createEmptySwitchForm(),
    temporaryMainline: buildTemporaryMainline(),
    personalizedFeedback: null,
    personalizedEntry: createEmptyPersonalizedEntry(),
    personalizedHistory: [],
    personalizedInsight: createEmptyPersonalizedInsight(),
    timeBudgetFeedback: null,
    timeBudgetEntry: createEmptyTimeBudgetEntry(),
    timeBlocks: [],
    timeBudgetInsight: createEmptyTimeBudgetInsight(),
    eveningReview: createEmptyEveningReviewState(),
    followthrough: createEmptyFollowthroughState(),
    execution: createEmptyExecutionState(),
    nextDayPlan: createEmptyNextDayPlan(),
    todayPlanStatusText: "今日计划尚未接管",
    todayPlanReconcileText: null,
    recentPlanHistory: [],
    ...overrides,
  }
}

function setShellState(overrides: Partial<MainlineShellState> = {}, expanded = true) {
  useUIStore.setState({
    mainlineExpanded: expanded,
    evidencePanelOpen: false,
    keyEvidencePanelOpen: false,
    switchPanelOpen: false,
    personalizedPanelOpen: false,
    timeBudgetPanelOpen: false,
    mainlineShell: buildMainlineShell(overrides),
  })
}

function renderShell(overrides: Partial<MainlineShellState> = {}, expanded = true) {
  setShellState(overrides, expanded)
  render(<MainlineShell />)
}

function fillStrongEvidence() {
  fireEvent.click(screen.getByText("补关键证据"))
  fillEvidenceScores({
    date: "2026-05-07",
    duration: "125",
    total: "468",
    listening: "142",
    reading: "168",
    writing: "158",
  })
  fireEvent.click(screen.getByRole("checkbox", { name: "是否完整按考试条件完成" }))
}

function fillEvidenceScores(values: {
  date: string
  duration: string
  total: string
  listening: string
  reading: string
  writing: string
}) {
  fireEvent.change(screen.getByLabelText("做题日期"), { target: { value: values.date } })
  fireEvent.change(screen.getByLabelText("总用时(分钟)"), { target: { value: values.duration } })
  fireEvent.change(screen.getByLabelText("总分"), { target: { value: values.total } })
  fireEvent.change(screen.getByLabelText("听力分"), { target: { value: values.listening } })
  fireEvent.change(screen.getByLabelText("阅读分"), { target: { value: values.reading } })
  fireEvent.change(screen.getByLabelText("写作与翻译分"), { target: { value: values.writing } })
}

function openSwitchForm(values: { goal: string; reason: string; due: string }) {
  fireEvent.click(screen.getByText("临时切主线"))
  fireEvent.change(screen.getByLabelText("临时目标"), { target: { value: values.goal } })
  fireEvent.change(screen.getByLabelText("切换原因"), { target: { value: values.reason } })
  fireEvent.change(screen.getByLabelText("截止时间"), { target: { value: values.due } })
}

function openTimeBudgetForm() {
  fireEvent.click(screen.getByText("时间预算接管"))
  fireEvent.change(screen.getByLabelText("今日唯一核心任务"), { target: { value: "四级词汇 90 分钟" } })
  fireEvent.change(screen.getByLabelText("现实时间预算变化"), { target: { value: "下午临时有事，只剩一小段可用时间。" } })
  fireEvent.change(screen.getByLabelText("开始时间"), { target: { value: "19:00" } })
  fireEvent.change(screen.getByLabelText("结束时间"), { target: { value: "19:45" } })
  fireEvent.change(screen.getByLabelText("时间块说明"), { target: { value: "晚间复习窗口" } })
}

function withRestoredHistory() {
  return {
    history: [
      {
        temporaryGoalLabel: "算法考试冲刺",
        originalGoalLabel: "四级冲刺",
        reason: "临时考试",
        dueDate: "2026-05-08 18:00",
        approvedByReview: true,
        userInsisted: false,
        switchedAt: "2026-05-07T08:00:00.000Z",
        restoredAt: "2026-05-07T20:00:00.000Z",
      },
    ],
  }
}

describe("MainlineShell", () => {
  it("展示主目标与未知状态，并可展开补证提示", () => {
    renderShell({}, false)
    expect(screen.getByText("四级冲刺")).toBeInTheDocument()
    expect(screen.getByText("未知")).toBeInTheDocument()
    fireEvent.click(screen.getByRole("button"))
    expect(screen.getByText("请尽快补充关键证据")).toBeInTheDocument()
    expect(screen.getByText(/状态：现在进入固定查看窗口/)).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：优先补关键证据" })).toBeInTheDocument()
    expect(screen.getByText("晚间证据")).toBeInTheDocument()
    expect(screen.getByText("补关键证据")).toBeInTheDocument()
    expect(screen.getByText("执行状态：未进入执行态")).toBeInTheDocument()
  })

  it("可填写并提交晚间证据包", () => {
    renderShell({
      execution: {
        status: "executing",
        currentTaskLabel: "四级阅读专项",
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: "正在执行今天核心任务",
        helperText: "当前只盯这一项，其余动作全部降级。",
        activeDate: "2026-05-07",
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        targetDate: "2026-05-07",
        todayTaskLabel: "四级阅读专项",
        coreTaskLabel: "四级阅读专项",
        takeoverStatus: "taken_over",
      },
      todayPlanStatusText: "今天计划已接管",
      todayPlanReconcileText: "今晚对账：今天计划仍在执行中，提交证据时需说明偏差或未完成部分。",
    })
    fireEvent.click(screen.getByText("晚间证据"))
    fireEvent.change(screen.getByLabelText("下一步调整"), {
      target: { value: "明天先做一套阅读并记录错因" },
    })
    fireEvent.click(screen.getByText("提交证据"))
    expect(screen.getByText("今晚证据已提交")).toBeInTheDocument()
    expect(screen.getByText("刚完成：已提交晚间证据")).toBeInTheDocument()
    expect(screen.getAllByText("今晚对账：今天计划仍在执行中，已提交证据时需说明未完成部分。").length).toBeGreaterThan(0)
  })

  it("概率未知时主按钮优先进入关键证据录入", () => {
    renderShell()
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：优先补关键证据" }))
    expect(screen.getByText("录入最新模拟题 / 真题结果")).toBeInTheDocument()
  })

  it("完整关键证据会更新概率与模块级调整建议", () => {
    renderShell()
    fillStrongEvidence()
    fireEvent.click(screen.getByText("更新安全通过概率"))
    expect(screen.getByText("70%")).toBeInTheDocument()
    expect(screen.getByText(/已按关键证据更新安全通过概率：70%/)).toBeInTheDocument()
    expect(screen.getByText(/模块级调整：继续围绕 听力 做模块级补强/)).toBeInTheDocument()
    expect(screen.getByText("刚完成：已更新关键证据")).toBeInTheDocument()
  })

  it("非完整考试条件结果只记录，不直接改概率", () => {
    renderShell()
    fireEvent.click(screen.getByText("补关键证据"))
    fillEvidenceScores({
      date: "2026-05-07",
      duration: "70",
      total: "430",
      listening: "130",
      reading: "150",
      writing: "150",
    })
    fireEvent.click(screen.getByText("更新安全通过概率"))
    expect(screen.getByText("未知")).toBeInTheDocument()
    expect(screen.getByText(/未按完整考试条件完成/)).toBeInTheDocument()
  })

  it("48 小时无关键证据时会降为未知", () => {
    renderShell({
      probabilityValue: 82,
      probabilityState: "known",
      riskLevel: "green",
      evidenceExpired: false,
      lastCriticalEvidenceAt: "2026-05-04T00:00:00.000Z",
      latestAdjustment: "保持当前主线节奏，优先稳住阅读。",
    })
    expect(screen.getByText("未知")).toBeInTheDocument()
    expect(screen.getByText("请尽快补充关键证据")).toBeInTheDocument()
  })

  it("紧急且重要事项可通过复核并临时切主线，再自动恢复", () => {
    renderShell()
    openSwitchForm({
      goal: "算法考试冲刺",
      reason: "两天后考试，无法事后补救",
      due: "2026-05-09 18:00",
    })
    fireEvent.click(screen.getByText("提交切主线申请"))
    expect(screen.getByText("算法考试冲刺")).toBeInTheDocument()
    expect(screen.getByText(/已通过复核：属于紧急且重要事项/)).toBeInTheDocument()
    fireEvent.click(screen.getByText("自动恢复"))
    expect(screen.getByText("四级冲刺")).toBeInTheDocument()
    expect(screen.getByText(/已自动恢复原主线优先级结构/)).toBeInTheDocument()
  })

  it("未通过复核但用户坚持时会记账后执行", () => {
    renderShell()
    openSwitchForm({
      goal: "整理长期笔记体系",
      reason: "想顺手把体系搭起来",
      due: "2026-05-20 18:00",
    })
    fireEvent.click(screen.getByRole("checkbox", { name: "属于紧急事项" }))
    fireEvent.click(screen.getByRole("checkbox", { name: "属于重要事项" }))
    fireEvent.click(screen.getByRole("checkbox", { name: "若未通过复核，仍坚持并记账" }))
    fireEvent.click(screen.getByText("提交切主线申请"))
    expect(screen.getByText("整理长期笔记体系")).toBeInTheDocument()
    expect(screen.getByText(/未通过复核，但已按你的坚持执行并记账/)).toBeInTheDocument()
  })

  it("可记录个性化反馈并更新更适合你的推进方式", () => {
    renderShell()
    expect(screen.getByText(/先通用、后个性化/)).toBeInTheDocument()
    fireEvent.click(screen.getByText("个性化记录"))
    fireEvent.click(screen.getByText("阅读"))
    fireEvent.click(screen.getAllByText("是")[0])
    fireEvent.click(screen.getAllByText("是")[1])
    fireEvent.change(screen.getByLabelText("结果反馈 / 备注"), {
      target: { value: "今天按计划做完阅读，错因也复盘了。" },
    })
    fireEvent.click(screen.getByText("记录这次反馈"))
    expect(screen.getByText(/阅读 这类任务对你当前阶段有正收益/)).toBeInTheDocument()
    expect(screen.getByText("基于记录：1 次")).toBeInTheDocument()
    expect(screen.getByText("更易完成：阅读")).toBeInTheDocument()
  })

  it("可录入今日时间块并更新时间预算接管建议", () => {
    renderShell()
    openTimeBudgetForm()
    fireEvent.click(screen.getByText("更新今日主线接管"))
    expect(screen.getByText(/已接管今日主线/)).toBeInTheDocument()
    expect(screen.getByText("今日唯一核心任务：四级词汇 90 分钟")).toBeInTheDocument()
    expect(screen.getByText("可用时间：45 分钟")).toBeInTheDocument()
    expect(screen.getByText("预算状态：极紧")).toBeInTheDocument()
    expect(screen.getAllByText(/只保留 1 个不可协商核心任务/).length).toBeGreaterThan(0)
    expect(screen.getByText("19:00-19:45 · 晚间复习窗口")).toBeInTheDocument()
    expect(screen.getByText("执行状态：正在执行今天核心任务")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：标记今天核心任务完成" })).toBeInTheDocument()
  })

  it("可生成明日计划并展示恢复原主线说明", () => {
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      lastCriticalEvidenceAt: "2026-05-07T18:00:00.000Z",
      evidenceExpired: false,
      latestAdjustment: "继续围绕 听力 做模块级补强，同时保留词汇记忆主轴。",
      personalizedInsight: { ...createEmptyPersonalizedInsight(), boostTaskType: "reading" },
      timeBudgetInsight: { ...createEmptyTimeBudgetInsight(), level: "steady" },
      temporaryMainline: { ...buildTemporaryMainline(), ...withRestoredHistory() },
    })
    fireEvent.click(screen.getByText("生成明日计划"))
    expect(screen.getByText("状态：计划目标日：2026-05-08")).toBeInTheDocument()
    expect(screen.getByText("目标日期：2026-05-08")).toBeInTheDocument()
    expect(screen.getByText("依据证据日期：2026-05-08")).toBeInTheDocument()
    expect(screen.getByText(/明日唯一核心任务：继续围绕 听力 做模块级补强/)).toBeInTheDocument()
    expect(screen.getByText("补充项：继续安排 阅读")).toBeInTheDocument()
    expect(screen.getByText(/依据：概率状态：70%/)).toBeInTheDocument()
    expect(screen.getByText("已恢复原主线：四级冲刺")).toBeInTheDocument()
  })

  it("晚间证据已提交且概率已知时主按钮优先生成明日计划", () => {
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      evidenceSubmitted: true,
      lastCriticalEvidenceAt: "2026-05-07T18:00:00.000Z",
      eveningReview: {
        ...createEmptyEveningReviewState(),
        status: "submitted",
        statusText: "今晚证据已提交",
        branchText: "已完成 2026-05-07 的晚间对账",
        lastSubmittedForDate: "2026-05-07",
      },
    })
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：优先生成明日计划" }))
    expect(screen.getByText(/明日唯一核心任务：/)).toBeInTheDocument()
    expect(screen.getByText("刚完成：已生成明日计划")).toBeInTheDocument()
  })

  it("次日上午可补交昨晚证据", () => {
    vi.setSystemTime(new Date("2026-05-08T09:00:00+08:00"))
    renderShell({
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: "2026-05-06T20:30:00.000+08:00",
        lastSubmittedForDate: "2026-05-06",
      },
    })
    expect(screen.getByText(/状态：昨晚证据可补交/)).toBeInTheDocument()
    expect(screen.getByText("当前裁决：先补昨天")).toBeInTheDocument()
    expect(screen.getByText("补交截止：次日中午前")).toBeInTheDocument()
    expect(screen.getByText("补交昨晚证据")).toBeInTheDocument()
  })

  it("补交窗口超时后会分流到继续今天", () => {
    vi.setSystemTime(new Date("2026-05-08T13:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      timeBudgetInsight: { ...createEmptyTimeBudgetInsight(), level: "critical" },
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: "2026-05-06T20:30:00.000+08:00",
        lastSubmittedForDate: "2026-05-06",
      },
    })
    expect(screen.getByText(/状态：补交窗口已过/)).toBeInTheDocument()
    expect(screen.getAllByText("当前裁决：继续今天").length).toBeGreaterThan(0)
    expect(screen.getByText("先保今天唯一核心任务，昨晚证据改为可追记。")).toBeInTheDocument()
  })

  it("补交窗口超时且继续今天时主按钮优先保今天核心任务", () => {
    vi.setSystemTime(new Date("2026-05-08T13:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      lastCriticalEvidenceAt: "2026-05-08T08:00:00.000Z",
      timeBudgetInsight: { ...createEmptyTimeBudgetInsight(), level: "critical" },
      eveningReview: {
        ...createEmptyEveningReviewState(),
        status: "reroute",
        branchDecision: "continue_today",
        statusText: "补交窗口已过",
        branchText: "当前裁决：继续今天",
        guidance: "先保今天唯一核心任务，昨晚证据改为可追记。",
        lastReviewedAt: "2026-05-07T20:30:00.000+08:00",
        lastSubmittedForDate: "2026-05-06",
      },
    })
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：优先保今天核心任务" }))
    expect(screen.getByText("时间预算 / 时间块接管")).toBeInTheDocument()
  })

  it("晚间证据提交后若概率未知会自动引导补关键证据", () => {
    renderShell()
    fireEvent.click(screen.getByText("晚间证据"))
    fireEvent.change(screen.getByLabelText("下一步调整"), {
      target: { value: "今晚先收口，再补关键证据。" },
    })
    fireEvent.click(screen.getByText("提交证据"))
    expect(screen.getByText("下一步：优先补关键证据")).toBeInTheDocument()
  })

  it("关键证据更新后会提示下一步优先生成明日计划", () => {
    renderShell()
    fillStrongEvidence()
    fireEvent.click(screen.getByText("更新安全通过概率"))
    expect(screen.getByText(/下一步：优先生成明日计划|下一步：更新明日计划/)).toBeInTheDocument()
    expect(screen.getByText("当前计划已基于最新关键证据")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：优先生成明日计划" })).toBeInTheDocument()
  })

  it("主按钮区会高亮当前动作并弱化非当前动作", () => {
    renderShell()
    const currentButton = screen.getByText("补关键证据").closest("button")
    const otherButton = screen.getByText("晚间证据").closest("button")
    expect(currentButton).toHaveAttribute("data-recommended", "true")
    expect(otherButton).toHaveAttribute("data-muted", "true")
  })

  it("今天核心任务完成后会切到收尾动作", () => {
    renderShell()
    openTimeBudgetForm()
    fireEvent.click(screen.getByText("更新今日主线接管"))
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：标记今天核心任务完成" }))
    expect(screen.getByText("执行状态：今天核心任务已完成")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：进入今天收尾" })).toBeInTheDocument()
  })

  it("今天收尾后不再继续强推其他动作", () => {
    renderShell()
    openTimeBudgetForm()
    fireEvent.click(screen.getByText("更新今日主线接管"))
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：标记今天核心任务完成" }))
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：进入今天收尾" }))
    expect(screen.getByText("执行状态：今天已收尾")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：今天已收尾" })).toBeDisabled()
  })

  it("跨天后会清空昨天执行态并要求重开今天主线", () => {
    vi.setSystemTime(new Date("2026-05-08T08:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "closed",
        currentTaskLabel: "四级词汇 90 分钟",
        todayCoreTaskCompleted: true,
        todayClosed: true,
        statusText: "今天已收尾",
        helperText: "今天不再继续强推次优动作，等待新的关键证据或新的现实约束变化。",
        activeDate: "2026-05-07",
        needsReopen: false,
        staleFromDate: null,
      },
    })
    expect(screen.getByText("已进入新的一天，请先重开今天主线。")).toBeInTheDocument()
    expect(screen.getByText("执行状态：新的一天待重开")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "下一步动作：重开今天主线" })).toBeInTheDocument()
  })

  it("昨天未收尾跨天后不会直接延续执行，而是要求先重开", () => {
    vi.setSystemTime(new Date("2026-05-08T09:00:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "executing",
        currentTaskLabel: "四级阅读专项",
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: "正在执行今天核心任务",
        helperText: "当前只盯这一项，其余动作全部降级。",
        activeDate: "2026-05-07",
        needsReopen: false,
        staleFromDate: null,
      },
    })
    expect(screen.getByText("执行状态：昨日未收尾，今天待重开")).toBeInTheDocument()
    expect(screen.getAllByText(/昨天未收口；今天先重新录入现实时间预算/).length).toBeGreaterThan(0)
    expect(screen.getByRole("button", { name: "下一步动作：重开今天主线" })).toBeInTheDocument()
  })

  it("补交窗口超时但仍需补昨天关键判断时，优先补关键证据", () => {
    vi.setSystemTime(new Date("2026-05-08T13:30:00+08:00"))
    renderShell({
      probabilityValue: null,
      probabilityState: "unknown",
      eveningReview: {
        ...createEmptyEveningReviewState(),
        lastReviewedAt: "2026-05-07T20:30:00.000+08:00",
        lastSubmittedForDate: "2026-05-06",
      },
    })
    expect(screen.getAllByRole("button", { name: "下一步动作：优先补关键证据" }).length).toBeGreaterThan(0)
  })

  it("进入计划目标日但今天尚未重开主线时，会提示计划待重算", () => {
    vi.setSystemTime(new Date("2026-05-08T08:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "closed",
        currentTaskLabel: "四级词汇 90 分钟",
        todayCoreTaskCompleted: true,
        todayClosed: true,
        statusText: "今天已收尾",
        helperText: "今天不再继续强推次优动作，等待新的关键证据或新的现实约束变化。",
        activeDate: "2026-05-07",
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        generatedAt: "2026-05-07T21:00:00.000+08:00",
        targetDate: "2026-05-08",
        basedOnEvidenceDate: "2026-05-07",
        coreTaskLabel: "四级词汇 90 分钟",
        supportTaskLabel: "补充项：无，先保唯一核心任务",
        rationale: "概率状态：70%；个性化短期收益：尚无判断；时间预算：正常",
      },
    })
    expect(screen.getByText("状态：今天计划待重算")).toBeInTheDocument()
    expect(screen.getAllByText("已进入计划目标日，请结合今天现实时间预算重新生成。").length).toBeGreaterThan(0)
  })

  it("进入计划目标日后，时间预算接管会默认承接计划核心任务", () => {
    vi.setSystemTime(new Date("2026-05-08T08:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "idle",
        currentTaskLabel: null,
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: "新的一天待重开",
        helperText: "昨天已收尾；今天需要重新录入现实时间预算后再进入执行态。",
        activeDate: "2026-05-08",
        needsReopen: true,
        staleFromDate: "2026-05-07",
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        generatedAt: "2026-05-07T21:00:00.000+08:00",
        targetDate: "2026-05-08",
        basedOnEvidenceDate: "2026-05-07",
        coreTaskLabel: "按计划先做四级听力 90 分钟",
        supportTaskLabel: "补充项：无，先保唯一核心任务",
        rationale: "概率状态：70%；个性化短期收益：尚无判断；时间预算：正常",
      },
    })
    expect(screen.getByText("今日接管：今日待接管")).toBeInTheDocument()
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：重开今天主线" }))
    expect(screen.getByText("今日计划")).toBeInTheDocument()
    expect(screen.getByText("计划来源：昨晚生成的目标日计划")).toBeInTheDocument()
    expect(screen.getByDisplayValue("按计划先做四级听力 90 分钟")).toBeInTheDocument()
    expect(screen.getAllByText("今日任务：按计划先做四级听力 90 分钟").length).toBeGreaterThan(0)
  })

  it("提交时间预算后，计划卡会显示已接管到今天", () => {
    vi.setSystemTime(new Date("2026-05-08T08:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "idle",
        currentTaskLabel: null,
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: "新的一天待重开",
        helperText: "昨天已收尾；今天需要重新录入现实时间预算后再进入执行态。",
        activeDate: "2026-05-08",
        needsReopen: true,
        staleFromDate: "2026-05-07",
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        generatedAt: "2026-05-07T21:00:00.000+08:00",
        targetDate: "2026-05-08",
        basedOnEvidenceDate: "2026-05-07",
        coreTaskLabel: "按计划先做四级听力 90 分钟",
        supportTaskLabel: "补充项：无，先保唯一核心任务",
        rationale: "概率状态：70%；个性化短期收益：尚无判断；时间预算：正常",
      },
    })
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：重开今天主线" }))
    fireEvent.change(screen.getByLabelText("现实时间预算变化"), { target: { value: "今天上午可以直接按计划推进。" } })
    fireEvent.change(screen.getByLabelText("开始时间"), { target: { value: "08:30" } })
    fireEvent.change(screen.getByLabelText("结束时间"), { target: { value: "10:00" } })
    fireEvent.change(screen.getByLabelText("时间块说明"), { target: { value: "晨间执行窗口" } })
    fireEvent.click(screen.getByText("更新今日主线接管"))
    expect(screen.getByText("今日接管：已接管到今天")).toBeInTheDocument()
    expect(screen.getAllByText("今日任务：按计划先做四级听力 90 分钟").length).toBeGreaterThan(0)
    expect(screen.getByText("今天主线已承接这份计划。")).toBeInTheDocument()
  })

  it("今天核心任务完成并收尾后，计划卡会回写执行结果", () => {
    vi.setSystemTime(new Date("2026-05-08T08:30:00+08:00"))
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      execution: {
        status: "executing",
        currentTaskLabel: "按计划先做四级听力 90 分钟",
        todayCoreTaskCompleted: false,
        todayClosed: false,
        statusText: "正在执行今天核心任务",
        helperText: "当前只盯这一项，其余动作全部降级。",
        activeDate: "2026-05-08",
        needsReopen: false,
        staleFromDate: null,
      },
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        generatedAt: "2026-05-07T21:00:00.000+08:00",
        targetDate: "2026-05-08",
        basedOnEvidenceDate: "2026-05-07",
        coreTaskLabel: "按计划先做四级听力 90 分钟",
        supportTaskLabel: "补充项：无，先保唯一核心任务",
        rationale: "概率状态：70%；个性化短期收益：尚无判断；时间预算：正常",
      },
    })
    expect(screen.getByText("今日接管：已接管到今天")).toBeInTheDocument()
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：标记今天核心任务完成" }))
    expect(screen.getByText("今日接管：计划核心任务已完成")).toBeInTheDocument()
    expect(screen.getAllByText("计划核心任务已完成。").length).toBeGreaterThan(0)
    fireEvent.click(screen.getByRole("button", { name: "下一步动作：进入今天收尾" }))
    expect(screen.getByText("今日接管：今日已按计划收尾")).toBeInTheDocument()
    expect(screen.getByText("今日已按计划收尾。")).toBeInTheDocument()
    expect(screen.getByText("最近计划记录")).toBeInTheDocument()
    expect(screen.getAllByText("目标日期：2026-05-08").length).toBeGreaterThan(0)
    expect(screen.getAllByText("任务：按计划先做四级听力 90 分钟").length).toBeGreaterThan(0)
    expect(screen.getAllByText("状态：今天计划已收尾").length).toBeGreaterThan(0)
  })

  it("关键证据更新后，旧计划会提示不是最新版本", () => {
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      lastCriticalEvidenceAt: "2026-05-07T18:00:00.000Z",
      nextDayPlan: {
        ...createEmptyNextDayPlan(),
        ready: true,
        generatedAt: "2026-05-07T17:00:00.000+08:00",
        targetDate: "2026-05-08",
        basedOnEvidenceDate: "2026-05-06",
        coreTaskLabel: "旧计划核心任务",
        supportTaskLabel: "补充项：无，先保唯一核心任务",
        rationale: "概率状态：70%；个性化短期收益：尚无判断；时间预算：正常",
      },
    })
    expect(screen.getByText("状态：计划依据已变化")).toBeInTheDocument()
    expect(screen.getAllByText("关键证据已更新，原计划已不是最新版本。").length).toBeGreaterThan(0)
  })

  it("最近计划记录会显示待对账", () => {
    renderShell({
      execution: { ...createEmptyExecutionState(), status: "closed", todayCoreTaskCompleted: true, todayClosed: true, activeDate: "2026-05-07" },
      nextDayPlan: { ...createEmptyNextDayPlan(), ready: true, targetDate: "2026-05-07", coreTaskLabel: "四级听力 90 分钟" },
    })
    expect(screen.getByText("最近计划记录")).toBeInTheDocument()
    expect(screen.getByText("对账：待对账")).toBeInTheDocument()
  })

  it("最近计划记录会显示已对账", () => {
    renderShell({
      probabilityValue: 70,
      probabilityState: "known",
      evidenceExpired: false,
      lastCriticalEvidenceAt: "2026-05-07T18:00:00.000Z",
      evidenceSubmitted: true,
      eveningReview: {
        ...createEmptyEveningReviewState(),
        status: "submitted",
        statusText: "今晚证据已提交",
        lastReviewedAt: "2026-05-07T20:30:00.000+08:00",
        lastSubmittedForDate: "2026-05-07",
      },
      execution: { ...createEmptyExecutionState(), status: "closed", todayCoreTaskCompleted: true, todayClosed: true, activeDate: "2026-05-07" },
      nextDayPlan: { ...createEmptyNextDayPlan(), ready: true, targetDate: "2026-05-07", coreTaskLabel: "四级听力 90 分钟" },
    })
    expect(screen.getByText("最近计划记录")).toBeInTheDocument()
    expect(screen.getByText("对账：已对账")).toBeInTheDocument()
  })
})
