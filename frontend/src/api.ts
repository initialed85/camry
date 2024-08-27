import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import { persistQueryClient, removeOldestQuery } from "@tanstack/react-query-persist-client";

import { default as createFetchClient } from "openapi-fetch";
import createClientForReactQuery from "openapi-react-query";

import { QueryClient } from "@tanstack/react-query";
import type { paths } from "./api/api";

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

const localStoragePersister = createSyncStoragePersister({
  storage: window.localStorage,
  retry: removeOldestQuery,
});

persistQueryClient({
  queryClient,
  persister: localStoragePersister,
});

export const clientForReactQuery = createFetchClient<paths>({
  baseUrl: "/",
});

export const { useQuery, useMutation, useSuspenseQuery } = createClientForReactQuery(clientForReactQuery);
