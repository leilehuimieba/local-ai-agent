import { ElementType, ReactNode } from "react";

type SectionHeaderProps = {
  action?: ReactNode;
  className?: string;
  description?: string;
  kicker?: string;
  kind?: "page" | "section" | "head";
  level?: "h1" | "h2" | "h3";
  title: string;
};

export function SectionHeader(props: SectionHeaderProps) {
  const Tag = (props.level || "h3") as ElementType;
  const baseClass = readHeaderClass(props.kind);
  const className = props.className ? `${baseClass} ${props.className}` : baseClass;
  return (
    <div className={className}>
      <div>
        {props.kicker ? <span className="section-kicker">{props.kicker}</span> : null}
        <Tag>{props.title}</Tag>
        {props.description ? <p>{props.description}</p> : null}
      </div>
      {props.action}
    </div>
  );
}

function readHeaderClass(kind?: SectionHeaderProps["kind"]) {
  if (kind === "page") return "page-shell-header";
  if (kind === "head") return "section-head";
  return "section-header";
}
