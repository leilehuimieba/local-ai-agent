import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { reducer, toast, useToast } from "../use-toast";

describe("use-toast reducer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  it("ADD_TOAST appends toast and respects limit", () => {
    const state = { toasts: [] };
    const action = { type: "ADD_TOAST" as const, toast: { id: "1", title: "T1", open: true } };
    const result = reducer(state, action);
    expect(result.toasts).toHaveLength(1);
    expect(result.toasts[0].id).toBe("1");
  });

  it("ADD_TOAST truncates to TOAST_LIMIT", () => {
    const state = { toasts: [{ id: "0", title: "T0", open: true }] };
    const action = { type: "ADD_TOAST" as const, toast: { id: "1", title: "T1", open: true } };
    const result = reducer(state, action);
    expect(result.toasts).toHaveLength(1);
    expect(result.toasts[0].id).toBe("1");
  });

  it("UPDATE_TOAST patches existing toast by id", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: true }] };
    const action = { type: "UPDATE_TOAST" as const, toast: { id: "1", title: "T1-updated" } };
    const result = reducer(state, action);
    expect(result.toasts[0].title).toBe("T1-updated");
  });

  it("UPDATE_TOAST ignores unknown id", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: true }] };
    const action = { type: "UPDATE_TOAST" as const, toast: { id: "2", title: "T2" } };
    const result = reducer(state, action);
    expect(result.toasts[0].title).toBe("T1");
  });

  it("DISMISS_TOAST marks toast as closed by id", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: true }] };
    const action = { type: "DISMISS_TOAST" as const, toastId: "1" };
    const result = reducer(state, action);
    expect(result.toasts[0].open).toBe(false);
  });

  it("DISMISS_TOAST marks all toasts as closed when toastId omitted", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: true }, { id: "2", title: "T2", open: true }] };
    const action = { type: "DISMISS_TOAST" as const };
    const result = reducer(state, action);
    expect(result.toasts.every((t) => t.open === false)).toBe(true);
  });

  it("REMOVE_TOAST filters out toast by id", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: false }, { id: "2", title: "T2", open: false }] };
    const action = { type: "REMOVE_TOAST" as const, toastId: "1" };
    const result = reducer(state, action);
    expect(result.toasts).toHaveLength(1);
    expect(result.toasts[0].id).toBe("2");
  });

  it("REMOVE_TOAST clears all toasts when toastId omitted", () => {
    const state = { toasts: [{ id: "1", title: "T1", open: false }] };
    const action = { type: "REMOVE_TOAST" as const };
    const result = reducer(state, action);
    expect(result.toasts).toHaveLength(0);
  });
});

describe("useToast hook", () => {
  it("useToast renders with empty toasts", () => {
    const { result } = renderHook(() => useToast());
    expect(result.current.toasts).toEqual([]);
  });

  it("toast() adds a toast and useToast sees it", async () => {
    const { result } = renderHook(() => useToast());
    act(() => {
      toast({ title: "Hello" });
    });
    expect(result.current.toasts).toHaveLength(1);
    expect(result.current.toasts[0].title).toBe("Hello");
  });

  it("dismiss() closes a toast", async () => {
    const { result } = renderHook(() => useToast());
    let id: string;
    act(() => {
      const t = toast({ title: "Hello" });
      id = t.id;
    });
    act(() => {
      result.current.dismiss(id);
    });
    expect(result.current.toasts[0].open).toBe(false);
  });
});
