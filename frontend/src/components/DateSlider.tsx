import Box from "@mui/joy/Box";
import Slider, { sliderClasses } from "@mui/joy/Slider";
import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { useAudioTimeline } from "../api/audioTimeline";
import { useDetectionTimeline } from "../api/detectionTimeline";
import {
  audioStripFullCountEvents,
  audioStripMinScore,
  dateSliderStepMillis,
  dateSliderWidthMillis,
  detectionStripFullCountDetections,
  detectionStripFullCountObjects,
  detectionStripMinScore,
  stripRowHeightPx,
} from "../config";
import {
  audioScoreColor,
  formatDateWithoutMillis,
  OverlayMode,
} from "../helpers";

export interface DateSliderProps {
  responsive: boolean;
  cameraId: string | undefined;
  date: string | undefined;
  setDate: Dispatch<SetStateAction<string | undefined>>;
  overlayMode: OverlayMode;
}

const bucketCount =
  Math.round(dateSliderWidthMillis / dateSliderStepMillis) + 1;

// min/max are snapped to the step grid because the Slider normalises every value
// onto min + k*step: unaligned bounds would snap a strip click to an off-hour time.
const getDateMillis = (): [number, number] => {
  const now = new Date();
  const maxDateMillisLive =
    Math.floor(now.getTime() / dateSliderStepMillis) * dateSliderStepMillis;
  const minDateMillisLive =
    Math.floor((now.getTime() - dateSliderWidthMillis) / dateSliderStepMillis) *
    dateSliderStepMillis;
  return [maxDateMillisLive, minDateMillisLive];
};

// The strip window is hour-aligned so its query key only moves once an hour;
// the slider's own window ticks every second and would refetch every tick.
const getStripWindowMillis = (): [number, number] => {
  const now = new Date();
  const maxDateMillisLive = Math.floor(now.getTime() / dateSliderStepMillis);
  const minDateMillisLive =
    maxDateMillisLive -
    Math.round(dateSliderWidthMillis / dateSliderStepMillis);
  return [
    minDateMillisLive * dateSliderStepMillis,
    (maxDateMillisLive + 1) * dateSliderStepMillis,
  ];
};

// One row of the strip: bars anchored to the row's bottom edge (direction "up") or top edge
// ("down"), so a pair of rows mirrors around the strip's centre line, as in a waveform.
interface StripBar {
  index: number;
  millis: number;
  heightPercent: number;
  color: string;
  // Objects sit inside the detection bar as its solid core; the detections halo is translucent.
  innerHeightPercent: number;
  innerColor: string;
  title: string;
}

const stripBarHeight = (count: number, fullCount: number): number => {
  return count > 0 ? 8 + Math.min(1, count / fullCount) * 92 : 0;
};

export default function DateSlider(props: DateSliderProps) {
  const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

  const [maxDateMillis, setMaxDateMillis] = useState(maxDateMillisLive);
  const [minDateMillis, setMinDateMillis] = useState(minDateMillisLive);
  const [currentMillis, setCurrentMillis] = useState(
    maxDateMillis + dateSliderStepMillis * 1,
  );

  const [slidByUser, setSlidByUser] = useState(false);

  const [stripWindowMillis, setStripWindowMillis] = useState<[number, number]>(
    getStripWindowMillis(),
  );

  useEffect(() => {
    const intervalId = setInterval(() => {
      const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

      setMaxDateMillis(maxDateMillisLive);
      setMinDateMillis(minDateMillisLive);

      if (!slidByUser && maxDateMillisLive - currentMillis > 1000) {
        setCurrentMillis(maxDateMillisLive);
      }

      const [stripMinMillis, stripMaxMillis] = getStripWindowMillis();
      if (
        stripMinMillis !== stripWindowMillis[0] ||
        stripMaxMillis !== stripWindowMillis[1]
      ) {
        setStripWindowMillis([stripMinMillis, stripMaxMillis]);
      }
    }, 1_000);

    if (slidByUser && currentMillis > maxDateMillis) {
      setSlidByUser(false);
    }

    return () => {
      clearInterval(intervalId);
    };
  }, [
    currentMillis,
    maxDateMillis,
    maxDateMillisLive,
    minDateMillis,
    props,
    slidByUser,
    stripWindowMillis,
  ]);

  const stripStartedAtGte = new Date(stripWindowMillis[0]).toISOString();
  const stripStartedAtLt = new Date(stripWindowMillis[1]).toISOString();

  const { data: audioTimelineData } = useAudioTimeline(
    {
      cameraId: props.cameraId || undefined,
      // The strip window is hour-aligned, so its buckets line up with the view's rows exactly.
      startedAtGte: stripStartedAtGte,
      startedAtLt: stripStartedAtLt,
      minScore: audioStripMinScore,
      limit: 5_000,
    },
    60_000,
  );

  const { data: detectionTimelineData } = useDetectionTimeline(
    {
      cameraId: props.cameraId || undefined,
      startedAtGte: stripStartedAtGte,
      startedAtLt: stripStartedAtLt,
      minScore: detectionStripMinScore,
      limit: 5_000,
    },
    60_000,
  );

  const audioBars = useMemo<StripBar[]>(() => {
    const byIndex = new Map<
      number,
      { count: number; score: number; prominence: number }
    >();

    audioTimelineData?.objects?.forEach((row) => {
      if (!row?.bucket || !row?.event_count) {
        return;
      }

      const offsetMillis =
        new Date(row.bucket).getTime() - stripWindowMillis[0];
      if (offsetMillis < 0) {
        return;
      }

      const index = Math.floor(offsetMillis / dateSliderStepMillis);
      if (index < 0 || index >= bucketCount) {
        return;
      }

      const bucket = byIndex.get(index) || {
        count: 0,
        score: 0,
        prominence: 0,
      };
      bucket.count += row.event_count;
      bucket.score = Math.max(bucket.score, row.top_score);
      bucket.prominence = Math.max(
        bucket.prominence,
        row.peak_prominence_db || 0,
      );

      byIndex.set(index, bucket);
    });

    return Array.from({ length: bucketCount }, (_, index) => {
      const bucket = byIndex.get(index);
      const millis = stripWindowMillis[0] + index * dateSliderStepMillis;

      if (!bucket) {
        return {
          index,
          millis,
          heightPercent: 0,
          color: "#00000022",
          innerHeightPercent: 0,
          innerColor: "#00000022",
          title: formatDateWithoutMillis(new Date(millis)),
        };
      }

      return {
        index,
        millis,
        heightPercent: stripBarHeight(bucket.count, audioStripFullCountEvents),
        color: audioScoreColor(bucket.score),
        innerHeightPercent: 0,
        innerColor: audioScoreColor(bucket.score),
        title: `${formatDateWithoutMillis(new Date(millis))} | ${
          bucket.count
        } events | peak +${bucket.prominence.toFixed(1)} dB | score ${bucket.score.toFixed(
          2,
        )}`,
      };
    });
  }, [audioTimelineData, stripWindowMillis]);

  const detectionBars = useMemo<StripBar[]>(() => {
    const byIndex = new Map<
      number,
      {
        detections: number;
        objects: number;
        score: number;
        className: string;
      }
    >();

    detectionTimelineData?.objects?.forEach((row) => {
      if (!row?.bucket) {
        return;
      }

      const offsetMillis =
        new Date(row.bucket).getTime() - stripWindowMillis[0];
      if (offsetMillis < 0) {
        return;
      }

      const index = Math.floor(offsetMillis / dateSliderStepMillis);
      if (index < 0 || index >= bucketCount) {
        return;
      }

      const bucket = byIndex.get(index) || {
        detections: 0,
        objects: 0,
        score: 0,
        className: "",
      };
      bucket.detections += row.detection_count;
      bucket.objects += row.object_count;
      if (row.top_score >= bucket.score) {
        bucket.score = row.top_score;
        bucket.className = row.top_class_name || bucket.className;
      }

      byIndex.set(index, bucket);
    });

    const { showDetections, showObjects } = props.overlayMode;

    return Array.from({ length: bucketCount }, (_, index) => {
      const bucket = byIndex.get(index);
      const millis = stripWindowMillis[0] + index * dateSliderStepMillis;

      const detections = showDetections ? (bucket?.detections ?? 0) : 0;
      const objects = showObjects ? (bucket?.objects ?? 0) : 0;

      if (!bucket || (!detections && !objects)) {
        return {
          index,
          millis,
          heightPercent: 0,
          color: "#00000022",
          innerHeightPercent: 0,
          innerColor: "#00000022",
          title: formatDateWithoutMillis(new Date(millis)),
        };
      }

      // A bucket with objects and no detections (the detector wrote tracks, prune took the
      // detections) still needs a bar, so the bar covers whichever series is taller.
      const detectionHeight = stripBarHeight(
        detections,
        detectionStripFullCountDetections,
      );
      const objectHeight = stripBarHeight(
        objects,
        detectionStripFullCountObjects,
      );
      const heightPercent = Math.max(detectionHeight, objectHeight);

      const parts = [`${detections} detections`, `${objects} objects`];
      if (bucket.className) {
        parts.push(`score ${bucket.score.toFixed(2)} (${bucket.className})`);
      }

      return {
        index,
        millis,
        heightPercent,
        color: `${audioScoreColor(bucket.score)}77`,
        innerHeightPercent:
          heightPercent > 0 ? (objectHeight / heightPercent) * 100 : 0,
        innerColor: audioScoreColor(bucket.score),
        title: `${formatDateWithoutMillis(new Date(millis))} | ${parts.join(
          " | ",
        )}`,
      };
    });
  }, [detectionTimelineData, props.overlayMode, stripWindowMillis]);

  const bucketMillis = (index: number): number => {
    const millis = stripWindowMillis[0] + index * dateSliderStepMillis;

    return millis > maxDateMillis ? maxDateMillis : millis;
  };

  // The strip and the slider share one position: a bar covers [bar, bar + step),
  // so clicking it sets the thumb to the bar and the table window to that same
  // hour. The thumb is the window's start, not its end.
  const applyMillis = (millis: number) => {
    setSlidByUser(true);
    setCurrentMillis(millis);
    props.setDate(new Date(millis + dateSliderStepMillis).toISOString());
  };

  const renderStrip = (
    bars: StripBar[],
    prefix: string,
    onClickBar: (bar: StripBar) => void,
  ) => {
    const rows: ("up" | "down")[] = ["up", "down"];

    return rows.map((direction) => (
      <Box
        key={`${prefix}-strip-${direction}`}
        sx={{
          display: "flex",
          alignItems: direction === "up" ? "flex-end" : "flex-start",
          height: `${stripRowHeightPx}px`,
        }}
      >
        {bars.map((bar) => (
          <Box
            key={`${prefix}-strip-${direction}-${bar.index}`}
            title={bar.title}
            onClick={() => onClickBar(bar)}
            sx={{
              position: "relative",
              flex: 1,
              mx: "0.5px",
              height: bar.heightPercent ? `${bar.heightPercent}%` : "2px",
              backgroundColor: bar.heightPercent ? bar.color : "#00000022",
              borderRadius: "1px",
              cursor: bar.heightPercent ? "pointer" : "default",
            }}
          >
            {bar.innerHeightPercent > 0 && (
              <Box
                sx={{
                  position: "absolute",
                  left: 0,
                  right: 0,
                  [direction === "up" ? "bottom" : "top"]: 0,
                  height: `${bar.innerHeightPercent}%`,
                  backgroundColor: bar.innerColor,
                  borderRadius: "1px",
                }}
              />
            )}
          </Box>
        ))}
      </Box>
    ));
  };

  const labelSx = {
    width: props.responsive ? "46px" : "108px",
    flexShrink: 0,
    fontSize: "12px",
    color: "text.tertiary",
    textAlign: "right" as const,
    pr: "6px",
    lineHeight: `${stripRowHeightPx * 2}px`,
  };

  const clickStripBar = (bar: StripBar) => {
    if (!bar.heightPercent) {
      return;
    }

    applyMillis(bucketMillis(bar.index));
  };

  return (
    <Box sx={{ width: "88%", display: "flex", flexDirection: "column" }}>
      <Box sx={{ display: "flex", flexDirection: "row", mb: "2px" }}>
        <Box sx={labelSx}>
          {props.responsive ? "S" : "Sound"}
          <br />
          {props.responsive ? "D/O" : "Detections / Objects"}
        </Box>
        <Box sx={{ flex: 1, display: "flex", flexDirection: "column" }}>
          {renderStrip(audioBars, "audio", clickStripBar)}
          {renderStrip(detectionBars, "detection", clickStripBar)}
        </Box>
      </Box>
      <Slider
        variant="soft"
        size={"lg"}
        color={slidByUser ? "danger" : "success"}
        sx={{
          width: "100%",
          py: "0px",
          my: "10px",
          // Need both of the selectors to make it works on the server-side and client-side
          [`& [style*="left:0%"], & [style*="left: 0%"]`]: {
            [`&.${sliderClasses.markLabel}`]: {
              transform: "none",
            },
            [`& .${sliderClasses.valueLabel}`]: {
              left: "calc(var(--Slider-thumbSize) / 2)",
              borderBottomLeftRadius: 0,
              "&::before": {
                left: 0,
                transform: "translateY(100%)",
                borderLeftColor: "currentColor",
              },
            },
          },
          [`& [style*="left:100%"], & [style*="left: 100%"]`]: {
            [`&.${sliderClasses.markLabel}`]: {
              transform: "translateX(-100%)",
            },
            [`& .${sliderClasses.valueLabel}`]: {
              right: "calc(var(--Slider-thumbSize) / 2)",
              borderBottomRightRadius: 0,
              "&::before": {
                left: "initial",
                right: 0,
                transform: "translateY(100%)",
                borderRightColor: "currentColor",
              },
            },
          },
        }}
        value={currentMillis}
        onChange={(_, value) => {
          setSlidByUser(true);

          if (typeof value === "number" && value !== currentMillis) {
            setCurrentMillis(value);
          }
        }}
        onChangeCommitted={(_, value) => {
          if (typeof value === "number") {
            applyMillis(value > maxDateMillis ? maxDateMillis : value);
          }
        }}
        step={dateSliderStepMillis}
        min={minDateMillis}
        max={maxDateMillis + dateSliderStepMillis * 1}
        valueLabelDisplay="auto"
        valueLabelFormat={(value: number): string => {
          if (value > maxDateMillis) {
            return "(live)";
          }

          return formatDateWithoutMillis(new Date(value));
        }}
        aria-label="Always visible"
        scale={(x: number) => {
          return x; // TODO: logarithmic
        }}
      />
    </Box>
  );
}
