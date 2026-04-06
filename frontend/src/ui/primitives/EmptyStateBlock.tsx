type EmptyStateBlockProps = {
  compact?: boolean;
  text: string;
  title: string;
};

export function EmptyStateBlock(props: EmptyStateBlockProps) {
  const className = props.compact ? "empty-state compact" : "empty-state";
  return (
    <div className={className}>
      <h3>{props.title}</h3>
      <p>{props.text}</p>
    </div>
  );
}
