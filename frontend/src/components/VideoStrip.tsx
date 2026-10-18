import Box from "@mui/joy/Box";
import { useMemo } from "react";
import { components } from "../api/api";
import {
  videoStripBucketMillis,
  videoStripFullCountDetections,
  videoStripFullCountEvents,
  videoStripFullCountObjects,
  videoStripMaxBuckets,
  videoStripRowHeightPx,
} from "../config";
import { audioScoreColor, OverlayMode } from "../helpers";

type Detection = components["schemas"]["Detection"];
type Track = components["schemas"]["Track"];

interface VideoStripProps {
  responsive: boolean;
  // Everything is relative to the playback timeline: the same zero the overlay boxes use.
  durationMillis: number;
  playbackStartedAtMillis: number;
  audioSummary: any[];
  detections: Detection[];
  tracks: Track[];
  overlayMode: OverlayMode;
  currentTimeMillis: number;
  onSeek: (millis: number) => void;
}

interface AudioSummaryItem {
  offset_ms?: number;
  seen_at?: string;
  kind?: string;
  score?: number;
  prominence_db?: number;
}

interface StripBar {
  index: number;
  heightPercent: number;
  color: string;
  innerHeightPercent: number;
  innerColor: string;
  title: string;
}

const barHeight = (count: number, fullCount: number): number => {
  return count > 0 ? 10 + Math.min(1, count / fullCount) * 90 : 0;
};

const emptyBar = (index: number): StripBar => ({
  index,
  heightPercent: 0,
  color: "#ffffff14",
  innerHeightPercent: 0,
  innerColor: "#ffffff14",
  title: "",
});

// The strip is the video's own timeline: one bar per second, capped so a long recording stays
// readable. Bars are mirrored around the strip's centre line, as in the top bar.
const bucketLayout = (durationMillis: number) => {
  const bucketMillis = Math.max(
    videoStripBucketMillis,
    Math.ceil(durationMillis / videoStripMaxBuckets),
  );
  const count = Math.max(
    1,
    Math.min(videoStripMaxBuckets, Math.ceil(durationMillis / bucketMillis)),
  );

  return { bucketMillis, count };
};

export default function VideoStrip(props: VideoStripProps) {
  const { bucketMillis, count: bucketTotal } = bucketLayout(
    props.durationMillis,
  );

  const audioBars = useMemo<StripBar[]>(() => {
    const counts = new Array<number>(bucketTotal).fill(0);
    const scores = new Array<number>(bucketTotal).fill(0);
    const kinds = new Array<string>(bucketTotal).fill("");

    ((props.audioSummary as AudioSummaryItem[]) || []).forEach((event) => {
      const offsetMillis = event?.offset_ms;
      if (offsetMillis === undefined || offsetMillis < 0) {
        return;
      }

      const index = Math.floor(offsetMillis / bucketMillis);
      if (index < 0 || index >= bucketTotal) {
        return;
      }

      counts[index] += 1;
      if ((event?.score || 0) >= scores[index]) {
        scores[index] = event?.score || 0;
        kinds[index] = event?.kind || kinds[index];
      }
    });

    return Array.from({ length: bucketTotal }, (_, index) => {
      if (!counts[index]) {
        return emptyBar(index);
      }

      return {
        index,
        heightPercent: barHeight(counts[index], videoStripFullCountEvents),
        color: `${audioScoreColor(scores[index])}bf`,
        innerHeightPercent: 0,
        innerColor: audioScoreColor(scores[index]),
        title: `+${((index * bucketMillis) / 1000).toFixed(0)}s | ${
          counts[index]
        } events | score ${scores[index].toFixed(2)} (${kinds[index]})`,
      };
    });
  }, [props.audioSummary, bucketMillis, bucketTotal]);

  const detectionBars = useMemo<StripBar[]>(() => {
    const detectionCounts = new Array<number>(bucketTotal).fill(0);
    const objectCounts = new Array<number>(bucketTotal).fill(0);
    const scores = new Array<number>(bucketTotal).fill(0);
    const classNames = new Array<string>(bucketTotal).fill("");

    props.detections.forEach((detection) => {
      const seenAtMillis = Date.parse(detection?.seen_at || "");
      if (!Number.isFinite(seenAtMillis)) {
        return;
      }

      const index = Math.floor(
        (seenAtMillis - props.playbackStartedAtMillis) / bucketMillis,
      );
      if (index < 0 || index >= bucketTotal) {
        return;
      }

      detectionCounts[index] += 1;
      if ((detection?.score || 0) >= scores[index]) {
        scores[index] = detection?.score || 0;
        classNames[index] = detection?.class_name || classNames[index];
      }
    });

    // Objects are counted where they are visible, so a track spans its buckets.
    props.tracks.forEach((track) => {
      const startedAtMillis = Date.parse(track?.started_at || "");
      if (!Number.isFinite(startedAtMillis)) {
        return;
      }

      const parsedEndedAtMillis = Date.parse(track?.ended_at || "");
      const endedAtMillis = Number.isFinite(parsedEndedAtMillis)
        ? parsedEndedAtMillis
        : startedAtMillis;

      const firstIndex = Math.max(
        0,
        Math.floor(
          (startedAtMillis - props.playbackStartedAtMillis) / bucketMillis,
        ),
      );
      const lastIndex = Math.min(
        bucketTotal - 1,
        Math.floor(
          (endedAtMillis - props.playbackStartedAtMillis) / bucketMillis,
        ),
      );

      for (let index = firstIndex; index <= lastIndex; index += 1) {
        objectCounts[index] += 1;
        if ((track?.max_score || 0) >= scores[index]) {
          scores[index] = track?.max_score || 0;
          classNames[index] = track?.class_name || classNames[index];
        }
      }
    });

    const { showDetections, showObjects } = props.overlayMode;

    return Array.from({ length: bucketTotal }, (_, index) => {
      const detections = showDetections ? detectionCounts[index] : 0;
      const objects = showObjects ? objectCounts[index] : 0;

      if (!detections && !objects) {
        return emptyBar(index);
      }

      const detectionHeight = barHeight(
        detections,
        videoStripFullCountDetections,
      );
      const objectHeight = barHeight(objects, videoStripFullCountObjects);
      const heightPercent = Math.max(detectionHeight, objectHeight);

      return {
        index,
        heightPercent,
        color: `${audioScoreColor(scores[index])}77`,
        innerHeightPercent:
          heightPercent > 0 ? (objectHeight / heightPercent) * 100 : 0,
        innerColor: audioScoreColor(scores[index]),
        title: `+${((index * bucketMillis) / 1000).toFixed(0)}s | ${detections} detections | ${
          objects
        } objects | score ${scores[index].toFixed(2)} (${classNames[index]})`,
      };
    });
  }, [
    props.detections,
    props.tracks,
    props.overlayMode,
    props.playbackStartedAtMillis,
    bucketMillis,
    bucketTotal,
  ]);

  const renderStrip = (bars: StripBar[], prefix: string) => {
    const rows: ("up" | "down")[] = ["up", "down"];

    return rows.map((direction) => (
      <Box
        key={`${prefix}-strip-${direction}`}
        sx={{
          display: "flex",
          alignItems: direction === "up" ? "flex-end" : "flex-start",
          height: `${props.responsive ? videoStripRowHeightPx - 4 : videoStripRowHeightPx}px`,
        }}
      >
        {bars.map((bar) => (
          <Box
            key={`${prefix}-strip-${direction}-${bar.index}`}
            title={bar.title}
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

  const seekFromEvent = (event: React.MouseEvent<HTMLDivElement>) => {
    const rect = event.currentTarget.getBoundingClientRect();
    if (rect.width <= 0) {
      return;
    }

    const fraction = (event.clientX - rect.left) / rect.width;
    props.onSeek(fraction * props.durationMillis);
  };

  const playheadPercent =
    props.durationMillis > 0
      ? Math.min(100, (props.currentTimeMillis / props.durationMillis) * 100)
      : 0;

  const rowHeight = props.responsive
    ? videoStripRowHeightPx - 4
    : videoStripRowHeightPx;

  return (
    <Box
      onMouseDown={seekFromEvent}
      title="Click to seek"
      sx={{
        position: "absolute",
        left: 0,
        right: 0,
        // Above the video.js control bar (.vjs-control-bar is 3em tall) so its buttons stay clickable.
        bottom: "30px",
        // The strip is video chrome: translucent so the footage under it stays readable.
        backgroundColor: "rgba(0, 0, 0, 0.35)",
        display: "flex",
        flexDirection: "row",
        alignItems: "stretch",
        px: "2px",
        pb: "2px",
        zIndex: 400,
        cursor: "text",
      }}
    >
      <Box
        sx={{
          width: "34px",
          flexShrink: 0,
          fontSize: "10px",
          lineHeight: `${rowHeight * 2}px`,
          color: "rgba(255, 255, 255, 0.75)",
          textAlign: "right",
          pr: "4px",
          userSelect: "none",
        }}
      >
        S
        <br />
        D/O
      </Box>
      <Box sx={{ flex: 1, position: "relative" }}>
        {renderStrip(audioBars, "audio")}
        {renderStrip(detectionBars, "detection")}
        <Box
          sx={{
            position: "absolute",
            top: 0,
            bottom: 0,
            left: `${playheadPercent}%`,
            width: "2px",
            backgroundColor: "rgba(255, 255, 255, 0.9)",
            pointerEvents: "none",
          }}
        />
      </Box>
    </Box>
  );
}
