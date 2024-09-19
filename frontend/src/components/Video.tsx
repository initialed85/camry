import { useEffect, useRef } from "react";
import { useQuery } from "../api";
import { components } from "../api/api";
import VideoJS from "./VideoJS";

type Detection = components["schemas"]["Detection"];

type Point = components["schemas"]["Detection"]["centroid"];

export interface VideoProps {
  video: components["schemas"]["Video"];
}

export function Video(props: VideoProps) {
  const { isLoading, error, data } = useQuery("get", "/api/detections", {
    params: { query: { video_id__eq: props.video?.id || "", limit: 1_000_000 } },
  });

  const enrichedDetectionsRef = useRef<components["schemas"]["Detection"][]>([]);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const readyRef = useRef(false);

  useEffect(() => {
    if (enrichedDetectionsRef.current?.length) {
      return;
    }

    let enrichedDetections: Detection[] = [];

    const detections: Detection[] = data?.objects || [];
    detections.forEach((detection: Detection) => {
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
    return <div style={{ fontWeight: "bold", color: "red" }}>ERROR: {JSON.stringify(error)}</div>;
  }

  if (isLoading) {
    return null;
  }

  return (
    <>
      <canvas
        ref={canvasRef}
        style={{
          position: "fixed",
          display: "block",
          zIndex: 500,
          left: 0,
          top: 0,
          width: 0,
          height: 0,
          cursor: "not-allowed",
          pointerEvents: "none",
        }}
        width={1920}
        height={1080}
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
        onTimeUpdate={(left: number, top: number, width: number, height: number, relativeTimeMilliseconds: number) => {
          if (!canvasRef.current) {
            return;
          }

          const canvas = canvasRef.current as HTMLCanvasElement;

          const firstUpdate = canvas.style.left === "0px";

          canvas.style.left = `${left}px`;
          canvas.style.top = `${top}px`;
          canvas.style.width = `${width}px`;
          canvas.style.height = `${height}px`;

          if (!readyRef?.current) {
            return;
          }

          const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
          if (!ctx) {
            return;
          }

          ctx.clearRect(0, 0, 1920, 1080);

          if (firstUpdate) {
            return;
          }

          const absoluteTimeMilliseconds = absoluteTimeMillisecondsRef + relativeTimeMilliseconds;

          const enrichedDetections = enrichedDetectionsRef.current || [];
          enrichedDetections.forEach((detection: Detection) => {
            const deltaMilliseconds = absoluteTimeMilliseconds - new Date(detection.seen_at || "").getTime();

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

            ctx.lineWidth = 2;
            ctx.strokeStyle = `rgba(255, 0, 0, 0.75)`;

            ctx.fillStyle = `rgba(255, 255, 255, 0.75)`;
            ctx.font = "18px sans-serif";
            ctx.textAlign = "left";

            if (deltaMilliseconds <= 200) {
              ctx.fillText(`${detection.class_name} @ ${detection.score.toFixed(3)}`, topLeft.X + 3, bottomRight.Y - 4);

              ctx.strokeRect(
                topLeft.X,
                topLeft.Y,
                Math.abs(bottomRight.X - topLeft.X),
                Math.abs(bottomRight.Y - topLeft.Y)
              );
            }

            const color = (1.0 - deltaMilliseconds / 5_000) * 255;
            const alpha = 1.0 - deltaMilliseconds / 5_000;

            ctx.strokeStyle = `rgba(${color}, ${color}, ${color}, ${alpha})`;

            ctx.beginPath();

            ctx.arc(centroid.X, centroid.Y, 5, 0, Math.PI * 2);

            ctx.stroke();
          });
        }}
      />
    </>
  );
}
