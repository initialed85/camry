const ref = new Date();
const offset = -ref.getTimezoneOffset();

export const parseDate = (date: string): Date => {
  return new Date(date);
};

const getOffsetSuffix = (): string => {
  const h = Math.round(offset / 60);
  const m = offset - h * 60;

  const formattedH =
    (h >= 0 ? "+" : "-") + Math.abs(h).toString().padStart(2, "0");
  const formattedM = m.toString().padStart(2, "0");

  return `${formattedH}:${formattedM}`;
};

export const formatDate = (date: Date): string => {
  const d = new Date(date.getTime() + offset * 60 * 1000);

  let s = d.toISOString();
  s = s.slice(0, s.length - 1);

  return `${s}${getOffsetSuffix()}`;
};

export const formatDateWithoutMillis = (date: Date): string => {
  const d = new Date(date.getTime() + offset * 60 * 1000);

  let s = d.toISOString();
  s = s.slice(0, s.length - 1);
  s = s.split(".")[0];

  return `${s}`;
};

export const getDateString = (date: Date): string => {
  return formatDate(date).split("T")[0];
};

// A live query ("give me whatever is newest now") has to carry a time in its URL. Every cache
// between the app and Postgres - the browser's HTTP cache, a service worker's CacheStorage, a
// proxy in front of nginx - keys on the request URL, so a live request whose URL never changes
// can be replayed for as long as the entry is kept: the symptom is opening the app and seeing
// videos from the last time it was opened. Rounding up to the minute keeps every request inside
// one minute on the same URL (react-query and offset pagination keep their loaded pages), while
// staying a minute or two ahead of now so a segment recorded seconds ago is still inside the
// window. djangolang reads a bound within an hour of now as "recent" and answers
// `Cache-Control: max-age=60`, so the newest page is at worst about two minutes stale.
export const liveQueryCeiling = (now: Date = new Date()): string => {
  const minuteCeiling =
    Math.floor(now.getTime() / 60_000) * 60_000 + 2 * 60_000;

  // Strict RFC 3339 (a trailing Z): the API's filter parser rejects the +0800 form.
  return new Date(minuteCeiling).toISOString();
};

export const audioScoreColor = (score: number): string => {
  if (score >= 0.7) {
    return "#d32f2f";
  }
  if (score >= 0.4) {
    return "#f6a800";
  }
  return "#66bb6a";
};

export const truncateDate = (date: Date): Date => {
  return parseDate(`${getDateString(date)}T00:00:00${getOffsetSuffix()}`);
};

export const overlayModeKey = "camry.overlayMode.v1";

export type OverlayMode = {
  showDetections: boolean;
  showObjects: boolean;
};

export const defaultOverlayMode: OverlayMode = {
  showDetections: true,
  showObjects: false,
};

// A recording is written twice: a high-resolution half and a low-resolution half,
// by two independent ffmpeg segmenters in the same pod. The low-resolution one opens
// its next file 1-3 seconds earlier, so the two file names are never exact siblings
// (measured on dev across the whole 7-day retention window: 0 of 126 high-resolution
// rows had a "<name>_low_res.mp4" row). Pair the two halves the way the detector
// pairs them, by camera and start time, not by name.
export const recordingSiblingWindowMilliseconds = 20_000;

export type RecordingSiblingQuery = {
  camera_id__eq: string;
  is_low_res__eq: true;
  started_at__gte: string;
  started_at__lte: string;
  limit: number;
};

export const recordingSiblingQuery = (
  cameraId: string | null | undefined,
  startedAt: string | null | undefined,
  windowMilliseconds: number = recordingSiblingWindowMilliseconds,
): RecordingSiblingQuery | undefined => {
  const startedAtMilliseconds = Date.parse(startedAt || "");

  if (!cameraId || !Number.isFinite(startedAtMilliseconds)) {
    return undefined;
  }

  // Strict RFC 3339 (a trailing Z): the API's filter parser rejects the +0800 form.
  return {
    camera_id__eq: cameraId,
    is_low_res__eq: true,
    started_at__gte: new Date(
      startedAtMilliseconds - windowMilliseconds,
    ).toISOString(),
    started_at__lte: new Date(
      startedAtMilliseconds + windowMilliseconds,
    ).toISOString(),
    limit: 10,
  };
};

// Segments are 60 seconds long, so a 20 second window holds a single sibling; the
// nearest start time is the tie-breaker for the rows a window can still reach.
export const pickNearestStartedAt = <T extends { started_at?: string | null }>(
  candidates: T[] | null | undefined,
  startedAt: string | null | undefined,
): T | undefined => {
  const target = Date.parse(startedAt || "");

  let nearest: T | undefined = undefined;
  let nearestDistance = Number.POSITIVE_INFINITY;

  for (const candidate of candidates || []) {
    const candidateAt = Date.parse(candidate?.started_at || "");

    if (!Number.isFinite(candidateAt)) {
      continue;
    }

    const distance = Math.abs(candidateAt - target);

    if (distance < nearestDistance) {
      nearest = candidate;
      nearestDistance = distance;
    }
  }

  return nearest;
};

export const scoreOf = (summary: any): number | undefined =>
  typeof summary?.mean_score === "number"
    ? summary.mean_score
    : summary?.average_score;

export const formatScore = (summary: any): string => {
  const score = scoreOf(summary);

  return typeof score === "number" && Number.isFinite(score)
    ? score.toFixed(2)
    : "-";
};

export const readOverlayMode = (): OverlayMode => {
  const raw = localStorage.getItem(overlayModeKey);

  if (!raw) {
    return defaultOverlayMode;
  }

  try {
    const parsed = JSON.parse(raw);

    return {
      showDetections: parsed?.showDetections !== false,
      showObjects: parsed?.showObjects === true,
    };
  } catch {
    return defaultOverlayMode;
  }
};
