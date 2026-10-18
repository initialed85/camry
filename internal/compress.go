package internal

import (
	"compress/flate"
	"net/http"

	"github.com/go-chi/chi/v5/middleware"
)

// JSONCompressMiddleware compresses JSON responses for clients that advertise
// Accept-Encoding. API list pages are megabytes of JSON and Traefik routes
// /api straight to this binary, so nothing upstream of the browser compresses
// them.
func JSONCompressMiddleware() func(http.Handler) http.Handler {
	return middleware.Compress(flate.DefaultCompression, "application/json")
}
