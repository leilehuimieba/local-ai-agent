type StatusPillProps = {
  className?: string;
  label: string;
  title?: string;
};

export function StatusPill(props: StatusPillProps) {
  const className = props.className ? `status-badge ${props.className}` : "status-badge";
  return <span className={className} title={props.title}>{props.label}</span>;
}
