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
      <main className="app-shell">
        {props.topbar}
        {props.overlays}
        <div id="main-content" className="app-shell-main" tabIndex={-1}>
          {props.content}
        </div>
        {props.bottomPanel}
      </main>
    </>
  );
}
