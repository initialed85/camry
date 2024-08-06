import { default as createFetchClient } from "openapi-fetch";
import createClientForReactQuery from "openapi-react-query";

import type { paths } from "./api/api";

const clientForReactQuery = createFetchClient<paths>({
  baseUrl: "http://localhost:3000",
});

export const { useQuery, useMutation, useSuspenseQuery } =
  createClientForReactQuery(clientForReactQuery);
