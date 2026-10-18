package internal

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// page mimics an /api/audio-events page: every event carries its expanded
// video row, which is what makes these responses megabytes.
func page(events int) []byte {
	objects := make([]map[string]any, 0, events)
	for i := 0; i < events; i++ {
		objects = append(objects, map[string]any{
			"id":            "a471e10c-ad80-4f87-9600-b6d65738301c",
			"video_id":      "326da5dd-8a6a-4534-9598-effeb03abe61",
			"seen_at":       "2026-10-17T02:24:01.082Z",
			"kind":          "impulsive",
			"score":         0.87,
			"prominence_db": 21.4,
			"duration_ms":   320,
			"spectral":      map[string]any{"tonality": 0.12, "flux": 0.41},
			"video_id_object": map[string]any{
				"id":                "326da5dd-8a6a-4534-9598-effeb03abe61",
				"file_name":         "side-gate-2026-10-17T02:24:01.mp4",
				"status":            "needs tracking",
				"started_at":        "2026-10-17T02:24:01Z",
				"ended_at":          "2026-10-17T02:54:01Z",
				"detection_summary": []map[string]any{{"class_name": "car", "count": 13}},
				"track_summary":     []map[string]any{{"class_name": "car", "count": 3}},
				"audio_summary":     []map[string]any{{"kind": "impulsive", "score": 0.87}},
			},
		})
	}
	body, err := json.Marshal(map[string]any{"status": 200, "success": true, "objects": objects, "total_count": events})
	if err != nil {
		panic(err)
	}
	return body
}

// jsonHandler writes the way djangolang does: Content-Type first, then the
// status, then the encoded body.
func jsonHandler(contentType string, body []byte) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-Type", contentType)
		w.Header().Add("Cache-Control", "max-age=1")
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write(body)
	}
}

func TestJSONCompressMiddlewareCompressesJSON(t *testing.T) {
	body := page(2000)
	handler := JSONCompressMiddleware()(jsonHandler("application/json", body))

	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/audio-events", nil)
	req.Header.Set("Accept-Encoding", "gzip")
	handler.ServeHTTP(rec, req)
	res := rec.Result()

	if got := res.Header.Get("Content-Encoding"); got != "gzip" {
		t.Fatalf("Content-Encoding = %q, want gzip", got)
	}
	if !strings.Contains(res.Header.Get("Vary"), "Accept-Encoding") {
		t.Fatalf("Vary = %q, want Accept-Encoding", res.Header.Get("Vary"))
	}

	reader, err := gzip.NewReader(res.Body)
	if err != nil {
		t.Fatalf("gzip.NewReader: %v", err)
	}
	got, err := io.ReadAll(reader)
	if err != nil {
		t.Fatalf("read gzip body: %v", err)
	}
	if !bytes.Equal(got, body) {
		t.Fatalf("decompressed body is %d bytes, want the original %d bytes", len(got), len(body))
	}

	compressed := rec.Body.Len()
	if compressed*2 > len(body) {
		t.Fatalf("compressed %d bytes is not under half of %d", compressed, len(body))
	}
	t.Logf("json %d bytes -> gzip %d bytes (%.1f%%)", len(body), compressed, float64(compressed)*100/float64(len(body)))
}

func TestJSONCompressMiddlewareLeavesClientsWithoutAcceptEncodingAlone(t *testing.T) {
	body := page(200)
	handler := JSONCompressMiddleware()(jsonHandler("application/json", body))

	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/api/audio-events", nil))
	res := rec.Result()

	if got := res.Header.Get("Content-Encoding"); got != "" {
		t.Fatalf("Content-Encoding = %q, want no encoding for a client that cannot decode", got)
	}
	if !bytes.Equal(rec.Body.Bytes(), body) {
		t.Fatalf("body was rewritten: %d bytes, want %d", rec.Body.Len(), len(body))
	}
}

func TestJSONCompressMiddlewareSkipsOtherContentTypes(t *testing.T) {
	body := []byte(strings.Repeat("media stream status\n", 5000))
	handler := JSONCompressMiddleware()(jsonHandler("text/plain", body))

	rec := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/api/custom/health", nil)
	req.Header.Set("Accept-Encoding", "gzip")
	handler.ServeHTTP(rec, req)

	if got := rec.Result().Header.Get("Content-Encoding"); got != "" {
		t.Fatalf("Content-Encoding = %q, want gzip reserved for JSON", got)
	}
}
