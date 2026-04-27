package api

import (
	"fmt"
	"path/filepath"
	"time"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/memory"
	"local-agent/gateway/internal/state"
	"local-agent/gateway/internal/util"
)

func buildSettingsResponse(repoRoot string, cfg config.AppConfig, store *state.SettingsStore) SettingsResponse {
	mode, model, models, workspace, workspaces, directoryPromptEnabled, showRiskLevel, approvals := store.Snapshot()
	runtimeStatus := fetchRuntimeStatus(cfg.RuntimePort)
	return SettingsResponse{
		AppName:                cfg.AppName,
		Mode:                   mode,
		Model:                  model,
		AvailableModels:        models,
		Providers:              providerOptions(cfg.Providers),
		Workspace:              workspace,
		AvailableWorkspaces:    workspaces,
		ApprovedDirectories:    approvedDirectories(approvals),
		DirectoryPromptEnabled: directoryPromptEnabled,
		ShowRiskLevel:          showRiskLevel,
		Ports:                  map[string]int{"gateway": cfg.GatewayPort, "runtime": cfg.RuntimePort},
		RuntimeStatus:          runtimeStatus,
		MemoryPolicy:           buildMemoryPolicy(repoRoot, workspace),
		Diagnostics:            buildDiagnostics(repoRoot, cfg, runtimeStatus, len(models), len(workspaces), len(approvals)),
		ExternalConnections:    buildExternalConnections(repoRoot, cfg, workspace),
	}
}

func buildMemoryPolicy(repoRoot string, workspace config.WorkspaceRef) MemoryPolicyStatus {
	storageRoot := filepath.Join(repoRoot, "data")
	longTermPath := filepath.Join(storageRoot, "long_term_memory", fmt.Sprintf("%s.jsonl", workspace.WorkspaceID))
	knowledgePath := filepath.Join(storageRoot, "knowledge_base", fmt.Sprintf("%s.jsonl", workspace.WorkspaceID))
	return MemoryPolicyStatus{
		Enabled:             true,
		RecallStrategy:      "按需召回，使用摘要注入，不做全量记忆加载。",
		WriteStrategy:       "短期工作记忆实时落盘，任务收口阶段补长期记忆与知识沉淀。",
		CleanupStrategy:     "SQLite 主存储保留正式数据，legacy JSONL 在导入与访问时做去重压实。",
		StorageRoot:         storageRoot,
		SQLitePath:          filepath.Join(storageRoot, "storage", "main.db"),
		WorkingMemoryDir:    filepath.Join(storageRoot, "working_memory"),
		LongTermMemoryPath:  longTermPath,
		KnowledgeBasePath:   knowledgePath,
		LongTermMemoryCount: memoryCount(repoRoot, workspace.WorkspaceID),
		KnowledgeCount:      util.CountJSONLLines(knowledgePath),
		WorkingMemoryFiles:  util.CountDirEntries(filepath.Join(storageRoot, "working_memory")),
	}
}

func buildDiagnostics(repoRoot string, cfg config.AppConfig, runtimeStatus RuntimeStatus, modelCount int, workspaceCount int, approvalCount int) DiagnosticsStatus {
	storageRoot := filepath.Join(repoRoot, "data")
	settingsPath, runLogPath, eventLogPath := diagnosticsStoragePaths(repoRoot, storageRoot)
	status := buildDiagnosticsBase(diagnosticsBaseArgs{
		repoRoot:       repoRoot,
		storageRoot:    storageRoot,
		settingsPath:   settingsPath,
		runLogPath:     runLogPath,
		eventLogPath:   eventLogPath,
		runtimeStatus:  runtimeStatus,
		providerCount:  len(cfg.Providers),
		modelCount:     modelCount,
		workspaceCount: workspaceCount,
		approvalCount:  approvalCount,
	})
	status = withSiyuanDiagnostics(status, cfg.Siyuan)
	return finalizeDiagnostics(status)
}

func diagnosticsStoragePaths(repoRoot string, storageRoot string) (string, string, string) {
	settingsPath := filepath.Join(storageRoot, "settings", "ui-state.json")
	runLogPath := filepath.Join(repoRoot, "logs", "run-logs.jsonl")
	eventLogPath := filepath.Join(repoRoot, "logs", "run-events.jsonl")
	return settingsPath, runLogPath, eventLogPath
}

type diagnosticsBaseArgs struct {
	repoRoot       string
	storageRoot    string
	settingsPath   string
	runLogPath     string
	eventLogPath   string
	runtimeStatus  RuntimeStatus
	providerCount  int
	modelCount     int
	workspaceCount int
	approvalCount  int
}

func buildDiagnosticsBase(args diagnosticsBaseArgs) DiagnosticsStatus {
	return DiagnosticsStatus{
		CheckedAt:               fmt.Sprintf("%d", time.Now().UnixMilli()),
		RepoRoot:                args.repoRoot,
		RepoRootExists:          util.PathExists(args.repoRoot),
		StorageRoot:             args.storageRoot,
		StorageRootExists:       util.PathExists(args.storageRoot),
		SettingsPath:            args.settingsPath,
		SettingsPathExists:      util.PathExists(args.settingsPath),
		RunLogPath:              args.runLogPath,
		RunLogPathExists:        util.PathExists(args.runLogPath),
		EventLogPath:            args.eventLogPath,
		EventLogPathExists:      util.PathExists(args.eventLogPath),
		WorkingMemoryDirExists:  util.PathExists(filepath.Join(args.storageRoot, "working_memory")),
		KnowledgeBasePathExists: util.PathExists(filepath.Join(args.storageRoot, "knowledge_base")),
		RuntimeReachable:        args.runtimeStatus.OK,
		RuntimeVersion:          fmt.Sprintf("%s / %s", args.runtimeStatus.Name, args.runtimeStatus.Version),
		ProviderCount:           args.providerCount,
		ModelCount:              args.modelCount,
		WorkspaceCount:          args.workspaceCount,
		ApprovedDirectoryCount:  args.approvalCount,
	}
}

func withSiyuanDiagnostics(status DiagnosticsStatus, siyuan config.SiyuanConfig) DiagnosticsStatus {
	status.SiyuanRoot = siyuan.RootDir
	status.SiyuanRootExists = util.PathExists(siyuan.RootDir)
	status.SiyuanExportDir = siyuan.ExportDir
	status.SiyuanExportDirExists = util.PathExists(siyuan.ExportDir)
	status.SiyuanAutoWriteEnabled = siyuan.AutoWriteEnabled
	status.SiyuanSyncEnabled = siyuan.SyncEnabled
	return status
}

func buildExternalConnections(repoRoot string, cfg config.AppConfig, workspace config.WorkspaceRef) []ExternalConnectionSlot {
	return []ExternalConnectionSlot{
		localFilesConnection(repoRoot, workspace),
		localNotesConnection(repoRoot, cfg),
		browserCaptureConnection(),
		personalManagementConnection(),
	}
}

func localFilesConnection(repoRoot string, workspace config.WorkspaceRef) ExternalConnectionSlot {
	slot := makeExternalConnection("local_files_project", "本地文件与项目目录", 1, "active", []string{"workspace_list", "workspace_read", "workspace_write", "workspace_delete", "run_command"}, "继续作为主链路第一优先级，不引入外部 SaaS 依赖。", "本地项目目录校验通过，可继续使用目录读写与命令能力。")
	return applyConnectionCheck(slot, validateLocalFilesProject(repoRoot, workspace))
}

func localNotesConnection(repoRoot string, cfg config.AppConfig) ExternalConnectionSlot {
	slot := makeExternalConnection("local_notes_knowledge", "本地笔记与知识库", 2, "active", []string{"knowledge_search", "search_siyuan_notes", "read_siyuan_note", "write_siyuan_knowledge"}, "继续坚持 SQLite 主索引、思源外挂正文库，不让笔记系统承接高频主存储。", "本地知识库与思源目录校验通过，可继续使用知识读写能力。")
	return applyConnectionCheck(slot, validateLocalNotesKnowledge(repoRoot, cfg))
}

func browserCaptureConnection() ExternalConnectionSlot {
	return makeExternalConnection("browser_capture_ingest", "浏览器摘录与网页入库", 3, "reserved", []string{"knowledge_search", "write_siyuan_knowledge", "project_answer"}, "本阶段只保留知识读写接入口，不提前接重型浏览器插件或云同步。", "当前不支持动作校验，只保留规划位。")
}

func personalManagementConnection() ExternalConnectionSlot {
	return makeExternalConnection("calendar_reminder_management", "日历、提醒与更重的个人管理连接", 4, "reserved", []string{"session_context", "project_answer"}, "当前阶段只保留规划位，不接日历、提醒、任务中心等重连接器。", "当前不支持动作校验，只保留规划位。")
}

func makeExternalConnection(slotID string, displayName string, priority int, status string, currentTools []string, boundary string, nextStep string) ExternalConnectionSlot {
	return ExternalConnectionSlot{
		SlotID: slotID, DisplayName: displayName, Priority: priority, Status: status,
		Scope: "external_connection", CurrentTools: currentTools,
		SupportedActions: supportedExternalConnectionActions(slotID),
		Boundary:         boundary, NextStep: nextStep,
	}
}

func supportedExternalConnectionActions(slotID string) []string {
	if supportsExternalConnectionAction(slotID) {
		return []string{"recheck"}
	}
	return nil
}


func memoryCount(repoRoot string, workspaceID string) int {
	items, err := memory.NewStore(repoRoot).List(workspaceID)
	if err != nil {
		return 0
	}
	return len(items)
}

func approvedDirectories(items []state.ApprovedDirectoryRecord) []DirectoryApproval {
	approvals := make([]DirectoryApproval, 0, len(items))
	for _, item := range items {
		approvals = append(approvals, DirectoryApproval{
			ApprovalID:  item.ApprovalID,
			WorkspaceID: item.WorkspaceID,
			Name:        item.Name,
			RootPath:    item.RootPath,
			CreatedAt:   item.CreatedAt,
		})
	}
	return approvals
}

