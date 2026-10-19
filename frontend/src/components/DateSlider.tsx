import Box from "@mui/joy/Box";
import Slider, { sliderClasses } from "@mui/joy/Slider";
import {
  Dispatch,
  SetStateAction,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from "react";
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
  // The time range the video table is currently showing, so the strip can mark that section.
  visibleRangeMillis: [number, number] | undefined;
}

// The slider's own domain is the strip window: hour-aligned bounds, `stripIntervals` steps across
// the full width. Bar i covers interval i, so a strip pixel and a slider pixel name the same hour,
// and a strip click snaps exactly like a slider drag at the same x.
const stripIntervals = Math.round(dateSliderWidthMillis / dateSliderStepMillis);

const getStripWindowMillis = (): [number, number] => {
  const maxHour = Math.floor(new Date().getTime() / dateSliderStepMillis);

  return [
    (maxHour - stripIntervals) * dateSliderStepMillis,
    maxHour * dateSliderStepMillis,
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

// The bar at index i covers interval i of the strip window, which is the same interval the slider
// puts at the same fraction of its track.
const stripMillis = (stripMinMillis: number, index: number): number => {
  return stripMinMillis + index * dateSliderStepMillis;
};

const stripBarHeight = (count: number, fullCount: number): number => {
  return count > 0 ? 8 + Math.min(1, count / fullCount) * 92 : 0;
};

export default function DateSlider(props: DateSliderProps) {
  const [stripWindowMillis, setStripWindowMillis] = useState<[number, number]>(
    getStripWindowMillis(),
  );
  const [stripMinMillis, stripMaxMillis] = stripWindowMillis;

  const [currentMillis, setCurrentMillis] = useState(stripMaxMillis);

  const [slidByUser, setSlidByUser] = useState(false);

  // The strip bars have to share the slider's own box for a strip click and a slider click at the
  // same x to pick the same hour. That box is the **rail** (the full-width base line): the track is
  // the filled part and shrinks to 0 as the thumb moves left.
  const [railInsetsPx, setRailInsetsPx] = useState<[number, number]>([0, 0]);

  const rootRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const root = rootRef.current;
    const rail = root?.querySelector<HTMLElement>(`.${sliderClasses.rail}`);

    if (!root || !rail) {
      return;
    }

    const measure = () => {
      const rootRect = root.getBoundingClientRect();
      const railRect = rail.getBoundingClientRect();

      if (railRect.width <= 0) {
        // Mid-mount / mid-transition: keep the last good inset rather than collapsing the strip.
        return;
      }

      const next: [number, number] = [
        Math.max(0, Math.round(railRect.left - rootRect.left)),
        Math.max(0, Math.round(rootRect.right - railRect.right)),
      ];

      setRailInsetsPx((current) =>
        current[0] === next[0] && current[1] === next[1] ? current : next,
      );
    };

    measure();

    const observer = new ResizeObserver(measure);
    observer.observe(root);
    observer.observe(rail);
    window.addEventListener("resize", measure);

    return () => {
      observer.disconnect();
      window.removeEventListener("resize", measure);
    };
  }, []);

  useEffect(() => {
    const intervalId = setInterval(() => {
      const [stripMinMillisLive, stripMaxMillisLive] = getStripWindowMillis();

      if (
        stripMinMillisLive !== stripMinMillis ||
        stripMaxMillisLive !== stripMaxMillis
      ) {
        setStripWindowMillis([stripMinMillisLive, stripMaxMillisLive]);
      }

      if (!slidByUser && stripMaxMillisLive - currentMillis > 1000) {
        setCurrentMillis(stripMaxMillisLive);
      }

      if (slidByUser && currentMillis > stripMaxMillisLive) {
        setSlidByUser(false);
      }
    }, 1000);

    return () => clearInterval(intervalId);
  }, [slidByUser, currentMillis, stripMinMillis, stripMaxMillis]);

  const { data: audioTimelineData } = useAudioTimeline(
    {
      cameraId: props.cameraId || undefined,
      // The strip window is hour-aligned, so its buckets line up with the view's rows exactly.
      startedAtGte: new Date(stripMinMillis).toISOString(),
      startedAtLt: new Date(stripMaxMillis).toISOString(),
      minScore: audioStripMinScore,
      limit: 5_000,
    },
    60_000,
  );

  const { data: detectionTimelineData } = useDetectionTimeline(
    {
      cameraId: props.cameraId || undefined,
      startedAtGte: new Date(stripMinMillis).toISOString(),
      startedAtLt: new Date(stripMaxMillis).toISOString(),
      minScore: detectionStripMinScore,
      limit: 5_000,
    },
    60_000,
  );

  const applyMillis = (millis: number) => {
    props.setDate(new Date(millis + dateSliderStepMillis).toISOString());
  };

  const seekFromStrip = (event: React.MouseEvent<HTMLDivElement>) => {
    const rect = event.currentTarget.getBoundingClientRect();

    if (rect.width <= 0) {
      return;
    }

    const fraction = Math.min(
      1,
      Math.max(0, (event.clientX - rect.left) / rect.width),
    );
    const millis =
      stripMinMillis +
      Math.round(fraction * stripIntervals) * dateSliderStepMillis;

    setSlidByUser(true);
    setCurrentMillis(millis);
    applyMillis(millis);
  };

  const audioBars = useMemo<StripBar[]>(() => {
    // The view is per camera-hour, so a bar sums the cameras it covers.
    const byIndex = new Map<
      number,
      { count: number; score: number; kind: string; prominence: number }
    >();

    (audioTimelineData?.objects || []).forEach((row) => {
      if (!row?.bucket || !row?.event_count) {
        return;
      }

      const index = Math.floor(
        (Date.parse(row.bucket) - stripMinMillis) / dateSliderStepMillis,
      );

      if (index < 0 || index >= stripIntervals) {
        return;
      }

      const bucket = byIndex.get(index) || {
        count: 0,
        score: 0,
        kind: "",
        prominence: 0,
      };

      bucket.count += row.event_count;
      bucket.prominence = Math.max(
        bucket.prominence,
        row.peak_prominence_db || 0,
      );

      if (row.top_score >= bucket.score) {
        bucket.score = row.top_score;
        bucket.kind = row.top_kind || bucket.kind;
      }

      byIndex.set(index, bucket);
    });

    return Array.from({ length: stripIntervals }, (_, index) => {
      const millis = stripMillis(stripMinMillis, index);
      const bucket = byIndex.get(index);

      if (!bucket) {
        return {
          index,
          millis,
          heightPercent: 0,
          color: "#ffffff14",
          innerHeightPercent: 0,
          innerColor: "#ffffff14",
          title: formatDateWithoutMillis(new Date(millis)),
        };
      }

      return {
        index,
        millis,
        heightPercent: stripBarHeight(bucket.count, audioStripFullCountEvents),
        color: `${audioScoreColor(bucket.score)}bf`,
        innerHeightPercent: 0,
        innerColor: audioScoreColor(bucket.score),
        title: `${formatDateWithoutMillis(new Date(millis))} | ${
          bucket.count
        } events | peak +${bucket.prominence.toFixed(1)} dB | score ${bucket.score.toFixed(
          2,
        )} (${bucket.kind})`,
      };
    });
  }, [audioTimelineData?.objects, stripMinMillis]);

  const detectionBars = useMemo<StripBar[]>(() => {
    const byIndex = new Map<
      number,
      { detections: number; objects: number; score: number; className: string }
    >();

    (detectionTimelineData?.objects || []).forEach((row) => {
      if (!row?.bucket) {
        return;
      }

      const index = Math.floor(
        (Date.parse(row.bucket) - stripMinMillis) / dateSliderStepMillis,
      );

      if (index < 0 || index >= stripIntervals) {
        return;
      }

      const bucket = byIndex.get(index) || {
        detections: 0,
        objects: 0,
        score: 0,
        className: "",
      };

      bucket.detections += row.detection_count || 0;
      bucket.objects += row.object_count || 0;

      if (row.top_score >= bucket.score) {
        bucket.score = row.top_score;
        bucket.className = row.top_class_name || bucket.className;
      }

      byIndex.set(index, bucket);
    });

    const { showDetections, showObjects } = props.overlayMode;

    return Array.from({ length: stripIntervals }, (_, index) => {
      const millis = stripMillis(stripMinMillis, index);
      const bucket = byIndex.get(index);
      const detections = showDetections ? bucket?.detections || 0 : 0;
      const objects = showObjects ? bucket?.objects || 0 : 0;

      if (!detections && !objects) {
        return {
          index,
          millis,
          heightPercent: 0,
          color: "#ffffff14",
          innerHeightPercent: 0,
          innerColor: "#ffffff14",
          title: formatDateWithoutMillis(new Date(millis)),
        };
      }

      const detectionHeight = stripBarHeight(
        detections,
        detectionStripFullCountDetections,
      );
      const objectHeight = stripBarHeight(
        objects,
        detectionStripFullCountObjects,
      );
      const heightPercent = Math.max(detectionHeight, objectHeight);
      const score = bucket?.score || 0;

      return {
        index,
        millis,
        heightPercent,
        color: `${audioScoreColor(score)}77`,
        innerHeightPercent: heightPercent
          ? (objectHeight / heightPercent) * 100
          : 0,
        innerColor: audioScoreColor(score),
        title: `${formatDateWithoutMillis(new Date(millis))} | ${detections} detections | ${
          objects
        } objects | score ${score.toFixed(2)} (${bucket?.className || "?"})`,
      };
    });
  }, [detectionTimelineData?.objects, props.overlayMode, stripMinMillis]);

  // Bar tooltips: the native `title` tooltip pops under the cursor, which is exactly where the
  // slider's value label sits. Draw ours above the strip instead, so the two never share a spot.
  const stripRef = useRef<HTMLDivElement | null>(null);
  const hoverTipRef = useRef<HTMLDivElement | null>(null);
  const [hoverBar, setHoverBar] = useState<{
    prefix: string;
    index: number;
  } | null>(null);

  const onStripMouseMove = (event: React.MouseEvent<HTMLDivElement>) => {
    const bar = (event.target as HTMLElement).closest("[data-bar-index]");
    const row = bar?.closest(
      '[data-test$="-strip-up"], [data-test$="-strip-down"]',
    );

    if (!bar || !row) {
      setHoverBar(null);

      return;
    }

    const prefix = (row.getAttribute("data-test") || "").split("-strip-")[0];
    const index = Number(bar.getAttribute("data-bar-index"));

    // Only the hovered bar matters, so a sweep across a bar does not re-render 168 of them.
    setHoverBar((previous) =>
      previous?.prefix === prefix && previous.index === index
        ? previous
        : { prefix, index },
    );
  };

  const hoverText = hoverBar
    ? (hoverBar.prefix === "audio" ? audioBars : detectionBars)[hoverBar.index]
        ?.title || ""
    : "";

  // Centred on the bar, then clamped into the strip's own x range whatever the text width is.
  useLayoutEffect(() => {
    const tip = hoverTipRef.current;
    const strip = stripRef.current;

    if (!tip || !strip || !hoverBar) {
      return;
    }

    const areaWidth = strip.getBoundingClientRect().width;
    const centre = ((hoverBar.index + 0.5) / stripIntervals) * areaWidth;
    const half = tip.offsetWidth / 2;

    tip.style.left = `${Math.min(
      Math.max(centre, half + 2),
      areaWidth - half - 2,
    )}px`;
  }, [hoverBar]);

  const renderStrip = (bars: StripBar[], prefix: string) => {
    const rows: ("up" | "down")[] = ["up", "down"];

    return rows.map((direction) => (
      <Box
        key={`${prefix}-strip-${direction}`}
        data-test={`${prefix}-strip-${direction}`}
        sx={{
          position: "relative",
          zIndex: 1,
          display: "flex",
          alignItems: direction === "up" ? "flex-end" : "flex-start",
          height: `${props.responsive ? stripRowHeightPx - 4 : stripRowHeightPx}px`,
        }}
      >
        {bars.map((bar) => (
          <Box
            key={`${prefix}-strip-${direction}-${bar.index}`}
            data-test={`${prefix}-bar`}
            data-bar-index={bar.index}
            data-bar-millis={bar.millis}
            // The tooltip text, on the bar, for the hover tip above the strip and for checks.
            data-bar-title={bar.title}
            sx={{
              position: "relative",
              flex: 1,
              mx: "0.5px",
              height: bar.heightPercent ? `${bar.heightPercent}%` : "2px",
              backgroundColor: bar.heightPercent ? bar.color : "#ffffff14",
              borderRadius: "1px",
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

  const spanMillis = stripMaxMillis - stripMinMillis;

  const percentOf = (millis: number): string => {
    const fraction = Math.min(
      1,
      Math.max(0, (millis - stripMinMillis) / spanMillis),
    );

    return `${(fraction * 100).toFixed(4)}%`;
  };

  const cursorPercent = percentOf(Math.min(currentMillis, stripMaxMillis));

  const visibleRange = props.visibleRangeMillis;

  // Snapped to the strip's own bars, so the box always covers whole bars: rows inside one hour read
  // as one bar wide, not as a sliver at the hour boundary.
  const boxBarIndex = (millis: number): number => {
    return Math.min(
      stripIntervals - 1,
      Math.max(0, Math.floor((millis - stripMinMillis) / dateSliderStepMillis)),
    );
  };

  const boxRangeMillis: [number, number] | undefined = visibleRange
    ? [
        stripMinMillis + boxBarIndex(visibleRange[0]) * dateSliderStepMillis,
        Math.min(
          stripMaxMillis,
          stripMinMillis +
            (boxBarIndex(visibleRange[1]) + 1) * dateSliderStepMillis,
        ),
      ]
    : undefined;

  return (
    <Box
      ref={rootRef}
      sx={{
        // Full width: the strip is 168 bars, so every pixel of width is click precision.
        width: "100%",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Box
        ref={stripRef}
        data-test="topbar-strip"
        onMouseDown={seekFromStrip}
        onClick={seekFromStrip}
        onMouseMove={onStripMouseMove}
        onMouseLeave={() => setHoverBar(null)}
        sx={{
          position: "relative",
          pl: `${railInsetsPx[0]}px`,
          pr: `${railInsetsPx[1]}px`,
          cursor: "text",
        }}
      >
        {boxRangeMillis && (
          <Box
            data-test="topbar-visible-box"
            sx={{
              position: "absolute",
              top: 0,
              bottom: 0,
              left: percentOf(boxRangeMillis[0]),
              width: `calc(${percentOf(boxRangeMillis[1])} - ${percentOf(
                boxRangeMillis[0],
              )})`,
              zIndex: 0,
              pointerEvents: "none",
              backgroundColor: "rgba(0, 0, 0, 0.05)",
              border: "1px solid rgba(0, 0, 0, 0.22)",
              borderRadius: "2px",
              boxSizing: "border-box",
            }}
          />
        )}
        {renderStrip(audioBars, "audio")}
        {renderStrip(detectionBars, "detection")}
        <Box
          data-test="topbar-cursor"
          sx={{
            position: "absolute",
            top: 0,
            bottom: 0,
            left: cursorPercent,
            width: "2px",
            zIndex: 2,
            pointerEvents: "none",
            backgroundColor: "rgba(0, 0, 0, 0.55)",
          }}
        />
        {hoverBar && (
          <Box
            ref={hoverTipRef}
            data-test="topbar-hover-tip"
            sx={{
              position: "absolute",
              // Above the strip: below it is the slider value label's space.
              bottom: "100%",
              mb: "4px",
              left: `${((hoverBar.index + 0.5) / stripIntervals) * 100}%`,
              transform: "translateX(-50%)",
              padding: "0.25rem 0.5rem",
              borderRadius: "6px",
              backgroundColor: "rgba(0, 0, 0, 0.75)",
              color: "#fff",
              fontSize: "0.75rem",
              lineHeight: 1.2,
              whiteSpace: "nowrap",
              pointerEvents: "none",
              zIndex: 3,
            }}
          >
            {hoverText}
          </Box>
        )}
      </Box>
      <Slider
        sx={{
          mt: "6px",
          // The value label hangs **below** the thumb (see the valueLabel overrides): the strip
          // bars sit above the slider, and above the thumb they covered the label. This bottom
          // margin is that label's room, so it does not paint over the table header.
          mb: "34px",
          [`& .${sliderClasses.valueLabel}`]: {
            transform:
              "translateX(-50%) translateY(calc(var(--Slider-thumbSize) + var(--Slider-valueLabelArrowSize) + 2px))",
            transformOrigin: "top center",
            padding: "0.25rem 0.5rem",
            borderRadius: "6px",
            backgroundColor: "rgba(0, 0, 0, 0.75)",
            color: "#fff",
            boxShadow: "none",
            whiteSpace: "nowrap",
          },
          // Joy's open-state rule (thumb hover, active, drag) re-points the label above the thumb,
          // straight onto the strip bars. Outrank it so the label stays below in every state.
          [`& .${sliderClasses.valueLabel}.${sliderClasses.valueLabelOpen}`]: {
            transform:
              "translateX(-50%) translateY(calc(var(--Slider-thumbSize) + var(--Slider-valueLabelArrowSize) + 2px))",
            transformOrigin: "top center",
          },
          [`& .${sliderClasses.valueLabel}::before`]: {
            content: '""',
            position: "absolute",
            left: "50%",
            bottom: "100%",
            transform: "translateX(-50%)",
            border: "5px solid transparent",
            borderBottomColor: "rgba(0, 0, 0, 0.75)",
          },
        }}
        size="lg"
        value={currentMillis}
        onChange={(_, value) => {
          setSlidByUser(true);

          if (typeof value === "number" && value !== currentMillis) {
            setCurrentMillis(value);
          }
        }}
        onChangeCommitted={(_, value) => {
          if (typeof value === "number") {
            applyMillis(value > stripMaxMillis ? stripMaxMillis : value);
          }
        }}
        step={dateSliderStepMillis}
        min={stripMinMillis}
        max={stripMaxMillis}
        valueLabelDisplay="auto"
        valueLabelFormat={(value: number): string => {
          if (value >= stripMaxMillis) {
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
