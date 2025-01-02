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
import { useInfiniteQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { useInView } from "react-intersection-observer";
import { clientForReactQuery, useQuery } from "../api";
import { components } from "../api/api";
import { getDateString } from "../helpers";
import { Video } from "./Video";

const defaultLimit = 10;
const desiredWidthRatio = 1920 / 1080;
const desiredHeightRatio = 1080 / 1920;

export interface VideoTableProps {
  responsive: boolean;
  portrait: boolean;
  windowWidth: number;
  windowHeight: number;
  cameraId: string | undefined;
  startedAtGt: string | undefined;
  startedAtLte: string | undefined;
  classNameFilter: string;
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

  const relevantLimit = defaultLimit * visibleCameraCount;

  const queryHash = JSON.stringify(props);

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
            detection_summary__contains: props.classNameFilter
              ? JSON.stringify([
                  { class_name: props.classNameFilter.replaceAll("?", "") },
                ])
              : undefined,
            // TODO: skipping this means we can keep infinitely scrolling back forever, I think
            // started_at__gt: props.startedAtGt && props.startedAtGt,
            started_at__lte: props.startedAtLte && props.startedAtLte,
            started_at__desc: "",
            limit: relevantLimit,
            offset: pageParam,
          },
        },
      });
      return res.data;
    },
    initialPageParam: 0,
    getNextPageParam: (lastPage, pages) => {
      /*
      TODO: this doesn't cater for the fact that we have new data coming in- really we should
      use something like timestamp for the cursor, and even then we should probably split out
      finished videos from processing videos
      */

      if (lastPage?.count === 0) {
        return lastPage?.offset;
      }

      return (lastPage?.offset || 0) + relevantLimit;
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

  return (
    <>
      <Table
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
                if (props.classNameFilter) {
                  const detectionSummaries = video?.detection_summary as [];

                  const matchingClassNames = detectionSummaries.filter(
                    (detectionSummary: any) => {
                      // if (
                      //   detectionSummary.average_score < 0.5 ||
                      //   detectionSummary.detected_frame_count < 8
                      // ) {
                      //   return false;
                      // }

                      return (detectionSummary.class_name as string).includes(
                        props.classNameFilter,
                      );
                    },
                  );

                  return matchingClassNames.length;
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
                  classNames = (video?.detection_summary as [])
                    .filter((detectionSummary: any) => {
                      // if (
                      //   detectionSummary.average_score < 0.5 ||
                      //   detectionSummary.detected_frame_count < 8
                      // ) {
                      //   return false;
                      // }

                      return true;
                    })
                    .map((x: any) => {
                      if (props.responsive) {
                        return (
                          <span
                            key={x.class_name}
                            style={{
                              color:
                                props.classNameFilter &&
                                (x.class_name as string).includes(
                                  props.classNameFilter,
                                )
                                  ? "#ff0000"
                                  : undefined,
                            }}
                          >
                            {x.class_name} @ {x.average_score.toFixed(2)} <br />
                          </span>
                        );
                      }

                      return (
                        <span
                          key={x.class_name}
                          style={{
                            color:
                              props.classNameFilter &&
                              (x.class_name as string).includes(
                                props.classNameFilter,
                              )
                                ? "#ff0000"
                                : undefined,
                          }}
                        >
                          {x.class_name} @ {x.average_score.toFixed(2)} (over{" "}
                          {x.detected_frame_count} frames)
                          <br />
                        </span>
                      );
                    });
                }

                return (
                  <tr key={`video-table-row-${video.id}`}>
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
              <td colSpan={5}>
                <Typography color={"neutral"}>
                  (No videos for the selected camera / date)
                </Typography>
              </td>
            </tr>
          )}
          <tr>
            <td colSpan={5} ref={ref}>
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
            />
          )}
        </ModalDialog>
      </Modal>
    </>
  );
}
