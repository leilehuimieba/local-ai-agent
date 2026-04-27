package service

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestReleaseSpecAllowsOnlyKnownSteps(t *testing.T) {
	_, err := releaseSpec("D:/repo", "unknown")
	require.Error(t, err)
}

func TestReleaseSpecUsesWhitelistedScript(t *testing.T) {
	spec, err := releaseSpec("D:/repo", "doctor")
	require.NoError(t, err)
	require.Equal(t, "doctor.ps1", spec.Script)
	require.Contains(t, spec.Args, "-OutFile")
}
