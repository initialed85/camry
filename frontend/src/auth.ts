// Camry sits behind the cluster sign-in gate: Traefik's auth-oauth2 forwardAuth -> oauth2-proxy ->
// Google, with the family addresses in the oauth2-proxy ConfigMap and Google's Test users. See
// home-ops clusters/dev/applications/auth (and clusters/prod/applications/auth for prod).
//
// Two consequences for this file:
//
//   1. There is no app-level credential any more. Traefik authenticates every request to the host,
//      including the /media/ and HLS requests a browser cannot attach an Authorization header to -
//      which is exactly the hole the old camry_token cookie could not cover. The fixed
//      PASSWORD/TOKEN exchange and the localStorage token are gone.
//   2. Signing in is not something this page does. An unauthenticated navigation is redirected to
//      Google before the SPA loads and comes back to the same URL, so all this page can offer is a
//      top-level navigation the gate is free to intercept.

export interface AuthStatus {
  authenticated: boolean;
  auth_required: boolean;
  email?: string;
}

export const AUTH_REQUIRED_EVENT = "camry-auth-required";

let identity: AuthStatus | undefined;

// signedInEmail is the address the gate injected, if any. It is only ever the verified one: Traefik
// deletes client-supplied X-Auth-Request-* headers before injecting its own.
export const signedInEmail = (): string | undefined => identity?.email;

// signInHost is the cluster's sign-in host for the current environment: camry.dev.initialed85.cc ->
// auth.dev.initialed85.cc. Empty for a host with no gate in front of it (localhost/Compose), where
// the app simply stays open.
export const signInHost = (): string => {
  const host = window.location.hostname;

  if (!host.includes(".")) {
    return "";
  }

  const labels = host.split(".");
  labels[0] = "auth";

  return `https://${labels.join(".")}`;
};

// signInUrl is where the Sign in button sends the browser: oauth2-proxy's own start route on the
// auth host, with the current URL as the return target. Two reasons not to just reload this page:
// a cross-origin navigation cannot be answered by a stale service worker, and ?rd= preserves the
// deep link through Google the way the gate's own redirect does.
export const signInUrl = (): string => {
  const host = signInHost();

  if (!host) {
    return window.location.href;
  }

  return `${host}/oauth2/start?rd=${encodeURIComponent(window.location.href)}`;
};

export const checkAuth = async (): Promise<AuthStatus> => {
  const response = await fetch("/api/custom/auth", {
    method: "GET",
    credentials: "same-origin",
    // redirect: manual so a session-less probe is visible as a redirect instead of a CORS failure
    // on Google, which arrives as a plain "fetch failed" and is indistinguishable from Camry being down.
    redirect: "manual",
  });

  identity = undefined;

  // The gate answered with a 302 to Google. fetch() cannot follow it across origins, and with
  // redirect:manual it shows up as an opaque 0 rather than an exception.
  if (response.type === "opaqueredirect" || response.status === 0) {
    return { authenticated: false, auth_required: true };
  }

  if (!response.ok) {
    if (response.status === 401) {
      return { authenticated: false, auth_required: true };
    }

    throw new Error(`Camry is not available yet: HTTP ${response.status}`);
  }

  const payload = await response.json();
  const object = payload?.objects?.[0];

  const status: AuthStatus = {
    authenticated: object?.authenticated === true,
    auth_required: object?.auth_required === true,
    email: typeof object?.email === "string" ? object.email : undefined,
  };

  identity = status;

  return status;
};

// requireSignIn is what a 401 from the API means now: the session is gone, or the address the gate
// verified is not one Camry accepts. App responds by offering signInUrl().
export const requireSignIn = (): void => {
  identity = undefined;
  window.dispatchEvent(new Event(AUTH_REQUIRED_EVENT));
};
