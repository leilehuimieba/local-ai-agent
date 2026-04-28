package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"path/filepath"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/contracts"
	"local-agent/gateway/internal/state"
)

func externalConnectionActionHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, err := decodeExternalConnectionAction(w, r)
		if err != nil {
			return
		}
		response, status := executeExternalConnectionAction(repoRoot, cfg, store, payload)
		writeJSON(w, status, response)
	}
}

func decodeExternalConnectionAction(w http.ResponseWriter, r *http.Request) (ExternalConnectionActionRequest, error) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return ExternalConnectionActionRequest{}, errors.New("method not allowed")
	}
	var payload ExternalConnectionActionRequest
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return ExternalConnectionActionRequest{}, err
	}
	return payload, nil
}

func executeExternalConnectionAction(repoRoot string, cfg config.AppConfig, store *state.SettingsStore, payload ExternalConnectionActionRequest) (ExternalConnectionActionResponse, int) {
	if err := validateExternalConnectionAction(payload); err != nil {
		return ExternalConnectionActionResponse{SlotID: payload.SlotID, Action: payload.Action, OK: false, Message: err.Error()}, http.StatusBadRequest
	}
	settings := buildSettingsResponse(repoRoot, cfg, store)
	slot, ok := findExternalConnection(settings.ExternalConnections, payload.SlotID)
	if !ok {
		return ExternalConnectionActionResponse{SlotID: payload.SlotID, Action: payload.Action, OK: false, Message: "slot_id \u4e0d\u5b58\u5728"}, http.StatusNotFound
	}
	if !supportsExternalConnectionAction(payload.SlotID) {
		return buildExternalConnectionActionResponse(payload, slot, false, "\u5f53\u524d\u4e0d\u652f\u6301\u8be5\u8fde\u63a5\u52a8\u4f5c", settings.ExternalConnections), http.StatusOK
	}
	return buildExternalConnectionActionResponse(payload, slot, slot.Status == "active", slot.NextStep, settings.ExternalConnections), http.StatusOK
}

func validateExternalConnectionAction(payload ExternalConnectionActionRequest) error {
	if payload.SlotID == "" {
		return errors.New("slot_id is required")
	}
	if payload.Action == "" {
		return errors.New("action is required")
	}
	if payload.Action != "validate" && payload.Action != "recheck" {
		return errors.New("action must be validate or recheck")
	}
	return nil
}

func buildExternalConnectionActionResponse(payload ExternalConnectionActionRequest, slot ExternalConnectionSlot, ok bool, message string, slots []ExternalConnectionSlot) ExternalConnectionActionResponse {
	slotCopy := slot
	return ExternalConnectionActionResponse{
		SlotID: payload.SlotID, Action: payload.Action, OK: ok, Message: message,
		UpdatedSlot: &slotCopy, ExternalConnections: slots,
	}
}

func findExternalConnection(slots []ExternalConnectionSlot, slotID string) (ExternalConnectionSlot, bool) {
	for _, slot := range slots {
		if slot.SlotID == slotID {
			return slot, true
		}
	}
	return ExternalConnectionSlot{}, false
}

func supportsExternalConnectionAction(slotID string) bool {
	return slotID == "local_files_project" || slotID == "local_notes_knowledge"
}

func applyConnectionCheck(slot ExternalConnectionSlot, err error) ExternalConnectionSlot {
	if err == nil {
		return slot
	}
	slot.Status = "limited"
	slot.NextStep = err.Error()
	return slot
}

func validateLocalFilesProject(repoRoot string, workspace contracts.WorkspaceRef) error {
	return errors.Join(
		checkAccessibleDirectory("\u9879\u76ee\u6839\u76ee\u5f55", repoRoot),
		checkAccessibleDirectory("\u5de5\u4f5c\u533a\u6839\u76ee\u5f55", workspace.RootPath),
	)
}

func validateLocalNotesKnowledge(repoRoot string, cfg config.AppConfig) error {
	return errors.Join(
		checkAccessibleDirectory("\u77e5\u8bc6\u5e93\u76ee\u5f55", filepath.Join(repoRoot, "data", "knowledge_base")),
		checkAccessibleDirectory("\u601d\u6e90\u6839\u76ee\u5f55", cfg.Siyuan.RootDir),
		checkAccessibleDirectory("\u601d\u6e90\u5bfc\u51fa\u76ee\u5f55", cfg.Siyuan.ExportDir),
	)
}

func checkAccessibleDirectory(label string, path string) error {
	if path == "" {
		return fmt.Errorf("%s\u672a\u914d\u7f6e", label)
	}
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("%s\u4e0d\u53ef\u7528: %w", label, err)
	}
	if !info.IsDir() {
		return fmt.Errorf("%s\u4e0d\u662f\u76ee\u5f55: %s", label, path)
	}
	_, err = os.ReadDir(path)
	if err != nil {
		return fmt.Errorf("%s\u4e0d\u53ef\u8bbf\u95ee: %w", label, err)
	}
	return nil
}
