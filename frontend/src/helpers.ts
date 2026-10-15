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

export const truncateDate = (date: Date): Date => {
  return parseDate(`${getDateString(date)}T00:00:00${getOffsetSuffix()}`);
};

export const overlayModeKey = "camry.overlayMode.v1";

// The high-res and low-res segment writers open their recorders a second or two
// apart, so the two rows' started_at values are not the content clock. Frame
// cross-correlation of matched segments (10 fps, 12 s windows, 18 pairs) shows
// the media timelines line up frame-for-frame, and the residual is a per-camera
// constant: Driveway -0.8s, Front Door -0.7s, Side Gate -0.1s.
const cameraOverlayOffsetsMs: Record<string, number> = {
  "85aead5a-e9d8-49d8-a091-03f737342c34": -800, // Driveway
  "4f03400f-6be0-4d3e-b205-da65478dbd46": -700, // Front Door
  "326da5dd-8a6a-4534-9598-effeb03abe61": -100, // Side Gate
};

export const cameraOverlayOffsetMs = (cameraId?: string | null): number =>
  cameraOverlayOffsetsMs[cameraId || ""] ?? 0;

export type OverlayMode = {
  showDetections: boolean;
  showObjects: boolean;
};

export const defaultOverlayMode: OverlayMode = {
  showDetections: true,
  showObjects: false,
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
