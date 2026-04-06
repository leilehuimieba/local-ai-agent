type MetricChipProps = {
  className?: string;
  label: string;
  title?: string;
  value: string;
};

export function MetricChip(props: MetricChipProps) {
  const className = props.className ? `summary-chip ${props.className}` : "summary-chip";
  return (
    <div className={className}>
      <span>{props.label}</span>
      <strong title={props.title || props.value}>{props.value}</strong>
    </div>
  );
}
