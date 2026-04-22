import { ReactNode } from "react";

type AppShellProps = {
  topbar: ReactNode;
  overlays: ReactNode;
  content: ReactNode;
  bottomPanel: ReactNode;
};

export function AppShell(props: AppShellProps) {
  return (
    <>
      <a className="skip-link" href="#main-content">跳到主内容</a>
      <div className="app-shell-frame">
        <main className="app-shell">
          <header className="app-shell-topbar">{props.topbar}</header>
          {props.overlays ? <div className="app-shell-overlays">{props.overlays}</div> : null}
          <div className="app-shell-body">
            <div id="main-content" className="app-shell-main" tabIndex={-1}>
              {props.content}
            </div>
          </div>
          {props.bottomPanel ? <div className="app-shell-drawer">{props.bottomPanel}</div> : null}
        </main>
      </div>
    </>
  );
}
