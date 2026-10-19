import CloudDownloadOutlinedIcon from "@mui/icons-material/CloudDownloadOutlined";
import ErrorOutlineOutlinedIcon from "@mui/icons-material/ErrorOutlineOutlined";
import HourglassEmptyOutlinedIcon from "@mui/icons-material/HourglassEmptyOutlined";
import PlayCircleOutlineIcon from "@mui/icons-material/PlayCircleOutline";
import Button from "@mui/joy/Button";
import CircularProgress from "@mui/joy/CircularProgress";
import Container from "@mui/joy/Container";
import Modal from "@mui/joy/Modal";
import ModalDialog from "@mui/joy/ModalDialog";
import Table from "@mui/joy/Table";
import Tooltip from "@mui/joy/Tooltip";
import Typography from "@mui/joy/Typography";
import VolumeOffOutlinedIcon from "@mui/icons-material/VolumeOffOutlined";
import { useInfiniteQuery } from "@tanstack/react-query";
import { Dispatch, SetStateAction, useEffect, useRef, useState } from "react";
import { useInView } from "react-intersection-observer";
import { clientForReactQuery, useQuery } from "../api";
import { components } from "../api/api";
import { audioScoreColor, getDateString, OverlayMode } from "../helpers";
import { Video } from "./Video";

const defaultLimit = 10;
const desiredWidthRatio = 3840 / 2160;
// Rows are one media segment each, and the newest visible row is still running: its own duration is
// part of the section the strip should mark.
const visibleRowSpanMillis = 60_000;

type ParsedDetectionFilter = {
  className: string;
  minimumAverageScore?: number;
};

function parseDetectionFilter(value: string): ParsedDetectionFilter {
  const normalizedValue = value.trim().toLowerCase();
  const confidenceMatch = normalizedValue.match(
    /^(.+?)\s*>=?\s*(0(?:\.\d+)?|1(?:\.0+)?)$/,
  );

  if (confidenceMatch) {
    return {
      className: confidenceMatch[1].trim().replaceAll("?", ""),
      minimumAverageScore: Number(confidenceMatch[2]),
    };
  }

  return { className: normalizedValue.replaceAll("?", "") };
}
const desiredHeightRatio = 2160 / 3840;

export interface VideoTableProps {
  responsive: boolean;
  portrait: boolean;
  windowWidth: number;
  windowHeight: number;
  cameraId: string | undefined;
  startedAtGt: string | undefined;
  startedAtLte: string | undefined;
  classNameFilter: string;
  overlayMode: OverlayMode;
  // The time range of the rows currently on screen, so the top strip can mark that section.
  setVisibleRange: Dispatch<SetStateAction<[number, number] | undefined>>;
}

export function VideoTable(props: VideoTableProps) {
  const [ref, inView] = useInView();

  const [currentVideo, setCurrentVideo] = useState<
    components["schemas"]["Video"] | undefined
  >(undefined);
  const [showPlayModal, setShowPlayModal] = useState(false);

  const { data: allCamerasData } = useQuery("get", "/api/cameras", {
    params: {
      query: {
        name__asc: "",
      },
    },
  });

  const visibleCameraCount = props.cameraId
    ? 1
    : allCamerasData?.objects?.length || 1;

  const detectionFilter = parseDetectionFilter(props.classNameFilter);
  // In objects mode the row is described by its tracked objects, so the class
  // filter has to read the track summary rather than the raw detection counts.
  const summariesOf = (video: components["schemas"]["Video"]) =>
    ((props.overlayMode.showObjects
      ? video?.track_summary
      : video?.detection_summary) as any[] | null) || [];
  const scoreOf = (summary: any) =>
    typeof summary?.mean_score === "number"
      ? summary.mean_score
      : summary?.average_score;
  // Numeric confidence thresholds are applied client-side because the API
  // only supports JSON containment on detection_summary. Fetch a larger batch
  // so a high threshold does not make the search appear empty one page at a time.
  const relevantLimit =
    detectionFilter.minimumAverageScore === undefined
      ? defaultLimit * visibleCameraCount
      : 500;

  const queryHash = JSON.stringify({ ...props, detectionFilter });

  const {
    data: infiniteVideosData,
    hasNextPage,
    fetchNextPage,
  } = useInfiniteQuery({
    queryKey: ["videos"],
    queryHash: queryHash,
    queryFn: async ({ pageParam = 0 }) => {
      const res = await clientForReactQuery.GET("/api/videos", {
        params: {
          query: {
            camera_id__eq: props.cameraId || undefined,
            detection_summary__contains:
              detectionFilter.className && !props.overlayMode.showObjects
                ? JSON.stringify([{ class_name: detectionFilter.className }])
                : undefined,
            track_summary__contains:
              detectionFilter.className && props.overlayMode.showObjects
                ? JSON.stringify([{ class_name: detectionFilter.className }])
                : undefined,
            // TODO: skipping this means we can keep infinitely scrolling back forever, I think
            // started_at__gt: props.startedAtGt && props.startedAtGt,
            started_at__lte: props.startedAtLte && props.startedAtLte,
            started_at__desc: "",
            is_low_res__eq: false,
            limit: relevantLimit,
            offset: pageParam,
          },
        },
      });
      return res.data;
    },
    initialPageParam: 0,
    // Don't refetch every loaded video page while a modal is playing. Each
    // update can reconcile many rows and thumbnail elements on the main thread.
    refetchInterval: showPlayModal ? false : 30_000,
    refetchIntervalInBackground: false,
    getNextPageParam: (lastPage, pages) => {
      /*
      TODO: this doesn't cater for the fact that we have new data coming in- really we should
      use something like timestamp for the cursor, and even then we should probably split out
      finished videos from processing videos
      */

      if (!lastPage) {
        return undefined;
      }

      const pageCount = lastPage.count || 0;
      if (pageCount === 0) {
        return undefined;
      }

      const pageLimit = lastPage.limit || relevantLimit;
      if (pageCount < pageLimit) {
        return undefined;
      }

      return (lastPage.offset || 0) + pageLimit;
    },
  });

  const videosData: {
    objects: components["schemas"]["Video"][];
  } = {
    objects: [],
  };

  const ids = new Set<string>();

  infiniteVideosData?.pages.forEach((page) => {
    page?.objects?.forEach((object) => {
      if (!object?.id) {
        return;
      }

      if (ids.has(object.id)) {
        return;
      }

      ids.add(object.id);

      videosData?.objects.push(object);
    });
  });

  const { data: camerasData } = useQuery("get", "/api/cameras", {});

  const cameraById = new Map<string, components["schemas"]["Camera"]>();
  camerasData?.objects?.forEach((camera) => {
    if (!camera?.id) {
      return;
    }

    cameraById.set(camera?.id, camera);
  });

  const truncateStyleProps = props.responsive
    ? {
        maxWidth: "100%",
        overflow: "hidden",
        textOverflow: "ellipsis",
      }
    : {};

  useEffect(() => {
    if (inView && hasNextPage) {
      void fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, inView]);

  let desiredMaxWidth;
  let desiredMaxHeight;
  let desiredWidth;
  let desiredHeight;

  if (props.portrait) {
    desiredWidth = props.windowWidth * 0.96;
    desiredMaxWidth = props.windowWidth * 0.96;
    desiredHeight = desiredWidth * desiredHeightRatio;
    desiredMaxHeight = desiredWidth * desiredHeightRatio;
  } else {
    desiredHeight = props.windowHeight * 0.96;
    desiredMaxHeight = props.windowHeight * 0.96;
    desiredWidth = desiredHeight * desiredWidthRatio;
    desiredMaxWidth = desiredHeight * desiredWidthRatio;
  }

  // Which rows are on screen is what the top strip needs: the table is the app's only scroller, so
  // the rows intersecting the viewport are the rows the user is looking at.
  const tableRef = useRef<HTMLTableElement | null>(null);
  const rowTimesRef = useRef(new Map<string, number>());
  const lastRangeRef = useRef<[number, number] | undefined>(undefined);

  const { setVisibleRange } = props;

  const rowKeys = (videosData?.objects || [])
    .map((video) => `${video.id}:${video.started_at}`)
    .join(",");

  useEffect(() => {
    const times = new Map<string, number>();

    (videosData?.objects || []).forEach((video) => {
      const millis = Date.parse(video.started_at || "");

      if (video.id && !Number.isNaN(millis)) {
        times.set(video.id, millis);
      }
    });

    rowTimesRef.current = times;

    const rows =
      tableRef.current?.querySelectorAll<HTMLTableRowElement>(
        "tr[data-row-id]",
      );

    if (!rows?.length) {
      return;
    }

    const visible = new Set<string>();

    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        const id = (entry.target as HTMLElement).dataset.rowId || "";

        if (entry.isIntersecting) {
          visible.add(id);
        } else {
          visible.delete(id);
        }
      });

      const millis: number[] = [];

      visible.forEach((id) => {
        const rowMillis = rowTimesRef.current.get(id);

        if (rowMillis !== undefined) {
          millis.push(rowMillis);
        }
      });

      const next: [number, number] | undefined = millis.length
        ? [Math.min(...millis), Math.max(...millis) + visibleRowSpanMillis]
        : undefined;
      const last = lastRangeRef.current;

      if (next?.[0] !== last?.[0] || next?.[1] !== last?.[1]) {
        lastRangeRef.current = next;
        setVisibleRange(next);
      }
    });

    rows.forEach((row) => observer.observe(row));

    return () => observer.disconnect();
  }, [rowKeys, videosData?.objects, setVisibleRange]);

  return (
    <>
      <Table
        ref={tableRef}
        size="sm"
        sx={{
          th: {
            textAlign: "center",
            p: 0,
            m: 0,
          },
          td: {
            textAlign: "center",
            p: 0,
            m: 0,
            pt: 0.66,
            ...truncateStyleProps,
          },
        }}
        stickyHeader={true}
        stripe={"odd"}
        borderAxis="y"
      >
        <thead>
          <tr>
            <th
              style={{
                width: props.responsive ? 70 : 250,
                ...truncateStyleProps,
              }}
            >
              {props.responsive ? "T" : "Time"}
            </th>
            <th
              style={{
                width: props.responsive ? 70 : 140,
                ...truncateStyleProps,
              }}
            >
              {props.responsive ? "S" : "Summary"}
            </th>
            <th style={{ ...truncateStyleProps }}>
              {props.responsive ? "D" : "Detected"}
            </th>
            <th
              style={{
                width: props.responsive ? 70 : 150,
                ...truncateStyleProps,
              }}
            >
              {props.responsive ? "A" : "Audio"}
            </th>
            <th
              style={{
                width: props.responsive ? 80 : 160,
                ...truncateStyleProps,
              }}
            >
              Preview
            </th>
            <th style={{ width: "7.5%", ...truncateStyleProps }}>
              {props.responsive ? "M" : "Media"}
            </th>
          </tr>
        </thead>
        <tbody>
          {videosData?.objects?.length ? (
            videosData?.objects
              ?.filter((video) => {
                if (detectionFilter.className) {
                  const summaries = summariesOf(video);

                  return summaries.some((summary: any) => {
                    const classMatches = (
                      summary.class_name as string
                    ).includes(detectionFilter.className);
                    const scoreMatches =
                      detectionFilter.minimumAverageScore === undefined ||
                      (typeof scoreOf(summary) === "number" &&
                        scoreOf(summary) >=
                          detectionFilter.minimumAverageScore);
                    return classMatches && scoreMatches;
                  });
                }

                return true;
              })
              .map((video) => {
                if (!video.started_at) {
                  return undefined;
                }

                const camera = cameraById.get(video?.camera_id || "");

                const startedAt = new Date(video.started_at);
                const endedAt = video.ended_at && new Date(video.ended_at);

                const available = video?.status !== "recording";

                const minutes = Math.floor(
                  (video?.duration || 0) / (1_000_000_000 * 60),
                );
                const seconds = Math.floor(
                  (video?.duration || 0) / 1_000_000_000 - minutes * 60,
                );

                const fileSize = (video?.file_size || 0.0).toFixed(2);

                var cameraName = camera?.name || "-";

                var thumbnail;

                if (video?.thumbnail_name) {
                  thumbnail = (
                    <a
                      target="_blank"
                      rel="noreferrer"
                      href={`/media/${video?.thumbnail_name}`}
                    >
                      <img
                        alt={`still from ${video?.camera_id_object?.name} @ ${startedAt}`}
                        src={`/media/${video?.thumbnail_name}`}
                        loading="lazy"
                        decoding="async"
                        style={{
                          width: props.responsive ? 80 : 160,
                          height: props.responsive ? 45 : 90,
                        }}
                      />
                    </a>
                  );
                } else if (video?.status === "failed") {
                  thumbnail = (
                    <Tooltip title="Failed">
                      <ErrorOutlineOutlinedIcon />
                    </Tooltip>
                  );
                } else if (video?.status === "needs detection") {
                  thumbnail = (
                    <Tooltip title="Needs detection">
                      <HourglassEmptyOutlinedIcon />
                    </Tooltip>
                  );
                } else if (video?.status === "detecting") {
                  thumbnail = (
                    <Tooltip title="Detecting">
                      <CircularProgress variant="soft" size="sm" />
                    </Tooltip>
                  );
                } else if (video?.status === "needs tracking") {
                  thumbnail = (
                    <Tooltip title="Needs tracking">
                      <HourglassEmptyOutlinedIcon />
                    </Tooltip>
                  );
                } else {
                  thumbnail = (
                    <Tooltip title="Recording">
                      <CircularProgress variant="soft" size="sm" />
                    </Tooltip>
                  );
                }

                let classNames;

                if (video?.status === "failed") {
                  classNames = (
                    <Tooltip title="Failed">
                      <ErrorOutlineOutlinedIcon />
                    </Tooltip>
                  );
                } else if (video?.status === "needs detection") {
                  classNames = (
                    <Tooltip title="Needs detection">
                      <HourglassEmptyOutlinedIcon />
                    </Tooltip>
                  );
                } else if (video?.status === "detecting") {
                  classNames = (
                    <Tooltip title="Detecting">
                      <CircularProgress variant="soft" size="sm" />
                    </Tooltip>
                  );
                } else if (video?.status === "needs tracking") {
                  classNames = summariesOf(video)
                    .filter((summary: any) => {
                      return true;
                    })
                    .map((x: any) => {
                      if (props.responsive) {
                        return (
                          <span
                            key={x.class_name}
                            style={{
                              color:
                                detectionFilter.className &&
                                (x.class_name as string).includes(
                                  detectionFilter.className,
                                )
                                  ? "#ff0000"
                                  : undefined,
                            }}
                          >
                            {x.class_name} @ {scoreOf(x).toFixed(2)} <br />
                          </span>
                        );
                      }

                      return (
                        <span
                          key={x.class_name}
                          style={{
                            color:
                              detectionFilter.className &&
                              (x.class_name as string).includes(
                                detectionFilter.className,
                              )
                                ? "#ff0000"
                                : undefined,
                          }}
                        >
                          {props.overlayMode.showObjects
                            ? `${x.class_name} x${x.count} @ ${scoreOf(
                                x,
                              ).toFixed(2)} (${x.frame_count} frames)`
                            : `${x.class_name} @ ${scoreOf(x).toFixed(
                                2,
                              )} (over ${x.detected_frame_count} frames)`}
                          <br />
                        </span>
                      );
                    });
                }

                const audioSummary = (
                  (video?.audio_summary as any[] | null) || []
                )
                  .slice()
                  .sort((a: any, b: any) => (b?.score || 0) - (a?.score || 0));

                let audioCell;

                if ((video?.audio_profile as any)?.no_audio === true) {
                  audioCell = (
                    <Tooltip title="No audio stream">
                      <VolumeOffOutlinedIcon />
                    </Tooltip>
                  );
                } else if (!video?.audio_detected_at) {
                  audioCell = (
                    <Tooltip title="Audio pending">
                      <HourglassEmptyOutlinedIcon />
                    </Tooltip>
                  );
                } else if (!audioSummary.length) {
                  audioCell = <Typography color="neutral">quiet</Typography>;
                } else {
                  audioCell = audioSummary.slice(0, 3).map((event: any) => {
                    const score = (event?.score || 0).toFixed(2);
                    const label = props.responsive
                      ? `${event?.kind} @ ${score}`
                      : `${event?.kind} @ ${score} (+${(
                          event?.prominence_db || 0
                        ).toFixed(1)} dB, ${event?.duration_ms} ms)`;

                    return (
                      <span
                        key={`${video.id}-audio-${event?.offset_ms}`}
                        style={{ color: audioScoreColor(event?.score || 0) }}
                      >
                        {label}
                        <br />
                      </span>
                    );
                  });
                }

                return (
                  <tr
                    key={`video-table-row-${video.id}`}
                    data-row-id={video.id}
                    data-started-at={startedAt.getTime()}
                  >
                    <td>
                      <Typography style={{ display: "inline" }}>
                        {getDateString(startedAt)}{" "}
                        {startedAt.toTimeString().split(" ")[0]}
                      </Typography>
                      {props.responsive ? <br /> : " -> "}
                      <Typography color="neutral" style={{ display: "inline" }}>
                        {endedAt
                          ? endedAt.toTimeString().split(" ")[0]
                          : new Date().toTimeString().split(" ")[0]}{" "}
                        <br />
                      </Typography>
                    </td>
                    <td>
                      <Typography>{cameraName}</Typography>
                      <Typography color="neutral">
                        {minutes}m{seconds}s
                      </Typography>
                      <Typography color="neutral">{fileSize} MB</Typography>
                    </td>
                    <td>{classNames}</td>
                    <td>{audioCell}</td>
                    <td>
                      <Container
                        sx={{
                          width: props.responsive ? 80 : 160,
                          height: props.responsive ? 45 : 90,
                          display: "flex",
                          flexDirection: "column",
                          justifyContent: "center",
                          alignItems: "center",
                        }}
                      >
                        {thumbnail}
                      </Container>
                    </td>
                    <td>
                      {available ? (
                        <>
                          <Button
                            variant={"plain"}
                            sx={{ px: 0, mx: 0, py: 0, my: 0 }}
                            onClick={() => {
                              window.open(
                                `/media/${video?.file_name}`,
                                "_blank",
                              );
                            }}
                          >
                            <CloudDownloadOutlinedIcon color={"success"} />
                          </Button>
                          <br />
                          <Button
                            variant={"plain"}
                            sx={{ px: 0, mx: 0, py: 0, my: 0 }}
                            onClick={() => {
                              setCurrentVideo(video);
                              setShowPlayModal(true);
                            }}
                          >
                            <PlayCircleOutlineIcon color={"success"} />
                          </Button>
                        </>
                      ) : (
                        "-"
                      )}
                    </td>
                  </tr>
                );
              })
          ) : (
            <tr>
              <td colSpan={6}>
                <Typography color={"neutral"}>
                  (No videos for the selected camera / date)
                </Typography>
              </td>
            </tr>
          )}
          <tr>
            <td colSpan={6} ref={ref}>
              <Typography color={"neutral"}> </Typography>
            </td>
          </tr>
        </tbody>
      </Table>
      <Modal
        sx={{ p: 0, m: 0 }}
        open={showPlayModal}
        onClose={() => {
          setShowPlayModal(false);
        }}
      >
        <ModalDialog
          variant="plain"
          size="sm"
          sx={{
            width: desiredWidth,
            maxWidth: desiredMaxWidth,
            height: desiredHeight,
            maxHeight: desiredMaxHeight,
            p: "1px",
            borderRadius: 0,
            m: 0,
            display: "flex",
            justifyContent: "center",
            verticalAlign: "center",
          }}
        >
          {currentVideo && (
            <Video
              video={currentVideo}
              width={desiredWidth}
              height={desiredHeight}
              overlayMode={props.overlayMode}
            />
          )}
        </ModalDialog>
      </Modal>
    </>
  );
}
