package api

import (
	"net/http"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

func diagnosticsCheckHandler(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		settings := buildSettingsResponse(repoRoot, cfg, store)
		diagnostics := settings.Diagnostics
		writeJSON(w, http.StatusOK, DiagnosticsCheckResponse{
			CheckedAt: diagnostics.CheckedAt, OverallOK: len(diagnostics.Errors) == 0,
			Diagnostics: diagnostics, Warnings: diagnostics.Warnings, Errors: diagnostics.Errors,
		})
	}
}

func finalizeDiagnostics(status DiagnosticsStatus) DiagnosticsStatus {
	status.Warnings = diagnosticsWarnings(status)
	status.Errors = diagnosticsErrors(status)
	return status
}

func diagnosticsWarnings(status DiagnosticsStatus) []string {
	var warnings []string
	appendIfMissing(&warnings, !status.SettingsPathExists, "\u8bbe\u7f6e\u5feb\u7167\u6587\u4ef6\u5c1a\u672a\u751f\u6210\u3002")
	appendIfMissing(&warnings, !status.RunLogPathExists, "\u8fd0\u884c\u65e5\u5fd7\u6587\u4ef6\u5c1a\u672a\u751f\u6210\u3002")
	appendIfMissing(&warnings, !status.EventLogPathExists, "\u4e8b\u4ef6\u65e5\u5fd7\u6587\u4ef6\u5c1a\u672a\u751f\u6210\u3002")
	appendIfMissing(&warnings, !status.StorageRootExists, "\u5b58\u50a8\u6839\u76ee\u5f55\u5c1a\u672a\u751f\u6210\u3002")
	appendIfMissing(&warnings, !status.WorkingMemoryDirExists, "\u77ed\u671f\u5de5\u4f5c\u8bb0\u5fc6\u76ee\u5f55\u5c1a\u672a\u751f\u6210\u3002")
	appendIfMissing(&warnings, !status.KnowledgeBasePathExists, "\u77e5\u8bc6\u5e93\u76ee\u5f55\u5c1a\u672a\u751f\u6210\u3002")
	return warnings
}

func diagnosticsErrors(status DiagnosticsStatus) []string {
	var errors []string
	appendIfMissing(&errors, !status.RepoRootExists, "\u4ed3\u5e93\u6839\u76ee\u5f55\u4e0d\u5b58\u5728\u6216\u4e0d\u53ef\u8bbf\u95ee\u3002")
	appendIfMissing(&errors, !status.RuntimeReachable, "Runtime \u5f53\u524d\u4e0d\u53ef\u8fbe\u3002")
	appendIfMissing(&errors, !status.SiyuanRootExists && status.SiyuanSyncEnabled, "\u601d\u6e90\u6839\u76ee\u5f55\u4e0d\u5b58\u5728\u6216\u4e0d\u53ef\u8bbf\u95ee\u3002")
	appendIfMissing(&errors, !status.SiyuanExportDirExists && status.SiyuanAutoWriteEnabled, "\u601d\u6e90\u5bfc\u51fa\u76ee\u5f55\u4e0d\u5b58\u5728\u6216\u4e0d\u53ef\u8bbf\u95ee\u3002")
	return errors
}

func appendIfMissing(items *[]string, condition bool, message string) {
	if condition {
		*items = append(*items, message)
	}
}
