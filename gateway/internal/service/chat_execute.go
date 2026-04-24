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
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	registry.Register(runRequest, cancel)
	defer registry.Finish(runRequest.RunID)
	defer cancel()
	eventBus.Publish(RunStartedEvent(runRequest))
	response, err := runtimeClient.Run(ctx, runRequest)
	if err != nil {
		publishRuntimeError(runRequest, eventBus, registry, err)
		return
	}
	eventBus.Publish(RuntimeReturnedEvent(runRequest, len(response.Events)))
	persistConfirmation(runRequest, response, confirmationStore)
	publishRuntimeEvents(response.Events, eventBus)
}

func publishRuntimeError(
	runRequest contracts.RunRequest,
	eventBus *session.EventBus,
	registry *ExecutionRegistry,
	err error,
) {
	if registry.WasCancelled(runRequest.RunID) || errors.Is(err, context.Canceled) {
		detail := "任务已被用户中断，Runtime 请求已取消。"
		eventBus.Publish(RunCancelledEvent(runRequest, detail))
		eventBus.Publish(RunCancelledFinishEvent(runRequest, detail))
		return
	}
	eventBus.Publish(RuntimeFailureEvent(runRequest, err.Error()))
	eventBus.Publish(RuntimeFailureFinishEvent(runRequest, err.Error()))
}

func persistConfirmation(
	runRequest contracts.RunRequest,
	response contracts.RuntimeRunResponse,
	confirmationStore *state.ConfirmationStore,
) {
	if response.ConfirmationRequest == nil {
		return
	}
	confirmationStore.Save(state.PendingConfirmation{
		Request:      runRequest,
		Confirmation: *response.ConfirmationRequest,
		CheckpointID: stringValue(response.Result.CheckpointID),
	})
}

func publishRuntimeEvents(events []contracts.RunEvent, eventBus *session.EventBus) {
	for _, item := range events {
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
