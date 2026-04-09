package state

import (
	"fmt"
	"os"
	"path/filepath"
	"time"
)

func withFileLock(lockPath string, fn func() error) error {
	deadline := time.Now().Add(3 * time.Second)
	for {
		file, err := os.OpenFile(lockPath, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o600)
		if err == nil {
			defer releaseFileLock(file, lockPath)
			return fn()
		}
		if !os.IsExist(err) {
			return err
		}
		if staleLock(lockPath) {
			_ = os.Remove(lockPath)
			continue
		}
		if time.Now().After(deadline) {
			return fmt.Errorf("lock timeout: %s", filepath.Base(lockPath))
		}
		time.Sleep(50 * time.Millisecond)
	}
}

func releaseFileLock(file *os.File, lockPath string) {
	_ = file.Close()
	_ = os.Remove(lockPath)
}

func staleLock(lockPath string) bool {
	info, err := os.Stat(lockPath)
	if err != nil {
		return false
	}
	return time.Since(info.ModTime()) > 30*time.Second
}
