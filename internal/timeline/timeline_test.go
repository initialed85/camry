package timeline

import (
	"net/url"
	"strconv"
	"strings"
	"testing"
	"time"
)

const cameraA = "326da5dd-8a6a-4534-9598-effeb03abe61"
const cameraB = "4f03400f-6be0-4d3e-b205-da65478dbd46"

func params(raw string) url.Values {
	values, err := url.ParseQuery(raw)
	if err != nil {
		panic(err)
	}
	return values
}

func TestParseParamsDefaults(t *testing.T) {
	query, err := ParseParams(params("started_at__gte=2026-10-11T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z"))
	if err != nil {
		t.Fatalf("ParseParams: %v", err)
	}

	if !query.StartedAtGte.Equal(time.Date(2026, 10, 11, 0, 0, 0, 0, time.UTC)) {
		t.Fatalf("StartedAtGte = %v", query.StartedAtGte)
	}
	if query.MinScore != 0 {
		t.Fatalf("MinScore = %v, want 0", query.MinScore)
	}
	if query.Limit != defaultLimit {
		t.Fatalf("Limit = %v, want %v", query.Limit, defaultLimit)
	}
	if query.Offset != 0 {
		t.Fatalf("Offset = %v, want 0", query.Offset)
	}
	if len(query.CameraIDs) != 0 {
		t.Fatalf("CameraIDs = %v, want none (all cameras)", query.CameraIDs)
	}
}

func TestParseParamsCameras(t *testing.T) {
	query, err := ParseParams(params("started_at__gte=2026-10-17T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z&camera_id__eq=" + cameraA + "&camera_id__eq=" + cameraB + "," + cameraA))
	if err != nil {
		t.Fatalf("ParseParams: %v", err)
	}

	if len(query.CameraIDs) != 2 {
		t.Fatalf("CameraIDs = %v, want 2 deduped ids", query.CameraIDs)
	}
	if query.CameraIDs[0] != cameraA || query.CameraIDs[1] != cameraB {
		t.Fatalf("CameraIDs = %v, want [A B] in request order", query.CameraIDs)
	}
}

func TestParseParamsRejectsBadInput(t *testing.T) {
	base := "started_at__gte=2026-10-17T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z"

	cases := []struct {
		name  string
		raw   string
		wants string
	}{
		{"missing lower bound", "started_at__lt=2026-10-18T00:00:00Z", "started_at__gte is required"},
		{"missing upper bound", "started_at__gte=2026-10-17T00:00:00Z", "started_at__lt is required"},
		{"compact offset rejected", "started_at__gte=2026-10-17T00:00:00%2B0800&started_at__lt=2026-10-18T00:00:00Z", "failed to parse started_at__gte"},
		{"window runs backwards", "started_at__gte=2026-10-18T00:00:00Z&started_at__lt=2026-10-17T00:00:00Z", "must be after"},
		{"window too wide", "started_at__gte=2026-01-01T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z", "must be at most"},
		{"camera id is not a uuid", base + "&camera_id__eq=not-a-uuid", "failed to parse camera_id__eq"},
		{"min_score above 1", base + "&min_score=1.5", "min_score must be between 0 and 1"},
		{"limit above the cap", base + "&limit=6000", "limit must be between 1 and 5000"},
		{"negative offset", base + "&offset=-1", "offset must not be negative"},
	}

	for _, item := range cases {
		t.Run(item.name, func(t *testing.T) {
			_, err := ParseParams(params(item.raw))
			if err == nil {
				t.Fatalf("ParseParams(%q) succeeded, want an error containing %q", item.raw, item.wants)
			}
			if !strings.Contains(err.Error(), item.wants) {
				t.Fatalf("error = %q, want it to contain %q", err.Error(), item.wants)
			}
		})
	}
}

func TestParseTimeAcceptsColonOffsets(t *testing.T) {
	// a literal + in a query string decodes to a space, so a colon offset arrives percent-encoded
	parsed, err := parseTime(params("started_at__gte=2026-10-17T00:00:00%2B08%3A00"), "started_at__gte")
	if err != nil {
		t.Fatalf("parseTime: %v", err)
	}
	if !parsed.Equal(time.Date(2026, 10, 16, 16, 0, 0, 0, time.UTC)) {
		t.Fatalf("parsed = %v, want 2026-10-16T16:00:00Z", parsed)
	}
	if parsed.Location() != time.UTC {
		t.Fatalf("parsed location = %v, want UTC", parsed.Location())
	}
}

func TestBuildSQLWithoutFilters(t *testing.T) {
	query, err := ParseParams(params("started_at__gte=2026-10-17T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z"))
	if err != nil {
		t.Fatalf("ParseParams: %v", err)
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildSQL()

	wants := []string{
		"FROM public.v_audio_event_hour WHERE bucket >= $1 AND bucket < $2",
		"ORDER BY bucket ASC, camera_id ASC LIMIT $3 OFFSET $4",
	}
	for _, want := range wants {
		if !strings.Contains(selectSQL, want) {
			t.Fatalf("selectSQL %q does not contain %q", selectSQL, want)
		}
	}
	if !strings.Contains(countSQL, "SELECT count(*) FROM public.v_audio_event_hour WHERE bucket >= $1 AND bucket < $2") {
		t.Fatalf("countSQL %q is not the matching count query", countSQL)
	}
	if strings.Contains(selectSQL, "camera_id IN") || strings.Contains(selectSQL, "top_score >=") {
		t.Fatalf("selectSQL %q carries a filter that was not asked for", selectSQL)
	}
	if len(selectValues) != 4 || selectValues[2] != int64(defaultLimit) || selectValues[3] != int64(0) {
		t.Fatalf("selectValues = %v, want [gte lt limit offset]", selectValues)
	}
	// the count query has no LIMIT/OFFSET, so its argument list must stop at the filters
	if len(countValues) != 2 {
		t.Fatalf("countValues = %v, want [gte lt]", countValues)
	}
	if got := maxPlaceholder(selectSQL); got != len(selectValues) {
		t.Fatalf("selectSQL top placeholder = %v, want %v", got, len(selectValues))
	}
	if got := maxPlaceholder(countSQL); got != len(countValues) {
		t.Fatalf("countSQL top placeholder = %v, want %v", got, len(countValues))
	}
}

func TestBuildSQLWithCameraAndScoreFilters(t *testing.T) {
	raw := "started_at__gte=2026-10-17T00:00:00Z&started_at__lt=2026-10-18T00:00:00Z&camera_id__eq=" + cameraA + "&camera_id__eq=" + cameraB + "&min_score=0.2&limit=100&offset=200"

	query, err := ParseParams(params(raw))
	if err != nil {
		t.Fatalf("ParseParams: %v", err)
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildSQL()

	wants := []string{
		"bucket >= $1 AND bucket < $2 AND camera_id IN ($3::uuid, $4::uuid) AND top_score >= $5",
		"ORDER BY bucket ASC, camera_id ASC LIMIT $6 OFFSET $7",
	}
	for _, want := range wants {
		if !strings.Contains(selectSQL, want) {
			t.Fatalf("selectSQL %q does not contain %q", selectSQL, want)
		}
	}
	if !strings.Contains(countSQL, sharedClause(selectSQL)) {
		t.Fatalf("countSQL %q is not the matching count query", countSQL)
	}
	if len(selectValues) != 7 {
		t.Fatalf("len(selectValues) = %v, want 7", len(selectValues))
	}
	if selectValues[2] != cameraA || selectValues[3] != cameraB || selectValues[4] != 0.2 || selectValues[5] != int64(100) || selectValues[6] != int64(200) {
		t.Fatalf("selectValues = %v, want [gte lt A B 0.2 100 200]", selectValues)
	}
	if len(countValues) != 5 {
		t.Fatalf("len(countValues) = %v, want 5", len(countValues))
	}
	if got := maxPlaceholder(countSQL); got != len(countValues) {
		t.Fatalf("countSQL top placeholder = %v, want %v", got, len(countValues))
	}
}

// maxPlaceholder returns the highest $N in a query, which must match its argument count.
func maxPlaceholder(sql string) int {
	top := 0

	for i, char := range sql {
		if char != '$' {
			continue
		}

		digits := ""
		for _, next := range sql[i+1:] {
			if next < '0' || next > '9' {
				break
			}
			digits += string(next)
		}

		if digits == "" {
			continue
		}

		number, err := strconv.Atoi(digits)
		if err != nil {
			panic("bad placeholder in " + sql)
		}
		if number > top {
			top = number
		}
	}

	return top
}

// sharedClause extracts the shared WHERE clause so the count query can be compared against the select.
func sharedClause(selectSQL string) string {
	start := strings.Index(selectSQL, "FROM public.v_audio_event_hour WHERE ")
	end := strings.Index(selectSQL, " ORDER BY ")
	if start < 0 || end < 0 {
		panic("no shared clause in " + selectSQL)
	}
	return "SELECT count(*) " + selectSQL[start:end]
}
