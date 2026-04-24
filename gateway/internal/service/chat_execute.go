package service

import (
	"context"
	"errors"
	"time"

	"local-agent/gateway/internal/contracts"
	runtimeclient "local-agent/gateway/internal/runtime"
	"local-agent/gateway/internal/session"
	"local-agent/gateway/internal/state"
)

func Execute(
	runRequest contracts.RunRequest,
	runtimeClient *runtimeclient.Client,
	eventBus *session.EventBus,
	confirmationStore *state.ConfirmationStore,
	registry *ExecutionRegistry,
) {
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	registry.Register(runRequest, cancel)
	defer registry.Finish(runRequest.RunID)
	defer cancel()
	response, err := runtimeClient.Run(ctx, runRequest)
	if err != nil {
		if registry.WasCancelled(runRequest.RunID) || errors.Is(err, context.Canceled) {
			detail := "任务已被用户中断，Runtime 请求已取消。"
			eventBus.Publish(RunCancelledEvent(runRequest, detail))
			eventBus.Publish(RunCancelledFinishEvent(runRequest, detail))
			return
		}
		eventBus.Publish(RuntimeFailureEvent(runRequest, err.Error()))
		eventBus.Publish(RuntimeFailureFinishEvent(runRequest, err.Error()))
		return
	}
	if response.ConfirmationRequest != nil {
		confirmationStore.Save(state.PendingConfirmation{
			Request:      runRequest,
			Confirmation: *response.ConfirmationRequest,
			CheckpointID: stringValue(response.Result.CheckpointID),
		})
	}
	for _, item := range response.Events {
		eventBus.Publish(item)
		time.Sleep(120 * time.Millisecond)
	}
}

func stringValue(value *string) string {
	if value == nil {
		return ""
	}
	return *value
}
