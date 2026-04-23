import { FormEvent } from "react";

type HomeStateKind = "first_use" | "resume" | "blocked";

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
    contextItems: Array<{ label: string; value: string }>;
    evidenceItems: Array<{ label: string; value: string }>;
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
    <div className="home-workspace">
      <HomeHero {...props} />
      {props.kind !== "blocked" ? <HomeComposer {...props} /> : null}
      <HomeQuickActions {...props} />
    </div>
  );
}

function HomeHero(props: WorkbenchOverviewProps) {
  const { title, subtitle } = readHeroCopy(props);
  return (
    <header className="home-hero">
      <h1>{title}</h1>
      <p>{subtitle}</p>
    </header>
  );
}

function readHeroCopy(props: WorkbenchOverviewProps) {
  if (props.kind === "blocked") {
    return {
      title: props.blockCard?.title || "当前无法继续",
      subtitle: props.blockCard?.body || "当前存在阻塞，请先处理后继续。",
    };
  }
  if (props.kind === "resume") {
    return {
      title: props.resumeCard.recentTask,
      subtitle: props.hasConfirmation
        ? "当前任务停在确认节点，处理后才能继续推进。"
        : "进入任务页继续主线程，或去记录页查看历史复盘。",
    };
  }
  return {
    title: "今天想让本地智能体帮你完成什么？",
    subtitle: "它可以执行命令、处理文件、调用本地程序，并把过程和结果展示清楚。",
  };
}

function HomeComposer(props: WorkbenchOverviewProps) {
  const disabled = !props.canSubmit || props.isSubmitting || !props.composeValue.trim();
  return (
    <form className="home-composer" onSubmit={props.onSubmit}>
      <textarea
        id="home-task-input"
        name="home_task_input"
        rows={4}
        value={props.composeValue}
        placeholder="例如：帮我检查这个项目为什么构建失败，并给出最小修改方案"
        onChange={(event) => props.onComposeValueChange(event.target.value)}
      />
      <div className="home-composer-footer">
        <p>提交后会进入任务页，持续展示执行过程和结果。</p>
        <button type="submit" disabled={disabled}>
          {props.isSubmitting ? "开始任务中" : "开始任务"}
        </button>
      </div>
    </form>
  );
}

function HomeQuickActions(props: WorkbenchOverviewProps) {
  if (props.kind === "blocked") return <BlockedActions {...props} />;
  if (props.kind === "resume") return <ResumeActions {...props} />;
  return <FirstUseActions {...props} />;
}

function FirstUseActions(props: WorkbenchOverviewProps) {
  return (
    <section className="home-quick-actions">
      {props.examples.map((item) => (
        <button
          key={item.id}
          type="button"
          className="home-example-chip"
          onClick={() => props.onPrefillExample(item.prompt)}
        >
          {item.label}
        </button>
      ))}
    </section>
  );
}

function ResumeActions(props: WorkbenchOverviewProps) {
  const primaryLabel = props.hasConfirmation ? "处理待确认" : "继续任务";
  const primaryHandler = props.hasConfirmation
    ? props.onOpenTaskPageForConfirmation
    : props.onOpenTaskPage;
  return (
    <section className="home-quick-actions">
      <button type="button" className="home-action-card" onClick={primaryHandler}>
        <strong>{primaryLabel}</strong>
        <span>回到任务主线程继续推进</span>
      </button>
      <button type="button" className="home-action-card" onClick={props.onOpenLogsPage}>
        <strong>查看记录</strong>
        <span>查看历史复盘和事件轨迹</span>
      </button>
      <button type="button" className="home-action-card" onClick={() => props.onPrefillExample("")}>
        <strong>新建任务</strong>
        <span>开始一个全新的任务</span>
      </button>
    </section>
  );
}

function BlockedActions(props: WorkbenchOverviewProps) {
  const primaryHandler = props.blockCard?.action === "reconnect"
    ? props.onReconnect
    : props.onOpenSettingsPage;
  return (
    <section className="home-quick-actions">
      <button type="button" className="home-action-card" onClick={primaryHandler}>
        <strong>{readBlockButtonLabel(props.blockCard?.action)}</strong>
        <span>{props.blockCard?.detail || "请先处理阻塞问题"}</span>
      </button>
      <button type="button" className="home-action-card" onClick={props.onOpenLogsPage}>
        <strong>查看记录</strong>
        <span>查看最近错误和诊断信息</span>
      </button>
    </section>
  );
}

function readBlockButtonLabel(action?: "reconnect" | "settings" | "workspace" | "model") {
  if (action === "reconnect") return "重新连接";
  if (action === "workspace") return "检查工作区";
  if (action === "model") return "切换模型";
  return "前往设置";
}
