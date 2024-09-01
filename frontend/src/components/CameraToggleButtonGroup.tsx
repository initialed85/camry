import LiveTvIcon from "@mui/icons-material/LiveTv";
import { ButtonGroup, IconButton, Typography } from "@mui/joy";
import Button from "@mui/joy/Button";
import ToggleButtonGroup from "@mui/joy/ToggleButtonGroup";
import { Dispatch, SetStateAction, useState } from "react";
import { useQuery } from "../api";

export interface CameraToggleButtonGroupProps {
  responsive: boolean;
  cameraId: string | null | undefined;
  setCameraId: Dispatch<SetStateAction<string | null | undefined>>;
}

export default function CameraToggleButtonGroup(
  props: CameraToggleButtonGroupProps,
) {
  const [value, setValue] = useState<string | null | undefined>(props.cameraId);

  const { data, error } = useQuery("get", "/api/cameras", {
    params: {
      query: {
        name__asc: "",
      },
    },
  });

  if (error) {
    return (
      <Typography color="danger">
        Failed to load cameras: {error?.error || error.toString()}
      </Typography>
    );
  }

  return (
    <ToggleButtonGroup
      variant="soft"
      spacing="1"
      color="primary"
      size={"sm"}
      sx={{ ml: 0.5 }}
      value={value}
      onChange={(event, newValue) => {
        setValue(newValue);
        props.setCameraId(newValue);
      }}
    >
      {data?.objects?.map((c) => {
        if (!c || !c?.name) {
          return undefined;
        }

        return (
          <ButtonGroup
            variant="soft"
            color="primary"
            size={"sm"}
            sx={{ px: 0.5 }}
          >
            <Button
              key={c.id}
              value={c.id}
              sx={{ width: props.responsive ? 33 : 133 }}
              size={"sm"}
            >
              {props.responsive ? c.name[0] : c.name}
            </Button>
            <IconButton size={"sm"}>
              <a
                target="_blank"
                rel="noreferrer"
                href={`https://camry-stream.initialed85.cc/streams/${c.id}`}
              >
                <LiveTvIcon />
              </a>
            </IconButton>
          </ButtonGroup>
        );
      })}
    </ToggleButtonGroup>
  );
}
