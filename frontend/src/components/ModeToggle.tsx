import DarkModeIcon from "@mui/icons-material/DarkMode";
import LightModeIcon from "@mui/icons-material/LightMode";
import { useColorScheme } from "@mui/joy/styles";

import Button from "@mui/joy/Button";
import React, { useEffect } from "react";

export interface ModeToggleProps {
  responsive: boolean;
}

export default function ModeToggle(props: ModeToggleProps) {
  const { mode, setMode } = useColorScheme();

  useEffect(() => {
    setMode("system");
  }, [setMode]);

  const [mounted, setMounted] = React.useState(false);

  React.useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return null;
  }

  return (
    <Button
      variant="soft"
      size="sm"
      color="warning"
      sx={{ marginLeft: 1, width: 33 }}
      onClick={() => {
        setMode(mode === "light" ? "dark" : "light");
      }}
    >
      {mode === "light" ? <DarkModeIcon /> : <LightModeIcon />}
    </Button>
  );
}
