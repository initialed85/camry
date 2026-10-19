import { useQuery } from "@tanstack/react-query";
import { clientForReactQuery } from "../api";

// One strip bucket of a single recording, bucketed by the server (internal/timeline/video.go).
export interface VideoTimelineBucket {
  bucket_ms: number;
  detection_count: number;
  object_count: number;
  top_score: number;
  top_class_name: string;
}

// The overlay canvas's columns only: a box, a centroid, a score.
export interface VideoOverlayDetection {
  seen_at: string;
  class_name: string;
  score: number;
  centroid: [number, number];
  box: [number, number, number, number];
}

export interface VideoOverlayTrackSample {
  seen_at: string;
  bbox: number[];
}

export interface VideoOverlayTrack {
  id: string;
  key: string;
  class_name: string;
  mean_score: number;
  is_static: boolean;
  trajectory: VideoOverlayTrackSample[];
}

interface VideoRowsResponse<T> {
  status: number;
  success: boolean;
  error?: string[];
  objects?: T[];
  count: number;
  total_count: number;
  limit: number;
  offset: number;
}

export interface VideoParams {
  videoId: string;
  bucketMillis?: number;
  alignMillis?: number;
  limit?: number;
  offset?: number;
  // Set false while the row a query depends on is still being resolved.
  enabled?: boolean;
}

// These three are hand-written endpoints (internal/timeline), so they are not in the generated
// OpenAPI schema and have no generated client type. The generated client is still the transport:
// its middleware attaches the bearer token and bounces to login on a 401.
const fetchVideoRows = async <T>(
  endpoint: string,
  params: VideoParams,
): Promise<VideoRowsResponse<T>> => {
  const query: Record<string, string> = {
    video_id__eq: params.videoId,
  };

  if (params.bucketMillis !== undefined) {
    query.bucket_ms = String(params.bucketMillis);
  }
  if (params.alignMillis !== undefined) {
    query.align_ms = String(params.alignMillis);
  }
  if (params.limit !== undefined) {
    query.limit = String(params.limit);
  }
  if (params.offset !== undefined) {
    query.offset = String(params.offset);
  }

  const { data, error, response } = await (clientForReactQuery as any).GET(
    `/api/custom/${endpoint}`,
    { params: { query } },
  );

  if (error || !response?.ok) {
    const message =
      (data as VideoRowsResponse<T> | undefined)?.error?.join("; ") ||
      `${endpoint} request failed (${response?.status})`;

    throw new Error(message);
  }

  return data as VideoRowsResponse<T>;
};

export const useVideoTimeline = (
  params: VideoParams,
  refetchIntervalMillis: number,
) =>
  useQuery({
    queryKey: [
      "video-timeline",
      params.videoId,
      params.bucketMillis,
      params.alignMillis,
      params.limit,
      params.offset,
    ],
    queryFn: () =>
      fetchVideoRows<VideoTimelineBucket>("video-timeline", params),
    enabled: Boolean(params.videoId) && params.enabled !== false,
    refetchInterval: refetchIntervalMillis,
  });

export const useVideoOverlayDetections = (params: VideoParams) =>
  useQuery({
    queryKey: ["video-overlay-detections", params.videoId, params.limit],
    queryFn: () =>
      fetchVideoRows<VideoOverlayDetection>("video-detections", params),
    enabled: Boolean(params.videoId) && params.enabled !== false,
  });

export const useVideoOverlayTracks = (
  params: VideoParams,
  tracksEnabled: boolean,
) =>
  useQuery({
    queryKey: ["video-overlay-tracks", params.videoId, params.limit],
    queryFn: () => fetchVideoRows<VideoOverlayTrack>("video-tracks", params),
    enabled:
      Boolean(params.videoId) && tracksEnabled && params.enabled !== false,
  });
