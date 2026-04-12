package api

import "testing"

func TestPickRunIdentityUsesGivenValue(t *testing.T) {
	value := pickRunIdentity("run-fixed", "run")
	if value != "run-fixed" {
		t.Fatalf("value=%s want run-fixed", value)
	}
}

func TestPickRunIdentityFallsBackToGenerated(t *testing.T) {
	value := pickRunIdentity("", "trace")
	if len(value) < 6 {
		t.Fatalf("expect generated id")
	}
	if value[:6] != "trace-" {
		t.Fatalf("value=%s want trace-*", value)
	}
}
