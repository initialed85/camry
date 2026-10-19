package segment_producer

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/google/uuid"
	"github.com/initialed85/camry/pkg/helpers"
)

func TestFinalPathFor(t *testing.T) {
	tmp := "/srv/media/Segment_2026-10-19T01:47:00_4f03400f-6be0-4d3e-b205-da65478dbd46_low_res.mp4.tmp"
	final := "/srv/media/Segment_2026-10-19T01:47:00_4f03400f-6be0-4d3e-b205-da65478dbd46_low_res.mp4"

	if got := finalPathFor(tmp); got != final {
		t.Fatalf("finalPathFor = %v, want %v", got, final)
	}
	if got := finalPathFor(final); got != final {
		t.Fatalf("finalPathFor is not stable on a final name: got %v, want %v", got, final)
	}
}

func TestFinalizeSegment(t *testing.T) {
	dir := t.TempDir()
	tmp := filepath.Join(dir, "Segment_x.mp4"+helpers.TempFileSuffix)
	final := filepath.Join(dir, "Segment_x.mp4")

	if err := os.WriteFile(tmp, []byte("segment"), 0o644); err != nil {
		t.Fatalf("WriteFile: %v", err)
	}

	got, err := finalizeSegment(tmp)
	if err != nil {
		t.Fatalf("finalizeSegment: %v", err)
	}
	if got != final {
		t.Fatalf("finalizeSegment = %v, want %v", got, final)
	}
	if _, err := os.Stat(tmp); !os.IsNotExist(err) {
		t.Fatalf("temp file survived the rename: %v", err)
	}
	if _, err := os.Stat(final); err != nil {
		t.Fatalf("final file missing after rename: %v", err)
	}

	// A final name that already exists is never overwritten.
	if _, err := finalizeSegment(tmp); err == nil {
		t.Fatal("expected an error finalizing a segment whose final name already exists")
	}

	if _, err := finalizeSegment(filepath.Join(dir, "Segment_missing.mp4"+helpers.TempFileSuffix)); err == nil {
		t.Fatal("expected an error finalizing a segment that does not exist")
	}
}

func TestRecoveredPath(t *testing.T) {
	dir := t.TempDir()
	final := filepath.Join(dir, "Segment_live.mp4")
	if err := os.WriteFile(final, []byte("live"), 0o644); err != nil {
		t.Fatalf("WriteFile: %v", err)
	}
	if got := recoveredPath(final); got != final {
		t.Fatalf("recoveredPath = %v, want %v", got, final)
	}

	abandoned := filepath.Join(dir, "Segment_abandoned.mp4")
	if err := os.WriteFile(abandoned+helpers.TempFileSuffix, []byte("abandoned"), 0o644); err != nil {
		t.Fatalf("WriteFile: %v", err)
	}
	if got := recoveredPath(abandoned); got != abandoned {
		t.Fatalf("recoveredPath = %v, want %v", got, abandoned)
	}
	if _, err := os.Stat(abandoned); err != nil {
		t.Fatalf("leftover temp segment was not recovered: %v", err)
	}

	missing := filepath.Join(dir, "Segment_missing.mp4")
	if got := recoveredPath(missing); got != missing {
		t.Fatalf("recoveredPath = %v, want %v", got, missing)
	}
}

func TestGetCommandLineWritesTempFilesOnly(t *testing.T) {
	cameraID := uuid.MustParse("4f03400f-6be0-4d3e-b205-da65478dbd46")

	for _, isLowRes := range []bool{false, true} {
		arguments := getCommandLine(true, false, "rtsp://cam/stream", 60, "/srv/media", cameraID, isLowRes)
		pattern := arguments[len(arguments)-1]

		if !strings.HasSuffix(pattern, ".mp4"+helpers.TempFileSuffix) {
			t.Fatalf("isLowRes=%v: output pattern %v does not end in a temp suffix", isLowRes, pattern)
		}
		if strings.Contains(strings.TrimSuffix(pattern, helpers.TempFileSuffix), helpers.TempFileSuffix) {
			t.Fatalf("isLowRes=%v: output pattern %v has a doubled temp suffix", isLowRes, pattern)
		}
		if !strings.Contains(pattern, cameraID.String()) {
			t.Fatalf("isLowRes=%v: output pattern %v is missing the camera id", isLowRes, pattern)
		}

		suffix := ""
		if isLowRes {
			suffix = "_low_res"
		}
		want := "/srv/media/Segment_%Y-%m-%dT%H:%M:%S_" + cameraID.String() + suffix + ".mp4" + helpers.TempFileSuffix
		if pattern != want {
			t.Fatalf("output pattern = %v, want %v", pattern, want)
		}

		joined := strings.Join(arguments, " ")
		for _, want := range []string{"-y", "-segment_format_options movflags=+faststart", "-c copy", "-f segment"} {
			if !strings.Contains(joined, want) {
				t.Fatalf("isLowRes=%v: arguments are missing %q: %v", isLowRes, want, joined)
			}
		}
		if strings.Contains(joined, "-movflags") {
			t.Fatalf("isLowRes=%v: -movflags is accepted and ignored by -f segment; faststart has to go through -segment_format_options", isLowRes)
		}
	}
}

func TestClaimLockKey(t *testing.T) {
	a := uuid.MustParse("4f03400f-6be0-4d3e-b205-da65478dbd46")
	b := uuid.MustParse("326da5dd-1111-4222-8333-444444444444")

	if claimLockKey(a) != claimLockKey(a) {
		t.Fatal("claimLockKey is not stable")
	}
	if claimLockKey(a) == claimLockKey(b) {
		t.Fatalf("claimLockKey collides between %v and %v", a, b)
	}
}
