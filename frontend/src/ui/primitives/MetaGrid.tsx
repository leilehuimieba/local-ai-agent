import { ReactNode } from "react";

type MetaItem = {
  label: string;
  value: ReactNode;
};

type MetaGridProps = {
  className?: string;
  items: MetaItem[];
};

export function MetaGrid(props: MetaGridProps) {
  const className = props.className ? `detail-meta-grid ${props.className}` : "detail-meta-grid";
  return (
    <div className={className}>
      {props.items.map((item) => <MetaItemCard key={item.label} label={item.label} value={item.value} />)}
    </div>
  );
}

function MetaItemCard(props: MetaItem) {
  return (
    <div className="detail-meta-item">
      <span>{props.label}</span>
      <strong>{props.value}</strong>
    </div>
  );
}
