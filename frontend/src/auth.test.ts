import { checkAuth, requireSignIn, signedInEmail, signInHost } from "./auth";

const setHost = (hostname: string) => {
  Object.defineProperty(window, "location", {
    value: { hostname, href: `https://${hostname}/some/deep/link` },
    writable: true,
  });
};

const fakeFetch = (status: number, body: unknown) => {
  (global as any).fetch = async () => ({
    ok: status >= 200 && status < 300,
    status,
    json: async () => body,
  });
};

describe("sign-in helpers", () => {
  it("derives the cluster sign-in host from the app host", () => {
    setHost("camry.dev.initialed85.cc");
    expect(signInHost()).toBe("https://auth.dev.initialed85.cc");

    setHost("camry.prod.initialed85.cc");
    expect(signInHost()).toBe("https://auth.prod.initialed85.cc");
  });

  it("returns nothing for a host with no gate in front of it", () => {
    setHost("localhost");
    expect(signInHost()).toBe("");
  });

  it("reports the identity the gate injected", async () => {
    setHost("camry.dev.initialed85.cc");
    fakeFetch(200, {
      objects: [
        {
          authenticated: true,
          auth_required: true,
          email: "amy.annisbrown@gmail.com",
        },
      ],
    });

    const status = await checkAuth();

    expect(status.authenticated).toBe(true);
    expect(status.auth_required).toBe(true);
    expect(signedInEmail()).toBe("amy.annisbrown@gmail.com");
  });

  it("treats a 401 as signed out, with no email to show", async () => {
    setHost("camry.dev.initialed85.cc");
    fakeFetch(401, { status: 401, error: ["not authorised"] });

    const status = await checkAuth();

    expect(status).toEqual({ authenticated: false, auth_required: true });
    expect(signedInEmail()).toBeUndefined();
  });

  it("clears the identity when the API rejects a live session", () => {
    setHost("camry.dev.initialed85.cc");

    let required = 0;
    window.addEventListener("camry-auth-required", () => (required += 1));

    requireSignIn();

    expect(signedInEmail()).toBeUndefined();
    expect(required).toBe(1);
  });
});
