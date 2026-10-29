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
// Authorization header (Video.js <video> and <img> thumbnail requests under /media). It belongs to
// the legacy static-token mode only: behind the SSO gate the media requests are gated by Traefik
// with the session cookie, so nothing here needs an app-level cookie any more.
const CookieName = "camry_token"

// Headers oauth2-proxy returns to Traefik and Traefik injects into the upstream request. Traefik
// deletes any client-supplied copies before injecting them, so a header that arrives is a verified
// one - it cannot be forged by the caller.
const (
	EmailHeader     = "X-Auth-Request-Email"
	UserHeader      = "X-Auth-Request-User"
	PreferredHeader = "X-Auth-Request-Preferred-Username"
)

// Identity is who the gate says the caller is.
type Identity struct {
	Email     string
	User      string
	Preferred string
}

// Auth holds the credential model for this deployment.
//
// Mode comes from AUTH_MODE:
//
//   - "sso"   - Traefik's auth-oauth2 forwardAuth (see home-ops clusters/{dev,prod}/applications/auth)
//     is the gate. The API only has to believe the identity headers Traefik injects.
//   - unset   - legacy. If both PASSWORD and TOKEN are set, this is the old fixed-password-for-fixed-
//     token exchange; otherwise auth is off so the local Compose environment keeps working.
//
// AllowedEmails is optional on top of either mode and narrows access to a per-app list. It is not
// where the family lives: that list belongs to oauth2-proxy (ConfigMap oauth2-proxy-emails plus
// Google's Test users) and is enforced before the request ever gets here. Keeping a second copy per
// app invites drift; it exists only for the case where one app must be stricter than the cluster.
type Auth struct {
	Mode          string
	Password      string
	Token         string
	AllowedEmails []string
}

func New() (*Auth, error) {
	mode, err := internal.GetEnvironment[string]("AUTH_MODE", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	password, err := internal.GetEnvironment[string]("PASSWORD", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	token, err := internal.GetEnvironment[string]("TOKEN", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	allowedRaw, err := internal.GetEnvironment[string]("ALLOWED_EMAILS", false, internal.Ptr(""))
	if err != nil {
		return nil, err
	}

	return &Auth{
		Mode:          strings.ToLower(strings.TrimSpace(mode)),
		Password:      password,
		Token:         token,
		AllowedEmails: parseEmails(allowedRaw),
	}, nil
}

func parseEmails(raw string) []string {
	out := make([]string, 0)

	seen := make(map[string]struct{})

	for _, part := range strings.Split(raw, ",") {
		email := strings.ToLower(strings.TrimSpace(part))
		if email == "" {
			continue
		}

		if _, ok := seen[email]; ok {
			continue
		}

		seen[email] = struct{}{}
		out = append(out, email)
	}

	return out
}

// SSOEnabled reports that this deployment sits behind the cluster gate.
func (a *Auth) SSOEnabled() bool {
	return a.Mode == "sso"
}

// Enabled reports the legacy static-token mode. Kept so the Compose environment (and any deployment
// that has not been gated yet) keeps working unchanged.
func (a *Auth) Enabled() bool {
	return !a.SSOEnabled() && a.Password != "" && a.Token != ""
}

// Identity reads the verified identity headers and applies the optional per-app allowlist.
func (a *Auth) Identity(req *http.Request) (Identity, bool) {
	email := strings.ToLower(strings.TrimSpace(req.Header.Get(EmailHeader)))
	if email == "" {
		return Identity{}, false
	}

	if len(a.AllowedEmails) > 0 {
		found := false

		for _, allowed := range a.AllowedEmails {
			if subtle.ConstantTimeCompare([]byte(allowed), []byte(email)) == 1 {
				found = true
				break
			}
		}

		if !found {
			return Identity{}, false
		}
	}

	return Identity{
		Email:     email,
		User:      strings.TrimSpace(req.Header.Get(UserHeader)),
		Preferred: strings.TrimSpace(req.Header.Get(PreferredHeader)),
	}, true
}

// cameThroughGateway distinguishes traffic Traefik proxied from traffic that reached the pod directly.
//
// Camry's own workloads (segment-producer, object-detector, audio-detector, stream-producer, the
// frontend's health probes) call http://api:7070 in-cluster and carry no identity headers - they are
// trusted by network position, exactly as they were before the gate existed. Traefik sets
// X-Forwarded-Host on everything it proxies, so "has X-Forwarded-Host" is the discriminator: public
// traffic must show a verified identity, cluster-internal traffic is not re-checked.
//
// The point of checking at all is to fail closed. If an Ingress is ever edited so that it loses
// auth-oauth2@kubernetescrd (or a new Ingress is added in front of the API without the gate), the
// API answers 401 instead of quietly becoming public.
func cameThroughGateway(req *http.Request) bool {
	return strings.TrimSpace(req.Header.Get("X-Forwarded-Host")) != ""
}

// isAuthEndpoint exempts the two custom auth endpoints from Middleware: /custom/login is how the
// legacy mode obtains the token it later validates, and /custom/auth is the "who am I" probe the
// frontend calls before it has anything to present. Both answer their own, clearer, errors.
func isAuthEndpoint(req *http.Request) bool {
	path := strings.TrimRight(req.URL.Path, "/")

	return strings.HasSuffix(path, "/custom/login") || strings.HasSuffix(path, "/custom/auth")
}

// Middleware enforces the gate on every API route. It is mounted innermost of the djangolang
// middlewares and ahead of the response cache, so a cache hit is only ever replayed to a caller that
// already passed it.
func (a *Auth) Middleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {
		if isAuthEndpoint(req) || !cameThroughGateway(req) {
			next.ServeHTTP(w, req)
			return
		}

		if a.SSOEnabled() {
			if _, ok := a.Identity(req); !ok {
				handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("no verified identity: this deployment sits behind the cluster sign-in gate, and the request reached the API through Traefik without %s", EmailHeader))
				return
			}
		}

		if a.Enabled() && TokenFromRequest(req) == "" {
			handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("missing token (Authorization: Bearer or the %v cookie)", CookieName))
			return
		}

		next.ServeHTTP(w, req)
	})
}

// Routes mounts the auth endpoints under djangolang's /custom, i.e. /api/custom/login and
// /api/custom/auth. Custom routes are not emitted into the generated OpenAPI schema.
func (a *Auth) Routes(r chi.Router) {
	r.Post("/login", a.HandleLogin)
	r.Get("/auth", a.HandleAuth)
}

// HandleLogin exchanges the fixed password for the fixed token (legacy mode only).
func (a *Auth) HandleLogin(w http.ResponseWriter, req *http.Request) {
	if a.SSOEnabled() {
		handleErrorResponse(w, http.StatusGone, fmt.Errorf("password login is retired: sign in through the cluster sign-in host (https://auth.dev.initialed85.cc) and this request will arrive with a verified identity"))
		return
	}

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

// HandleAuth answers "who is this request?". The frontend calls it on load to decide whether to show
// the app or the sign-in card; it used to be Traefik's forwardAuth target, which the cluster gate
// replaced.
func (a *Auth) HandleAuth(w http.ResponseWriter, req *http.Request) {
	if a.SSOEnabled() {
		identity, ok := a.Identity(req)
		if !ok {
			handleErrorResponse(w, http.StatusUnauthorized, fmt.Errorf("not authorised (no %s header, or the address is not in ALLOWED_EMAILS)", EmailHeader))
			return
		}

		handleResponse(w, http.StatusOK, map[string]any{
			"auth_required": true,
			"authenticated": true,
			"email":         identity.Email,
			"user":          identity.User,
		})

		return
	}

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
