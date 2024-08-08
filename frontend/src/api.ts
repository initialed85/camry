import { createSyncStoragePersister } from "@tanstack/query-sync-storage-persister";
import {
  persistQueryClient,
  removeOldestQuery,
} from "@tanstack/react-query-persist-client";

import { default as createFetchClient } from "openapi-fetch";
import createClientForReactQuery from "openapi-react-query";

import { QueryClient } from "@tanstack/react-query";
import type { paths } from "./api/api";

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60, // 1 minute,
      gcTime: 1000 * 60 * 60 * 24 * 7, // 7 days
      refetchOnMount: true,
      refetchOnReconnect: true,
      refetchOnWindowFocus: true,
      refetchInterval: 1000 * 60, // 1 minute
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

const clientForReactQuery = createFetchClient<paths>({
  baseUrl: "http://localhost:3000",
});

export const { useQuery, useMutation, useSuspenseQuery } =
  createClientForReactQuery(clientForReactQuery);
