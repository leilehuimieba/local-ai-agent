import { useEffect, useMemo, useState } from "react";

import { fetchArtifactContent } from "../artifactApi";
import { LogEntry } from "../../shared/contracts";

export function ArtifactOutputSection(props: { focusLog: LogEntry | null }) {
  const refPath = useMemo(() => readRawOutputRef(props.focusLog), [props.focusLog]);
  const [expanded, setExpanded] = useState(false);
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    setExpanded(false);
    setContent("");
    setLoading(false);
    setError("");
  }, [refPath, props.focusLog?.log_id]);

  useEffect(() => {
    if (!expanded || !refPath) return;
    const controller = new AbortController();
    setLoading(true);
    setError("");
    void fetchArtifactContent(refPath, controller.signal)
      .then((value) => setContent(value))
      .catch(() => {
        if (!controller.signal.aborted) setError("原文读取失败，请检查产物路径是否可访问。");
      })
      .finally(() => {
        if (!controller.signal.aborted) setLoading(false);
      });
    return () => controller.abort();
  }, [expanded, refPath]);

  if (!refPath) return null;
  return (
    <section className="detail-card muted-card">
      <strong>命令原文输出</strong>
      <p className="timeline-detail">{`引用：${refPath}`}</p>
      <button type="button" className="secondary-button" onClick={() => setExpanded(!expanded)}>
        {expanded ? "收起原文" : "展开原文"}
      </button>
      {expanded ? <ArtifactOutputBody loading={loading} error={error} content={content} /> : null}
    </section>
  );
}

function ArtifactOutputBody(props: { loading: boolean; error: string; content: string }) {
  if (props.loading) return <p className="timeline-detail">正在读取原文输出…</p>;
  if (props.error) return <p className="timeline-detail">{props.error}</p>;
  return <pre className="shell-output-block">{props.content || "(原文为空)"}</pre>;
}

function readRawOutputRef(log: LogEntry | null) {
  if (!log) return "";
  return log.raw_output_ref || log.artifact_path || "";
}

