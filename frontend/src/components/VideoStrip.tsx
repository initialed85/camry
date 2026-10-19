import Box from "@mui/joy/Box";
import { useMemo } from "react";
import { VideoTimelineBucket } from "../api/videoTimeline";
import {
  videoStripBucketMillis,
  videoStripFullCountDetections,
  videoStripFullCountEvents,
  videoStripFullCountObjects,
  videoStripMaxBuckets,
  videoStripRowHeightPx,
} from "../config";
import { audioScoreColor, OverlayMode } from "../helpers";

interface VideoStripProps {
  responsive: boolean;
  // Everything is relative to the playback timeline: the same zero the overlay boxes use, and the
  // same bucket width the strip endpoint was queried with.
  durationMillis: number;
  playbackStartedAtMillis: number;
  audioSummary: any[];
  buckets: VideoTimelineBucket[];
  overlayMode: OverlayMode;
  currentTimeMillis: number;
  // The video.js scrubber's own box inside the stage. The strip shares it, so the playhead lands
  // exactly above the scrubber handle and a click here is a click on the scrubber.
  insetLeftPx: number;
  insetRightPx: number;
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
// readable. Bars are mirrored around the strip's centre line, as in the top bar. The querying
// component uses this too, so the endpoint buckets and the drawn bars are the same widths.
export const bucketLayout = (durationMillis: number) => {
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

    // The endpoint buckets against the strip's own zero, so a bar index is exact.
    props.buckets.forEach((row) => {
      const index = Math.round(
        (row.bucket_ms - props.playbackStartedAtMillis) / bucketMillis,
      );
      if (index < 0 || index >= bucketTotal) {
        return;
      }

      detectionCounts[index] += row.detection_count;
      objectCounts[index] += row.object_count;

      if ((row.top_score || 0) >= scores[index]) {
        scores[index] = row.top_score || 0;
        classNames[index] = row.top_class_name || classNames[index];
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
    props.buckets,
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
        data-test={`${prefix}-strip-${direction}`}
        sx={{
          display: "flex",
          alignItems: direction === "up" ? "flex-end" : "flex-start",
          height: `${props.responsive ? videoStripRowHeightPx - 4 : videoStripRowHeightPx}px`,
        }}
      >
        {bars.map((bar) => (
          <Box
            key={`${prefix}-strip-${direction}-${bar.index}`}
            data-test={`${prefix}-bar`}
            data-bar-index={bar.index}
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

  return (
    <Box
      onMouseDown={seekFromEvent}
      onClick={seekFromEvent}
      title="Click to seek"
      data-test="modal-strip"
      sx={{
        position: "absolute",
        // The scrubber's box, so bar index, click fraction, playhead and scrubber handle all share
        // one width.
        left: `${props.insetLeftPx}px`,
        right: `${props.insetRightPx}px`,
        // Above the video.js control bar (.vjs-control-bar is 3em tall) so its buttons stay clickable.
        bottom: "30px",
        // The strip is video chrome: translucent so the footage under it stays readable.
        backgroundColor: "rgba(0, 0, 0, 0.35)",
        pb: "2px",
        zIndex: 400,
        cursor: "text",
      }}
    >
      {renderStrip(audioBars, "audio")}
      {renderStrip(detectionBars, "detection")}
      <Box
        data-test="modal-playhead"
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
  );
}
