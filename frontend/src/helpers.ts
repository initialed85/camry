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
