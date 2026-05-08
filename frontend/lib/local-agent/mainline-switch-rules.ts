import type {
  MainlineSnapshot,
  MainlineSwitchReviewStatus,
  TemporaryMainlineForm,
  TemporaryMainlineHistory,
} from "./types"

type SwitchReview = {
  reviewStatus: MainlineSwitchReviewStatus
  reviewFeedback: string
  approvedByReview: boolean
  userInsisted: boolean
}

export function createEmptySwitchForm(): TemporaryMainlineForm {
  return {
    temporaryGoalLabel: "",
    reason: "",
    dueDate: "",
    urgent: true,
    important: true,
    insistAfterReject: false,
  }
}

export function reviewTemporaryMainline(form: TemporaryMainlineForm): SwitchReview {
  if (form.urgent && form.important) {
    return {
      reviewStatus: "approved",
      reviewFeedback: "已通过复核：属于紧急且重要事项，可临时切主线。",
      approvedByReview: true,
      userInsisted: false,
    }
  }
  if (form.insistAfterReject) {
    return {
      reviewStatus: "rejected",
      reviewFeedback: "未通过复核，但已按你的坚持执行并记账，恢复后会回到原主线优先级结构。",
      approvedByReview: false,
      userInsisted: true,
    }
  }
  return {
    reviewStatus: "rejected",
    reviewFeedback: "未通过复核：当前不满足“紧急且重要”，若仍要切主线，请显式坚持并记账。",
    approvedByReview: false,
    userInsisted: false,
  }
}

export function buildMainlineSnapshot(input: MainlineSnapshot): MainlineSnapshot {
  return { ...input }
}

export function buildSwitchHistory(
  form: TemporaryMainlineForm,
  snapshot: MainlineSnapshot,
  review: SwitchReview,
  now: string,
): TemporaryMainlineHistory {
  return {
    temporaryGoalLabel: form.temporaryGoalLabel.trim(),
    originalGoalLabel: snapshot.currentGoalLabel,
    reason: form.reason.trim(),
    dueDate: form.dueDate,
    approvedByReview: review.approvedByReview,
    userInsisted: review.userInsisted,
    switchedAt: now,
    restoredAt: null,
  }
}

export function buildTemporaryAdjustment(form: TemporaryMainlineForm) {
  return `临时主线已切到“${form.temporaryGoalLabel.trim()}”，结束后自动恢复原计划优先级结构。`
}

export function buildRestoreAdjustment(snapshot: MainlineSnapshot) {
  return `已恢复原主线优先级结构，先回到“${snapshot.currentGoalLabel}”，并继续服从当前现实时间预算。`
}
