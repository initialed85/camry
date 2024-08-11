import Text from "@carefully-coded/react-text-gradient";
import CloudDownloadOutlinedIcon from "@mui/icons-material/CloudDownloadOutlined";
import ErrorOutlineOutlinedIcon from "@mui/icons-material/ErrorOutlineOutlined";
import CircularProgress from "@mui/joy/CircularProgress";
import Table from "@mui/joy/Table";
import Typography from "@mui/joy/Typography";
import { useQuery } from "../api";

export interface VideoTableProps {
  responsive: boolean;
  cameraId: string | null | undefined;
  date: string | null | undefined;
}

export function VideoTable(props: VideoTableProps) {
  const { data, error, isLoading } = useQuery("get", "/api/videos", {
    params: {
      query: {
        started_at__desc: "",
        camera_id__eq: props.cameraId || undefined,
        started_at__gte: props.date
          ? `${props.date}T00:00:00+08:00`
          : undefined,
        started_at__lte: props.date
          ? `${props.date}T23:59:59+08:00`
          : undefined,
      },
    },
  });

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
          <th style={{ width: "5%" }}>{props.responsive ? "C" : "Camera"}</th>
          <th>Details</th>
          <th>Status</th>
          <th style={{ ...truncateStyleProps }}>
            Detections
            {!props.responsive && (
              <>
                <br />
                <Typography color="neutral">(frames @ score)</Typography>
              </>
            )}
          </th>
          <th style={{ width: props.responsive ? 160 : 320 }}>Preview</th>
          <th style={{ width: "5%" }}>Media</th>
        </tr>
      </thead>
      <tbody>
        {data?.objects?.length ? (
          data?.objects?.map((video) => {
            if (!video.started_at) {
              return undefined;
            }

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

            var cameraName =
              video?.camera_id_object && video?.camera_id_object?.name
                ? video.camera_id_object.name
                : "-";

            if (props.responsive) {
              cameraName = cameraName[0];
            }

            const statusText = (
              video?.status
                ? video.status[0].toUpperCase() + video.status.slice(1)
                : "-"
            ).trim();

            const status =
              statusText === "Recording" ? (
                <span>
                  <Text
                    gradient={{
                      from: "#ff5555",
                      to: "#5555ff",
                      type: "linear",
                      degree: 45,
                    }}
                    animate
                    animateDuration={1_000}
                    style={{ ...truncateStyleProps }}
                  >
                    {statusText}
                  </Text>
                </span>
              ) : (
                statusText
              );

            var thumbnail;

            if (video?.thumbnail_name) {
              thumbnail = (
                <a
                  target="_blank"
                  rel="noreferrer"
                  href={`/media/${video?.thumbnail_name}`}
                >
                  <img
                    style={{ width: props.responsive ? 160 : 320 }}
                    alt={`still from ${video?.camera_id_object?.name} @ ${props.date} ${startedAt}`}
                    src={`/media/${video?.thumbnail_name}`}
                  />
                </a>
              );
            } else if (video?.status !== "failed") {
              thumbnail = <CircularProgress variant="soft" size="sm" />;
            } else {
              thumbnail = <ErrorOutlineOutlinedIcon />;
            }

            return (
              <tr key={video.id}>
                <td>{cameraName}</td>
                <td>
                  <Typography>
                    {startedAt.toTimeString().split(" ")[0]} <br />
                    {endedAt ? endedAt.toTimeString().split(" ")[0] : "-"}{" "}
                    <br />
                  </Typography>
                  <Typography color="neutral">
                    {minutes}m{seconds}s
                  </Typography>
                  <Typography color="neutral">{fileSize} MB</Typography>
                </td>
                <td>{status}</td>
                <td>-</td>
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
            <td colSpan={6}>
              <Typography color={"neutral"}>
                (No videos for the selected camera / date)
              </Typography>
            </td>
          </tr>
        )}
      </tbody>
    </Table>
  );
}
