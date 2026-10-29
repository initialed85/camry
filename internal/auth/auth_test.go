package auth

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/go-chi/chi/v5"
)

func testRouter(a *Auth) chi.Router {
	r := chi.NewRouter()

	// djangolang mounts custom handlers under /{API_ROOT}/custom
	r.Route("/api/custom", func(sub chi.Router) {
		a.Routes(sub)
	})

	return r
}

func postLogin(t *testing.T, a *Auth, password string) *httptest.ResponseRecorder {
	t.Helper()

	req := httptest.NewRequest(http.MethodPost, "/api/custom/login", strings.NewReader(`{"password":"`+password+`"}`))
	w := httptest.NewRecorder()
	testRouter(a).ServeHTTP(w, req)

	return w
}

func getAuth(t *testing.T, a *Auth, headers map[string]string) *httptest.ResponseRecorder {
	t.Helper()

	req := httptest.NewRequest(http.MethodGet, "/api/custom/auth", nil)
	for k, v := range headers {
		req.Header.Set(k, v)
	}

	w := httptest.NewRecorder()
	testRouter(a).ServeHTTP(w, req)

	return w
}

func TestNewReadsEnvironment(t *testing.T) {
	t.Setenv("PASSWORD", "cctv-test-password")
	t.Setenv("TOKEN", "test-token")

	a, err := New()
	if err != nil {
		t.Fatalf("New: %v", err)
	}

	if a.Password != "cctv-test-password" {
		t.Fatalf("password: got %q", a.Password)
	}

	if a.Token != "test-token" {
		t.Fatalf("token: got %q", a.Token)
	}

	if !a.Enabled() {
		t.Fatal("expected auth to be enabled")
	}
}

func TestLogin(t *testing.T) {
	a := &Auth{Password: "cctv-test-password", Token: "test-token"}

	w := postLogin(t, a, "cctv-test-password")

	if w.Code != http.StatusOK {
		t.Fatalf("status: got %d, body: %s", w.Code, w.Body.String())
	}

	var response struct {
		Objects []struct {
			Token string `json:"token"`
		} `json:"objects"`
	}

	if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
		t.Fatalf("decode: %v (body: %s)", err, w.Body.String())
	}

	if len(response.Objects) != 1 || response.Objects[0].Token != "test-token" {
		t.Fatalf("token: got %v (body: %s)", response.Objects, w.Body.String())
	}

	cookie := w.Result().Cookies()
	if len(cookie) != 1 || cookie[0].Name != CookieName || cookie[0].Value != "test-token" {
		t.Fatalf("cookie: got %v", cookie)
	}

	w = postLogin(t, a, "wrong-password")
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("wrong password status: got %d", w.Code)
	}
}

func TestLoginWhenDisabled(t *testing.T) {
	a := &Auth{}

	w := postLogin(t, a, "")
	if w.Code != http.StatusServiceUnavailable {
		t.Fatalf("status: got %d", w.Code)
	}
}

func TestAuth(t *testing.T) {
	a := &Auth{Password: "cctv-test-password", Token: "test-token"}

	w := getAuth(t, a, map[string]string{"Authorization": "Bearer test-token"})
	if w.Code != http.StatusOK {
		t.Fatalf("bearer status: got %d (body: %s)", w.Code, w.Body.String())
	}

	w = getAuth(t, a, map[string]string{"Authorization": "bearer test-token"})
	if w.Code != http.StatusOK {
		t.Fatalf("lowercase bearer status: got %d", w.Code)
	}

	req := httptest.NewRequest(http.MethodGet, "/api/custom/auth", nil)
	req.AddCookie(&http.Cookie{Name: CookieName, Value: "test-token"})

	w = httptest.NewRecorder()
	testRouter(a).ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("cookie status: got %d", w.Code)
	}

	w = getAuth(t, a, map[string]string{"Authorization": "Bearer wrong-token"})
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("wrong token status: got %d", w.Code)
	}

	w = getAuth(t, a, nil)
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("missing token status: got %d", w.Code)
	}
}

func TestAuthWhenDisabled(t *testing.T) {
	a := &Auth{}

	w := getAuth(t, a, nil)
	if w.Code != http.StatusOK {
		t.Fatalf("status: got %d", w.Code)
	}

	var response struct {
		Objects []struct {
			AuthRequired  bool `json:"auth_required"`
			Authenticated bool `json:"authenticated"`
		} `json:"objects"`
	}

	if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
		t.Fatalf("decode: %v (body: %s)", err, w.Body.String())
	}

	if len(response.Objects) != 1 || response.Objects[0].AuthRequired || !response.Objects[0].Authenticated {
		t.Fatalf("response: got %v (body: %s)", response.Objects, w.Body.String())
	}
}

// --- SSO (the cluster gate) ---------------------------------------------------------------------

func gatedRouter(a *Auth) chi.Router {
	r := chi.NewRouter()

	// mirrors cmd/api/main.go: the identity middleware wraps everything, and the custom endpoints sit
	// under it (and are exempted by name inside the middleware).
	r.Use(a.Middleware)
	r.Route("/api", func(api chi.Router) {
		api.Route("/custom", func(sub chi.Router) {
			a.Routes(sub)
		})
		api.Get("/videos", func(w http.ResponseWriter, req *http.Request) {
			w.WriteHeader(http.StatusOK)
			_, _ = w.Write([]byte("videos"))
		})
	})

	return r
}

func do(t *testing.T, a *Auth, method, path string, headers map[string]string) *httptest.ResponseRecorder {
	t.Helper()

	req := httptest.NewRequest(method, path, nil)
	for k, v := range headers {
		req.Header.Set(k, v)
	}

	w := httptest.NewRecorder()
	gatedRouter(a).ServeHTTP(w, req)

	return w
}

// traefikHeaders are what a browser request looks like after Traefik proxied it through
// auth-oauth2@kubernetescrd: X-Forwarded-* set by Traefik, identity headers injected.
func traefikHeaders(email string) map[string]string {
	headers := map[string]string{
		"X-Forwarded-Host":  "camry.dev.initialed85.cc",
		"X-Forwarded-Proto": "https",
	}

	if email != "" {
		headers[EmailHeader] = email
	}

	return headers
}

func TestNewReadsSSOEnvironment(t *testing.T) {
	t.Setenv("AUTH_MODE", "sso")
	t.Setenv("ALLOWED_EMAILS", " Amy.AnnisBrown@gmail.com , initialed85@gmail.com ,amyannisbrown@gmail.com,")

	a, err := New()
	if err != nil {
		t.Fatalf("New: %v", err)
	}

	if !a.SSOEnabled() {
		t.Fatal("expected SSOEnabled")
	}

	if a.Enabled() {
		t.Fatal("SSO mode must not report the legacy token mode as enabled")
	}

	want := []string{"amy.annisbrown@gmail.com", "initialed85@gmail.com", "amyannisbrown@gmail.com"}
	if strings.Join(a.AllowedEmails, ",") != strings.Join(want, ",") {
		t.Fatalf("allowed emails: got %v want %v", a.AllowedEmails, want)
	}
}

func TestSSOIdentity(t *testing.T) {
	a := &Auth{Mode: "sso", AllowedEmails: []string{"initialed85@gmail.com"}}

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	req.Header.Set(EmailHeader, " Initialed85@Gmail.com ")
	req.Header.Set(UserHeader, "113070394191408311024")

	identity, ok := a.Identity(req)
	if !ok {
		t.Fatal("expected identity to match, header casing/whitespace should not matter")
	}

	if identity.Email != "initialed85@gmail.com" || identity.User != "113070394191408311024" {
		t.Fatalf("identity: got %+v", identity)
	}

	// no allowlist configured: whatever the gate let through is accepted
	open := &Auth{Mode: "sso"}

	req = httptest.NewRequest(http.MethodGet, "/", nil)
	req.Header.Set(EmailHeader, "someone@else.org")

	if _, ok := open.Identity(req); !ok {
		t.Fatal("with no ALLOWED_EMAILS the app must defer to the gate's allowlist")
	}

	req = httptest.NewRequest(http.MethodGet, "/", nil)
	if _, ok := a.Identity(req); ok {
		t.Fatal("a request with no identity header must not be accepted")
	}

	req = httptest.NewRequest(http.MethodGet, "/", nil)
	req.Header.Set(EmailHeader, "stranger@example.com")
	if _, ok := a.Identity(req); ok {
		t.Fatal("an address outside ALLOWED_EMAILS must not be accepted")
	}
}

func TestSSOMiddlewareFailsClosedThroughGateway(t *testing.T) {
	a := &Auth{Mode: "sso"}

	w := do(t, a, http.MethodGet, "/api/videos", traefikHeaders(""))
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("gateway request without identity: got %d (body: %s)", w.Code, w.Body.String())
	}

	w = do(t, a, http.MethodGet, "/api/videos", traefikHeaders("initialed85@gmail.com"))
	if w.Code != http.StatusOK || w.Body.String() != "videos" {
		t.Fatalf("gateway request with identity: got %d body %q", w.Code, w.Body.String())
	}

	// camry's own workloads call http://api:7070 in-cluster; they carry no identity headers and are
	// trusted by network position, exactly as before the gate existed.
	w = do(t, a, http.MethodGet, "/api/videos", nil)
	if w.Code != http.StatusOK || w.Body.String() != "videos" {
		t.Fatalf("in-cluster request: got %d body %q", w.Code, w.Body.String())
	}
}

func TestSSOMiddlewareHonoursAllowlist(t *testing.T) {
	a := &Auth{Mode: "sso", AllowedEmails: []string{"initialed85@gmail.com"}}

	w := do(t, a, http.MethodGet, "/api/videos", traefikHeaders("amy.annisbrown@gmail.com"))
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("per-app allowlist: got %d (body: %s)", w.Code, w.Body.String())
	}
}

func TestAuthEndpointsAreExemptFromMiddleware(t *testing.T) {
	// login is how the legacy mode gets its token; auth is the frontend's "who am I" probe. Both have
	// nothing to present yet, and both answer their own clearer errors.
	a := &Auth{Mode: "sso", Password: "pw", Token: "tok"}

	w := do(t, a, http.MethodPost, "/api/custom/login", traefikHeaders(""))
	if w.Code != http.StatusGone {
		t.Fatalf("login under SSO: got %d (body: %s)", w.Code, w.Body.String())
	}

	if !strings.Contains(w.Body.String(), "password login is retired") {
		t.Fatalf("login body: %s", w.Body.String())
	}

	w = do(t, a, http.MethodGet, "/api/custom/auth", traefikHeaders(""))
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("auth without identity: got %d", w.Code)
	}

	if !strings.Contains(w.Body.String(), "not authorised") {
		t.Fatalf("auth must answer with its own message, middleware answered: %s", w.Body.String())
	}

	legacy := &Auth{Password: "cctv-test-password", Token: "test-token"}

	req := httptest.NewRequest(http.MethodPost, "/api/custom/login", strings.NewReader(`{"password":"cctv-test-password"}`))
	req.Header.Set("X-Forwarded-Host", "camry.dev.initialed85.cc")

	w = httptest.NewRecorder()
	gatedRouter(legacy).ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("legacy login must stay reachable through the gateway: got %d (body: %s)", w.Code, w.Body.String())
	}
}

func TestHandleAuthReportsIdentityUnderSSO(t *testing.T) {
	a := &Auth{Mode: "sso"}

	w := do(t, a, http.MethodGet, "/api/custom/auth", traefikHeaders("amy.annisbrown@gmail.com"))
	if w.Code != http.StatusOK {
		t.Fatalf("status: got %d (body: %s)", w.Code, w.Body.String())
	}

	var response struct {
		Objects []struct {
			AuthRequired  bool   `json:"auth_required"`
			Authenticated bool   `json:"authenticated"`
			Email         string `json:"email"`
			User          string `json:"user"`
		} `json:"objects"`
	}

	if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
		t.Fatalf("decode: %v (body: %s)", err, w.Body.String())
	}

	if len(response.Objects) != 1 {
		t.Fatalf("objects: got %v (body: %s)", response.Objects, w.Body.String())
	}

	object := response.Objects[0]
	if !object.Authenticated || !object.AuthRequired || object.Email != "amy.annisbrown@gmail.com" {
		t.Fatalf("response: got %+v (body: %s)", object, w.Body.String())
	}
}

func TestLegacyMiddlewareStillRequiresToken(t *testing.T) {
	a := &Auth{Password: "cctv-test-password", Token: "test-token"}

	w := do(t, a, http.MethodGet, "/api/videos", traefikHeaders(""))
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("gateway request without a token: got %d", w.Code)
	}

	w = do(t, a, http.MethodGet, "/api/videos", map[string]string{
		"X-Forwarded-Host": "camry.dev.initialed85.cc",
		"Authorization":    "Bearer test-token",
	})
	if w.Code != http.StatusOK {
		t.Fatalf("gateway request with a token: got %d", w.Code)
	}

	w = do(t, a, http.MethodGet, "/api/videos", nil)
	if w.Code != http.StatusOK {
		t.Fatalf("in-cluster request in legacy mode: got %d", w.Code)
	}

	// nothing configured at all (the Compose environment) stays open
	open := &Auth{}

	w = do(t, open, http.MethodGet, "/api/videos", traefikHeaders(""))
	if w.Code != http.StatusOK {
		t.Fatalf("unconfigured deployment must stay open: got %d", w.Code)
	}
}
