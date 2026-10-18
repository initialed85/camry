import { useQuery } from "@tanstack/react-query";
import { clientForReactQuery } from "../api";

// One camera-hour of audio events, as projected by the v_audio_event_hour view.
export interface AudioTimelineBucket {
  bucket: string;
  camera_id: string;
  event_count: number;
  top_score: number;
  peak_prominence_db: number;
  top_kind: string;
  first_seen_at: string;
  last_seen_at: string;
}

export interface AudioTimelineResponse {
  status: number;
  success: boolean;
  error?: string[];
  objects?: AudioTimelineBucket[];
  count: number;
  total_count: number;
  limit: number;
  offset: number;
}

export interface AudioTimelineParams {
  startedAtGte: string;
  startedAtLt: string;
  cameraId?: string;
  minScore?: number;
  limit?: number;
  offset?: number;
}

// GET /api/custom/audio-timeline is a hand-written endpoint (internal/timeline), so it is not in
// the generated OpenAPI schema and has no generated client type. The generated client is still
// the transport: its middleware attaches the bearer token and bounces to login on a 401.
const fetchAudioTimeline = async (
  params: AudioTimelineParams,
): Promise<AudioTimelineResponse> => {
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

  const { data, error, response } = await (clientForReactQuery as any).raw(
    "GET",
    "/api/custom/audio-timeline",
    { query },
  );

  if (error || !response?.ok) {
    const message =
      (data as AudioTimelineResponse | undefined)?.error?.join("; ") ||
      `audio timeline request failed (${response?.status})`;

    throw new Error(message);
  }

  return data as AudioTimelineResponse;
};

export const useAudioTimeline = (
  params: AudioTimelineParams,
  refetchIntervalMillis: number,
) =>
  useQuery({
    queryKey: [
      "audio-timeline",
      params.startedAtGte,
      params.startedAtLt,
      params.cameraId,
      params.minScore,
      params.limit,
      params.offset,
    ],
    queryFn: () => fetchAudioTimeline(params),
    refetchInterval: refetchIntervalMillis,
  });
