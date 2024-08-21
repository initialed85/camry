import Grid from "@mui/joy/Grid";
import Sheet from "@mui/joy/Sheet";
import Typography from "@mui/joy/Typography";

import Input from "@mui/joy/Input";
import { useEffect, useState } from "react";
import CameraToggleButtonGroup from "./components/CameraToggleButtonGroup";
import DateDropdownMenu from "./components/DateDropdownMenu";
import ModeToggle from "./components/ModeToggle";
import { VideoTable } from "./components/VideoTable";

function App() {
  // TODO: for future reference
  // const [lastError, setLastError] = useState("");
  // const { mutateAsync } = useMutation("post", "/cameras");
  // const doMutate = async () => {
  //   try {
  //     await mutateAsync({ body: [{}] });
  //     await refetch();
  //     setLastError("");
  //   } catch (e) {
  //     setLastError(JSON.stringify(e, null, 2));
  //   }
  // };

  const [responsive, setResponsive] = useState(window.innerWidth < 992);
  const [cameraId, setCameraId] = useState<string | null>();
  const [date, setDate] = useState<string | null>();

  useEffect(() => {
    const handleResize = () => {
      setResponsive(window.innerWidth < 992);
    };

    window.addEventListener("resize", () => {
      handleResize();
    });
  }, []);

  return (
    <Sheet
      variant="soft"
      sx={{
        mx: 0,
        my: 0,
        py: 1,
        px: 1,
        display: "flex",
        flexDirection: "column",
        borderRadius: "none",
        boxShadow: "md",
        height: "100%",
      }}
    >
      <Grid container sx={{ pb: 1 }}>
        <Grid xs={1}>
          <Typography level="h4" component="h4" sx={{ pt: 0.1, textAlign: "center" }} color="neutral">
            {responsive ? "C" : "Camry"}
          </Typography>
        </Grid>
        <Grid
          xs={10}
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "flex-start",
          }}
        >
          <CameraToggleButtonGroup responsive={responsive} setCameraId={setCameraId} />
          <DateDropdownMenu responsive={responsive} date={date} setDate={setDate} />
          <Input size="sm" sx={{ mr: 1.5 }} />
        </Grid>
        <Grid xs={1} sx={{ display: "flex", justifyContent: "end", pr: 0.5 }}>
          <ModeToggle responsive={responsive} />
        </Grid>
      </Grid>

      <VideoTable responsive={responsive} cameraId={cameraId} date={date} />
    </Sheet>
  );
}

export default App;
