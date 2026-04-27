import { FormEvent, KeyboardEvent, useEffect, useState } from "react";

import { fetchKnowledgeItems } from "../knowledge-base/api";
import type { ChatPanelProps } from "./ChatPanel";

export function TaskComposer(props: { props: ChatPanelProps }) {
  const isDisabled = readComposerDisabled(props.props);
  const hint = readComposerHint(props.props);
  return (
    <form className="composer composer-simple" onSubmit={props.props.onSubmit}>
      <div className="simple-composer-shell">
        <input
          id="task-composer-input"
          name="task_composer_input"
          className="simple-composer-input"
          aria-label="任务输入"
          autoComplete="off"
          type="text"
          value={props.props.composeValue}
          disabled={!props.props.settings || props.props.isRunning}
          placeholder="输入任务，按回车发送"
          onChange={(event) => props.props.onComposeValueChange(event.target.value)}
          onKeyDown={(event) => handleComposerKeyDown(event, props.props.composeValue, props.props.onComposeValueChange)}
        />
        <ComposerActions props={props.props} isDisabled={isDisabled} />
      </div>
      <p className="simple-composer-hint">{hint}</p>
    </form>
  );
}

function ComposerActions(props: { props: ChatPanelProps; isDisabled: boolean }) {
  const showClear = shouldShowClearDraft(props.props);
  return (
    <div className="simple-composer-actions">
      {showClear ? <button type="button" className="composer-clear" onClick={() => props.props.onComposeValueChange("")}>清空</button> : null}
      <button className="primary-action" type="submit" disabled={props.isDisabled}>{readSubmitLabel(props.props.isRunning)}</button>
    </div>
  );
}

function KnowledgeBaseSelector() {
  const [items, setItems] = useState<Array<{ id: string; title: string }>>([]);
  useEffect(() => {
    fetchKnowledgeItems()
      .then((data) => {
        setItems(data.items.map((item) => ({ id: item.id, title: item.title })));
      })
      .catch(() => setItems([]));
  }, []);
  return (
    <label className="kb-selector">
      <span>引用知识库</span>
      <select name="knowledge_base_id" defaultValue="">
        <option value="">自动</option>
        <option value="_none_">不引用</option>
        {items.map((item) => (
          <option key={item.id} value={item.id}>{item.title}</option>
        ))}
      </select>
    </label>
  );
}

function readComposerDisabled(props: ChatPanelProps) {
  return !props.settings || props.isRunning || !props.composeValue.trim();
}

function shouldShowClearDraft(props: ChatPanelProps) {
  return Boolean(props.composeValue.trim() && !props.isRunning);
}

function handleComposerKeyDown(
  event: KeyboardEvent<HTMLInputElement>,
  value: string,
  onChange: (value: string) => void,
) {
  if (event.key !== "Escape" || !value) return;
  event.preventDefault();
  onChange("");
}

function readComposerHint(props: ChatPanelProps) {
  if (!props.settings) return "请先在设置页完成运行环境配置。";
  if (props.isRunning) return "任务执行中，输入区暂时禁用。";
  if (!props.composeValue.trim()) return "输入任务后按回车发送。";
  return "按回车发送，按 Esc 清空草稿。";
}

function readSubmitLabel(isRunning: boolean) {
  return isRunning ? "发送中" : "发送";
}
