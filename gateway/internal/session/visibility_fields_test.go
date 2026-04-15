package session

import (
	"testing"

	"local-agent/gateway/internal/contracts"

	"github.com/stretchr/testify/require"
)

func TestVisibilityFieldsFromMetadataAreNormalized(t *testing.T) {
	bus := NewEventBus(t.TempDir())
	bus.Publish(newVisibilityRunEvent())

	snapshot := bus.Snapshot("s-visibility")
	require.Len(t, snapshot, 1)
	assertVisibilityRunEvent(t, snapshot[0])

	logs := bus.RecentBy(10, "s-visibility", "r-visibility")
	require.Len(t, logs, 1)
	assertVisibilityLogEntry(t, logs[0])
}

func newVisibilityRunEvent() contracts.RunEvent {
	return contracts.RunEvent{
		EventID: "event-visibility-1", EventType: "action_requested", SessionID: "s-visibility",
		RunID: "r-visibility", TraceID: "trace-visibility", Sequence: 1, Timestamp: "1776236110850",
		Stage: "Act", Summary: "准备调用工具", Metadata: visibilityMetadata(),
	}
}

func visibilityMetadata() map[string]string {
	return map[string]string{
		"activity_state": "running", "heartbeat_at": "1776236110850", "stall_seconds": "0",
		"waiting_reason": "confirmation", "next_action_hint": "等待用户确认", "failure_route": "manual",
		"updated_at": "1776236110850", "task_title": "执行命令: Get-Date", "active_tool": "run_command",
		"evidence_ref": "event_id=event-visibility-1", "tool_name": "run_command",
	}
}

func assertVisibilityRunEvent(t *testing.T, item contracts.RunEvent) {
	require.Equal(t, "running", item.ActivityState)
	require.Equal(t, "1776236110850", item.HeartbeatAt)
	require.Equal(t, "0", item.StallSeconds)
	require.Equal(t, "confirmation", item.WaitingReason)
	require.Equal(t, "等待用户确认", item.NextActionHint)
	require.Equal(t, "manual", item.FailureRoute)
	require.Equal(t, "1776236110850", item.UpdatedAt)
	require.Equal(t, "执行命令: Get-Date", item.TaskTitle)
	require.Equal(t, "run_command", item.ActiveTool)
	require.Equal(t, "event_id=event-visibility-1", item.EvidenceRef)
}

func assertVisibilityLogEntry(t *testing.T, item contracts.LogEntry) {
	require.Equal(t, "running", item.ActivityState)
	require.Equal(t, "1776236110850", item.HeartbeatAt)
	require.Equal(t, "0", item.StallSeconds)
	require.Equal(t, "confirmation", item.WaitingReason)
	require.Equal(t, "等待用户确认", item.NextActionHint)
	require.Equal(t, "manual", item.FailureRoute)
	require.Equal(t, "1776236110850", item.UpdatedAt)
	require.Equal(t, "执行命令: Get-Date", item.TaskTitle)
	require.Equal(t, "run_command", item.ActiveTool)
	require.Equal(t, "event_id=event-visibility-1", item.EvidenceRef)
}
