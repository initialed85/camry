package timeline

import (
	"context"
	"encoding/json"
	"fmt"
	"math"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/google/uuid"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

const (
	defaultVideoBucketMillis = int64(1000)
	minVideoBucketMillis     = int64(100)
	maxVideoBucketMillis     = int64(60000)

	defaultVideoRowCount = int64(100000)
	maxVideoRowCount     = int64(200000)

	// A finished segment never changes, so its projections are cacheable. 15 s is short enough that
	// a segment still being written looks fresh on the next modal open.
	videoCacheControl = "private, max-age=15"
)

// VideoBucket is one strip bucket of a single recording: detections by seen_at, objects counted in
// every bucket their track is visible in. The modal used to derive this in the browser from the
// whole detection list, which costs 3.6 kB per detection because the generated route expands each
// row's video and camera objects; the strip reads a handful of numbers per bar.
type VideoBucket struct {
	BucketMillis   int64   `json:"bucket_ms"`
	DetectionCount int64   `json:"detection_count"`
	ObjectCount    int64   `json:"object_count"`
	TopScore       float64 `json:"top_score"`
	TopClassName   string  `json:"top_class_name"`
}

// VideoDetection is one overlay box: the columns the canvas draws, in flat arrays.
type VideoDetection struct {
	SeenAt    time.Time `json:"seen_at"`
	ClassName string    `json:"class_name"`
	Score     float64   `json:"score"`
	Centroid  [2]float64
	Box       [4]float64
}

type VideoTrack struct {
	ID         string
	Key        string
	ClassName  string
	MeanScore  float64
	IsStatic   bool
	Trajectory []VideoTrackSample
}

type VideoTrackSample struct {
	SeenAt time.Time
	BBox   []float64
}

func (d VideoDetection) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		SeenAt    string     `json:"seen_at"`
		ClassName string     `json:"class_name"`
		Score     float64    `json:"score"`
		Centroid  [2]float64 `json:"centroid"`
		Box       [4]float64 `json:"box"`
	}{
		SeenAt:    d.SeenAt.Format(time.RFC3339Nano),
		ClassName: d.ClassName,
		Score:     d.Score,
		Centroid:  d.Centroid,
		Box:       d.Box,
	})
}

func (t VideoTrack) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		ID         string             `json:"id"`
		Key        string             `json:"key"`
		ClassName  string             `json:"class_name"`
		MeanScore  float64            `json:"mean_score"`
		IsStatic   bool               `json:"is_static"`
		Trajectory []VideoTrackSample `json:"trajectory"`
	}{
		ID:         t.ID,
		Key:        t.Key,
		ClassName:  t.ClassName,
		MeanScore:  t.MeanScore,
		IsStatic:   t.IsStatic,
		Trajectory: t.Trajectory,
	})
}

func (s VideoTrackSample) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		SeenAt string    `json:"seen_at"`
		BBox   []float64 `json:"bbox"`
	}{
		SeenAt: s.SeenAt.Format(time.RFC3339Nano),
		BBox:   s.BBox,
	})
}

// HandleVideoTimeline answers GET /api/custom/video-timeline: the modal strip's bars, bucketed in
// SQL. Buckets are aligned to align_ms, the playback zero the strip draws from, so a bar index is
// (bucket_ms - align_ms) / bucket_ms with no rounding at the recording's edges.
func (t *Timeline) HandleVideoTimeline(w http.ResponseWriter, req *http.Request) {
	query, err := ParseVideoParams(req.URL.Query())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoTimelineSQL()

	buckets, totalCount, err := runBuckets(req.Context(), t.db, selectSQL, selectValues, countSQL, countValues, scanVideoBucket)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to read video timeline; %v", err))
		return
	}

	writeVideoRows(w, query.Limit, query.Offset, buckets, totalCount, videoCacheControl)
}

// HandleVideoDetections answers GET /api/custom/video-detections: every detection of one recording,
// stripped to the columns the overlay canvas draws.
func (t *Timeline) HandleVideoDetections(w http.ResponseWriter, req *http.Request) {
	query, err := ParseVideoParams(req.URL.Query())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoDetectionsSQL()

	detections, totalCount, err := runRows(req.Context(), t.db, selectSQL, selectValues, countSQL, countValues, scanVideoDetection)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to read video detections; %v", err))
		return
	}

	writeVideoRows(w, query.Limit, query.Offset, detections, totalCount, videoCacheControl)
}

// HandleVideoTracks answers GET /api/custom/video-tracks: every track of one recording with its
// trajectory samples, grouped per track in the order the canvas walks them.
func (t *Timeline) HandleVideoTracks(w http.ResponseWriter, req *http.Request) {
	query, err := ParseVideoParams(req.URL.Query())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildVideoTracksSQL()

	tracks, totalCount, err := queryTracks(req.Context(), t.db, selectSQL, selectValues, countSQL, countValues)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to read video tracks; %v", err))
		return
	}

	writeVideoRows(w, query.Limit, query.Offset, tracks, totalCount, videoCacheControl)
}

func scanVideoBucket(row pgx.Row) (*VideoBucket, error) {
	item := &VideoBucket{}

	err := row.Scan(
		&item.BucketMillis,
		&item.DetectionCount,
		&item.ObjectCount,
		&item.TopScore,
		&item.TopClassName,
	)

	return item, err
}

// scanVideoDetection reads the geometry as text: PostgreSQL 18 cannot subscript point or polygon,
// so the corners come over as their literal form and are reduced here.
func scanVideoDetection(row pgx.Row) (*VideoDetection, error) {
	item := &VideoDetection{}

	var centroidText string
	var boxText string

	err := row.Scan(
		&item.SeenAt,
		&item.ClassName,
		&item.Score,
		&centroidText,
		&boxText,
	)
	if err != nil {
		return nil, err
	}

	item.Centroid, err = parsePointText(centroidText)
	if err != nil {
		return nil, fmt.Errorf("failed to read centroid %q; %v", centroidText, err)
	}

	item.Box, err = parsePolygonBoxText(boxText)
	if err != nil {
		return nil, fmt.Errorf("failed to read bounding_box %q; %v", boxText, err)
	}

	return item, nil
}

// parsePointText parses PostgreSQL's point literal, "(x,y)".
func parsePointText(raw string) ([2]float64, error) {
	var point [2]float64

	parts := strings.Split(strings.Trim(strings.TrimSpace(raw), "()"), ",")
	if len(parts) != 2 {
		return point, fmt.Errorf("expected x and y")
	}

	x, err := strconv.ParseFloat(strings.TrimSpace(parts[0]), 64)
	if err != nil {
		return point, err
	}

	y, err := strconv.ParseFloat(strings.TrimSpace(parts[1]), 64)
	if err != nil {
		return point, err
	}

	point[0] = x
	point[1] = y

	return point, nil
}

// parsePolygonBoxText reduces PostgreSQL's closed polygon ring, "((x,y),(x,y),...)", to its
// bounding box as x0, y0, x1, y1. The ring can run either way, so the box is the extremes.
func parsePolygonBoxText(raw string) ([4]float64, error) {
	var box [4]float64

	text := strings.TrimSpace(raw)
	if !strings.HasPrefix(text, "((") || !strings.HasSuffix(text, "))") {
		return box, fmt.Errorf("expected a polygon ring")
	}

	vertices := strings.Split(text[1:len(text)-1], "),(")
	// A 2-vertex polygon is already the box (top-left, bottom-right); a live ring is 5 vertices
	// (first == last). Extremes over the vertices cover both, in either ring direction.
	if len(vertices) < 2 {
		return box, fmt.Errorf("expected at least 2 vertices")
	}

	for i, vertex := range vertices {
		point, err := parsePointText(vertex)
		if err != nil {
			return box, fmt.Errorf("vertex %v: %v", i, err)
		}

		if i == 0 {
			box[0], box[1], box[2], box[3] = point[0], point[1], point[0], point[1]
			continue
		}

		box[0] = math.Min(box[0], point[0])
		box[1] = math.Min(box[1], point[1])
		box[2] = math.Max(box[2], point[0])
		box[3] = math.Max(box[3], point[1])
	}

	return box, nil
}

// runRows answers a flat projection: page of rows plus the total row count, in one transaction.
func runRows[T any](ctx context.Context, db *pgxpool.Pool, selectSQL string, selectValues []any, countSQL string, countValues []any, scan func(pgx.Row) (*T, error)) ([]*T, int64, error) {
	tx, err := db.Begin(ctx)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to open video transaction; %v", err)
	}

	defer tx.Rollback(ctx)

	rows, err := queryBuckets(ctx, tx, selectSQL, selectValues, scan)
	if err != nil {
		return nil, 0, err
	}

	var totalCount int64

	err = tx.QueryRow(ctx, countSQL, countValues...).Scan(&totalCount)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to count video rows; %v", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to close video transaction; %v", err)
	}

	return rows, totalCount, nil
}

// queryTracks groups the per-sample rows the projection returns. One row per sample keeps the
// projection a plain index scan; grouping happens here, in the order SQL returned them.
func queryTracks(ctx context.Context, db *pgxpool.Pool, selectSQL string, selectValues []any, countSQL string, countValues []any) ([]*VideoTrack, int64, error) {
	tx, err := db.Begin(ctx)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to open video transaction; %v", err)
	}

	defer tx.Rollback(ctx)

	rows, err := tx.Query(ctx, selectSQL, selectValues...)
	if err != nil {
		return nil, 0, err
	}

	defer rows.Close()

	tracks := []*VideoTrack{}
	byID := map[string]*VideoTrack{}

	for rows.Next() {
		var (
			id        string
			key       string
			className string
			meanScore float64
			isStatic  bool
			seenAt    string
			bbox      []byte
		)

		err = rows.Scan(&id, &key, &className, &meanScore, &isStatic, &seenAt, &bbox)
		if err != nil {
			return nil, 0, err
		}

		track, found := byID[id]
		if !found {
			track = &VideoTrack{
				ID:         id,
				Key:        key,
				ClassName:  className,
				MeanScore:  meanScore,
				IsStatic:   isStatic,
				Trajectory: []VideoTrackSample{},
			}

			byID[id] = track
			tracks = append(tracks, track)
		}

		sample := VideoTrackSample{BBox: []float64{}}

		// Stored samples carry ISO stamps; a sample that predates that format keeps a zero time and
		// is skipped by the canvas, which is what it already does with an unparseable stamp.
		parsedSeenAt, err := time.Parse(time.RFC3339Nano, seenAt)
		if err == nil {
			sample.SeenAt = parsedSeenAt.UTC()
		}

		if len(bbox) > 0 {
			err = json.Unmarshal(bbox, &sample.BBox)
			if err != nil {
				return nil, 0, fmt.Errorf("failed to read track %v trajectory; %v", id, err)
			}
		}

		track.Trajectory = append(track.Trajectory, sample)
	}

	err = rows.Err()
	if err != nil {
		return nil, 0, err
	}

	var totalCount int64

	err = tx.QueryRow(ctx, countSQL, countValues...).Scan(&totalCount)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to count video tracks; %v", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		return nil, 0, fmt.Errorf("failed to close video transaction; %v", err)
	}

	return tracks, totalCount, nil
}

func writeVideoRows[T any](w http.ResponseWriter, limit int64, offset int64, rows []*T, totalCount int64, cacheControl string) {
	response := server.Response[T]{
		Status:     http.StatusOK,
		Success:    true,
		Objects:    rows,
		Count:      int64(len(rows)),
		TotalCount: totalCount,
		Limit:      limit,
		Offset:     offset,
	}

	b, err := json.Marshal(response)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to marshal video projection; %v", err))
		return
	}

	w.Header().Set("Cache-Control", cacheControl)

	server.WriteResponse(w, http.StatusOK, b)
}

// VideoQueryParams is the filter set all three per-video endpoints share.
type VideoQueryParams struct {
	VideoID      string
	BucketMillis int64
	AlignMillis  int64
	Limit        int64
	Offset       int64
}

func ParseVideoParams(values url.Values) (*VideoQueryParams, error) {
	query := &VideoQueryParams{
		BucketMillis: defaultVideoBucketMillis,
		AlignMillis:  0,
		Limit:        defaultVideoRowCount,
		Offset:       0,
	}

	rawVideoID := strings.TrimSpace(values.Get("video_id__eq"))

	parsedVideoID, err := uuid.Parse(rawVideoID)
	if err != nil {
		return nil, fmt.Errorf("failed to parse video_id__eq %q; %v", rawVideoID, err)
	}

	query.VideoID = parsedVideoID.String()

	bucketMillis, err := parseInt(values, "bucket_ms", defaultVideoBucketMillis)
	if err != nil {
		return nil, err
	}

	if bucketMillis < minVideoBucketMillis || bucketMillis > maxVideoBucketMillis {
		return nil, fmt.Errorf("bucket_ms must be between %v and %v", minVideoBucketMillis, maxVideoBucketMillis)
	}

	query.BucketMillis = bucketMillis

	alignMillis, err := parseInt(values, "align_ms", 0)
	if err != nil {
		return nil, err
	}

	query.AlignMillis = alignMillis

	limit, err := parseInt(values, "limit", defaultVideoRowCount)
	if err != nil {
		return nil, err
	}

	if limit < 1 || limit > maxVideoRowCount {
		return nil, fmt.Errorf("limit must be between 1 and %v", maxVideoRowCount)
	}

	query.Limit = limit

	offset, err := parseInt(values, "offset", 0)
	if err != nil {
		return nil, err
	}

	if offset < 0 {
		return nil, fmt.Errorf("offset must not be negative")
	}

	query.Offset = offset

	return query, nil
}

// BuildVideoTimelineSQL buckets detections by seen_at and spreads every track across the buckets it
// is visible in, matching what the modal strip drew client-side.
func (q *VideoQueryParams) BuildVideoTimelineSQL() (string, string, []any, []any) {
	values := []any{q.VideoID, q.BucketMillis, q.AlignMillis}

	shared := `FROM (
		WITH detections AS (
			SELECT (floor((extract(epoch FROM d.seen_at) * 1000 - $3) / $2) * $2 + $3)::bigint AS bucket,
			       d.score, d.class_name
			FROM public.detection d
			WHERE d.video_id = $1::uuid AND d.deleted_at IS NULL
		), spans AS (
			SELECT (floor((extract(epoch FROM LEAST(t.started_at, t.ended_at)) * 1000 - $3) / $2) * $2)::bigint AS first_offset,
			       (floor((extract(epoch FROM GREATEST(t.started_at, t.ended_at)) * 1000 - $3) / $2) * $2)::bigint AS last_offset,
			       t.max_score AS score, t.class_name
			FROM public.track t
			WHERE t.video_id = $1::uuid AND t.deleted_at IS NULL
		), tracks AS (
			SELECT (s.first_offset + off * $2 + $3)::bigint AS bucket, s.score, s.class_name
			FROM spans s
			CROSS JOIN LATERAL generate_series(0, greatest(0, (s.last_offset - s.first_offset) / $2)::int, 1) AS off
		), detection_buckets AS (
			SELECT bucket, count(*) AS detection_count, max(score) AS top_score,
			       (array_agg(class_name ORDER BY score DESC, class_name))[1] AS top_class_name
			FROM detections GROUP BY bucket
		), track_buckets AS (
			SELECT bucket, count(*) AS object_count, max(score) AS top_score,
			       (array_agg(class_name ORDER BY score DESC, class_name))[1] AS top_class_name
			FROM tracks GROUP BY bucket
		)
		SELECT coalesce(detection_buckets.bucket, track_buckets.bucket) AS bucket,
		       coalesce(detection_buckets.detection_count, 0) AS detection_count,
		       coalesce(track_buckets.object_count, 0) AS object_count,
		       coalesce(detection_buckets.top_score, track_buckets.top_score) AS top_score,
		       coalesce(detection_buckets.top_class_name, track_buckets.top_class_name) AS top_class_name
		FROM detection_buckets
		FULL OUTER JOIN track_buckets USING (bucket)
	) AS video_bucket`

	selectSQL := "SELECT bucket, detection_count, object_count, top_score, top_class_name " + shared +
		fmt.Sprintf(" ORDER BY bucket ASC LIMIT $%v OFFSET $%v", len(values)+1, len(values)+2)

	countSQL := "SELECT count(*) " + shared

	selectValues := append(append([]any{}, values...), q.Limit, q.Offset)

	return selectSQL, countSQL, selectValues, values
}

// BuildVideoDetectionsSQL projects the overlay canvas's columns only. point and polygon are not
// subscriptable in PostgreSQL 18, so the geometry arrives as text and is reduced in Go.
func (q *VideoQueryParams) BuildVideoDetectionsSQL() (string, string, []any, []any) {
	values := []any{q.VideoID}

	shared := "FROM public.detection WHERE video_id = $1::uuid AND deleted_at IS NULL"

	selectSQL := "SELECT seen_at, class_name, score, centroid::text, bounding_box::text " +
		shared +
		fmt.Sprintf(" ORDER BY seen_at ASC, id ASC LIMIT $%v OFFSET $%v", len(values)+1, len(values)+2)

	countSQL := "SELECT count(*) " + shared

	selectValues := append(append([]any{}, values...), q.Limit, q.Offset)

	return selectSQL, countSQL, selectValues, values
}

// BuildVideoTracksSQL returns one row per trajectory sample, dropping the x/y pair the canvas does
// not draw. Samples are paginated, so a page may end mid-track.
func (q *VideoQueryParams) BuildVideoTracksSQL() (string, string, []any, []any) {
	values := []any{q.VideoID}

	shared := `FROM public.track t
		CROSS JOIN LATERAL jsonb_array_elements(
			CASE WHEN jsonb_typeof(t.trajectory) = 'array' THEN t.trajectory ELSE '[]'::jsonb END
		) AS s` +
		" WHERE t.video_id = $1::uuid AND t.deleted_at IS NULL"

	selectSQL := "SELECT t.id::text, t.key::text, t.class_name, t.mean_score, t.is_static, s->>'seen_at', s->'bbox' " +
		shared +
		fmt.Sprintf(" ORDER BY t.started_at ASC, t.id ASC, s->>'seen_at' ASC LIMIT $%v OFFSET $%v", len(values)+1, len(values)+2)

	countSQL := "SELECT count(*) " + shared

	selectValues := append(append([]any{}, values...), q.Limit, q.Offset)

	return selectSQL, countSQL, selectValues, values
}

// videoRoutes mounts the per-video projections alongside the hour ones.
func (t *Timeline) videoRoutes(r chi.Router) {
	r.Get("/video-timeline", t.HandleVideoTimeline)
	r.Get("/video-detections", t.HandleVideoDetections)
	r.Get("/video-tracks", t.HandleVideoTracks)
}
