// Package cache replays the per-video detection and track lists out of Redis, so reopening a video
// modal does not re-serialise thousands of detection rows.
//
// djangolang caches list responses too (pkg/server/cache.go, key `<table>:<primaryKey>:<params>`),
// and the CDC stream deletes them: handleChange SCANs `<table>:*` for the changed table, its FK
// tables and its ReferencedByColumns tables, and DELs every match. Under detection write traffic the
// `detection:*` / `track:*` / `video:*` entries never outlive a request. These entries live under
// `camry:list:v1:`; no table is named `camry`, so the invalidator never matches them.
package cache

import (
	"context"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"log"
	"maps"
	"net/http"
	"slices"
	"strconv"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/initialed85/djangolang/pkg/config"
)

const (
	// Header reports what this middleware did with the request: hit, miss or bypass.
	Header = "X-Camry-Cache"

	// FinalizedStatus is the terminal video status: nothing writes detections for it, so its
	// projections are immutable and worth a long TTL.
	FinalizedStatus = "needs tracking"

	// FinalizedTTL is how long a finalized video's response is kept.
	FinalizedTTL = time.Hour * 24

	// ActiveTTL is how long anything else is kept. The object detector posts its tracks and then
	// reads them back with GET /api/tracks?video_id__eq=, and nothing invalidates these keys, so this
	// is also the window a non-finalized video's list can be served stale.
	ActiveTTL = time.Second * 60

	// MaxBodyBytes caps one cache entry. Redis runs with noeviction, and a depth-1 generated list of
	// a 2,235-detection recording is 14.7 MB: those are served from the database every time and
	// logged as oversize bypasses.
	MaxBodyBytes = 4 << 20

	// keyPrefix is the namespace the CDC invalidator cannot reach.
	keyPrefix = "camry:list:v1"

	statusHit    = "hit"
	statusMiss   = "miss"
	statusBypass = "bypass"

	gzipVariant     = "gzip"
	identityVariant = "identity"
)

// Store holds encoded entries.
type Store interface {
	Get(key string) ([]byte, error)
	Set(key string, value []byte, ttl time.Duration) error
}

// StatusFunc reports a video row's status. The TTL gate is the only database read this middleware
// makes, and it makes it on a miss.
type StatusFunc func(ctx context.Context, videoID string) (string, error)

// Options scopes the middleware.
type Options struct {
	Paths           map[string]struct{}
	FinalizedStatus string
	FinalizedTTL    time.Duration
	ActiveTTL       time.Duration
	MaxBodyBytes    int
}

// DefaultOptions caches the two generated per-video lists and the two per-video projections the
// modal fetches. The timeline endpoints stay out: they answer over a moving window and say so with
// Cache-Control: no-store.
func DefaultOptions() Options {

	apiRoot := "/" + strings.Trim(config.APIRoot(), "/")

	return Options{
		Paths: map[string]struct{}{
			apiRoot + "/detections":              {},
			apiRoot + "/tracks":                  {},
			apiRoot + "/custom/video-detections": {},
			apiRoot + "/custom/video-tracks":     {},
		},
		FinalizedStatus: FinalizedStatus,
		FinalizedTTL:    FinalizedTTL,
		ActiveTTL:       ActiveTTL,
		MaxBodyBytes:    MaxBodyBytes,
	}
}

// Middleware serves cacheable per-video list responses from the store, and records them on a miss.
func Middleware(store Store, statusOf StatusFunc, opts Options) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {

			key, videoID, variant, eligible := cacheKey(req, opts)
			if !eligible {
				w.Header().Set(Header, statusBypass)
				next.ServeHTTP(w, req)
				return
			}

			cached, found := lookup(store, key)
			if found {
				replay(w, cached)
				return
			}

			recorder := newRecorder(w, opts.MaxBodyBytes)

			next.ServeHTTP(recorder, req)

			entry, ttl, status := recorder.entry(req.Context(), statusOf, videoID, opts, variant)

			w.Header().Set(Header, status)

			recorder.finish()

			if status != statusMiss {
				if recorder.overflow {
					log.Printf("cache: bypass: key: %v: bytes: %v: over max body bytes: %v", key, recorder.recorded, opts.MaxBodyBytes)
				}
				return
			}

			value := encode(entry)
			if len(value) == 0 {
				log.Printf("cache: warning: failed to encode entry for key: %v", key)
				return
			}

			err := store.Set(key, value, ttl)
			if err != nil {
				log.Printf("cache: warning: failed to store key: %v: %v", key, err.Error())
			}
		})
	}
}

// cacheKey builds the store key, and reports whether the request is cacheable: a GET on a scoped
// path whose query carries exactly one video_id__eq, and that value is a single UUID. The key covers
// the path, the video, the whole sorted query and the encoding variant, so two videos, two
// endpoints, two param orders, and gzip versus identity never share an entry.
func cacheKey(req *http.Request, opts Options) (string, string, string, bool) {

	if req.Method != http.MethodGet {
		return "", "", "", false
	}

	path := "/" + strings.Trim(req.URL.Path, "/")

	if _, ok := opts.Paths[path]; !ok {
		return "", "", "", false
	}

	values, found := req.URL.Query()["video_id__eq"]
	if !found || len(values) != 1 {
		return "", "", "", false
	}

	parts := strings.Split(values[0], ",")
	if len(parts) != 1 {
		return "", "", "", false
	}

	videoID, err := uuid.Parse(strings.TrimSpace(parts[0]))
	if err != nil {
		return "", "", "", false
	}

	variant := requestVariant(req)

	query := req.URL.Query()

	identities := make([]string, 0, len(query))
	for _, name := range slices.Sorted(maps.Keys(query)) {
		for _, value := range query[name] {
			identities = append(identities, name+"="+value)
		}
	}

	sum := sha256.Sum256([]byte(strings.Join([]string{path, videoID.String(), strings.Join(identities, "&"), variant}, "\x00")))

	return keyPrefix + ":" + path + ":" + videoID.String() + ":" + hex.EncodeToString(sum[:]), videoID.String(), variant, true
}

// requestVariant is the encoding this request will be served with, following chi's rule: gzip is the
// highest-precedence encoder middleware.Compress registers, and it is matched by substring over the
// comma-split lowercased Accept-Encoding.
func requestVariant(req *http.Request) string {

	for _, accepted := range strings.Split(strings.ToLower(req.Header.Get("Accept-Encoding")), ",") {
		if strings.Contains(strings.TrimSpace(accepted), gzipVariant) {
			return gzipVariant
		}
	}

	return identityVariant
}

// lookup reads a stored entry. A store error is a miss: the request is served from the database.
func lookup(store Store, key string) (entry, bool) {

	value, err := store.Get(key)
	if err != nil {
		return entry{}, false
	}

	if len(value) == 0 {
		return entry{}, false
	}

	cached, ok := decode(value)
	if !ok {
		return entry{}, false
	}

	return cached, true
}

// replay writes a stored response exactly as it was recorded.
func replay(w http.ResponseWriter, cached entry) {

	headers := w.Header()

	for name, values := range cached.Headers {
		for _, value := range values {
			headers.Add(name, value)
		}
	}

	headers.Set(Header, statusHit)
	headers.Set("Content-Length", strconv.Itoa(len(cached.Body)))

	w.WriteHeader(cached.Status)

	_, err := w.Write(cached.Body)
	if err != nil {
		log.Printf("cache: warning: failed to replay: %v", err.Error())
	}
}

// entry is one cached response.
type entry struct {
	Status  int
	Headers http.Header
	Body    []byte
}

// entryMeta is the entry's small half; its body follows unencoded.
type entryMeta struct {
	Status  int         `json:"s"`
	Headers http.Header `json:"h"`
}

// encode packs an entry as [4-byte meta length][meta JSON][body], so a 4 MB body costs 4 MB.
func encode(cached entry) []byte {

	meta, err := json.Marshal(entryMeta{Status: cached.Status, Headers: cached.Headers})
	if err != nil {
		return nil
	}

	value := make([]byte, 4+len(meta)+len(cached.Body))

	binary.BigEndian.PutUint32(value, uint32(len(meta)))
	copy(value[4:], meta)
	copy(value[4+len(meta):], cached.Body)

	return value
}

func decode(value []byte) (entry, bool) {

	if len(value) < 4 {
		return entry{}, false
	}

	metaLength := int(binary.BigEndian.Uint32(value[:4]))
	if 4+metaLength > len(value) {
		return entry{}, false
	}

	var meta entryMeta

	err := json.Unmarshal(value[4:4+metaLength], &meta)
	if err != nil {
		return entry{}, false
	}

	return entry{Status: meta.Status, Headers: meta.Headers, Body: value[4+metaLength:]}, true
}

// recorder buffers the downstream response so it can be stored and forwarded in one piece. A
// response over the size cap is streamed out as it arrives and is not cached.
type recorder struct {
	http.ResponseWriter
	status   int
	body     []byte
	recorded int
	limit    int
	overflow bool
}

func newRecorder(w http.ResponseWriter, limit int) *recorder {
	return &recorder{ResponseWriter: w, limit: limit}
}

func (r *recorder) WriteHeader(status int) {

	if r.status != 0 {
		return
	}

	r.status = status
}

func (r *recorder) Write(b []byte) (int, error) {

	if r.status == 0 {
		r.status = http.StatusOK
	}

	if r.overflow {
		return r.ResponseWriter.Write(b)
	}

	if r.recorded+len(b) > r.limit {
		r.overflow = true
		r.flush()
		return r.ResponseWriter.Write(b)
	}

	r.body = append(r.body, b...)
	r.recorded += len(b)

	return len(b), nil
}

func (r *recorder) flush() {

	if r.status == 0 {
		r.status = http.StatusOK
	}

	if r.overflow {
		// This is the point where the headers go out, and after that the standard library ignores
		// anything set on them: an oversize response has to carry its verdict here.
		r.Header().Set(Header, statusBypass)
	}

	r.ResponseWriter.WriteHeader(r.status)

	if len(r.body) > 0 {
		_, _ = r.ResponseWriter.Write(r.body)
		r.body = nil
	}
}

func (r *recorder) finish() {

	if r.overflow {
		return
	}

	r.flush()
}

// entry reports whether what the handler produced can be cached, and with what TTL.
func (r *recorder) entry(ctx context.Context, statusOf StatusFunc, videoID string, opts Options, variant string) (entry, time.Duration, string) {

	if r.overflow {
		return entry{}, 0, statusBypass
	}

	if r.status != http.StatusOK {
		return entry{}, 0, statusBypass
	}

	headers := r.Header().Clone()

	if headers.Get("Content-Type") == "" {
		return entry{}, 0, statusBypass
	}

	// The key carries the variant this request asked for. If the response came back with a
	// different encoding than that variant serves, it is not ours to keep: replaying it under this
	// key would serve the wrong bytes.
	if headers.Get("Content-Encoding") != contentEncodingOfVariant(variant) {
		return entry{}, 0, statusBypass
	}

	status, err := statusOf(ctx, videoID)
	if err != nil {
		log.Printf("cache: warning: failed to read status of video: %v: %v", videoID, err.Error())
		return entry{}, 0, statusBypass
	}

	ttl := opts.ActiveTTL
	if status == opts.FinalizedStatus {
		ttl = opts.FinalizedTTL
	}

	// Content-Length is recomputed from the stored body on replay, and this middleware's verdict is
	// per-request.
	headers.Del("Content-Length")
	headers.Del(Header)

	return entry{Status: r.status, Headers: headers, Body: r.body}, ttl, statusMiss
}

func contentEncodingOfVariant(variant string) string {

	if variant == gzipVariant {
		return gzipVariant
	}

	return ""
}

// Log reports what was wired up, once, at startup.
func Log(opts Options) {
	log.Printf("cache: paths: %v: finalized status: %v: finalized ttl: %v: active ttl: %v: max body bytes: %v",
		strings.Join(slices.Sorted(maps.Keys(opts.Paths)), ","), opts.FinalizedStatus, opts.FinalizedTTL, opts.ActiveTTL, opts.MaxBodyBytes)
}
