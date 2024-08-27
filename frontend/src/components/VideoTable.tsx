import CloudDownloadOutlinedIcon from "@mui/icons-material/CloudDownloadOutlined";
import ErrorOutlineOutlinedIcon from "@mui/icons-material/ErrorOutlineOutlined";
import HourglassEmptyOutlinedIcon from "@mui/icons-material/HourglassEmptyOutlined";
import CircularProgress from "@mui/joy/CircularProgress";
import Table from "@mui/joy/Table";
import Tooltip from "@mui/joy/Tooltip";
import Typography from "@mui/joy/Typography";
import { useQuery } from "../api";
import { components } from "../api/api";

export interface VideoTableProps {
  responsive: boolean;
  cameraId: string | null | undefined;
  date: string | null | undefined;
}

export function VideoTable(props: VideoTableProps) {
  const { data: videosData } = useQuery("get", "/api/videos", {
    params: {
      query: {
        camera_id__eq: props.cameraId || undefined,
        started_at__gte: props.date ? `${props.date}T00:00:00+08:00` : undefined,
        started_at__lte: props.date ? `${props.date}T23:59:59+08:00` : undefined,
        started_at__desc: "",
      },
    },
  });

  // useInfiniteQuery({
  //   queryKey: ["videos"],
  //   queryFn: async ({ pageParam = 0 }) => {
  //     const res = await clientForReactQuery.GET("/api/cameras", { params: { query: {} } });
  //     return res.data;
  //   },
  //   initialPageParam: 0,
  //   getNextPageParam: (lastPage, pages) => 1,
  // });

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

  return (
    <Table
      size="sm"
      sx={{
        th: { textAlign: "center" },
        td: { textAlign: "center", ...truncateStyleProps },
      }}
      stickyHeader={true}
      stripe={"odd"}
      borderAxis="y"
    >
      <thead>
        <tr>
          <th style={{ width: 67, ...truncateStyleProps }}>{props.responsive ? "T" : "Time"}</th>
          <th style={{ width: 67, ...truncateStyleProps }}>{props.responsive ? "S" : "Summary"}</th>
          <th style={{ ...truncateStyleProps }}>
            {props.responsive ? "D" : "Detections"}
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
          <th style={{ width: "5%", ...truncateStyleProps }}>{props.responsive ? "M" : "Media"}</th>
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

            const minutes = Math.floor((video?.duration || 0) / (1_000_000_000 * 60));
            const seconds = Math.floor((video?.duration || 0) / 1_000_000_000 - minutes * 60);

            const fileSize = (video?.file_size || 0.0).toFixed(2);

            var cameraName = camera?.name || "-";

            var thumbnail;

            if (video?.thumbnail_name) {
              thumbnail = (
                <a target="_blank" rel="noreferrer" href={`/media/${video?.thumbnail_name}`}>
                  <img
                    style={{ width: props.responsive ? 160 : 320 }}
                    alt={`still from ${video?.camera_id_object?.name} @ ${props.date} ${startedAt}`}
                    src={`/media/${video?.thumbnail_name}`}
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

            const rawClassNames = video.id && classNamesByVideoId.get(video.id)?.keys();

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
              classNames = (rawClassNames ? Array.from(rawClassNames).sort() : []).join(", ");
            }

            return (
              <tr key={video.id}>
                <td>
                  <Typography>
                    {startedAt.toTimeString().split(" ")[0]} <br />
                  </Typography>
                  <Typography color="neutral">
                    {endedAt ? endedAt.toTimeString().split(" ")[0] : "-"} <br />
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
                <td
                  style={{
                    height: props.responsive ? 90 : 180,
                    paddingTop: 4,
                    paddingBottom: 0,
                    paddingLeft: 4,
                    paddingRight: 4,
                  }}
                >
                  {thumbnail}
                </td>
                <td>
                  {available ? (
                    <a target="_blank" rel="noreferrer" href={`/media/${video?.file_name}`}>
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
            <td colSpan={6}>
              <Typography color={"neutral"}>(No videos for the selected camera / date)</Typography>
            </td>
          </tr>
        )}
      </tbody>
    </Table>
  );
}
