import { useEffect, useRef, useState } from "react";
import { useQuery } from "../api";
import { components } from "../api/api";
import { cameraOverlayOffsetMs, OverlayMode } from "../helpers";
import VideoJS from "./VideoJS";

type Detection = components["schemas"]["Detection"];
type Track = components["schemas"]["Track"];

type EnrichedDetection = Detection & {
  timestampMilliseconds: number;
};

type TrajectorySample = {
  seen_at?: string;
  x?: number;
  y?: number;
  bbox?: number[];
};

type TrackSample = {
  track: Track;
  sample: TrajectorySample;
};

type FrameDimensions = {
  width: number;
  height: number;
};

const fallbackFrameDimensions: FrameDimensions = {
  width: 3840,
  height: 2160,
};

const detectionFrameMatchWindowMilliseconds = 100;

function findClosestTimestamp(
  timestamps: number[],
  targetMilliseconds: number,
): number | undefined {
  let low = 0;
  let high = timestamps.length;

  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    if (timestamps[middle] < targetMilliseconds) {
      low = middle + 1;
    } else {
      high = middle;
    }
  }

  let closestTimestamp: number | undefined;
  let closestDistance = Number.POSITIVE_INFINITY;
  for (const index of [low - 1, low]) {
    if (index < 0 || index >= timestamps.length) {
      continue;
    }

    const timestamp = timestamps[index];
    const distance = Math.abs(timestamp - targetMilliseconds);
    if (distance < closestDistance) {
      closestTimestamp = timestamp;
      closestDistance = distance;
    }
  }

  return closestTimestamp;
}

function getLowResFileName(fileName: string | undefined): string | undefined {
  if (!fileName || fileName.includes("_low_res.")) {
    return undefined;
  }

  const extensionIndex = fileName.lastIndexOf(".");
  if (extensionIndex < 0) {
    return `${fileName}_low_res`;
  }

  return `${fileName.slice(0, extensionIndex)}_low_res${fileName.slice(extensionIndex)}`;
}

export interface VideoProps {
  video: components["schemas"]["Video"];
  width: number;
  height: number;
  overlayMode: OverlayMode;
}

export function Video(props: VideoProps) {
  const lowResFileName = getLowResFileName(props.video.file_name);
  const shouldFindLowResVideo =
    props.video.is_low_res !== true && Boolean(lowResFileName);

  // Detection runs against the low-resolution recording, while the UI plays
  // the matching high-resolution recording. Find that sibling so that we can
  // load its detections and use its start time for timeline matching.
  const { data: lowResVideosData, isLoading: isLowResVideoLoading } = useQuery(
    "get",
    "/api/videos",
    {
      params: {
        query: {
          file_name__eq: lowResFileName || "",
          is_low_res__eq: true,
          limit: 1,
        },
      },
    },
    { enabled: shouldFindLowResVideo },
  );

  const lowResVideo = lowResVideosData?.objects?.[0];
  const detectionVideo = lowResVideo || props.video;
  const detectionVideoId = detectionVideo.id || "";
  // Detection timestamps are absolute wall-clock times, stamped from the
  // low-resolution row the detector read. Matched segments line up frame for
  // frame across the two resolutions, so the anchor is that row's start plus
  // the measured per-camera content offset. The high-resolution row's start
  // carries the segment writer's open jitter - up to 2.2s of it.
  const playbackVideoStartedAtRef = useRef(
    Date.parse(detectionVideo.started_at || "") +
      cameraOverlayOffsetMs(detectionVideo.camera_id),
  );

  useEffect(() => {
    playbackVideoStartedAtRef.current =
      Date.parse(detectionVideo.started_at || "") +
      cameraOverlayOffsetMs(detectionVideo.camera_id);
  }, [detectionVideo.started_at, detectionVideo.camera_id]);

  const { isLoading, error, data } = useQuery(
    "get",
    "/api/detections",
    {
      params: {
        query: { video_id__eq: detectionVideoId, limit: 1_000_000 },
      },
    },
    {
      enabled:
        Boolean(detectionVideoId) &&
        (!shouldFindLowResVideo || !isLowResVideoLoading),
    },
  );

  const { data: tracksData, isLoading: isTracksLoading } = useQuery(
    "get",
    "/api/tracks",
    {
      params: {
        query: { video_id__eq: detectionVideoId, limit: 10_000 },
      },
    },
    {
      enabled:
        Boolean(detectionVideoId) &&
        props.overlayMode.showObjects &&
        (!shouldFindLowResVideo || !isLowResVideoLoading),
    },
  );

  const [lowResDimensions, setLowResDimensions] =
    useState<FrameDimensions | null>(null);
  const detectionFrameDimensionsRef = useRef<FrameDimensions>(
    fallbackFrameDimensions,
  );
  const detectingLowResVideoRef = useRef(Boolean(lowResVideo));

  useEffect(() => {
    setLowResDimensions(null);
  }, [lowResVideo?.file_name]);

  useEffect(() => {
    detectingLowResVideoRef.current = Boolean(lowResVideo);
    detectionFrameDimensionsRef.current =
      lowResDimensions || fallbackFrameDimensions;
  }, [lowResDimensions, lowResVideo]);

  const detectionsByTimestampRef = useRef<Map<number, EnrichedDetection[]>>(
    new Map(),
  );
  const detectionTimestampsRef = useRef<number[]>([]);
  const tracksByTimestampRef = useRef<Map<number, TrackSample[]>>(new Map());
  const trackTimestampsRef = useRef<number[]>([]);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const readyRef = useRef(false);
  const overlayModeRef = useRef(props.overlayMode);

  // video.js keeps the callback it was created with, so the draw loop reads
  // the toggles through a ref to pick up changes during playback.
  useEffect(() => {
    overlayModeRef.current = props.overlayMode;
  }, [props.overlayMode]);

  useEffect(() => {
    const tracksByTimestamp = new Map<number, TrackSample[]>();

    const tracks: Track[] = tracksData?.objects || [];
    tracks.forEach((track: Track) => {
      const samples = (track.trajectory as TrajectorySample[]) || [];

      samples.forEach((sample: TrajectorySample) => {
        const timestampMilliseconds = Date.parse(sample?.seen_at || "");
        if (!Number.isFinite(timestampMilliseconds)) {
          return;
        }

        const frameSamples = tracksByTimestamp.get(timestampMilliseconds) || [];
        frameSamples.push({ track, sample });
        tracksByTimestamp.set(timestampMilliseconds, frameSamples);
      });
    });

    tracksByTimestampRef.current = tracksByTimestamp;
    trackTimestampsRef.current = Array.from(tracksByTimestamp.keys()).sort(
      (a, b) => a - b,
    );
  }, [tracksData?.objects]);

  useEffect(() => {
    const detectionsByTimestamp = new Map<number, EnrichedDetection[]>();

    const detections: Detection[] = data?.objects || [];
    detections.forEach((detection: Detection) => {
      if (detection?.score === undefined) {
        return;
      }

      const timestampMilliseconds = Date.parse(detection?.seen_at || "");
      if (!Number.isFinite(timestampMilliseconds)) {
        return;
      }

      const enrichedDetection: EnrichedDetection = {
        ...detection,
        timestampMilliseconds,
      };
      const frameDetections =
        detectionsByTimestamp.get(timestampMilliseconds) || [];
      frameDetections.push(enrichedDetection);
      detectionsByTimestamp.set(timestampMilliseconds, frameDetections);
    });

    detectionsByTimestampRef.current = detectionsByTimestamp;
    detectionTimestampsRef.current = Array.from(
      detectionsByTimestamp.keys(),
    ).sort((a, b) => a - b);
  }, [data?.objects]);

  const lowResMetadataVideo = lowResVideo?.file_name ? (
    <video
      aria-hidden="true"
      muted
      playsInline
      preload="metadata"
      src={`/media/${lowResVideo.file_name}`}
      style={{
        position: "absolute",
        width: 1,
        height: 1,
        opacity: 0,
        pointerEvents: "none",
      }}
      onLoadedMetadata={(event) => {
        const target = event.currentTarget;
        if (target.videoWidth > 0 && target.videoHeight > 0) {
          setLowResDimensions({
            width: target.videoWidth,
            height: target.videoHeight,
          });
        } else {
          setLowResDimensions(fallbackFrameDimensions);
        }
      }}
      onError={() => {
        // Keep the overlay usable if the low-res media is no longer available;
        // detections still have the canonical fallback dimensions.
        setLowResDimensions(fallbackFrameDimensions);
      }}
    />
  ) : null;

  if (error) {
    console.warn(error);
    return (
      <div style={{ fontWeight: "bold", color: "red" }}>
        ERROR: {JSON.stringify(error)}
      </div>
    );
  }

  // Wait for the low-res metadata before starting playback so the first
  // rendered frame uses the right coordinate system rather than jumping later.
  // Only the first render waits: the overlay needs its coordinate system and,
  // in objects mode, its tracks before playback starts. Toggling a layer later
  // must not tear the player down mid-playback.
  if (
    isLoading ||
    (shouldFindLowResVideo && isLowResVideoLoading) ||
    (lowResVideo?.file_name && !lowResDimensions) ||
    (!readyRef.current && props.overlayMode.showObjects && isTracksLoading)
  ) {
    return lowResMetadataVideo;
  }

  return (
    <>
      {lowResMetadataVideo}
      <canvas
        ref={canvasRef}
        style={{
          position: "absolute",
          display: "block",
          zIndex: 500,
          left: 0,
          top: 0,
          width: 0,
          height: 0,
          cursor: "not-allowed",
          pointerEvents: "none",
        }}
        width={props.width}
        height={props.height}
      />
      <VideoJS
        options={{
          autoplay: true,
          controls: true,
          responsive: true,
          fluid: true,
          ratio: "16:9",
          inactivityTimeout: 0,
          playsinline: true,
          preload: "auto",
          enableSmoothSeeking: true,
          sources: [
            {
              src: `/media/${props.video.file_name}`,
              type: "video/mp4",
            },
          ],
        }}
        onReady={() => {
          readyRef.current = true;
        }}
        onTimeUpdate={(
          left: number,
          top: number,
          width: number,
          height: number,
          relativeTimeMilliseconds: number,
          sourceWidth: number,
          sourceHeight: number,
        ) => {
          if (!canvasRef.current) {
            return;
          }

          const canvas = canvasRef.current;

          // The canvas is an absolute child of the modal. Convert the video's
          // viewport rect to that containing block so the overlay stays aligned
          // when the modal is moved or resized.
          const offsetParentRect = canvas.offsetParent?.getBoundingClientRect();
          const parentLeft = offsetParentRect?.left || 0;
          const parentTop = offsetParentRect?.top || 0;
          canvas.style.left = `${left - parentLeft}px`;
          canvas.style.top = `${top - parentTop}px`;
          canvas.style.width = `${width}px`;
          canvas.style.height = `${height}px`;
          canvas.width = Math.max(1, Math.round(width));
          canvas.height = Math.max(1, Math.round(height));

          if (!readyRef.current) {
            return;
          }

          // When detections are for the high-res video, use the actual source
          // dimensions reported by video.js instead of a hard-coded resolution.
          if (
            !detectingLowResVideoRef.current &&
            sourceWidth > 0 &&
            sourceHeight > 0
          ) {
            detectionFrameDimensionsRef.current = {
              width: sourceWidth,
              height: sourceHeight,
            };
          }

          const ctx = canvas.getContext("2d");
          if (!ctx) {
            return;
          }

          (ctx as any).webkitImageSmoothingEnabled = false;
          (ctx as any).mozImageSmoothingEnabled = false;
          ctx.imageSmoothingEnabled = false;

          ctx.clearRect(0, 0, width, height);

          const frameDimensions = detectionFrameDimensionsRef.current;
          const coordinateScaleX = width / frameDimensions.width;
          const coordinateScaleY = height / frameDimensions.height;

          // Keep the visual weight from the original high-res overlay. The
          // coordinate transform above is for low-res detector coordinates;
          // stroke/font/radius sizing must not be multiplied by that factor.
          const visualScaleX = width / fallbackFrameDimensions.width;
          const visualScaleY = height / fallbackFrameDimensions.height;
          const playbackVideoStartedAt = playbackVideoStartedAtRef.current;
          if (!Number.isFinite(playbackVideoStartedAt)) {
            return;
          }

          // Match playback to the closest detector frame. Index the timestamps
          // once when the query arrives rather than scanning and parsing every
          // detection on every animation frame. This matters for busy videos
          // with thousands of boxes and avoids burst/freeze behavior caused by
          // main-thread work competing with video playback.
          const targetTimestamp =
            playbackVideoStartedAt + relativeTimeMilliseconds;
          const overlay = overlayModeRef.current;

          if (overlay.showDetections) {
            const matchedTimestamp = findClosestTimestamp(
              detectionTimestampsRef.current,
              targetTimestamp,
            );
            const matchedDetections =
              matchedTimestamp === undefined ||
              Math.abs(matchedTimestamp - targetTimestamp) >
                detectionFrameMatchWindowMilliseconds
                ? []
                : detectionsByTimestampRef.current.get(matchedTimestamp) || [];
            matchedDetections.forEach((detection: EnrichedDetection) => {
              const topLeft = detection.bounding_box?.[0];
              const bottomRight = detection.bounding_box?.[2];
              const centroid = detection.centroid;

              if (topLeft?.X === undefined || topLeft?.Y === undefined) {
                return;
              }

              if (
                bottomRight?.X === undefined ||
                bottomRight?.Y === undefined
              ) {
                return;
              }

              if (centroid?.X === undefined || centroid?.Y === undefined) {
                return;
              }

              if (detection?.score === undefined) {
                return;
              }

              const lineWidth = 4 * visualScaleX;
              const textOffsetX = 6 * visualScaleX;
              const textOffsetY = 6 * visualScaleY;
              const centroidRadius = 9 * visualScaleX;

              const topLeftX = topLeft.X * coordinateScaleX;
              const topLeftY = topLeft.Y * coordinateScaleY;

              const bottomRightX = bottomRight.X * coordinateScaleX;
              const bottomRightY = bottomRight.Y * coordinateScaleY;

              const centroidX = centroid.X * coordinateScaleX;
              const centroidY = centroid.Y * coordinateScaleY;

              const grad = ctx.createLinearGradient(
                topLeftX,
                topLeftY,
                bottomRightX,
                bottomRightY,
              );
              grad.addColorStop(0.0, `rgba(255, 31, 31, 1.0)`);
              grad.addColorStop(0.2, `rgba(255, 255, 31, 1.0)`);
              grad.addColorStop(0.6, `rgba(31, 255, 31, 1.0)`);
              grad.addColorStop(0.8, `rgba(255, 31, 255, 1.0)`);
              grad.addColorStop(1.0, `rgba(31, 31, 255, 1.0)`);

              ctx.lineWidth = lineWidth;
              ctx.strokeStyle = grad;

              ctx.fillStyle = `rgba(255, 255, 255, 0.99)`;
              ctx.font = `${32 * visualScaleX}px monospace`;
              ctx.textAlign = "left";
              ctx.textRendering = "optimizeLegibility";

              ctx.fillText(
                `${detection.class_name} @ ${detection.score.toFixed(2)}`,
                topLeftX + textOffsetX,
                bottomRightY - textOffsetY,
              );

              ctx.strokeRect(
                topLeftX,
                topLeftY,
                Math.abs(bottomRightX - topLeftX),
                Math.abs(bottomRightY - topLeftY),
              );

              ctx.strokeStyle = `rgba(255, 255, 255, 0.99)`;
              ctx.beginPath();
              ctx.arc(centroidX, centroidY, centroidRadius, 0, Math.PI * 2);
              ctx.stroke();
            });
          }

          // Tracked objects draw on top of raw detections: the box is the
          // object's widest sighting, so a person stays readable in a crowd.
          if (overlay.showObjects) {
            const matchedTrackTimestamp = findClosestTimestamp(
              trackTimestampsRef.current,
              targetTimestamp,
            );
            const matchedSamples =
              matchedTrackTimestamp === undefined ||
              Math.abs(matchedTrackTimestamp - targetTimestamp) >
                detectionFrameMatchWindowMilliseconds
                ? []
                : tracksByTimestampRef.current.get(matchedTrackTimestamp) || [];

            matchedSamples.forEach(({ track, sample }: TrackSample) => {
              const bbox = sample.bbox || [];

              if (bbox.length < 4) {
                return;
              }

              const [x0, y0, x1, y1] = bbox;
              const lineWidth = 4 * visualScaleX;
              const textOffsetX = 6 * visualScaleX;
              const centroidRadius = 6 * visualScaleX;

              const topLeftX = x0 * coordinateScaleX;
              const topLeftY = y0 * coordinateScaleY;
              const bottomRightX = x1 * coordinateScaleX;
              const bottomRightY = y1 * coordinateScaleY;

              ctx.lineWidth = lineWidth;
              ctx.strokeStyle = track.is_static
                ? `rgba(140, 170, 255, 0.9)`
                : `rgba(0, 220, 255, 1)`;

              ctx.strokeRect(
                topLeftX,
                topLeftY,
                Math.abs(bottomRightX - topLeftX),
                Math.abs(bottomRightY - topLeftY),
              );

              ctx.fillStyle = `rgba(255, 255, 255, 0.99)`;
              ctx.font = `${32 * visualScaleX}px monospace`;
              ctx.textAlign = "left";
              ctx.textRendering = "optimizeLegibility";

              ctx.fillText(
                `${track.class_name} #${(track.key || track.id || "").slice(0, 4)} @ ${(track.mean_score || 0).toFixed(2)}`,
                topLeftX + textOffsetX,
                topLeftY + 32 * visualScaleY,
              );

              if (sample.x !== undefined && sample.y !== undefined) {
                ctx.beginPath();
                ctx.arc(
                  sample.x * coordinateScaleX,
                  sample.y * coordinateScaleY,
                  centroidRadius,
                  0,
                  Math.PI * 2,
                );
                ctx.stroke();
              }
            });
          }
        }}
      />
    </>
  );
}
