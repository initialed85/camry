package cache

import (
	"bytes"
	"compress/gzip"
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/go-chi/chi/v5/middleware"
)

const (
	videoFinalized = "37930f54-5c4d-433f-9efa-cae0d89f835b"
	videoActive    = "ee7a8679-7e35-4eb9-93eb-105ee70b36eb"
	videoMissing   = "0c875fc1-4cdf-4850-8f55-b90d266e9607"
)

// fakeStore is the in-package Store the tests run against: it records what was stored with what TTL.
type fakeStore struct {
	values map[string][]byte
	ttls   map[string]time.Duration
	setErr error
}

func newFakeStore() *fakeStore {
	return &fakeStore{values: map[string][]byte{}, ttls: map[string]time.Duration{}}
}

func (s *fakeStore) Get(key string) ([]byte, error) {

	value, found := s.values[key]
	if !found {
		return nil, nil
	}

	return value, nil
}

func (s *fakeStore) Set(key string, value []byte, ttl time.Duration) error {

	if s.setErr != nil {
		return s.setErr
	}

	s.values[key] = value
	s.ttls[key] = ttl

	return nil
}

func (s *fakeStore) storedKeys() []string {

	keys := make([]string, 0, len(s.values))
	for key := range s.values {
		keys = append(keys, key)
	}

	return keys
}

func statuses() StatusFunc {

	return func(ctx context.Context, videoID string) (string, error) {
		switch videoID {
		case videoFinalized:
			return FinalizedStatus, nil
		case videoActive:
			return "detecting", nil
		}

		return "", errors.New("video not found")
	}
}

func testOptions(maxBodyBytes int) Options {

	return Options{
		Paths: map[string]struct{}{
			"/api/detections":              {},
			"/api/tracks":                  {},
			"/api/custom/video-detections": {},
			"/api/custom/video-tracks":     {},
		},
		FinalizedStatus: FinalizedStatus,
		FinalizedTTL:    FinalizedTTL,
		ActiveTTL:       ActiveTTL,
		MaxBodyBytes:    maxBodyBytes,
	}
}

// echoHandler is the downstream handler the middleware is wrapped around. With parts set it answers
// in small writes, the way a streaming handler does.
type echoHandler struct {
	calls  int
	status int
	body   string
	parts  bool
	apply  func(http.ResponseWriter, *http.Request)
}

func (h *echoHandler) ServeHTTP(w http.ResponseWriter, req *http.Request) {

	h.calls++

	if h.apply != nil {
		h.apply(w, req)
	}

	if h.status != 0 {
		w.WriteHeader(h.status)
	}

	if !h.parts {
		if h.body != "" {
			_, _ = w.Write([]byte(h.body))
		}
		return
	}

	for written := 0; written < len(h.body); written += 7 {
		end := written + 7
		if end > len(h.body) {
			end = len(h.body)
		}
		_, _ = w.Write([]byte(h.body[written:end]))
	}
}

func jsonHandler(body string) *echoHandler {

	return &echoHandler{
		status: http.StatusOK,
		body:   body,
		apply: func(w http.ResponseWriter, req *http.Request) {
			w.Header().Set("Content-Type", "application/json")
			w.Header().Set("Cache-Control", "private, max-age=15")
			w.Header().Set("Access-Control-Allow-Origin", "https://camry.dev.initialed85.cc")
		},
	}
}

func request(method, target, acceptEncoding string) *http.Request {

	req := httptest.NewRequest(method, target, nil)
	if acceptEncoding != "" {
		req.Header.Set("Accept-Encoding", acceptEncoding)
	}

	return req
}

func serve(middleware func(http.Handler) http.Handler, handler http.Handler, req *http.Request) *httptest.ResponseRecorder {

	recorder := httptest.NewRecorder()

	middleware(handler).ServeHTTP(recorder, req)

	return recorder
}

func headerOf(recorder *httptest.ResponseRecorder) string {
	return recorder.Header().Get(Header)
}

func TestCacheKeySeparatesVideosEndpointsParamsAndVariants(t *testing.T) {

	opts := testOptions(MaxBodyBytes)

	detectionsA, videoA, variant, ok := cacheKey(request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized+"&limit=1000", ""), opts)
	if !ok || videoA != videoFinalized {
		t.Fatalf("cacheKey: ok = %v, video = %v", ok, videoA)
	}
	if variant != identityVariant {
		t.Fatalf("variant = %v, want %v", variant, identityVariant)
	}

	detectionsB, _, _, _ := cacheKey(request(http.MethodGet, "/api/detections?video_id__eq="+videoActive+"&limit=1000", ""), opts)
	detectionsReordered, _, _, _ := cacheKey(request(http.MethodGet, "/api/detections?limit=1000&video_id__eq="+videoFinalized, ""), opts)
	detectionsGzip, _, gzipVariantName, _ := cacheKey(request(http.MethodGet, "/api/detections?limit=1000&video_id__eq="+videoFinalized, "gzip, deflate, br, zstd"), opts)
	tracksA, _, _, _ := cacheKey(request(http.MethodGet, "/api/tracks?video_id__eq="+videoFinalized, ""), opts)
	customA, _, _, _ := cacheKey(request(http.MethodGet, "/api/custom/video-detections?video_id__eq="+videoFinalized, ""), opts)

	if detectionsA == detectionsB {
		t.Fatalf("two videos share a key: %v", detectionsA)
	}
	if detectionsA != detectionsReordered {
		t.Fatalf("param order split a key: %v vs %v", detectionsA, detectionsReordered)
	}
	if gzipVariantName != gzipVariant {
		t.Fatalf("variant = %v, want %v", gzipVariantName, gzipVariant)
	}
	if detectionsA == detectionsGzip {
		t.Fatalf("gzip and identity share a key: %v", detectionsA)
	}
	if detectionsA == tracksA || detectionsA == customA {
		t.Fatalf("endpoints share a key: %v", detectionsA)
	}
	if !strings.HasPrefix(detectionsA, keyPrefix+":") {
		t.Fatalf("key = %v, want prefix %v", detectionsA, keyPrefix)
	}

	// one entry per distinct request, and the second one is served from the store
	store := newFakeStore()
	handler := jsonHandler(`{"status":200}`)
	middleware := Middleware(store, statuses(), opts)

	for _, target := range []string{
		"/api/detections?video_id__eq=" + videoFinalized + "&limit=1000",
		"/api/detections?video_id__eq=" + videoActive + "&limit=1000",
		"/api/tracks?video_id__eq=" + videoFinalized,
		"/api/custom/video-detections?video_id__eq=" + videoFinalized,
	} {
		if got := headerOf(serve(middleware, handler, request(http.MethodGet, target, ""))); got != statusMiss {
			t.Fatalf("%v: %v = %v, want %v", target, Header, got, statusMiss)
		}
		if got := headerOf(serve(middleware, handler, request(http.MethodGet, target, ""))); got != statusHit {
			t.Fatalf("%v: %v = %v, want %v", target, Header, got, statusHit)
		}
	}

	if keys := store.storedKeys(); len(keys) != 4 {
		t.Fatalf("stored keys = %v, want 4 distinct entries", keys)
	}
	if handler.calls != 4 {
		t.Fatalf("handler calls = %v, want 4: one per distinct request", handler.calls)
	}
}

func TestHitReplaysStatusHeadersAndBody(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":200,"success":true,"objects":[{"score":0.5}],"count":1}`)
	middleware := Middleware(store, statuses(), testOptions(MaxBodyBytes))

	first := serve(middleware, handler, request(http.MethodGet, "/api/custom/video-detections?video_id__eq="+videoFinalized, ""))
	second := serve(middleware, handler, request(http.MethodGet, "/api/custom/video-detections?video_id__eq="+videoFinalized, ""))

	if handler.calls != 1 {
		t.Fatalf("handler calls = %v, want 1: a hit must not reach the handler", handler.calls)
	}

	if second.Code != first.Code {
		t.Fatalf("status = %v, want %v", second.Code, first.Code)
	}
	if second.Body.String() != first.Body.String() {
		t.Fatalf("body = %v, want %v", second.Body.String(), first.Body.String())
	}

	for _, name := range []string{"Content-Type", "Cache-Control", "Access-Control-Allow-Origin"} {
		if got := second.Header().Get(name); got != first.Header().Get(name) {
			t.Fatalf("%v = %v, want %v", name, got, first.Header().Get(name))
		}
	}

	if got := second.Header().Get("Content-Length"); got != strconv.Itoa(len(first.Body.Bytes())) {
		t.Fatalf("Content-Length = %v, want %v", got, len(first.Body.Bytes()))
	}
	if got := headerOf(second); got != statusHit {
		t.Fatalf("%v = %v, want %v", Header, got, statusHit)
	}
	if values := second.Header().Values(Header); len(values) != 1 {
		t.Fatalf("%v values = %v, want 1", Header, values)
	}
}

func TestFinalizedVideoGetsLongTTL(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":200}`)

	serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, ""))

	keys := store.storedKeys()
	if len(keys) != 1 {
		t.Fatalf("stored keys = %v, want 1", keys)
	}
	if ttl := store.ttls[keys[0]]; ttl != FinalizedTTL {
		t.Fatalf("ttl = %v, want %v", ttl, FinalizedTTL)
	}
}

func TestVideoStillInThePipelineGetsShortTTL(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":200}`)

	serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/tracks?video_id__eq="+videoActive, ""))

	keys := store.storedKeys()
	if len(keys) != 1 {
		t.Fatalf("stored keys = %v, want 1", keys)
	}
	if ttl := store.ttls[keys[0]]; ttl != ActiveTTL {
		t.Fatalf("ttl = %v, want %v", ttl, ActiveTTL)
	}
}

func TestNonGETPassesThrough(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":201,"objects":[{"id":"x"}]}`)

	recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodPost, "/api/tracks?video_id__eq="+videoFinalized, ""))

	if handler.calls != 1 {
		t.Fatalf("handler calls = %v, want 1", handler.calls)
	}
	if got := headerOf(recorder); got != statusBypass {
		t.Fatalf("%v = %v, want %v", Header, got, statusBypass)
	}
	if recorder.Body.String() != handler.body {
		t.Fatalf("body = %v, want %v", recorder.Body.String(), handler.body)
	}
	if keys := store.storedKeys(); len(keys) != 0 {
		t.Fatalf("stored keys = %v, want none", keys)
	}
}

func TestNonOKStatusPassesThrough(t *testing.T) {

	for _, status := range []int{http.StatusUnauthorized, http.StatusForbidden, http.StatusNotFound, http.StatusInternalServerError} {

		store := newFakeStore()
		handler := jsonHandler(`{"status":` + strconv.Itoa(status) + `,` + `"success":false}`)
		handler.status = status

		recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, ""))

		if recorder.Code != status {
			t.Fatalf("status = %v, want %v", recorder.Code, status)
		}
		if got := headerOf(recorder); got != statusBypass {
			t.Fatalf("%v = %v, want %v", Header, got, statusBypass)
		}
		if recorder.Body.String() != handler.body {
			t.Fatalf("body = %v, want %v", recorder.Body.String(), handler.body)
		}
		if keys := store.storedKeys(); len(keys) != 0 {
			t.Fatalf("status %v: stored keys = %v, want none", status, keys)
		}
	}
}

func TestQueryWithoutExactlyOneVideoIDPassesThrough(t *testing.T) {

	for _, target := range []string{
		"/api/detections?limit=1000",
		"/api/detections",
		"/api/detections?video_id__eq=",
		"/api/detections?video_id__eq=" + videoFinalized + "," + videoActive,
		"/api/detections?video_id__eq=" + videoFinalized + "&video_id__eq=" + videoActive,
		"/api/detections?video_id__eq=not-a-uuid",
		"/api/detections?video_id__in=" + videoFinalized,
	} {

		store := newFakeStore()
		handler := jsonHandler(`{"status":200}`)

		recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, target, ""))

		if handler.calls != 1 {
			t.Fatalf("%v: handler calls = %v, want 1", target, handler.calls)
		}
		if got := headerOf(recorder); got != statusBypass {
			t.Fatalf("%v: %v = %v, want %v", target, Header, got, statusBypass)
		}
		if recorder.Body.String() != handler.body {
			t.Fatalf("%v: body = %v, want %v", target, recorder.Body.String(), handler.body)
		}
		if keys := store.storedKeys(); len(keys) != 0 {
			t.Fatalf("%v: stored keys = %v, want none", target, keys)
		}
	}
}

func TestPathsOutsideTheScopePassThrough(t *testing.T) {

	for _, target := range []string{
		"/api/videos?video_id__eq=" + videoFinalized,
		"/api/videos/" + videoFinalized,
		"/api/detections/" + videoFinalized,
		"/api/custom/video-timeline?video_id__eq=" + videoFinalized + "&bucket_ms=1000",
		"/api/custom/audio-timeline?started_at__gte=2026-10-19T00:00:00Z",
		"/api/custom/detection-timeline?started_at__gte=2026-10-19T00:00:00Z",
		"/api/custom/auth",
	} {

		store := newFakeStore()
		handler := jsonHandler(`{"status":200}`)

		recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, target, ""))

		if got := headerOf(recorder); got != statusBypass {
			t.Fatalf("%v: %v = %v, want %v", target, Header, got, statusBypass)
		}
		if recorder.Body.String() != handler.body {
			t.Fatalf("%v: body = %v, want %v", target, recorder.Body.String(), handler.body)
		}
		if keys := store.storedKeys(); len(keys) != 0 {
			t.Fatalf("%v: stored keys = %v, want none", target, keys)
		}
	}
}

func TestOversizeResponseIsStreamedAndBypassed(t *testing.T) {

	body := `{"status":200,"objects":[` + strings.Repeat(`{"score":0.5},`, 200) + `{"score":0}]}`

	store := newFakeStore()
	handler := jsonHandler(body)
	handler.parts = true

	recorder := serve(Middleware(store, statuses(), testOptions(100)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, ""))

	if recorder.Body.String() != body {
		t.Fatalf("body length = %v, want %v", len(recorder.Body.String()), len(body))
	}
	if got := headerOf(recorder); got != statusBypass {
		t.Fatalf("%v = %v, want %v", Header, got, statusBypass)
	}
	if keys := store.storedKeys(); len(keys) != 0 {
		t.Fatalf("stored keys = %v, want none", keys)
	}
}

func TestGzipAndIdentityNeverCrossServe(t *testing.T) {

	store := newFakeStore()
	handler := &echoHandler{
		status: http.StatusOK,
		body:   `{"status":200}`,
		apply: func(w http.ResponseWriter, req *http.Request) {
			w.Header().Set("Content-Type", "application/json")
			// the response is encoded only when the request asked for gzip, as compress does
			if strings.Contains(req.Header.Get("Accept-Encoding"), gzipVariant) {
				w.Header().Set("Content-Encoding", gzipVariant)
			}
		},
	}

	middleware := Middleware(store, statuses(), testOptions(MaxBodyBytes))

	gzipRequest := request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, "gzip, deflate, br, zstd")

	first := serve(middleware, handler, gzipRequest)
	if got := headerOf(first); got != statusMiss {
		t.Fatalf("gzip first: %v = %v, want %v", Header, got, statusMiss)
	}
	if got := first.Header().Get("Content-Encoding"); got != gzipVariant {
		t.Fatalf("gzip first: Content-Encoding = %v, want %v", got, gzipVariant)
	}

	identity := serve(middleware, handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, ""))
	if handler.calls != 2 {
		t.Fatalf("handler calls = %v, want 2: the identity request must not be served the gzip entry", handler.calls)
	}
	if got := headerOf(identity); got != statusMiss {
		t.Fatalf("identity: %v = %v, want %v", Header, got, statusMiss)
	}
	if got := identity.Header().Get("Content-Encoding"); got != "" {
		t.Fatalf("identity: Content-Encoding = %v, want none", got)
	}

	second := serve(middleware, handler, gzipRequest)
	if got := headerOf(second); got != statusHit {
		t.Fatalf("gzip second: %v = %v, want %v", Header, got, statusHit)
	}
	if got := second.Header().Get("Content-Encoding"); got != gzipVariant {
		t.Fatalf("gzip second: Content-Encoding = %v, want %v", got, gzipVariant)
	}
	if handler.calls != 2 {
		t.Fatalf("handler calls = %v, want 2: the hit must not reach the handler", handler.calls)
	}
	if keys := store.storedKeys(); len(keys) != 2 {
		t.Fatalf("stored keys = %v, want 2 (gzip and identity)", keys)
	}
}

func TestUnexpectedContentEncodingPassesThrough(t *testing.T) {

	// chi picks deflate for a client advertising only deflate; that key is the identity variant, so
	// a deflate body must not be stored under it.
	store := newFakeStore()
	handler := &echoHandler{
		status: http.StatusOK,
		body:   `{"status":200}`,
		apply: func(w http.ResponseWriter, req *http.Request) {
			w.Header().Set("Content-Type", "application/json")
			w.Header().Set("Content-Encoding", "deflate")
		},
	}

	recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, "deflate"))

	if got := headerOf(recorder); got != statusBypass {
		t.Fatalf("%v = %v, want %v", Header, got, statusBypass)
	}
	if recorder.Body.String() != handler.body {
		t.Fatalf("body = %v, want %v", recorder.Body.String(), handler.body)
	}
	if keys := store.storedKeys(); len(keys) != 0 {
		t.Fatalf("stored keys = %v, want none", keys)
	}
}

func TestStatusLookupFailureServesTheResponse(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":200,"objects":[]}`)

	recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoMissing, ""))

	if recorder.Code != http.StatusOK {
		t.Fatalf("status = %v, want 200", recorder.Code)
	}
	if recorder.Body.String() != handler.body {
		t.Fatalf("body = %v, want %v", recorder.Body.String(), handler.body)
	}
	if got := headerOf(recorder); got != statusBypass {
		t.Fatalf("%v = %v, want %v", Header, got, statusBypass)
	}
	if keys := store.storedKeys(); len(keys) != 0 {
		t.Fatalf("stored keys = %v, want none", keys)
	}
}

func TestStoreFailureStillServesTheResponse(t *testing.T) {

	store := newFakeStore()
	store.setErr = errors.New("redis down")
	handler := jsonHandler(`{"status":200,"objects":[{"score":0.5}]}`)

	recorder := serve(Middleware(store, statuses(), testOptions(MaxBodyBytes)), handler, request(http.MethodGet, "/api/detections?video_id__eq="+videoFinalized, ""))

	if recorder.Code != http.StatusOK {
		t.Fatalf("status = %v, want 200", recorder.Code)
	}
	if recorder.Body.String() != handler.body {
		t.Fatalf("body = %v, want %v", recorder.Body.String(), handler.body)
	}
	if keys := store.storedKeys(); len(keys) != 0 {
		t.Fatalf("stored keys = %v, want none", keys)
	}
}

func TestTrailingSlashIsNormalised(t *testing.T) {

	store := newFakeStore()
	handler := jsonHandler(`{"status":200}`)
	middleware := Middleware(store, statuses(), testOptions(MaxBodyBytes))

	plain := serve(middleware, handler, request(http.MethodGet, "/api/custom/video-tracks?video_id__eq="+videoFinalized, ""))
	slash := serve(middleware, handler, request(http.MethodGet, "/api/custom/video-tracks/?video_id__eq="+videoFinalized, ""))

	if got := headerOf(plain); got != statusMiss {
		t.Fatalf("plain: %v = %v, want %v", Header, got, statusMiss)
	}
	if got := headerOf(slash); got != statusHit {
		t.Fatalf("trailing slash: %v = %v, want %v", Header, got, statusHit)
	}
	if handler.calls != 1 {
		t.Fatalf("handler calls = %v, want 1", handler.calls)
	}
}

func TestEncodeDecodeRoundTrip(t *testing.T) {

	headers := http.Header{}
	headers.Set("Content-Type", "application/json")
	headers.Set("Content-Encoding", gzipVariant)
	headers.Add("Vary", "Accept-Encoding")
	headers.Set("Cache-Control", "max-age=1")

	body := strings.Repeat("payload,", 1000)
	cached := entry{Status: http.StatusOK, Headers: headers, Body: []byte(body)}

	value := encode(cached)

	decoded, ok := decode(value)
	if !ok {
		t.Fatalf("decode: failed")
	}
	if decoded.Status != http.StatusOK {
		t.Fatalf("status = %v, want 200", decoded.Status)
	}
	for name, want := range map[string]string{"Content-Type": "application/json", "Content-Encoding": gzipVariant, "Cache-Control": "max-age=1"} {
		if got := decoded.Headers.Get(name); got != want {
			t.Fatalf("%v = %v, want %v", name, got, want)
		}
	}
	if values := decoded.Headers.Values("Vary"); len(values) != 1 || values[0] != "Accept-Encoding" {
		t.Fatalf("Vary = %v, want [Accept-Encoding]", values)
	}
	if string(decoded.Body) != body {
		t.Fatalf("body length = %v, want %v", len(decoded.Body), len(body))
	}

	meta, err := json.Marshal(entryMeta{Status: cached.Status, Headers: cached.Headers})
	if err != nil {
		t.Fatalf("marshal meta: %v", err)
	}
	if len(value) != 4+len(meta)+len(body) {
		t.Fatalf("encoded length = %v, want 4 + meta %v + body %v", len(value), len(meta), len(body))
	}
}

func TestDecodeRejectsGarbage(t *testing.T) {

	for _, value := range [][]byte{nil, []byte("ab"), {0, 0, 0, 8, 1, 2}, {0, 0, 0, 4, '{'}} {
		if _, ok := decode(value); ok {
			t.Fatalf("decode(%v): accepted garbage", value)
		}
	}
}

// TestCompressChainStoresAndReplaysGzipBytes runs the middleware over chi's real compressor, the
// way cmd/api wires them, and checks that what lands in the store is the gzip body the client got.
func TestCompressChainStoresAndReplaysGzipBytes(t *testing.T) {

	body := `{"status":200,"objects":[` + strings.Repeat(`{"score":0.5},`, 500) + `{"score":0}]}`

	store := newFakeStore()
	handler := jsonHandler(body)

	chain := Middleware(store, statuses(), testOptions(MaxBodyBytes))(middleware.Compress(-1, "application/json")(handler))

	serve := func(acceptEncoding string) *httptest.ResponseRecorder {

		recorder := httptest.NewRecorder()

		chain.ServeHTTP(recorder, request(http.MethodGet, "/api/custom/video-detections?video_id__eq="+videoFinalized, acceptEncoding))

		return recorder
	}

	gzipFirst := serve("gzip, deflate, br, zstd")

	if got := headerOf(gzipFirst); got != statusMiss {
		t.Fatalf("gzip first: %v = %v, want %v", Header, got, statusMiss)
	}
	if got := gzipFirst.Header().Get("Content-Encoding"); got != gzipVariant {
		t.Fatalf("gzip first: Content-Encoding = %v, want %v", got, gzipVariant)
	}
	if gzipFirst.Body.Len() >= len(body) {
		t.Fatalf("gzip first: served %v bytes for a %v byte body: not compressed", gzipFirst.Body.Len(), len(body))
	}

	keys := store.storedKeys()
	if len(keys) != 1 {
		t.Fatalf("stored keys = %v, want 1", keys)
	}

	cached, ok := decode(store.values[keys[0]])
	if !ok {
		t.Fatalf("decode: failed")
	}
	if cached.Headers.Get("Content-Encoding") != gzipVariant {
		t.Fatalf("stored Content-Encoding = %v, want %v", cached.Headers.Get("Content-Encoding"), gzipVariant)
	}
	if cached.Headers.Get("Vary") != "Accept-Encoding" {
		t.Fatalf("stored Vary = %v, want Accept-Encoding", cached.Headers.Get("Vary"))
	}
	if len(cached.Body) != gzipFirst.Body.Len() {
		t.Fatalf("stored body = %v bytes, want the %v bytes on the wire", len(cached.Body), gzipFirst.Body.Len())
	}

	gz, err := gzip.NewReader(bytes.NewReader(cached.Body))
	if err != nil {
		t.Fatalf("gzip reader: %v", err)
	}
	decompressed, err := io.ReadAll(gz)
	if err != nil {
		t.Fatalf("gzip read: %v", err)
	}
	if string(decompressed) != body {
		t.Fatalf("stored gzip body decompresses to %v bytes, want %v", len(decompressed), len(body))
	}

	gzipSecond := serve("gzip, deflate, br, zstd")

	if got := headerOf(gzipSecond); got != statusHit {
		t.Fatalf("gzip second: %v = %v, want %v", Header, got, statusHit)
	}
	if handler.calls != 1 {
		t.Fatalf("handler calls = %v, want 1: a hit must not reach the handler, compressor included", handler.calls)
	}
	if gzipSecond.Body.String() != gzipFirst.Body.String() {
		t.Fatalf("gzip second: replayed %v bytes, want the recorded %v bytes", gzipSecond.Body.Len(), gzipFirst.Body.Len())
	}
	if got := gzipSecond.Header().Get("Content-Encoding"); got != gzipVariant {
		t.Fatalf("gzip second: Content-Encoding = %v, want %v", got, gzipVariant)
	}
	if got := gzipSecond.Header().Get("Vary"); got != "Accept-Encoding" {
		t.Fatalf("gzip second: Vary = %v, want Accept-Encoding", got)
	}

	identity := serve("")

	if got := headerOf(identity); got != statusMiss {
		t.Fatalf("identity: %v = %v, want %v", Header, got, statusMiss)
	}
	if handler.calls != 2 {
		t.Fatalf("handler calls = %v, want 2: the identity request must be served its own encoding", handler.calls)
	}
	if got := identity.Header().Get("Content-Encoding"); got != "" {
		t.Fatalf("identity: Content-Encoding = %v, want none", got)
	}
	if identity.Body.String() != body {
		t.Fatalf("identity: body = %v bytes, want %v", identity.Body.Len(), len(body))
	}
	if keys := store.storedKeys(); len(keys) != 2 {
		t.Fatalf("stored keys = %v, want 2 (gzip and identity)", keys)
	}
}
