import type { Dispatch, SetStateAction } from "react";
import type { DiagnosticsCheckResponse, ExternalConnectionActionResponse, SettingsResponse } from "../shared/contracts";

export type SettingsActionKind =
  | "model"
  | "mode"
  | "workspace"
  | "directoryPrompt"
  | "riskLevel"
  | "revokeApproval"
  | "embedding"
  | "externalConnection"
  | "diagnosticsCheck";

export type SettingsActionFeedback = {
  action: SettingsActionKind;
  title: string;
  detail: string;
};

export type ProviderActionState = {
  pending?: boolean;
  success?: string;
  error?: string;
};

type PendingActions = Partial<Record<SettingsActionKind, true>>;
type ProviderActions = Record<string, ProviderActionState>;

export type ActionRunner<T> = {
  action: SettingsActionFeedback;
  execute: () => Promise<T>;
  onSuccess?: (result: T) => Promise<void> | void;
  setActionError: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
  setBootstrapError: Dispatch<SetStateAction<string | null>>;
  setLastSuccess: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
  setPendingAction: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
  setPendingActions: Dispatch<SetStateAction<PendingActions>>;
  successDetail: string;
};

export function buildActionRunner<T>(
  options: Pick<ActionRunner<T>, "action" | "execute" | "onSuccess" | "successDetail">,
  feedback: {
    setActionError: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
    setLastSuccess: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
    setPendingAction: Dispatch<SetStateAction<SettingsActionFeedback | null>>;
    setPendingActions: Dispatch<SetStateAction<PendingActions>>;
  },
  state: {
    setBootstrapError: Dispatch<SetStateAction<string | null>>;
  },
) {
  return {
    ...options,
    setActionError: feedback.setActionError,
    setBootstrapError: state.setBootstrapError,
    setLastSuccess: feedback.setLastSuccess,
    setPendingAction: feedback.setPendingAction,
    setPendingActions: feedback.setPendingActions,
  };
}

export async function runSettingsAction<T>(args: ActionRunner<T>) {
  setActionStarted(args);
  try {
    const result = await args.execute();
    await args.onSuccess?.(result);
    args.setLastSuccess(createFeedback(args.action.action, args.action.title, args.successDetail));
    args.setBootstrapError(null);
    return result;
  } catch (error) {
    args.setActionError(createFeedback(args.action.action, args.action.title, readErrorMessage(error, `${args.action.title}失败`)));
    throw error;
  } finally {
    clearActionPending(args);
  }
}

function setActionStarted<T>(args: ActionRunner<T>) {
  args.setPendingAction(args.action);
  args.setPendingActions((current) => ({ ...current, [args.action.action]: true }));
  args.setActionError(null);
  args.setLastSuccess(null);
}

function clearActionPending<T>(args: ActionRunner<T>) {
  args.setPendingAction((current) => current?.action === args.action.action ? null : current);
  args.setPendingActions((current) => clearPendingState(current, args.action.action));
}

function clearPendingState(current: PendingActions, action: SettingsActionKind) {
  const next = { ...current };
  delete next[action];
  return next;
}

export function isActionPending(current: PendingActions, action: SettingsActionKind) {
  return Boolean(current[action]);
}

export function createFeedback(action: SettingsActionKind, title: string, detail: string) {
  return { action, detail, title };
}

function readErrorMessage(error: unknown, fallback: string) {
  return error instanceof Error ? error.message : fallback;
}

export function applyNextSettings(
  setSettings: Dispatch<SetStateAction<SettingsResponse | null>>,
) {
  return (nextSettings: SettingsResponse) => {
    setSettings(nextSettings);
    return nextSettings;
  };
}

export function applyDiagnosticsResult(
  setSettings: Dispatch<SetStateAction<SettingsResponse | null>>,
) {
  return (result: DiagnosticsCheckResponse) => {
    setSettings((current) => current ? mergeDiagnostics(current, result) : current);
    return null;
  };
}

export function mergeExternalConnections(settings: SettingsResponse, result: ExternalConnectionActionResponse) {
  if (result.external_connections?.length) {
    return { ...settings, external_connections: result.external_connections };
  }
  if (!result.updated_slot) return settings;
  return {
    ...settings,
    external_connections: settings.external_connections.map((slot) =>
      slot.slot_id === result.updated_slot?.slot_id ? result.updated_slot : slot),
  };
}

function mergeDiagnostics(settings: SettingsResponse, result: DiagnosticsCheckResponse) {
  return {
    ...settings,
    diagnostics: {
      ...result.diagnostics,
      checked_at: result.checked_at,
      warnings: result.warnings,
      errors: result.errors,
    },
  };
}

export async function runProviderAction<T>(
  state: {
    setProviderActions: Dispatch<SetStateAction<ProviderActions>>;
  },
  providerId: string,
  execute: () => Promise<T>,
) {
  setProviderActionPending(state.setProviderActions, providerId);
  try {
    const result = await execute();
    setProviderActionSuccess(state.setProviderActions, providerId, readProviderSuccessMessage(result));
    return result;
  } catch (error) {
    setProviderActionError(state.setProviderActions, providerId, readErrorMessage(error, "操作失败"));
    throw error;
  }
}

function setProviderActionPending(
  setProviderActions: Dispatch<SetStateAction<ProviderActions>>,
  providerId: string,
) {
  setProviderActions((current) => ({ ...current, [providerId]: { pending: true } }));
}

function setProviderActionSuccess(
  setProviderActions: Dispatch<SetStateAction<ProviderActions>>,
  providerId: string,
  message: string,
) {
  setProviderActions((current) => ({ ...current, [providerId]: { success: message } }));
}

function setProviderActionError(
  setProviderActions: Dispatch<SetStateAction<ProviderActions>>,
  providerId: string,
  message: string,
) {
  setProviderActions((current) => ({ ...current, [providerId]: { error: message } }));
}

function readProviderSuccessMessage(result: unknown) {
  const data = result as { message?: string };
  return data.message || "操作成功";
}
