import { ReactNode } from "react";

type AppShellProps = {
  topbar: ReactNode;
  leftNav: ReactNode;
  overlays: ReactNode;
  content: ReactNode;
  rightPanel: ReactNode;
  bottomPanel: ReactNode;
};

export function AppShell(props: AppShellProps) {
  const hasRightPanel = Boolean(props.rightPanel);
  return (
    <div className="app-layout">
      <header className="app-topbar">{props.topbar}</header>
      <div className="app-body">
        {props.leftNav}
        <div className="app-main">
          {props.overlays}
          <main className="app-content">{props.content}</main>
          {props.bottomPanel}
        </div>
        {hasRightPanel ? (
          <aside className="app-right-panel">{props.rightPanel}</aside>
        ) : null}
      </div>
    </div>
  );
}
