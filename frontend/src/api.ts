import { default as createFetchClient } from "openapi-fetch";
import createClientForReactQuery from "openapi-react-query";

import { QueryClient } from "@tanstack/react-query";
import type { paths } from "./api/api";
import { AUTH_REQUIRED_EVENT, clearToken, getToken } from "./auth";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60, // 1 minute,
      gcTime: 1000 * 60 * 60 * 24 * 7, // 7 days
      refetchInterval: 1000 * 5, // 5 second
      refetchIntervalInBackground: true,
      refetchOnMount: true,
      refetchOnReconnect: true,
      refetchOnWindowFocus: true,
    },
  },
});

// The old synchronous React Query persister serialized the entire cache to
// localStorage on updates, blocking the main thread when video/detection pages
// grew large. The app needs the API online anyway, so don't persist API data.
try {
  window.localStorage.removeItem("REACT_QUERY_OFFLINE_CACHE");
} catch {
  // Ignore unavailable/disabled localStorage; queries work without it.
}

export const clientForReactQuery = createFetchClient<paths>({
  baseUrl: "/",
});

// The auth endpoints are not in the generated OpenAPI schema, but every generated
// route needs the bearer token, so attach it (and bounce back to the login page
// when Traefik rejects the request).
clientForReactQuery.use({
  onRequest: ({ request }) => {
    const token = getToken();

    if (token) {
      request.headers.set("Authorization", `Bearer ${token}`);
    }

    return request;
  },
  onResponse: ({ response }) => {
    if (response.status === 401) {
      clearToken();
      window.dispatchEvent(new Event(AUTH_REQUIRED_EVENT));
    }

    return response;
  },
});

export const { useQuery, useMutation, useSuspenseQuery } =
  createClientForReactQuery(clientForReactQuery);
