const SETTINGS_SECTIONS = [
  { id: "settings-module-runtime", label: "运行环境" },
  { id: "settings-module-model", label: "模型与模式" },
  { id: "settings-module-provider", label: "Provider 凭证" },
  { id: "settings-module-workspace", label: "工作区与授权" },
  { id: "settings-module-risk", label: "风险与权限" },
  { id: "settings-module-resources", label: "记忆与资源" },
  { id: "settings-module-diagnostics", label: "诊断与导出" },
];

export function SettingsNavPanel() {
  return (
    <section className="task-nav-panel settings-nav-panel">
      <header className="task-nav-panel-head">
        <strong>设置</strong>
        <span>模块导航</span>
      </header>
      <div className="settings-nav-list">
        {SETTINGS_SECTIONS.map((section) => (
          <button
            key={section.id}
            type="button"
            className="task-history-item"
            onClick={() => {
              document.getElementById(section.id)?.scrollIntoView({ behavior: "smooth", block: "start" });
            }}
          >
            <span>{section.label}</span>
          </button>
        ))}
      </div>
    </section>
  );
}
