package api

import "testing"

func TestEnsureContextBudgetHintsAddsDefaults(t *testing.T) {
	hints := map[string]string{}
	ensureContextBudgetHints(hints)
	if hints["context_budget_tokens"] != "512000" {
		t.Fatalf("context_budget_tokens=%s", hints["context_budget_tokens"])
	}
	if hints["codex_context_tokens"] != "512000" {
		t.Fatalf("codex_context_tokens=%s", hints["codex_context_tokens"])
	}
}

func TestEnsureContextBudgetHintsKeepsExistingValue(t *testing.T) {
	hints := map[string]string{"context_budget_tokens": "64000"}
	ensureContextBudgetHints(hints)
	if hints["context_budget_tokens"] != "64000" {
		t.Fatalf("context_budget_tokens=%s", hints["context_budget_tokens"])
	}
	if hints["codex_context_tokens"] != "64000" {
		t.Fatalf("codex_context_tokens=%s", hints["codex_context_tokens"])
	}
}
