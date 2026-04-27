import { ChatPanel } from "../../chat/ChatPanel";
import { LogsPanel } from "../../logs/LogsPanel";
import { ReleaseWizardPanel } from "../../release/ReleaseWizardPanel";
import { SettingsPanel } from "../../settings/SettingsPanel";
import { KnowledgeBasePanel } from "../../knowledge-base/KnowledgeBasePanel";
import { WorkbenchOverview } from "../../workspace/WorkbenchOverview";
import { TopBar } from "../../workspace/TopBar";
import { getChatPanelProps, getSettingsPanelProps, getTopBarProps } from "./props";
import type { AppModel } from "./types";

export function renderWorkspaceContent(app: AppModel) {
  return renderTaskView(app);
}

export function renderDrawerContent(app: AppModel) {
  if (app.view.currentView === "settings") return <SettingsPanel {...getSettingsPanelProps(app)} />;
  return null;
}

function renderKnowledgeBaseView(_app: AppModel) {
  return (
    <section className="single-view">
      <KnowledgeBasePanel />
    </section>
  );
}

export function renderTopBar(app: AppModel, rightPanelOpen: boolean, onToggleRightPanel: () => void) {
  return <TopBar {...getTopBarProps(app, rightPanelOpen, onToggleRightPanel)} />;
}

export function renderHomeView(app: AppModel) {
  return (
    <section className="single-view">
      <WorkbenchOverview {...app.home} />
    </section>
  );
}

export function renderGlobalLayers(app: AppModel) {
  const errorBanner = renderCriticalErrorBanner(app);
  if (!errorBanner) return null;
  return <section className="global-layer-stack">{errorBanner}</section>;
}

export function renderMainView(app: AppModel) {
  if (app.view.currentView === "logs") return renderLogsView(app);
  if (app.view.currentView === "release") return renderReleaseView();
  if (app.view.currentView === "knowledge") return renderKnowledgeBaseView(app);
  return renderTaskView(app);
}

function renderReleaseView() {
  return (
    <section className="single-view">
      <ReleaseWizardPanel />
    </section>
  );
}

export function renderLogsView(app: AppModel) {
  return (
    <section className="single-view">
      <LogsPanel logs={app.logs.filteredLogs} />
    </section>
  );
}

export function renderTaskView(app: AppModel) {
  return (
    <section className="single-view task-chat-layout">
      <ChatPanel {...getChatPanelProps(app)} />
    </section>
  );
}

function renderSettingsView(app: AppModel) {
  return (
    <section className="single-view">
      <SettingsPanel {...getSettingsPanelProps(app)} />
    </section>
  );
}

function renderCriticalErrorBanner(app: AppModel) {
  const message = readCriticalErrorBannerMessage(app);
  if (!message) return null;
  return (
    <div className="global-banner banner-error" role="alert">
      <div>
        <strong>严重错误</strong>
        <p>{message}</p>
      </div>
      <button
        type="button"
        className="secondary-button"
        onClick={app.actions.dismissCriticalError}
      >
        关闭提示
      </button>
    </div>
  );
}

function readCriticalErrorBannerMessage(
  app: Pick<AppModel, "runtime" | "settingsApi">,
) {
  if (app.runtime.criticalError === "home_preview_blocked") return app.settingsApi.bootstrapError;
  return app.runtime.criticalError || app.settingsApi.bootstrapError;
}
