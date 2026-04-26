import { type Dispatch, type SetStateAction, useState } from "react";

import { SectionHeader } from "../ui/primitives";
import { ReleaseRunResponse, ReleaseStepId, runReleaseStep } from "./api";

type ReleaseStep = {
  id: ReleaseStepId;
  title: string;
  goal: string;
  command: string;
  artifact: string;
  pass: string;
  failure: string;
};

const RELEASE_STEPS: ReleaseStep[] = [
  {
    id: "prelaunch",
    title: "上线前检查",
    goal: "确认 Rust、Go、前端构建和核心 E2E 都能通过。",
    command: "scripts/run-full-regression.ps1 -OutFile tmp/prelaunch-regression.json",
    artifact: "tmp/prelaunch-regression.json",
    pass: "6 项检查全绿，E2E 显示 strict_runtime_terminal passed。",
    failure: "先看失败项，再按原因 / 影响 / 不影响 / 建议修复格式收口。",
  },
  {
    id: "package",
    title: "安装包构建",
    goal: "把 gateway、runtime、frontend dist、配置和启动说明组装到安装目录。",
    command: "scripts/install-local-agent.ps1 -Mode install -Version <版本号>",
    artifact: "安装目录 current/ 与 current-version.txt",
    pass: "launcher、server、runtime、frontend/dist、config、start-agent.ps1 均存在。",
    failure: "优先检查构建日志、目标目录权限和 frontend dist 是否生成。",
  },
  {
    id: "doctor",
    title: "Doctor 诊断",
    goal: "用用户视角确认依赖、端口、配置、前端产物、服务健康与日志写入。",
    command: "scripts/doctor.ps1 -OutFile tmp/doctor-release.json",
    artifact: "tmp/doctor-release.json",
    pass: "核心依赖可用，配置存在，端口有效，日志可写；服务健康按发布口径通过。",
    failure: "把失败检查翻译成用户可执行的修复建议，不直接暴露原始堆栈。",
  },
  {
    id: "rc",
    title: "发布候选验证",
    goal: "聚合安装、诊断和核心入口样本，判断是否能作为发布候选。",
    command: "scripts/run-stage-f-rc-acceptance.ps1",
    artifact: "tmp/stage-f-rc/latest.json",
    pass: "RC 验收通过，阻塞项为 0；warning 有明确接受或修复说明。",
    failure: "阻塞项必须修复后重跑；warning 需要登记责任人与后续复检方式。",
  },
];

export function ReleaseWizardPanel() {
  const runner = useReleaseRunner();
  return (
    <article className="panel release-wizard-page">
      <ReleaseHero />
      <section className="release-step-list" aria-label="上线向导步骤">
        {RELEASE_STEPS.map((step, index) => <ReleaseStepCard key={step.id} index={index + 1} step={step} runner={runner} />)}
      </section>
      <ReleaseDecisionCard />
    </article>
  );
}

function ReleaseHero() {
  return (
    <section className="release-hero">
      <SectionHeader
        kind="page"
        kicker="产品化封装"
        level="h2"
        title="上线向导"
        description="不用记脚本名：按这条路径完成上线前检查、安装包构建、Doctor 诊断和发布候选验证。"
      />
    </section>
  );
}

function ReleaseStepCard(props: { index: number; step: ReleaseStep; runner: ReleaseRunner }) {
  return (
    <article className="release-step-card">
      <div className="release-step-index">{props.index}</div>
      <div className="release-step-body">
        <h3>{props.step.title}</h3>
        <p>{props.step.goal}</p>
        <ReleaseMeta label="执行命令" value={props.step.command} code />
        <ReleaseMeta label="关键产物" value={props.step.artifact} />
        <ReleaseMeta label="通过标准" value={props.step.pass} />
        <ReleaseMeta label="失败处理" value={props.step.failure} />
        <ReleaseStepAction step={props.step} runner={props.runner} />
      </div>
    </article>
  );
}

function ReleaseStepAction(props: { step: ReleaseStep; runner: ReleaseRunner }) {
  const state = props.runner.states[props.step.id];
  return (
    <div className="release-step-action">
      <button type="button" disabled={state.status === "running"} onClick={() => props.runner.run(props.step.id)}>
        {state.status === "running" ? "执行中" : `运行${props.step.title}`}
      </button>
      <ReleaseStepResult state={state} />
    </div>
  );
}

function ReleaseStepResult(props: { state: ReleaseRunState }) {
  if (props.state.status === "idle") return <span>尚未执行</span>;
  if (props.state.status === "running") return <span>正在运行脚本，请稍候。</span>;
  if (props.state.status === "error") return <span className="release-result-failed">{props.state.error}</span>;
  return <ReleaseCommandResult result={props.state.result} />;
}

function ReleaseCommandResult(props: { result?: ReleaseRunResponse }) {
  if (!props.result) return null;
  return (
    <span className={props.result.status === "passed" ? "release-result-passed" : "release-result-failed"}>
      {props.result.status === "passed" ? "通过" : "失败"} · {props.result.duration_ms}ms · {props.result.artifact}
    </span>
  );
}

function ReleaseMeta(props: { label: string; value: string; code?: boolean }) {
  return (
    <div className="release-meta-row">
      <span>{props.label}</span>
      {props.code ? <code>{props.value}</code> : <p>{props.value}</p>}
    </div>
  );
}

function ReleaseDecisionCard() {
  return (
    <section className="release-decision-card">
      <strong>发布建议</strong>
      <p>四步全部通过后，再汇总证据、已知风险和回退路径。未通过时不要进入发布；先修复阻塞项并重跑对应步骤。</p>
    </section>
  );
}

type ReleaseRunState = {
  status: "idle" | "running" | "done" | "error";
  result?: ReleaseRunResponse;
  error?: string;
};

type ReleaseRunner = {
  states: Record<ReleaseStepId, ReleaseRunState>;
  run: (step: ReleaseStepId) => void;
};

type ReleaseStateSetter = Dispatch<SetStateAction<Record<ReleaseStepId, ReleaseRunState>>>;

function useReleaseRunner(): ReleaseRunner {
  const [states, setStates] = useState<Record<ReleaseStepId, ReleaseRunState>>(initialReleaseStates);
  return { states, run: (step) => void runStep(step, setStates) };
}

async function runStep(step: ReleaseStepId, setStates: ReleaseStateSetter) {
  updateReleaseState(setStates, step, { status: "running" });
  try {
    const result = await runReleaseStep(step);
    updateReleaseState(setStates, step, { status: "done", result });
  } catch (error) {
    updateReleaseState(setStates, step, { status: "error", error: readError(error) });
  }
}

function updateReleaseState(setStates: ReleaseStateSetter, step: ReleaseStepId, state: ReleaseRunState) {
  setStates((current) => ({ ...current, [step]: state }));
}

function readError(error: unknown) {
  return error instanceof Error ? error.message : "未知错误";
}

const initialReleaseStates: Record<ReleaseStepId, ReleaseRunState> = {
  prelaunch: { status: "idle" },
  package: { status: "idle" },
  doctor: { status: "idle" },
  rc: { status: "idle" },
};