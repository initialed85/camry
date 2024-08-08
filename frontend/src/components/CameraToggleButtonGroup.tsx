import { useQuery } from "../api";

import { Typography } from "@mui/joy";
import Button from "@mui/joy/Button";
import ToggleButtonGroup from "@mui/joy/ToggleButtonGroup";
import { Dispatch, SetStateAction, useState } from "react";

export interface CameraToggleButtonGroupProps {
  responsive: boolean;
  setCameraId: Dispatch<SetStateAction<string | null | undefined>>;
}

export default function CameraToggleButtonGroup(
  props: CameraToggleButtonGroupProps,
) {
  const [value, setValue] = useState<string | null>();

  const { data, error } = useQuery("get", "/cameras", {
    params: {
      query: {
        order_by__asc: "name",
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
          <Button
            variant="soft"
            key={c.id}
            value={c.id}
            sx={{ mx: 0.5, width: props.responsive ? 25 : 100 }}
          >
            {props.responsive ? c.name[0] : c.name}
          </Button>
        );
      })}
    </ToggleButtonGroup>
  );
}
