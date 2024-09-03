import CloudDownloadOutlinedIcon from "@mui/icons-material/CloudDownloadOutlined";
import ErrorOutlineOutlinedIcon from "@mui/icons-material/ErrorOutlineOutlined";
import HourglassEmptyOutlinedIcon from "@mui/icons-material/HourglassEmptyOutlined";
import CircularProgress from "@mui/joy/CircularProgress";
import Table from "@mui/joy/Table";
import Tooltip from "@mui/joy/Tooltip";
import Typography from "@mui/joy/Typography";
import Container from "@mui/material/Container";
import { useInfiniteQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { useInView } from "react-intersection-observer";
import { clientForReactQuery, useQuery } from "../api";
import { components } from "../api/api";
import { formatDate } from "../helpers";

const defaultLimit = 60;

export interface VideoTableProps {
  responsive: boolean;
  cameraId: string | undefined;
  startedAtGt: string | undefined;
  startedAtLte: string | undefined;
}

export function VideoTable(props: VideoTableProps) {
  const [ref, inView] = useInView();

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

  const {
    data: infiniteVideosData,
    hasNextPage,
    fetchNextPage,
  } = useInfiniteQuery({
    queryKey: ["videos"],
    queryHash: JSON.stringify(props),
    queryFn: async ({ pageParam = 0 }) => {
      const res = await clientForReactQuery.GET("/api/videos", {
        params: {
          query: {
            camera_id__eq: props.cameraId || undefined,
            started_at__gt: props.startedAtGt && props.startedAtGt,
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

  infiniteVideosData?.pages.forEach((page) => {
    page?.objects?.forEach((object) => {
      if (!object) {
        return;
      }

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

  // const { data: detectionsData } = useQuery("get", "/api/detections", {
  //   params: {
  //     query: {
  //       camera_id__eq: props.cameraId || undefined,
  //       seen_at__gte: props.date ? `${props.date}T00:00:00+08:00` : undefined,
  //       seen_at__lte: props.date ? `${props.date}T23:59:59+08:00` : undefined,
  //       seen_at__desc: "",
  //     },
  //   },
  // });

  const classNamesByVideoId = new Map<string, Set<string>>();
  // detectionsData?.objects?.forEach((detection) => {
  //   if (!detection?.video_id || !detection.class_name) {
  //     return;
  //   }

  //   let classNames = classNamesByVideoId.get(detection?.video_id);
  //   if (!classNames) {
  //     classNames = new Set<string>();
  //   }

  //   classNames.add(detection?.class_name);
  //   classNamesByVideoId.set(detection?.video_id, classNames);
  // });

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

  return (
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
              width: props.responsive ? 75 : 250,
              ...truncateStyleProps,
            }}
          >
            {props.responsive ? "T" : "Time"}
          </th>
          <th style={{ width: 67, ...truncateStyleProps }}>
            {props.responsive ? "S" : "Summary"}
          </th>
          <th style={{ ...truncateStyleProps }}>
            {props.responsive ? "D" : "Detected"}
            {!props.responsive && (
              <>
                <br />
                <Typography color="neutral">(frames @ score)</Typography>
              </>
            )}
          </th>
          <th
            style={{
              width: props.responsive ? 160 : 320,
              ...truncateStyleProps,
            }}
          >
            Preview
          </th>
          <th style={{ width: "5%", ...truncateStyleProps }}>
            {props.responsive ? "M" : "Media"}
          </th>
        </tr>
      </thead>
      <tbody>
        {videosData?.objects?.length ? (
          videosData?.objects?.map((video) => {
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
                      width: props.responsive ? 160 : 320,
                      height: props.responsive ? 90 : 180,
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

            const rawClassNames =
              video.id && classNamesByVideoId.get(video.id)?.keys();

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
            } else {
              classNames = (
                rawClassNames ? Array.from(rawClassNames).sort() : []
              ).join(", ");
            }

            return (
              <tr key={`vidoe-table-row-${video.id}`}>
                <td>
                  <Typography style={{ display: "inline" }}>
                    {formatDate(startedAt)}{" "}
                    {startedAt.toTimeString().split(" ")[0]}
                  </Typography>
                  {props.responsive ? <br /> : " -> "}
                  <Typography color="neutral" style={{ display: "inline" }}>
                    {endedAt ? endedAt.toTimeString().split(" ")[0] : "..."}{" "}
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
                      width: props.responsive ? 160 : 320,
                      height: props.responsive ? 90 : 180,
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
                    <a
                      target="_blank"
                      rel="noreferrer"
                      href={`/media/${video?.file_name}`}
                    >
                      <CloudDownloadOutlinedIcon color={"success"} />
                    </a>
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
  );
}
