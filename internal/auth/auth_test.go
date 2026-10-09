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
