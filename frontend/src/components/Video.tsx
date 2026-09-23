import { useEffect, useRef, useState } from "react";
import { useQuery } from "../api";
import { components } from "../api/api";
import VideoJS from "./VideoJS";

type Detection = components["schemas"]["Detection"];

type Point = components["schemas"]["Detection"]["centroid"];

type FrameDimensions = {
  width: number;
  height: number;
};

const fallbackFrameDimensions: FrameDimensions = {
  width: 3840,
  height: 2160,
};

const detectionFrameMatchWindowMilliseconds = 100;

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
  const detectionVideoStartedAtRef = useRef(
    Date.parse(detectionVideo.started_at || props.video.started_at || ""),
  );

  useEffect(() => {
    detectionVideoStartedAtRef.current = Date.parse(
      detectionVideo.started_at || props.video.started_at || "",
    );
  }, [detectionVideo.started_at, props.video.started_at]);

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

  const enrichedDetectionsRef = useRef<Detection[]>([]);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const readyRef = useRef(false);

  useEffect(() => {
    const enrichedDetections: Detection[] = [];

    const detections: Detection[] = data?.objects || [];
    detections.forEach((detection: Detection) => {
      if (detection?.score === undefined) {
        return;
      }

      const boundingBoxPoints: Point[] = [];
      detection?.bounding_box?.forEach((point) => {
        boundingBoxPoints.push(point);
      });

      const centroidPoint = detection?.centroid;

      const enrichedDetection = {
        ...detection,
        timestampMilliseconds: Date.parse(detection?.seen_at || ""),
        boundingBoxPoints,
        centroidPoint,
      };

      enrichedDetections.push(enrichedDetection);
    });

    enrichedDetectionsRef.current = enrichedDetections;
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
  if (
    isLoading ||
    (shouldFindLowResVideo && isLowResVideoLoading) ||
    (lowResVideo?.file_name && !lowResDimensions)
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
          const detectionVideoStartedAt = detectionVideoStartedAtRef.current;

          const enrichedDetections = enrichedDetectionsRef.current || [];

          // Match playback to the closest detector frame. Even at full frame
          // rate, the browser clock usually falls between frame timestamps;
          // all detections from the matched frame share its timestamp.
          let closestDetectionTimestamp: number | undefined;
          let closestDetectionAge = Number.POSITIVE_INFINITY;
          enrichedDetections.forEach((detection: Detection) => {
            const detectionTimestamp = Date.parse(detection.seen_at || "");
            const detectionRelativeTimeMilliseconds =
              detectionTimestamp - detectionVideoStartedAt;
            const deltaMilliseconds =
              relativeTimeMilliseconds - detectionRelativeTimeMilliseconds;
            const age = Math.abs(deltaMilliseconds);

            if (
              Number.isFinite(deltaMilliseconds) &&
              age <= detectionFrameMatchWindowMilliseconds &&
              age < closestDetectionAge
            ) {
              closestDetectionTimestamp = detectionTimestamp;
              closestDetectionAge = age;
            }
          });

          if (closestDetectionTimestamp === undefined) {
            return;
          }
          const matchedTimestamp = closestDetectionTimestamp;

          enrichedDetections.forEach((detection: Detection) => {
            const detectionTimestamp = Date.parse(detection.seen_at || "");
            if (
              !Number.isFinite(detectionTimestamp) ||
              Math.abs(detectionTimestamp - matchedTimestamp) > 2
            ) {
              return;
            }

            const topLeft = detection.bounding_box?.[0];
            const bottomRight = detection.bounding_box?.[2];
            const centroid = detection.centroid;

            if (topLeft?.X === undefined || topLeft?.Y === undefined) {
              return;
            }

            if (bottomRight?.X === undefined || bottomRight?.Y === undefined) {
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
        }}
      />
    </>
  );
}
