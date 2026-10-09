package auth

import (
	"crypto/subtle"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/go-chi/chi/v5"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/djangolang/pkg/server"
)

// CookieName is the cookie the browser sends automatically for requests that cannot carry an
// Authorization header (Video.js <video> and <img> thumbnail requests under /media). Traefik's
// forwardAuth middleware only inspects headers, so the token has to arrive as a cookie for those.
const CookieName = "camry_token"

// Auth holds the single fixed password and the fixed token it hands out. Both are optional: when
// either is empty, auth is treated as disabled so the local Compose environment keeps working.
type Auth struct {
	Password string
	Token    string
}

func New() (*Auth, error) {
	password, err := internal.GetEnvironment[string]("PASSWORD", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	token, err := internal.GetEnvironment[string]("TOKEN", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	return &Auth{
		Password: password,
		Token:    token,
	}, nil
}

func (a *Auth) Enabled() bool {
	return a.Password != "" && a.Token != ""
}

// Routes mounts the auth endpoints under djangolang's /custom, i.e. /api/custom/login and
// /api/custom/auth. Custom routes are not emitted into the generated OpenAPI schema.
func (a *Auth) Routes(r chi.Router) {
	r.Post("/login", a.HandleLogin)
	r.Get("/auth", a.HandleAuth)
}

// HandleLogin exchanges the fixed password for the fixed token.
func (a *Auth) HandleLogin(w http.ResponseWriter, req *http.Request) {
	if !a.Enabled() {
		handleErrorResponse(w, http.StatusServiceUnavailable, fmt.Errorf("auth is not configured (PASSWORD and TOKEN must both be set)"))
		return
	}

	var item struct {
		Password string `json:"password"`
	}

	err := json.NewDecoder(req.Body).Decode(&item)
	if err != nil {
		handleErrorResponse(w, http.StatusBadRequest, fmt.Errorf("failed to decode request body; %v", err))
		return
	}

	if subtle.ConstantTimeCompare([]byte(a.Password), []byte(strings.TrimSpace(item.Password))) != 1 {
		handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("invalid password"))
		return
	}

	// the browser can't attach a bearer token to media requests, so hand the token back as a
	// cookie as well as in the response body
	http.SetCookie(w, &http.Cookie{
		Name:     CookieName,
		Value:    a.Token,
		Path:     "/",
		HttpOnly: true,
		Secure:   isSecureRequest(req),
		SameSite: http.SameSiteLaxMode,
	})

	handleResponse(w, http.StatusOK, map[string]any{
		"token": a.Token,
	})
}

// HandleAuth is Traefik's forwardAuth target: it answers whether the request carries a valid token.
func (a *Auth) HandleAuth(w http.ResponseWriter, req *http.Request) {
	if !a.Enabled() {
		handleResponse(w, http.StatusOK, map[string]any{
			"auth_required": false,
			"authenticated": true,
		})
		return
	}

	token := TokenFromRequest(req)

	if token == "" {
		handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("missing token (Authorization: Bearer or the %v cookie)", CookieName))
		return
	}

	if subtle.ConstantTimeCompare([]byte(a.Token), []byte(token)) != 1 {
		handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("invalid token"))
		return
	}

	handleResponse(w, http.StatusOK, map[string]any{
		"auth_required": true,
		"authenticated": true,
	})
}

// TokenFromRequest reads the token from the Authorization header or the cookie.
func TokenFromRequest(req *http.Request) string {
	authorization := strings.TrimSpace(req.Header.Get("Authorization"))

	if authorization != "" {
		parts := strings.Fields(authorization)

		if len(parts) == 2 && strings.EqualFold(parts[0], "bearer") {
			return strings.TrimSpace(parts[1])
		}
	}

	cookie, err := req.Cookie(CookieName)
	if err != nil {
		return ""
	}

	return strings.TrimSpace(cookie.Value)
}

func isSecureRequest(req *http.Request) bool {
	return req.TLS != nil || strings.EqualFold(req.Header.Get("X-Forwarded-Proto"), "https")
}

func handleResponse(w http.ResponseWriter, status int, object any) {
	status, _, b, err := server.GetResponse(status, nil, []*any{&object})
	if err != nil {
		handleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	server.WriteResponse(w, status, b)
}

func handleErrorResponse(w http.ResponseWriter, status int, err error) {
	status, _, b, _ := server.GetResponse(status, err, nil)
	server.WriteResponse(w, status, b)
}
