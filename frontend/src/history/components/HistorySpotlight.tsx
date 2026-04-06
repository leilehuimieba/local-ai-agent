import { LogEntry } from "../../shared/contracts";
import { MetricChip, SectionHeader } from "../../ui/primitives";
import { getReviewSpotlight, ReviewTone } from "../viewModel";

export function HistoryReviewSpotlight(props: { focusLog: LogEntry | null }) {
  if (!props.focusLog) return <EmptyReviewSpotlight />;
  const spotlight = getReviewSpotlight(props.focusLog);
  return (
    <section className="review-spotlight logs-focus-strip">
      <SectionHeader
        className="review-spotlight-head"
        kicker="焦点"
        title="复盘摘要"
        action={
          <div className="review-chip-row">
            {spotlight.chips.map((chip) => <MetricChip key={`${chip.label}-${chip.value}`} label={chip.label} value={chip.value} />)}
          </div>
        }
      />
      <div className="review-card-grid">
        {spotlight.cards.slice(0, 4).map((card) => <HistoryReviewCard key={card.label} {...card} />)}
      </div>
    </section>
  );
}

function EmptyReviewSpotlight() {
  return (
    <section className="review-spotlight logs-focus-strip">
      <SectionHeader className="review-spotlight-head" kicker="焦点" title="复盘摘要" />
    </section>
  );
}

function HistoryReviewCard(props: { label: string; value: string; tone: ReviewTone }) {
  return (
    <article className={`review-card tone-${props.tone}`}>
      <span>{props.label}</span>
      <strong>{props.value}</strong>
    </article>
  );
}
