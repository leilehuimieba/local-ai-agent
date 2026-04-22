import { HTMLAttributes, ReactNode } from "react";

type InfoCardProps = {
  children: ReactNode;
  className?: string;
} & HTMLAttributes<HTMLElement>;

export function InfoCard(props: InfoCardProps) {
  const { children, className, ...rest } = props;
  const nextClassName = className ? `info-card ${className}` : "info-card";
  return <section {...rest} className={nextClassName}>{children}</section>;
}
