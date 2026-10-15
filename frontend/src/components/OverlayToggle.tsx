import Box from "@mui/joy/Box";
import Button from "@mui/joy/Button";

import { OverlayMode } from "../helpers";

export interface OverlayToggleProps {
  responsive: boolean;
  mode: OverlayMode;
  setMode: (mode: OverlayMode) => void;
}

export default function OverlayToggle(props: OverlayToggleProps) {
  const label = (short: string, long: string) =>
    props.responsive ? short : long;

  return (
    <Box sx={{ display: "flex", gap: 0.5 }}>
      <Button
        size="sm"
        variant={props.mode.showDetections ? "solid" : "soft"}
        color={props.mode.showDetections ? "primary" : "neutral"}
        sx={{ width: props.responsive ? 33 : "auto" }}
        onClick={() =>
          props.setMode({
            ...props.mode,
            showDetections: !props.mode.showDetections,
          })
        }
      >
        {label("D", "Detections")}
      </Button>
      <Button
        size="sm"
        variant={props.mode.showObjects ? "solid" : "soft"}
        color={props.mode.showObjects ? "warning" : "neutral"}
        sx={{ width: props.responsive ? 33 : "auto" }}
        onClick={() =>
          props.setMode({
            ...props.mode,
            showObjects: !props.mode.showObjects,
          })
        }
      >
        {label("O", "Objects")}
      </Button>
    </Box>
  );
}
