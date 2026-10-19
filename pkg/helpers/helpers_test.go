package helpers

import (
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestCleanupStaleTempFiles(t *testing.T) {
	dir := t.TempDir()
	cameraID := "4f03400f-6be0-4d3e-b205-da65478dbd46"

	stale := filepath.Join(dir, "Segment_2026-10-19T01:47:00_"+cameraID+".mp4"+TempFileSuffix)
	staleLowRes := filepath.Join(dir, "Segment_2026-10-19T01:48:00_"+cameraID+"_low_res.mp4"+TempFileSuffix)
	live := filepath.Join(dir, "Segment_2026-10-19T01:49:00_"+cameraID+".mp4"+TempFileSuffix)
	otherCamera := filepath.Join(dir, "Segment_2026-10-19T01:47:00_326da5dd-1111-4222-8333-444444444444.mp4"+TempFileSuffix)
	finished := filepath.Join(dir, "Segment_2026-10-19T01:47:00_"+cameraID+".mp4")

	for _, name := range []string{stale, staleLowRes, live, otherCamera, finished} {
		if err := os.WriteFile(name, []byte("x"), 0o644); err != nil {
			t.Fatalf("WriteFile(%v): %v", name, err)
		}
	}

	// The two stale ones are what a producer that died an hour ago left behind; the live one is
	// this producer's current segment.
	older := time.Now().Add(-time.Hour)
	for _, name := range []string{stale, staleLowRes, otherCamera, finished} {
		if err := os.Chtimes(name, older, older); err != nil {
			t.Fatalf("Chtimes(%v): %v", name, err)
		}
	}

	cleaned, err := CleanupStaleTempFiles(dir, cameraID, time.Minute)
	if err != nil {
		t.Fatalf("CleanupStaleTempFiles: %v", err)
	}
	if cleaned != 2 {
		t.Fatalf("cleaned = %v, want 2", cleaned)
	}

	for _, name := range []string{stale, staleLowRes} {
		if _, err := os.Stat(name); !os.IsNotExist(err) {
			t.Fatalf("stale temp file %v survived: %v", name, err)
		}
	}

	for _, name := range []string{live, otherCamera, finished} {
		if _, err := os.Stat(name); err != nil {
			t.Fatalf("%v should have survived: %v", name, err)
		}
	}
}

func TestCleanupStaleTempFilesMissingDir(t *testing.T) {
	cleaned, err := CleanupStaleTempFiles(filepath.Join("this", "does", "not", "exist"), "4f03400f-6be0-4d3e-b205-da65478dbd46", time.Minute)
	if cleaned != 0 {
		t.Fatalf("cleaned = %v, want 0", cleaned)
	}
	if err != nil {
		t.Fatalf("CleanupStaleTempFiles on a missing dir = %v, want no error", err)
	}
}
