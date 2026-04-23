type ViewId = "home" | "task" | "logs" | "settings" | "knowledge";

type LeftNavProps = {
  currentView: ViewId;
  onViewChange: (view: ViewId) => void;
  onNewTask: () => void;
};

const NAV_ITEMS = [
  { id: "home" as ViewId, icon: "\u2302", label: "首页" },
  { id: "task" as ViewId, icon: "\u25C9", label: "任务" },
  { id: "logs" as ViewId, icon: "\u2261", label: "记录" },
  { id: "knowledge" as ViewId, icon: "\u{1F4DA}", label: "知识库" },
  { id: "settings" as ViewId, icon: "\u2699", label: "设置" },
] as const;

export function LeftNav(props: LeftNavProps) {
  return (
    <nav className="app-left-nav" aria-label="主导航">
      {NAV_ITEMS.map((item) => (
        <button
          key={item.id}
          type="button"
          className={item.id === props.currentView ? "app-left-nav-item active" : "app-left-nav-item"}
          aria-label={item.label}
          aria-current={item.id === props.currentView ? "page" : undefined}
          onClick={() => props.onViewChange(item.id)}
        >
          <span aria-hidden="true">{item.icon}</span>
        </button>
      ))}
      <div style={{ flex: 1 }} />
      <button
        type="button"
        className="app-left-nav-item"
        aria-label="新建任务"
        onClick={props.onNewTask}
      >
        <span aria-hidden="true">+</span>
      </button>
    </nav>
  );
}
