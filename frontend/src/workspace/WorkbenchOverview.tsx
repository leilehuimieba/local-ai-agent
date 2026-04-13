import { FormEvent } from "react";

import { MetricChip, SectionHeader } from "../ui/primitives";

type HomeStateKind = "first_use" | "resume" | "blocked";
type ResumeItem = { label: string; value: string };

type WorkbenchOverviewProps = {
  kind: HomeStateKind;
  navHint: string;
  composeValue: string;
  canSubmit: boolean;
  isSubmitting: boolean;
  eventCount: number;
  hasConfirmation: boolean;
  envItems: Array<{ label: string; value: string }>;
  examples: ReadonlyArray<{ id: string; label: string; prompt: string }>;
  resumeCard: {
    recentTask: string;
    recentStage: string;
    latestSummary: string;
    nextStep: string;
    runId: string;
    sessionId: string;
    contextItems: ResumeItem[];
    evidenceItems: ResumeItem[];
  };
  systemCard: {
    judgement: string;
    connection: string;
    mode: string;
    workspace: string;
  };
  blockCard: {
    action: "reconnect" | "settings" | "workspace" | "model";
    title: string;
    body: string;
    detail: string;
  } | null;
  recentActivities: Array<{ id: string; label: string; text: string }>;
  onComposeValueChange: (value: string) => void;
  onOpenLogsPage: () => void;
  onReconnect: () => void;
  onOpenSettingsPage: () => void;
  onOpenTaskPage: () => void;
  onOpenTaskPageForConfirmation: () => void;
  onPrefillExample: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};

export function WorkbenchOverview(props: WorkbenchOverviewProps) {
  return (
    <article className={`panel workbench-overview overview-page home-state-${props.kind}`}>
      <HomeContent props={props} />
    </article>
  );
}

function HomeContent(props: { props: WorkbenchOverviewProps }) {
  if (props.props.kind === "blocked") return <BlockedHome props={props.props} />;
  if (props.props.kind === "resume") return <ResumeHome props={props.props} />;
  return <FirstUseHome props={props.props} />;
}

function FirstUseHome(props: { props: WorkbenchOverviewProps }) {
  return (
    <>
      <section className="home-hero-section">
        <SectionHeader
          kicker="开始"
          kind="head"
          level="h2"
          title="今天想让本地智能体帮你完成什么？"
          description="它可以执行命令、处理文件、调用本地程序，并把过程、结果和风险提示清楚展示给你。"
        />
        <FirstUseForm props={props.props} />
      </section>
      <section className="home-grid home-grid-two">
        <ExamplesSection props={props.props} />
        <EnvironmentSection props={props.props} />
      </section>
      <CapabilitiesSection />
    </>
  );
}

function FirstUseForm(props: { props: WorkbenchOverviewProps }) {
  const disabled = !props.props.canSubmit || props.props.isSubmitting || !props.props.composeValue.trim();
  return (
    <form className="home-compose-form" onSubmit={props.props.onSubmit}>
      <label className="home-compose-label" htmlFor="home-task-input">输入任务</label>
      <textarea
        id="home-task-input"
        name="home_task_input"
        className="composer-input home-compose-input"
        rows={4}
        value={props.props.composeValue}
        placeholder="例如：帮我检查这个项目为什么构建失败，并给出最小修改方案"
        onChange={(event) => props.props.onComposeValueChange(event.target.value)}
      />
      <div className="home-compose-footer">
        <p>提交后会进入任务页，持续展示执行过程、风险确认和结果收口。</p>
        <button type="submit" className="primary-action" disabled={disabled}>
          {props.props.isSubmitting ? "开始任务中" : "开始任务"}
        </button>
      </div>
    </form>
  );
}

function ExamplesSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="快速开始" kind="head" title="快速开始" />
      <div className="home-example-list">
        {props.props.examples.map((item) => (
          <button
            key={item.id}
            type="button"
            className="utility-card home-example-card"
            onClick={() => props.props.onPrefillExample(item.prompt)}
          >
            <strong>{item.label}</strong>
            <span>{item.prompt}</span>
          </button>
        ))}
      </div>
    </section>
  );
}

function EnvironmentSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="当前环境" kind="head" title="当前环境" />
      <div className="home-metric-grid">
        {props.props.envItems.map((item) => (
          <MetricChip key={item.label} className="metric-chip" label={item.label} value={item.value} />
        ))}
      </div>
    </section>
  );
}

function CapabilitiesSection() {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="你可以这样使用" kind="head" title="你可以这样使用" />
      <div className="home-capability-list">
        <CapabilityItem title="本地执行" text="直接操作当前工作区内的文件和命令" />
        <CapabilityItem title="过程可见" text="任务推进、工具动作和结果摘要持续可见" />
        <CapabilityItem title="风险可控" text="高风险动作不会绕过确认" />
      </div>
    </section>
  );
}

function CapabilityItem(props: { title: string; text: string }) {
  return (
    <article className="summary-card home-capability-card">
      <strong>{props.title}</strong>
      <p>{props.text}</p>
    </article>
  );
}

function ResumeHome(props: { props: WorkbenchOverviewProps }) {
  const primaryLabel = readResumePrimaryLabel(props.props.hasConfirmation);
  const helperText = props.props.hasConfirmation
    ? "当前任务停在确认节点，处理后才能继续推进。"
    : "进入任务页继续主线程，或去记录页查看历史复盘。";
  return (
    <>
      <section className="home-grid home-grid-two home-grid-top">
        <ResumeContextSection props={props.props} />
        <ResumeActionSection helperText={helperText} primaryLabel={primaryLabel} props={props.props} />
      </section>
      <section className="home-grid home-grid-two">
        <SystemRiskSection props={props.props} />
        <RecentActivitySection props={props.props} />
      </section>
    </>
  );
}

function readResumePrimaryLabel(hasConfirmation: boolean) {
  return hasConfirmation ? "处理待确认动作" : "继续任务主线程";
}

function ResumeContextSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="恢复上下文" kind="head" title="恢复上下文" />
      <p className="sidebar-title">{props.props.resumeCard.recentTask}</p>
      <div className="detail-list">
        <Row label="最近任务" value={props.props.resumeCard.recentTask} />
        <Row label="当前阶段" value={props.props.resumeCard.recentStage} />
        <Row label="最近摘要" value={props.props.resumeCard.latestSummary} />
      </div>
      <ResumeDigestGrid items={props.props.resumeCard.contextItems} />
      <ResumeEvidenceSection items={props.props.resumeCard.evidenceItems} />
    </section>
  );
}

function ResumeActionSection(props: {
  props: WorkbenchOverviewProps;
  primaryLabel: string;
  helperText: string;
}) {
  return (
    <section className="home-card-section home-actions-section">
      <SectionHeader kicker="继续" kind="head" title="从这里继续" />
      <div className="home-primary-actions">
        <button
          type="button"
          className="primary-action"
          onClick={readPrimaryActionHandler(props.props)}
        >
          {props.primaryLabel}
        </button>
        <button type="button" className="secondary-button" onClick={props.props.onOpenLogsPage}>
          查看记录页复盘
        </button>
      </div>
      <p>{props.helperText}</p>
      <ResumeNextStepNote text={props.props.resumeCard.nextStep} />
    </section>
  );
}

function ResumeDigestGrid(props: { items: ResumeItem[] }) {
  return (
    <div className="home-resume-digest-grid">
      {props.items.map((item) => <ResumeDigestCard key={item.label} item={item} />)}
    </div>
  );
}

function ResumeDigestCard(props: { item: ResumeItem }) {
  return (
    <article className="detail-card home-resume-digest-card">
      <strong>{props.item.label}</strong>
      <p>{props.item.value}</p>
    </article>
  );
}

function ResumeEvidenceSection(props: { items: ResumeItem[] }) {
  return (
    <div className="home-resume-evidence-list">
      {props.items.map((item) => <ResumeEvidenceCard key={item.label} item={item} />)}
    </div>
  );
}

function ResumeEvidenceCard(props: { item: ResumeItem }) {
  return (
    <article className="summary-card home-resume-evidence-card">
      <strong>{props.item.label}</strong>
      <span>{props.item.value}</span>
    </article>
  );
}

function ResumeNextStepNote(props: { text: string }) {
  return (
    <div className="inline-note home-resume-next-step">
      <strong>下一步线索</strong>
      <p>{props.text}</p>
    </div>
  );
}

function readPrimaryActionHandler(props: WorkbenchOverviewProps) {
  return props.hasConfirmation ? props.onOpenTaskPageForConfirmation : props.onOpenTaskPage;
}

function SystemRiskSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="系统与风险" kind="head" title="系统与风险" />
      <div className="detail-list">
        <Row label="当前判断" value={props.props.systemCard.judgement} />
        <Row label="连接" value={props.props.systemCard.connection} />
        <Row label="模式" value={props.props.systemCard.mode} />
        <Row label="工作区" value={props.props.systemCard.workspace} />
        <Row label="会话事件" value={`${props.props.eventCount} 条`} />
      </div>
    </section>
  );
}

function RecentActivitySection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="最近活动" kind="head" title="最近活动" />
      {props.props.recentActivities.length > 0 ? (
        <div className="home-activity-list">
          {props.props.recentActivities.map((item) => (
            <article key={item.id} className="detail-card home-activity-card">
              <strong>{item.label}</strong>
              <p>{item.text}</p>
            </article>
          ))}
        </div>
      ) : (
        <p>当前还没有可恢复的活动记录。</p>
      )}
    </section>
  );
}

function BlockedHome(props: { props: WorkbenchOverviewProps }) {
  return (
    <>
      <section className="home-block-section">
        <SectionHeader
          kicker="阻塞"
          kind="head"
          level="h2"
          title={props.props.blockCard?.title || "当前无法继续"}
          description={props.props.blockCard?.body || "当前存在阻塞，请先处理后继续。"}
        />
        <div className="home-block-detail">
          <strong>更多信息</strong>
          <p>{props.props.blockCard?.detail || "暂无更多信息。"}</p>
        </div>
      </section>
      <section className="home-grid home-grid-two">
        <RecommendedActionSection props={props.props} />
        <BlockDiagnosticsSection props={props.props} />
      </section>
    </>
  );
}

function RecommendedActionSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section home-actions-section">
      <SectionHeader kicker="建议先做这一步" kind="head" title="建议先做这一步" />
      <div className="home-primary-actions">
        <button type="button" className="primary-action" onClick={readBlockAction(props.props)}>
          {readBlockButtonLabel(props.props.blockCard?.action)}
        </button>
        <button type="button" className="secondary-button" onClick={props.props.onOpenLogsPage}>
          查看记录页最近错误
        </button>
      </div>
    </section>
  );
}

function readBlockAction(props: WorkbenchOverviewProps) {
  return props.blockCard?.action === "reconnect"
    ? props.onReconnect
    : props.onOpenSettingsPage;
}

function readBlockButtonLabel(action?: "reconnect" | "settings" | "workspace" | "model") {
  if (action === "reconnect") return "重新连接";
  if (action === "workspace") return "前往设置检查工作区";
  if (action === "model") return "前往设置切换模型";
  return "前往设置检查";
}

function BlockDiagnosticsSection(props: { props: WorkbenchOverviewProps }) {
  return (
    <section className="home-card-section">
      <SectionHeader kicker="更多信息" kind="head" title="更多信息" />
      <div className="detail-list">
        <Row label="连接状态" value={props.props.systemCard.connection} />
        <Row label="Runtime" value={findEnvValue(props.props.envItems, "Runtime")} />
        <Row label="模型" value={findEnvValue(props.props.envItems, "模型")} />
        <Row label="工作区" value={findEnvValue(props.props.envItems, "工作区")} />
      </div>
    </section>
  );
}

function findEnvValue(items: WorkbenchOverviewProps["envItems"], label: string) {
  return items.find((item) => item.label === label)?.value || "未加载";
}

function Row(props: { label: string; value: string }) {
  return (
    <div className="sidebar-row">
      <strong>{props.label}</strong>
      <span title={props.value}>{props.value}</span>
    </div>
  );
}
