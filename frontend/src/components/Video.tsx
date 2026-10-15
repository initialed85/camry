import { useEffect, useRef, useState } from "react";
import { useQuery } from "../api";
import { components } from "../api/api";
import { OverlayMode } from "../helpers";
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

// The overlay canvas is CSS-scaled to the video, so its backing store can be
// smaller than the video's on-screen box. Zooming scales that box, and an
// uncapped backing store at 4x is well past the canvas area limit on iOS.
const maxCanvasWidth = 2048;

const minZoomScale = 1;
const maxZoomScale = 4;

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
  // The detector correlates the high-resolution recording against the
  // low-resolution recording its detections came from and stores the offset it
  // ended up using on the row: the measured content offset, or the camera's
  // constant when the scene had nothing moving to align. A row that carries its
  // own detections needs no offset: both timelines are the same recorder.
  const overlayOffsetMs = lowResVideo?.overlay_offset_ms ?? 0;
  const playbackVideoStartedAtRef = useRef(
    Date.parse(detectionVideo.started_at || "") + overlayOffsetMs,
  );

  useEffect(() => {
    playbackVideoStartedAtRef.current =
      Date.parse(detectionVideo.started_at || "") + overlayOffsetMs;
  }, [detectionVideo.started_at, overlayOffsetMs]);

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
  const stageFrameRef = useRef<HTMLDivElement | null>(null);
  // The stage only exists once the overlay has its coordinate system, so keep
  // it in state: the gesture wiring has to re-run when it mounts.
  const [stage, setStage] = useState<HTMLDivElement | null>(null);
  const zoomRef = useRef({ scale: 1, x: 0, y: 0 });
  const readyRef = useRef(false);
  const overlayModeRef = useRef(props.overlayMode);

  // video.js keeps the callback it was created with, so the draw loop reads
  // the toggles through a ref to pick up changes during playback.
  useEffect(() => {
    overlayModeRef.current = props.overlayMode;
  }, [props.overlayMode]);

  // Zoom belongs to the media, not to the app: the chrome is pinned by the
  // viewport meta tag and touch-action, and the stage runs its own pinch. The
  // overlay follows for free, because the canvas is re-positioned from the
  // video's rect on every animation frame.
  useEffect(() => {
    const frame = stageFrameRef.current;

    if (!stage || !frame) {
      return;
    }

    const applyZoom = () => {
      const { scale, x, y } = zoomRef.current;
      frame.style.transform = `translate(${x}px, ${y}px) scale(${scale})`;

      // The player controls live inside the scaled frame, so at 3x they are
      // dragged off the stage. Counter-translate them (no scaling) to keep the
      // control bar pinned to the bottom of the stage at its own size.
      const bar = stage.querySelector<HTMLElement>(".vjs-control-bar");

      if (bar) {
        // The bar's own layout box sits at the bottom of the scaled frame, so
        // undo the frame transform at that box: it lands back at the bottom of
        // the stage, at its unscaled size.
        const left = -x / scale - bar.offsetLeft;
        const top =
          (stage.clientHeight - bar.offsetHeight - y) / scale - bar.offsetTop;

        bar.style.transformOrigin = "0 0";
        bar.style.transform = `translate(${left.toFixed(3)}px, ${top.toFixed(
          3,
        )}px) scale(${(1 / scale).toFixed(5)})`;
      }
    };

    const clampZoom = () => {
      const zoom = zoomRef.current;

      zoom.x = Math.min(
        0,
        Math.max(stage.clientWidth * (1 - zoom.scale), zoom.x),
      );
      zoom.y = Math.min(
        0,
        Math.max(stage.clientHeight * (1 - zoom.scale), zoom.y),
      );
    };

    const zoomAround = (nextScale: number, pointX: number, pointY: number) => {
      const zoom = zoomRef.current;
      const scale = Math.min(maxZoomScale, Math.max(minZoomScale, nextScale));
      const ratio = scale / zoom.scale;

      zoom.x = pointX - ratio * (pointX - zoom.x);
      zoom.y = pointY - ratio * (pointY - zoom.y);
      zoom.scale = scale;

      clampZoom();
      applyZoom();
    };

    const stagePoint = (clientX: number, clientY: number) => {
      const rect = stage.getBoundingClientRect();

      return { x: clientX - rect.left, y: clientY - rect.top };
    };

    const spread = (touches: TouchList) =>
      Math.hypot(
        touches[0].clientX - touches[1].clientX,
        touches[0].clientY - touches[1].clientY,
      );

    let pinching = false;
    let pinchStartSpread = 0;
    let pinchStartScale = minZoomScale;
    let panning = false;
    let panStart = { x: 0, y: 0 };

    const onTouchStart = (event: TouchEvent) => {
      if (event.touches.length === 2) {
        pinching = true;
        pinchStartSpread = spread(event.touches);
        pinchStartScale = zoomRef.current.scale;
        return;
      }

      // One finger pans the zoomed media. The player controls stay tappable
      // while zoomed, so a drag that starts on the control bar is left alone.
      const onControls = (event.target as HTMLElement | null)?.closest?.(
        ".vjs-control-bar",
      );

      if (
        event.touches.length === 1 &&
        zoomRef.current.scale > minZoomScale &&
        !onControls
      ) {
        panning = true;
        panStart = {
          x: event.touches[0].clientX - zoomRef.current.x,
          y: event.touches[0].clientY - zoomRef.current.y,
        };
      }
    };

    const onTouchMove = (event: TouchEvent) => {
      if (pinching && event.touches.length >= 2 && pinchStartSpread > 0) {
        event.preventDefault();

        const centre = stagePoint(
          (event.touches[0].clientX + event.touches[1].clientX) / 2,
          (event.touches[0].clientY + event.touches[1].clientY) / 2,
        );

        zoomAround(
          pinchStartScale * (spread(event.touches) / pinchStartSpread),
          centre.x,
          centre.y,
        );
        return;
      }

      if (panning && event.touches.length === 1) {
        event.preventDefault();

        zoomRef.current.x = event.touches[0].clientX - panStart.x;
        zoomRef.current.y = event.touches[0].clientY - panStart.y;

        clampZoom();
        applyZoom();
      }
    };

    const onTouchEnd = (event: TouchEvent) => {
      if (event.touches.length < 2) {
        pinching = false;
      }

      if (event.touches.length === 0) {
        panning = false;
      }
    };

    const onWheel = (event: WheelEvent) => {
      // Desktop parity with the pinch gesture: shift + wheel zooms the media.
      if (!event.shiftKey) {
        return;
      }

      event.preventDefault();

      const point = stagePoint(event.clientX, event.clientY);

      zoomAround(
        zoomRef.current.scale * (event.deltaY > 0 ? 0.9 : 1.1),
        point.x,
        point.y,
      );
    };

    stage.addEventListener("touchstart", onTouchStart, { passive: false });
    stage.addEventListener("touchmove", onTouchMove, { passive: false });
    stage.addEventListener("touchend", onTouchEnd);
    stage.addEventListener("touchcancel", onTouchEnd);
    stage.addEventListener("wheel", onWheel, { passive: false });

    return () => {
      stage.removeEventListener("touchstart", onTouchStart);
      stage.removeEventListener("touchmove", onTouchMove);
      stage.removeEventListener("touchend", onTouchEnd);
      stage.removeEventListener("touchcancel", onTouchEnd);
      stage.removeEventListener("wheel", onWheel);
    };
  }, [props.video.id, stage]);

  useEffect(() => {
    zoomRef.current = { scale: 1, x: 0, y: 0 };

    if (stageFrameRef.current) {
      stageFrameRef.current.style.transform = "none";
    }

    const bar = stage?.querySelector<HTMLElement>(".vjs-control-bar");

    if (bar) {
      bar.style.transform = "";
      bar.style.transformOrigin = "";
    }
  }, [props.video.id, stage]);

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
      <div
        ref={setStage}
        style={{
          position: "absolute",
          left: 0,
          top: 0,
          width: "100%",
          height: "100%",
          overflow: "hidden",
          touchAction: "pan-x pan-y",
        }}
      >
        <div
          ref={stageFrameRef}
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            width: "100%",
            height: "100%",
            transformOrigin: "0 0",
          }}
        >
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
              const offsetParentRect =
                canvas.offsetParent?.getBoundingClientRect();
              const parentLeft = offsetParentRect?.left || 0;
              const parentTop = offsetParentRect?.top || 0;
              canvas.style.left = `${left - parentLeft}px`;
              canvas.style.top = `${top - parentTop}px`;
              canvas.style.width = `${width}px`;
              canvas.style.height = `${height}px`;

              // The overlay is drawn at the capped resolution and upscaled by CSS
              // to the video box, so zooming scales the drawing instead of growing
              // the backing store without limit.
              const backingWidth = Math.min(
                maxCanvasWidth,
                Math.max(1, Math.round(width)),
              );
              const backingHeight = Math.max(
                1,
                Math.round((height * backingWidth) / Math.max(1, width)),
              );

              if (canvas.width !== backingWidth) {
                canvas.width = backingWidth;
              }

              if (canvas.height !== backingHeight) {
                canvas.height = backingHeight;
              }

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

              ctx.clearRect(0, 0, canvas.width, canvas.height);

              const frameDimensions = detectionFrameDimensionsRef.current;
              const coordinateScaleX = canvas.width / frameDimensions.width;
              const coordinateScaleY = canvas.height / frameDimensions.height;

              // Keep the visual weight from the original high-res overlay. The
              // coordinate transform above is for low-res detector coordinates;
              // stroke/font/radius sizing must not be multiplied by that factor.
              const visualScaleX = canvas.width / fallbackFrameDimensions.width;
              const visualScaleY =
                canvas.height / fallbackFrameDimensions.height;
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
                    : detectionsByTimestampRef.current.get(matchedTimestamp) ||
                      [];
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
                    : tracksByTimestampRef.current.get(matchedTrackTimestamp) ||
                      [];

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
        </div>
      </div>
    </>
  );
}
