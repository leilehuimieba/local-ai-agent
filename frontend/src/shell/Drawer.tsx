import { useCallback, useState, type ReactNode } from "react";

type DrawerProps = {
  title: string;
  children: ReactNode;
  onClose: () => void;
};

export function Drawer(props: DrawerProps) {
  const [closing, setClosing] = useState(false);
  const handleClose = useCallback(() => {
    setClosing(true);
    setTimeout(() => {
      setClosing(false);
      props.onClose();
    }, 260);
  }, [props]);
  return (
    <div className={closing ? "drawer-overlay closing" : "drawer-overlay"} onClick={(event) => { if (event.target === event.currentTarget) handleClose(); }}>
      <aside className="drawer-panel" role="dialog" aria-label={props.title}>
        <div className="drawer-header">
          <strong>{props.title}</strong>
          <button type="button" className="drawer-close" aria-label="关闭" onClick={handleClose}>✕</button>
        </div>
        <div className="drawer-body">{props.children}</div>
      </aside>
    </div>
  );
}
