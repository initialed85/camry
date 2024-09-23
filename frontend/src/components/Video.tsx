import { useEffect, useRef } from "react";
import { useQuery } from "../api";
import { components } from "../api/api";
import VideoJS from "./VideoJS";

type Detection = components["schemas"]["Detection"];

type Point = components["schemas"]["Detection"]["centroid"];

export interface VideoProps {
  video: components["schemas"]["Video"];
  width: number;
  height: number;
}

export function Video(props: VideoProps) {
  const { isLoading, error, data } = useQuery("get", "/api/detections", {
    params: {
      query: { video_id__eq: props.video?.id || "", limit: 1_000_000 },
    },
  });

  const enrichedDetectionsRef = useRef<components["schemas"]["Detection"][]>(
    [],
  );
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const readyRef = useRef(false);

  useEffect(() => {
    if (enrichedDetectionsRef.current?.length) {
      return;
    }

    let enrichedDetections: Detection[] = [];

    const detections: Detection[] = data?.objects || [];
    detections.forEach((detection: Detection) => {
      if (!detection?.score) {
        return;
      }

      if (detection.score < 0.33) {
        return false;
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

  const absoluteTimeMillisecondsRef = Date.parse(props.video.started_at || "");

  if (error) {
    console.warn(error);
    return (
      <div style={{ fontWeight: "bold", color: "red" }}>
        ERROR: {JSON.stringify(error)}
      </div>
    );
  }

  if (isLoading) {
    return null;
  }

  return (
    <>
      <canvas
        ref={canvasRef}
        style={{
          position: "absolute",
          display: "inline",
          zIndex: 500,
          padding: 0,
          margin: 0,
          cursor: "not-allowed",
          pointerEvents: "none",
          outline: "1px solid blue",
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
        ) => {
          if (!canvasRef.current) {
            return;
          }

          const canvas = canvasRef.current as HTMLCanvasElement;

          const firstUpdate = canvas.style.left === "0px";

          canvas.width = width;
          canvas.height = height;

          if (!readyRef?.current) {
            return;
          }

          const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
          if (!ctx) {
            return;
          }

          ctx.clearRect(0, 0, width, height);

          if (firstUpdate) {
            return;
          }

          const scaleX = width / 1920;
          const scaleY = height / 1080;

          const absoluteTimeMilliseconds =
            absoluteTimeMillisecondsRef + relativeTimeMilliseconds;

          const enrichedDetections = enrichedDetectionsRef.current || [];
          enrichedDetections.forEach((detection: Detection) => {
            const deltaMilliseconds =
              absoluteTimeMilliseconds -
              new Date(detection.seen_at || "").getTime();

            if (deltaMilliseconds < 0 || deltaMilliseconds > 5_000) {
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

            const lineWidth = 2 * scaleX;
            const textOffsetX = 3 * scaleX;
            const textOffsetY = 4 * scaleY;
            const centroidRadius = 7 * scaleX;

            ctx.lineWidth = lineWidth;
            ctx.strokeStyle = `rgba(255, 0, 0, 0.95)`;

            ctx.fillStyle = `rgba(255, 255, 255, 0.95)`;
            ctx.font = `${28 * scaleX}px sans-serif`;
            ctx.textAlign = "left";

            const topLeftX = topLeft.X * scaleX;
            const topLeftY = topLeft.Y * scaleY;

            const bottomRightX = bottomRight.X * scaleX;
            const bottomRightY = bottomRight.Y * scaleY;

            const centroidX = centroid.X * scaleX;
            const centroidY = centroid.Y * scaleY;

            if (deltaMilliseconds <= 200) {
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
            }

            const color = (1.0 - deltaMilliseconds / 5_000) * 255;
            const alpha = 1.0 - deltaMilliseconds / 5_000;

            ctx.strokeStyle = `rgba(${color}, ${color}, ${color}, ${alpha})`;

            ctx.beginPath();

            ctx.arc(centroidX, centroidY, centroidRadius, 0, Math.PI * 2);

            ctx.stroke();
          });
        }}
      />
    </>
  );
}
