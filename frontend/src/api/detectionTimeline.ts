import { useQuery } from "@tanstack/react-query";
import { clientForReactQuery } from "../api";

// One camera-hour of detections and objects, as projected by the timeline schema (migration 00010).
export interface DetectionTimelineBucket {
  bucket: string;
  camera_id: string;
  detection_count: number;
  object_count: number;
  top_score: number;
  top_class_name: string;
  first_seen_at: string;
  last_seen_at: string;
}

export interface DetectionTimelineResponse {
  status: number;
  success: boolean;
  error?: string[];
  objects?: DetectionTimelineBucket[];
  count: number;
  total_count: number;
  limit: number;
  offset: number;
}

export interface DetectionTimelineParams {
  startedAtGte: string;
  startedAtLt: string;
  cameraId?: string;
  minScore?: number;
  limit?: number;
  offset?: number;
}

// GET /api/custom/detection-timeline is a hand-written endpoint (internal/timeline), so it is not
// in the generated OpenAPI schema and has no generated client type. The generated client is still
// the transport: its middleware attaches the bearer token and bounces to login on a 401.
const fetchDetectionTimeline = async (
  params: DetectionTimelineParams,
): Promise<DetectionTimelineResponse> => {
  const query: Record<string, string> = {
    started_at__gte: params.startedAtGte,
    started_at__lt: params.startedAtLt,
  };

  if (params.cameraId) {
    query.camera_id__eq = params.cameraId;
  }
  if (params.minScore !== undefined) {
    query.min_score = String(params.minScore);
  }
  if (params.limit !== undefined) {
    query.limit = String(params.limit);
  }
  if (params.offset !== undefined) {
    query.offset = String(params.offset);
  }

  const { data, error, response } = await (clientForReactQuery as any).GET(
    "/api/custom/detection-timeline",
    { params: { query } },
  );

  if (error || !response?.ok) {
    const message =
      (data as DetectionTimelineResponse | undefined)?.error?.join("; ") ||
      `detection timeline request failed (${response?.status})`;

    throw new Error(message);
  }

  return data as DetectionTimelineResponse;
};

export const useDetectionTimeline = (
  params: DetectionTimelineParams,
  refetchIntervalMillis: number,
) =>
  useQuery({
    queryKey: [
      "detection-timeline",
      params.startedAtGte,
      params.startedAtLt,
      params.cameraId,
      params.minScore,
      params.limit,
      params.offset,
    ],
    queryFn: () => fetchDetectionTimeline(params),
    refetchInterval: refetchIntervalMillis,
  });
