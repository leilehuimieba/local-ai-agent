import { useState } from "react";

import { readUnifiedStatusFromLabel, readUnifiedStatusMeta } from "../runtime/state";
import { ProviderSettingsItem, ProviderSettingsResponse } from "../shared/contracts";
import { EmptyStateBlock, MetaGrid, SectionHeader, StatusPill } from "../ui/primitives";
import { ProviderActionState } from "./useSettings";

type ProviderCredentialsSectionProps = {
  providerSettings: ProviderSettingsResponse | null;
  providerBootstrapError: string | null;
  providerActions: Record<string, ProviderActionState>;
  onRefreshProviderSettings: () => Promise<unknown>;
  onTestProvider: (providerId: string, apiKey: string, baseURL?: string) => Promise<unknown>;
  onSaveProvider: (providerId: string, apiKey: string) => Promise<unknown>;
  onApplyProvider: (providerId: string) => Promise<unknown>;
  onRemoveProvider: (providerId: string) => Promise<unknown>;
};

type ProviderFeedback = {
  tone: "running" | "failed" | "completed";
  detail: string;
};

export function ProviderCredentialsSection(props: ProviderCredentialsSectionProps) {
  return (
    <section className="settings-module control-module">
      <SectionHeader
        kind="head"
        kicker="控制"
        title="模型服务与密钥"
        description="先测试连接，再保存凭据，最后应用到运行时。"
        action={<ProviderHeaderAction props={props} />}
      />
      <ProviderSectionBody props={props} />
    </section>
  );
}

function ProviderHeaderAction(props: { props: ProviderCredentialsSectionProps }) {
  const badge = readProviderModuleBadge(props.props.providerSettings, props.props.providerBootstrapError);
  return (
    <div className="timeline-chip-row">
      <StatusPill className={badge.className} label={badge.label} />
      <button type="button" className="secondary-button" onClick={() => void props.props.onRefreshProviderSettings()}>
        刷新状态
      </button>
    </div>
  );
}

function ProviderSectionBody(props: { props: ProviderCredentialsSectionProps }) {
  const providers = props.props.providerSettings?.providers ?? [];
  if (!props.props.providerSettings && props.props.providerBootstrapError) return <ProviderLoadError message={props.props.providerBootstrapError} />;
  if (!props.props.providerSettings) return <EmptyStateBlock compact title="正在读取模型服务设置" text="读取完成后，这里会显示可配置的 provider 列表。" />;
  if (providers.length === 0) return <EmptyStateBlock compact title="暂无可配置 provider" text="后端当前没有返回可管理的模型服务条目。" />;
  return (
    <div className="settings-subsection">
      {props.props.providerBootstrapError ? <ProviderInlineFeedback feedback={{ tone: "failed", detail: props.props.providerBootstrapError }} /> : null}
      <div className="memory-list">
        {providers.map((item) => <ProviderCredentialCard key={item.provider_id} item={item} action={props.props.providerActions[item.provider_id]} activeProviderId={props.props.providerSettings?.active_provider_id} onTestProvider={props.props.onTestProvider} onSaveProvider={props.props.onSaveProvider} onApplyProvider={props.props.onApplyProvider} onRemoveProvider={props.props.onRemoveProvider} />)}
      </div>
    </div>
  );
}

function ProviderLoadError(props: { message: string }) {
  return (
    <div className="settings-subsection">
      <ProviderInlineFeedback feedback={{ tone: "failed", detail: props.message }} />
      <EmptyStateBlock compact title="模型服务设置加载失败" text="可先检查后端接口，再点击“刷新状态”重试。" />
    </div>
  );
}

function ProviderCredentialCard(props: {
  item: ProviderSettingsItem;
  action?: ProviderActionState;
  activeProviderId?: string;
  onTestProvider: ProviderCredentialsSectionProps["onTestProvider"];
  onSaveProvider: ProviderCredentialsSectionProps["onSaveProvider"];
  onApplyProvider: ProviderCredentialsSectionProps["onApplyProvider"];
  onRemoveProvider: ProviderCredentialsSectionProps["onRemoveProvider"];
}) {
  const [apiKey, setApiKey] = useState("");
  const [visible, setVisible] = useState(false);
  const pending = Boolean(props.action?.pending);
  return (
    <article className="memory-item">
      <ProviderCardHead item={props.item} activeProviderId={props.activeProviderId} />
      <MetaGrid items={buildProviderRows(props.item, props.activeProviderId)} />
      <ProviderKeyInput apiKey={apiKey} visible={visible} pending={pending} onChange={setApiKey} onToggle={() => setVisible((current) => !current)} />
      <ProviderActionButtons item={props.item} apiKey={apiKey} pending={pending} onTest={() => void handleProviderTest(props.item, apiKey, props.onTestProvider)} onSave={() => void handleProviderSave(props.item, apiKey, props.onSaveProvider, setApiKey, setVisible)} onApply={() => void props.onApplyProvider(props.item.provider_id)} onRemove={() => void handleProviderRemove(props.item.provider_id, props.onRemoveProvider, setApiKey, setVisible)} />
      <ProviderInlineFeedback feedback={readProviderFeedback(props.item, props.action, props.activeProviderId)} />
    </article>
  );
}

function ProviderCardHead(props: { item: ProviderSettingsItem; activeProviderId?: string }) {
  const pill = readProviderPill(props.item, props.activeProviderId);
  return (
    <div className="memory-item-head">
      <div>
        <strong>{props.item.display_name}</strong>
        <p>{props.item.provider_id}</p>
        <p className="workspace-root">{props.item.base_url || "未提供 base_url"}</p>
      </div>
      <StatusPill className={pill.className} label={pill.label} />
    </div>
  );
}

function ProviderKeyInput(props: {
  apiKey: string;
  visible: boolean;
  pending: boolean;
  onChange: (value: string) => void;
  onToggle: () => void;
}) {
  return (
    <div className="approval-item">
      <label className="control-field">
        <span>API key</span>
        <input type={props.visible ? "text" : "password"} value={props.apiKey} placeholder="输入新的 API key，仅保留在当前输入框" disabled={props.pending} onChange={(event) => props.onChange(event.target.value)} />
      </label>
      <button type="button" className="secondary-button" disabled={props.pending} onClick={props.onToggle}>
        {props.visible ? "隐藏" : "显示"}
      </button>
    </div>
  );
}

function ProviderActionButtons(props: {
  item: ProviderSettingsItem;
  apiKey: string;
  pending: boolean;
  onTest: () => void;
  onSave: () => void;
  onApply: () => void;
  onRemove: () => void;
}) {
  const hasInput = Boolean(props.apiKey.trim());
  const hasCredential = props.item.credential_status.has_credential;
  return (
    <div className="timeline-chip-row">
      <button type="button" className="secondary-button" disabled={props.pending || !props.item.supports_test || !hasInput} onClick={props.onTest}>测试连接</button>
      <button type="button" className="secondary-button" disabled={props.pending || !props.item.editable || !hasInput} onClick={props.onSave}>保存密钥</button>
      <button type="button" className="secondary-button" disabled={props.pending || !hasCredential} onClick={props.onApply}>应用到运行时</button>
      <button type="button" className="secondary-button" disabled={props.pending || !hasCredential} onClick={props.onRemove}>移除密钥</button>
    </div>
  );
}

async function handleProviderTest(
  item: ProviderSettingsItem,
  apiKey: string,
  onTestProvider: ProviderCredentialsSectionProps["onTestProvider"],
) {
  const value = apiKey.trim();
  if (!value) return;
  await onTestProvider(item.provider_id, value, item.base_url);
}

async function handleProviderSave(
  item: ProviderSettingsItem,
  apiKey: string,
  onSaveProvider: ProviderCredentialsSectionProps["onSaveProvider"],
  setApiKey: (value: string) => void,
  setVisible: (value: boolean) => void,
) {
  const value = apiKey.trim();
  if (!value) return;
  await onSaveProvider(item.provider_id, value);
  setApiKey("");
  setVisible(false);
}

async function handleProviderRemove(
  providerId: string,
  onRemoveProvider: ProviderCredentialsSectionProps["onRemoveProvider"],
  setApiKey: (value: string) => void,
  setVisible: (value: boolean) => void,
) {
  await onRemoveProvider(providerId);
  setApiKey("");
  setVisible(false);
}

function ProviderInlineFeedback(props: { feedback: ProviderFeedback | null }) {
  if (!props.feedback) return null;
  return <p className={`settings-inline-feedback settings-inline-feedback-${props.feedback.tone}`}>{props.feedback.detail}</p>;
}

function buildProviderRows(item: ProviderSettingsItem, activeProviderId?: string) {
  return [
    { label: "Provider ID", value: item.provider_id },
    { label: "密钥状态", value: readCredentialSummary(item) },
    { label: "最后测试结果", value: readLastTestSummary(item) },
    { label: "当前应用状态", value: readApplySummary(item, activeProviderId) },
  ];
}

function readProviderModuleBadge(
  providerSettings: ProviderSettingsResponse | null,
  providerBootstrapError: string | null,
) {
  if (providerBootstrapError) return { label: "失败", className: readProviderStatusClass("失败") };
  if (!providerSettings?.providers.length) return { label: "空闲", className: readProviderStatusClass("空闲") };
  if (providerSettings.active_provider_id) return { label: "已应用", className: readProviderStatusClass("已应用") };
  if (providerSettings.providers.some((item) => item.credential_status.apply_status === "saved_not_applied")) {
    return { label: "待应用", className: readProviderStatusClass("待应用") };
  }
  return { label: "就绪", className: readProviderStatusClass("就绪") };
}

function readProviderPill(item: ProviderSettingsItem, activeProviderId?: string) {
  if (item.credential_status.apply_status === "applied" || item.provider_id === activeProviderId) {
    return { label: "已应用", className: readProviderStatusClass("已应用") };
  }
  if (item.credential_status.apply_status === "saved_not_applied") {
    return { label: "已保存未应用", className: readProviderStatusClass("已保存未应用") };
  }
  return { label: "未配置", className: readProviderStatusClass("未配置") };
}

function readProviderStatusClass(label: string) {
  return readUnifiedStatusMeta(readUnifiedStatusFromLabel(label)).className;
}

function readCredentialSummary(item: ProviderSettingsItem) {
  if (!item.credential_status.has_credential) return "未配置";
  if (item.credential_status.api_key_masked) return `已保存（${item.credential_status.api_key_masked}）`;
  return "已保存";
}

function readLastTestSummary(item: ProviderSettingsItem) {
  const status = item.credential_status.last_test_status || "idle";
  if (status === "idle") return "尚未测试";
  const message = item.credential_status.last_test_message || (status === "success" ? "连接成功" : "测试失败");
  const suffix = readTimeSuffix(item.credential_status.last_test_at);
  return suffix ? `${message}（${suffix}）` : message;
}

function readApplySummary(item: ProviderSettingsItem, activeProviderId?: string) {
  if (item.credential_status.apply_status === "not_configured") return "未配置";
  if (item.credential_status.apply_status === "saved_not_applied") return "已保存，未应用";
  if (item.credential_status.pending_reload) return "已应用，需重启";
  return item.provider_id === activeProviderId ? "已应用到当前运行时" : "已应用";
}

function readProviderFeedback(
  item: ProviderSettingsItem,
  action: ProviderActionState | undefined,
  activeProviderId?: string,
) {
  if (action?.pending) return { tone: "running", detail: "正在处理当前 provider 设置。" } satisfies ProviderFeedback;
  if (action?.error) return { tone: "failed", detail: action.error } satisfies ProviderFeedback;
  if (action?.success) return { tone: "completed", detail: action.success } satisfies ProviderFeedback;
  if (item.credential_status.apply_status === "saved_not_applied") {
    return { tone: "running", detail: "密钥已保存，尚未应用到运行时。点击“应用到运行时”后，新任务才会使用新配置。" } satisfies ProviderFeedback;
  }
  if (item.provider_id === activeProviderId) {
    return { tone: "completed", detail: "当前 provider 已应用到运行时，新发起的任务会优先使用这套配置。" } satisfies ProviderFeedback;
  }
  return null;
}

function readTimeSuffix(value?: string) {
  if (!value) return "";
  return value.replace("T", " ").replace("Z", "").slice(0, 16);
}
