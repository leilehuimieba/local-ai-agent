package service

import (
	"fmt"

	"local-agent/gateway/internal/contracts"
)

func RunStartedEvent(runRequest contracts.RunRequest) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:    NewID("event"),
		Kind:       "run_event",
		Source:     "gateway",
		AgentID:    "primary",
		AgentLabel: "主智能体",
		EventType:  "run_started",
		TraceID:    runRequest.TraceID,
		SessionID:  runRequest.SessionID,
		RunID:      runRequest.RunID,
		Sequence:   1,
		Timestamp:  timestampNow(),
		Stage:      "Analyze",
		Summary:    "Gateway 已接收任务",
		Detail:     "Gateway 已开始调用 Runtime。",
		Metadata: map[string]string{
			"task_title": runRequest.UserInput,
			"next_step":  "等待 Runtime 返回执行事件",
		},
	}
}

func RuntimeReturnedEvent(runRequest contracts.RunRequest, eventCount int) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:    NewID("event"),
		Kind:       "run_event",
		Source:     "gateway",
		AgentID:    "primary",
		AgentLabel: "主智能体",
		EventType:  "runtime_returned",
		TraceID:    runRequest.TraceID,
		SessionID:  runRequest.SessionID,
		RunID:      runRequest.RunID,
		Sequence:   2,
		Timestamp:  timestampNow(),
		Stage:      "Execute",
		Summary:    "Runtime 已返回响应",
		Detail:     "Gateway 已收到 Runtime 响应。",
		Metadata: map[string]string{
			"runtime_event_count": fmt.Sprintf("%d", eventCount),
			"task_title":          runRequest.UserInput,
			"next_step":           "发布 Runtime 事件",
		},
	}
}

func RuntimeFailureEvent(runRequest contracts.RunRequest, errorText string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:       NewID("event"),
		Kind:          "run_event",
		Source:        "gateway",
		AgentID:       "primary",
		AgentLabel:    "主智能体",
		EventType:     "run_failed",
		TraceID:       runRequest.TraceID,
		SessionID:     runRequest.SessionID,
		RunID:         runRequest.RunID,
		Sequence:      1,
		Timestamp:     timestampNow(),
		Stage:         "Failed",
		Summary:       "运行时调用失败",
		Detail:        errorText,
		ResultSummary: "Gateway 未能拿到 Runtime 返回结果。",
		Metadata: map[string]string{
			"error_code":     "runtime_unavailable",
			"error_message":  errorText,
			"error_source":   "gateway",
			"retryable":      "true",
			"result_summary": "Gateway 未能拿到 Runtime 返回结果。",
			"task_title":     runRequest.UserInput,
			"next_step":      "等待运行时恢复后重试",
		},
	}
}

func RuntimeFailureFinishEvent(runRequest contracts.RunRequest, errorText string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID:    NewID("event"),
		Kind:       "run_event",
		Source:     "gateway",
		AgentID:    "primary",
		AgentLabel: "主智能体",
		EventType:  "run_finished",
		TraceID:    runRequest.TraceID,
		SessionID:  runRequest.SessionID,
		RunID:      runRequest.RunID,
		Sequence:   2,
		Timestamp:  timestampNow(),
		Stage:      "Finish",
		Summary:    "任务因运行时不可达而结束",
		Detail:     errorText,
		Metadata: map[string]string{
			"error_code":    "runtime_unavailable",
			"error_message": errorText,
			"error_source":  "gateway",
			"final_answer":  "运行时当前不可达，本次任务未能执行。请先检查 Runtime 进程后重试。",
			"task_title":    runRequest.UserInput,
			"next_step":     "任务已结束",
		},
	}
}

func RunCancelledEvent(runRequest contracts.RunRequest, detail string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: NewID("event"), Kind: "run_event", Source: "gateway", AgentID: "primary", AgentLabel: "主智能体",
		EventType: "run_failed", TraceID: runRequest.TraceID, SessionID: runRequest.SessionID, RunID: runRequest.RunID,
		Sequence: 1, Timestamp: timestampNow(), Stage: "Failed", Summary: "任务被用户中断", Detail: detail,
		ResultSummary: "Gateway 已取消该运行请求。",
		Metadata: map[string]string{
			"error_code": "run_cancelled", "error_message": detail, "error_source": "gateway",
			"retryable": "true", "result_summary": "Gateway 已取消该运行请求。", "task_title": runRequest.UserInput,
			"next_step": "如需继续，请重新发起任务",
		},
	}
}

func RunCancelledFinishEvent(runRequest contracts.RunRequest, detail string) contracts.RunEvent {
	return contracts.RunEvent{
		EventID: NewID("event"), Kind: "run_event", Source: "gateway", AgentID: "primary", AgentLabel: "主智能体",
		EventType: "run_finished", TraceID: runRequest.TraceID, SessionID: runRequest.SessionID, RunID: runRequest.RunID,
		Sequence: 2, Timestamp: timestampNow(), Stage: "Finish", Summary: "任务已中断", Detail: detail,
		CompletionStatus: "cancelled", CompletionReason: "用户主动中断当前运行。",
		Metadata: map[string]string{
			"error_code": "run_cancelled", "error_message": detail, "error_source": "gateway",
			"completion_status": "cancelled", "completion_reason": "用户主动中断当前运行。",
			"final_answer": "任务已根据中断请求停止执行。", "task_title": runRequest.UserInput, "next_step": "任务已结束",
		},
	}
}
