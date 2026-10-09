export const TOKEN_STORAGE_KEY = "camry-token";

export const AUTH_REQUIRED_EVENT = "camry-auth-required";

export type AuthStatus = {
  authenticated: boolean;
  auth_required: boolean;
};

// Custom endpoints are not emitted into the generated OpenAPI schema, so these
// calls use plain fetch rather than the typed openapi-fetch client.
export const getToken = (): string => {
  try {
    return window.localStorage.getItem(TOKEN_STORAGE_KEY) || "";
  } catch {
    return "";
  }
};

const setToken = (token: string): void => {
  try {
    window.localStorage.setItem(TOKEN_STORAGE_KEY, token);
  } catch {
    // Ignore unavailable/disabled localStorage; the login still works for this session.
  }
};

export const clearToken = (): void => {
  try {
    window.localStorage.removeItem(TOKEN_STORAGE_KEY);
  } catch {
    // Ignore unavailable/disabled localStorage.
  }
};

export const tokenHeaders = (): Record<string, string> => {
  const token = getToken();
  return token ? { Authorization: `Bearer ${token}` } : {};
};

const readObject = async (
  response: Response,
): Promise<Record<string, any> | undefined> => {
  try {
    const body = await response.json();
    return body?.objects?.[0];
  } catch {
    // Traefik answers 401 with its own non-JSON body, so a parse failure here
    // just means "not authenticated".
    return undefined;
  }
};

export const checkAuth = async (): Promise<AuthStatus> => {
  const response = await fetch("/api/custom/auth", { headers: tokenHeaders() });

  const object = await readObject(response);

  return {
    authenticated: response.status === 200 && Boolean(object?.authenticated),
    auth_required: Boolean(object?.auth_required),
  };
};

export const login = async (password: string): Promise<string> => {
  const response = await fetch("/api/custom/login", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ password }),
  });

  if (response.status !== 200) {
    throw new Error("That password is not correct.");
  }

  const object = await readObject(response);
  const token = object?.token;

  if (!token) {
    throw new Error("Login succeeded but no token was returned.");
  }

  setToken(token);

  return token;
};

export const logout = (): void => {
  clearToken();
};
