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

export const checkAuth = async (): Promise<AuthStatus> => {
  const response = await fetch("/api/custom/auth", {
    method: "GET",
    credentials: "same-origin",
  });

  // An unauthenticated probe under the gate is answered with a redirect to Google, which fetch
  // cannot follow across origins. The caller treats that as "not signed in".
  if (!response.ok) {
    identity = undefined;

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
// verified is not one Camry accepts. Reload is the sign-in gesture - the gate turns it into a Google
// redirect and returns to the same URL.
export const requireSignIn = (): void => {
  identity = undefined;
  window.dispatchEvent(new Event(AUTH_REQUIRED_EVENT));
};
