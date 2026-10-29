import { default as createFetchClient } from "openapi-fetch";
import createClientForReactQuery from "openapi-react-query";

import { QueryClient } from "@tanstack/react-query";
import type { paths } from "./api/api";
import { requireSignIn } from "./auth";

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

// The auth endpoints are not in the generated OpenAPI schema. Nothing is attached to outgoing
// requests: the sign-in gate injects the identity headers on its way in, so a request that reached
// this client is already authenticated. The only job left is to notice a 401 - a session that expired
// while the page was open - and hand it back to App, which offers sign-in.
clientForReactQuery.use({
  onRequest: ({ request }) => request,
  onResponse: ({ response }) => {
    if (response.status === 401) {
      requireSignIn();
    }

    return response;
  },
});

export const { useQuery, useMutation, useSuspenseQuery } =
  createClientForReactQuery(clientForReactQuery);
