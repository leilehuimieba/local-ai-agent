package service

import "testing"

func TestRunContextHintsAddsDefaultContextBudget(t *testing.T) {
	hints := RunContextHints(map[string]string{}, "D:/repo", true)
	if hints["context_budget_tokens"] != "512000" {
		t.Fatalf("context_budget_tokens=%s", hints["context_budget_tokens"])
	}
	if hints["codex_context_tokens"] != "512000" {
		t.Fatalf("codex_context_tokens=%s", hints["codex_context_tokens"])
	}
}

func TestRunContextHintsKeepsProvidedBudget(t *testing.T) {
	hints := RunContextHints(
		map[string]string{"context_budget_tokens": "64000"},
		"D:/repo",
		false,
	)
	if hints["context_budget_tokens"] != "64000" {
		t.Fatalf("context_budget_tokens=%s", hints["context_budget_tokens"])
	}
	if hints["codex_context_tokens"] != "64000" {
		t.Fatalf("codex_context_tokens=%s", hints["codex_context_tokens"])
	}
}

func TestPickRunIdentityUsesGivenValue(t *testing.T) {
	value := PickRunIdentity("run-fixed", "run")
	if value != "run-fixed" {
		t.Fatalf("value=%s want run-fixed", value)
	}
}

func TestPickRunIdentityFallsBackToGenerated(t *testing.T) {
	value := PickRunIdentity("", "trace")
	if len(value) < 6 {
		t.Fatalf("expect generated id")
	}
	if value[:6] != "trace-" {
		t.Fatalf("value=%s want trace-*", value)
	}
}
