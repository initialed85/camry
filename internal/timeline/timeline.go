package timeline

import (
	"context"
	"encoding/json"
	"fmt"
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

// BucketSize is the projection's grain: one row per camera per calendar hour. The date slider
// steps in whole hours, so its bars line up with this grain exactly.
const BucketSize = time.Hour

const (
	defaultLimit = 1000
	maxLimit     = 5000
	maxWindow    = 31 * 24 * time.Hour
	workMem      = "64MB"
)

// Bucket is one camera-hour of audio events: everything the date slider strip draws, and nothing
// more. The generated audio_event list route cannot deliver this shape - it returns whole event
// rows with their expanded video and camera objects, which is ~8.7 kB per event against the few
// dozen bytes the strip reads.
type Bucket struct {
	Bucket           time.Time `json:"bucket"`
	CameraID         string    `json:"camera_id"`
	EventCount       int64     `json:"event_count"`
	TopScore         float64   `json:"top_score"`
	PeakProminenceDb float64   `json:"peak_prominence_db"`
	TopKind          string    `json:"top_kind"`
	FirstSeenAt      time.Time `json:"first_seen_at"`
	LastSeenAt       time.Time `json:"last_seen_at"`
}

type Timeline struct {
	db *pgxpool.Pool
}

func New(db *pgxpool.Pool) *Timeline {
	return &Timeline{
		db: db,
	}
}

// Routes mounts the projection under djangolang's /custom, i.e. GET /api/custom/audio-timeline.
// Custom routes are not emitted into the generated OpenAPI schema, so the frontend calls this one
// by hand (frontend/src/api/audioTimeline.ts). Traefik gates all of /api with camry-auth, so the
// route is token-gated with no handler-side auth code.
func (t *Timeline) Routes(r chi.Router) {
	r.Get("/audio-timeline", t.HandleAudioTimeline)
}

// HandleAudioTimeline answers GET /api/custom/audio-timeline from the v_audio_event_hour view.
func (t *Timeline) HandleAudioTimeline(w http.ResponseWriter, req *http.Request) {
	query, err := ParseParams(req.URL.Query())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	selectSQL, countSQL, selectValues, countValues := query.BuildSQL()

	// At 7-day retention the strip window covers nearly the whole audio_event table, so the
	// aggregate sorts every in-window row to pick top_kind. The default 4 MB work_mem spills that
	// sort to disk (measured 241 ms with temp files); 64 MB keeps it in memory (measured 71 ms).
	// Both queries run in one transaction so the setting applies to each.
	tx, err := t.db.Begin(req.Context())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to open audio timeline transaction; %v", err))
		return
	}

	defer tx.Rollback(req.Context())

	_, err = tx.Exec(req.Context(), "select set_config('work_mem', $1, true)", workMem)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to set work_mem; %v", err))
		return
	}

	buckets, err := t.queryBuckets(req.Context(), tx, selectSQL, selectValues)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to query audio timeline; %v", err))
		return
	}

	var totalCount int64

	err = tx.QueryRow(req.Context(), countSQL, countValues...).Scan(&totalCount)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to count audio timeline buckets; %v", err))
		return
	}

	err = tx.Commit(req.Context())
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to close audio timeline transaction; %v", err))
		return
	}

	response := server.Response[Bucket]{
		Status:     http.StatusOK,
		Success:    true,
		Objects:    buckets,
		Count:      int64(len(buckets)),
		TotalCount: totalCount,
		Limit:      query.Limit,
		Offset:     query.Offset,
	}

	b, err := json.Marshal(response)
	if err != nil {
		server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to marshal audio timeline; %v", err))
		return
	}

	// the strip polls on a moving window, so an intermediate cache is never useful
	w.Header().Set("Cache-Control", "no-store")

	server.WriteResponse(w, http.StatusOK, b)
}

func (t *Timeline) queryBuckets(ctx context.Context, tx pgx.Tx, selectSQL string, values []any) ([]*Bucket, error) {
	rows, err := tx.Query(ctx, selectSQL, values...)
	if err != nil {
		return nil, err
	}

	defer rows.Close()

	buckets := []*Bucket{}

	for rows.Next() {
		item := &Bucket{}

		err = rows.Scan(
			&item.Bucket,
			&item.CameraID,
			&item.EventCount,
			&item.TopScore,
			&item.PeakProminenceDb,
			&item.TopKind,
			&item.FirstSeenAt,
			&item.LastSeenAt,
		)
		if err != nil {
			return nil, err
		}

		buckets = append(buckets, item)
	}

	err = rows.Err()
	if err != nil {
		return nil, err
	}

	return buckets, nil
}

type QueryParams struct {
	StartedAtGte time.Time
	StartedAtLt  time.Time
	CameraIDs    []string
	MinScore     float64
	Limit        int64
	Offset       int64
}

// ParseParams reads the query string. Times must be RFC3339 (a plain offset like +0800 is
// rejected, the same rule the generated datetime filters follow). camera_id__eq may be repeated
// and may carry comma-separated ids. Unknown keys are ignored, as in the generated routes.
func ParseParams(values url.Values) (*QueryParams, error) {
	query := &QueryParams{
		Limit:  defaultLimit,
		Offset: 0,
	}

	startedAtGte, err := parseTime(values, "started_at__gte")
	if err != nil {
		return nil, err
	}

	startedAtLt, err := parseTime(values, "started_at__lt")
	if err != nil {
		return nil, err
	}

	if !startedAtLt.After(startedAtGte) {
		return nil, fmt.Errorf("started_at__lt must be after started_at__gte")
	}

	if startedAtLt.Sub(startedAtGte) > maxWindow {
		return nil, fmt.Errorf("started_at__lt - started_at__gte must be at most %v", maxWindow)
	}

	query.StartedAtGte = startedAtGte
	query.StartedAtLt = startedAtLt

	seen := map[string]bool{}

	for _, raw := range values["camera_id__eq"] {
		for _, part := range strings.Split(raw, ",") {
			part = strings.TrimSpace(part)
			if part == "" {
				continue
			}

			parsed, err := uuid.Parse(part)
			if err != nil {
				return nil, fmt.Errorf("failed to parse camera_id__eq %q; %v", part, err)
			}

			if !seen[parsed.String()] {
				seen[parsed.String()] = true
				query.CameraIDs = append(query.CameraIDs, parsed.String())
			}
		}
	}

	minScore, err := parseFloat(values, "min_score", 0)
	if err != nil {
		return nil, err
	}

	if minScore < 0 || minScore > 1 {
		return nil, fmt.Errorf("min_score must be between 0 and 1")
	}

	query.MinScore = minScore

	limit, err := parseInt(values, "limit", defaultLimit)
	if err != nil {
		return nil, err
	}

	if limit < 1 || limit > maxLimit {
		return nil, fmt.Errorf("limit must be between 1 and %v", maxLimit)
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

// BuildSQL returns the bucket query, the matching count query, and their arguments. The count
// query has no LIMIT/OFFSET, so it cannot share the bucket query's argument list.
func (q *QueryParams) BuildSQL() (string, string, []any, []any) {
	where := []string{"bucket >= $1", "bucket < $2"}
	countValues := []any{q.StartedAtGte, q.StartedAtLt}

	if len(q.CameraIDs) > 0 {
		placeholders := make([]string, 0, len(q.CameraIDs))

		for _, cameraID := range q.CameraIDs {
			placeholders = append(placeholders, fmt.Sprintf("$%v::uuid", len(countValues)+1))
			countValues = append(countValues, cameraID)
		}

		where = append(where, fmt.Sprintf("camera_id IN (%v)", strings.Join(placeholders, ", ")))
	}

	if q.MinScore > 0 {
		where = append(where, fmt.Sprintf("top_score >= $%v", len(countValues)+1))
		countValues = append(countValues, q.MinScore)
	}

	shared := "FROM public.v_audio_event_hour WHERE " + strings.Join(where, " AND ")

	selectSQL := "SELECT bucket, camera_id::text AS camera_id, event_count, top_score, peak_prominence_db, top_kind, first_seen_at, last_seen_at " +
		shared +
		fmt.Sprintf(" ORDER BY bucket ASC, camera_id ASC LIMIT $%v OFFSET $%v", len(countValues)+1, len(countValues)+2)

	countSQL := "SELECT count(*) " + shared

	selectValues := append(append([]any{}, countValues...), q.Limit, q.Offset)

	return selectSQL, countSQL, selectValues, countValues
}

func parseTime(values url.Values, key string) (time.Time, error) {
	raw := strings.TrimSpace(values.Get(key))
	if raw == "" {
		return time.Time{}, fmt.Errorf("%v is required (RFC3339)", key)
	}

	parsed, err := time.Parse(time.RFC3339, raw)
	if err != nil {
		return time.Time{}, fmt.Errorf("failed to parse %v %q; %v", key, raw, err)
	}

	return parsed.UTC(), nil
}

func parseFloat(values url.Values, key string, defaultValue float64) (float64, error) {
	raw := strings.TrimSpace(values.Get(key))
	if raw == "" {
		return defaultValue, nil
	}

	parsed, err := strconv.ParseFloat(raw, 64)
	if err != nil {
		return 0, fmt.Errorf("failed to parse %v %q; %v", key, raw, err)
	}

	return parsed, nil
}

func parseInt(values url.Values, key string, defaultValue int64) (int64, error) {
	raw := strings.TrimSpace(values.Get(key))
	if raw == "" {
		return defaultValue, nil
	}

	parsed, err := strconv.ParseInt(raw, 10, 64)
	if err != nil {
		return 0, fmt.Errorf("failed to parse %v %q; %v", key, raw, err)
	}

	return parsed, nil
}
