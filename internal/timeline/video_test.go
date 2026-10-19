package timeline

import (
	"encoding/json"
	"os"
	"strings"
	"testing"
	"time"
)

const videoA = "cc556ab1-1a4a-4c1a-846e-7123be7bb936"

func TestParseVideoParamsDefaults(t *testing.T) {
	query, err := ParseVideoParams(params("video_id__eq=" + videoA))
	if err != nil {
		t.Fatalf("ParseVideoParams: %v", err)
	}

	if query.VideoID != videoA {
		t.Fatalf("VideoID = %v, want %v", query.VideoID, videoA)
	}
	if query.BucketMillis != defaultVideoBucketMillis {
		t.Fatalf("BucketMillis = %v, want %v", query.BucketMillis, defaultVideoBucketMillis)
	}
	if query.AlignMillis != 0 {
		t.Fatalf("AlignMillis = %v, want 0", query.AlignMillis)
	}
	if query.Limit != defaultVideoRowCount {
		t.Fatalf("Limit = %v, want %v", query.Limit, defaultVideoRowCount)
	}
	if query.Offset != 0 {
		t.Fatalf("Offset = %v, want 0", query.Offset)
	}
}

func TestParseVideoParamsRejectsBadInput(t *testing.T) {
	base := "video_id__eq=" + videoA

	cases := []struct {
		name  string
		raw   string
		wants string
	}{
		{"missing video id", "bucket_ms=1000", "failed to parse video_id__eq"},
		{"video id is not a uuid", "video_id__eq=not-a-uuid", "failed to parse video_id__eq"},
		{"bucket too small", base + "&bucket_ms=50", "bucket_ms must be between"},
		{"bucket too large", base + "&bucket_ms=60001", "bucket_ms must be between"},
		{"align not a number", base + "&align_ms=soon", "failed to parse align_ms"},
		{"limit above the cap", base + "&limit=200001", "limit must be between 1 and 200000"},
		{"negative offset", base + "&offset=-1", "offset must not be negative"},
	}

	for _, item := range cases {
		t.Run(item.name, func(t *testing.T) {
			_, err := ParseVideoParams(params(item.raw))
			if err == nil {
				t.Fatalf("ParseVideoParams(%q) = nil error, want %q", item.raw, item.wants)
			}
			if !strings.Contains(err.Error(), item.wants) {
				t.Fatalf("ParseVideoParams(%q) = %q, want it to contain %q", item.raw, err.Error(), item.wants)
			}
		})
	}
}

func TestBuildVideoTimelineSQL(t *testing.T) {
	query := &VideoQueryParams{
		VideoID:      videoA,
		BucketMillis: 1000,
		AlignMillis:  1791986400000,
		Limit:        500,
		Offset:       0,
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoTimelineSQL()

	for _, wants := range []string{
		"SELECT bucket, detection_count, object_count, top_score, top_class_name ",
		"d.video_id = $1::uuid",
		"d.deleted_at IS NULL",
		"t.video_id = $1::uuid",
		"t.deleted_at IS NULL",
		"extract(epoch FROM d.seen_at) * 1000 - $3",
		"generate_series(0, greatest(0, (s.last_offset - s.first_offset) / $2)::int, 1)",
		"FULL OUTER JOIN track_buckets USING (bucket)",
		") AS video_bucket",
		"ORDER BY bucket ASC LIMIT $4 OFFSET $5",
	} {
		if !strings.Contains(selectSQL, wants) {
			t.Fatalf("selectSQL %q missing %q", selectSQL, wants)
		}
	}

	if strings.Contains(selectSQL, "video_id_object") || strings.Contains(selectSQL, "camera_id_object") {
		t.Fatalf("selectSQL %q expands related objects; the strip reads counts only", selectSQL)
	}

	if !strings.HasPrefix(countSQL, "SELECT count(*) FROM (") {
		t.Fatalf("countSQL %q is not the same projection counted", countSQL)
	}

	if len(countValues) != 3 || len(selectValues) != 5 {
		t.Fatalf("len(countValues) = %v, len(selectValues) = %v, want 3 and 5", countValues, selectValues)
	}
	if selectValues[3] != int64(500) || selectValues[4] != int64(0) {
		t.Fatalf("selectValues tail = %v, want limit then offset", selectValues[3:])
	}
}

func TestBuildVideoDetectionsSQL(t *testing.T) {
	query := &VideoQueryParams{VideoID: videoA, Limit: defaultVideoRowCount}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoDetectionsSQL()

	for _, wants := range []string{
		"SELECT seen_at, class_name, score, centroid::text, bounding_box::text ",
		"FROM public.detection WHERE video_id = $1::uuid AND deleted_at IS NULL",
		"ORDER BY seen_at ASC, id ASC LIMIT $2 OFFSET $3",
	} {
		if !strings.Contains(selectSQL, wants) {
			t.Fatalf("selectSQL %q missing %q", selectSQL, wants)
		}
	}

	for _, forbidden := range []string{"video_id_object", "camera_id_object", "created_at", "JOIN", "bounding_box["} {
		if strings.Contains(selectSQL, forbidden) {
			t.Fatalf("selectSQL %q contains %q; the canvas draws boxes, not whole rows", selectSQL, forbidden)
		}
	}

	if countSQL != "SELECT count(*) FROM public.detection WHERE video_id = $1::uuid AND deleted_at IS NULL" {
		t.Fatalf("countSQL %q does not match the projection", countSQL)
	}

	if len(countValues) != 1 || len(selectValues) != 3 {
		t.Fatalf("len(countValues) = %v, len(selectValues) = %v, want 1 and 3", countValues, selectValues)
	}
}

func TestBuildVideoTracksSQL(t *testing.T) {
	query := &VideoQueryParams{VideoID: videoA, Limit: defaultVideoRowCount}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoTracksSQL()

	for _, wants := range []string{
		"SELECT t.id::text, t.key::text, t.class_name, t.mean_score, t.is_static, s->>'seen_at', s->'bbox' ",
		"jsonb_array_elements(",
		"jsonb_typeof(t.trajectory) = 'array'",
		"WHERE t.video_id = $1::uuid AND t.deleted_at IS NULL",
		"ORDER BY t.started_at ASC, t.id ASC, s->>'seen_at' ASC LIMIT $2 OFFSET $3",
	} {
		if !strings.Contains(selectSQL, wants) {
			t.Fatalf("selectSQL %q missing %q", selectSQL, wants)
		}
	}

	for _, forbidden := range []string{"video_id_object", "camera_id_object", "s->'x'", "s->'y'"} {
		if strings.Contains(selectSQL, forbidden) {
			t.Fatalf("selectSQL %q contains %q; the canvas draws bbox stamps, not whole rows", selectSQL, forbidden)
		}
	}

	if !strings.Contains(countSQL, "SELECT count(*) FROM public.track t") {
		t.Fatalf("countSQL %q counts the same sample rows", countSQL)
	}

	if len(countValues) != 1 || len(selectValues) != 3 {
		t.Fatalf("len(countValues) = %v, len(selectValues) = %v, want 1 and 3", countValues, selectValues)
	}
}

func TestPrintVideoSQL(t *testing.T) {
	if os.Getenv("PRINT_VIDEO_SQL") == "" {
		t.Skip("set PRINT_VIDEO_SQL=1 to print the projections for a live database run")
	}

	query := &VideoQueryParams{
		VideoID:      videoA,
		BucketMillis: 1000,
		AlignMillis:  1791986400000,
		Limit:        500,
		Offset:       0,
	}

	for _, build := range []func() (string, string, []any, []any){
		query.BuildVideoTimelineSQL,
		query.BuildVideoDetectionsSQL,
		query.BuildVideoTracksSQL,
	} {
		selectSQL, countSQL, _, _ := build()
		t.Logf("SELECT\n%s\n", selectSQL)
		t.Logf("COUNT\n%s\n", countSQL)
	}
}

func TestParseGeometryText(t *testing.T) {
	point, err := parsePointText("(458.693359375,78.4130859375)")
	if err != nil {
		t.Fatalf("parsePointText: %v", err)
	}
	if point[0] != 458.693359375 || point[1] != 78.4130859375 {
		t.Fatalf("point = %v", point)
	}

	// A real ring from the detector: closed, clockwise, five vertices.
	box, err := parsePolygonBoxText("((401.08,1.87),(516.31,1.87),(516.31,154.95),(401.08,154.95),(401.08,1.87))")
	if err != nil {
		t.Fatalf("parsePolygonBoxText: %v", err)
	}
	if box != [4]float64{401.08, 1.87, 516.31, 154.95} {
		t.Fatalf("box = %v, want the ring's bounding box", box)
	}

	// Rings are not guaranteed to start at the top-left, so the box comes from the extremes.
	box, err = parsePolygonBoxText("((516.31,154.95),(401.08,154.95),(401.08,1.87),(516.31,1.87),(516.31,154.95))")
	if err != nil {
		t.Fatalf("parsePolygonBoxText reversed: %v", err)
	}
	if box != [4]float64{401.08, 1.87, 516.31, 154.95} {
		t.Fatalf("reversed box = %v, want the same bounding box", box)
	}

	// The seeded/legacy shape is a 2-vertex polygon: the box's own corners.
	box, err = parsePolygonBoxText("((0.1,0.1),(0.3,0.3))")
	if err != nil {
		t.Fatalf("parsePolygonBoxText two vertices: %v", err)
	}
	if box != [4]float64{0.1, 0.1, 0.3, 0.3} {
		t.Fatalf("two-vertex box = %v, want the corners", box)
	}

	for _, item := range []string{"", "(1,2)", "(1,2),(3,4)", "((1,2))"} {
		if _, err := parsePolygonBoxText(item); err == nil {
			t.Fatalf("parsePolygonBoxText(%q) = nil error, want a rejection", item)
		}
	}

	if _, err := parsePointText("(1,2,3)"); err == nil {
		t.Fatalf("parsePointText with three numbers = nil error, want a rejection")
	}
}

func TestVideoRowMarshallingIsFlat(t *testing.T) {
	detection := VideoDetection{
		SeenAt:    time.Date(2026, 10, 14, 14, 13, 0, 554091000, time.UTC),
		ClassName: "vase",
		Score:     0.39422670006752014,
		Centroid:  [2]float64{458.69, 78.41},
		Box:       [4]float64{401.07, 1.87, 516.31, 154.95},
	}

	detectionJSON, err := json.Marshal(detection)
	if err != nil {
		t.Fatalf("marshal detection: %v", err)
	}

	if !strings.Contains(string(detectionJSON), `"centroid":[458.69,78.41]`) ||
		!strings.Contains(string(detectionJSON), `"box":[401.07,1.87,516.31,154.95]`) ||
		strings.Contains(string(detectionJSON), `"video_id_object"`) {
		t.Fatalf("detectionJSON = %v", string(detectionJSON))
	}

	track := VideoTrack{
		ID:        "0d0e7f0e-1c37-4a5e-8f57-2a5ef2a2b0d1",
		Key:       cameraA,
		ClassName: "car",
		MeanScore: 0.51,
		IsStatic:  true,
		Trajectory: []VideoTrackSample{
			{
				SeenAt: time.Date(2026, 10, 18, 7, 45, 7, 896179000, time.UTC),
				BBox:   []float64{80.23, 198.73, 193.67, 275.15},
			},
		},
	}

	trackJSON, err := json.Marshal(track)
	if err != nil {
		t.Fatalf("marshal track: %v", err)
	}

	if !strings.Contains(string(trackJSON), `"bbox":[80.23,198.73,193.67,275.15]`) ||
		strings.Contains(string(trackJSON), `"trajectory_samples"`) {
		t.Fatalf("trackJSON = %v", string(trackJSON))
	}
}
