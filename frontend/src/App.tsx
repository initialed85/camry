import Grid from "@mui/joy/Grid";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";
import Typography from "@mui/joy/Typography";
import { useEffect, useState } from "react";
import useLocalStorageState from "use-local-storage-state";
import CameraDropdownMenu from "./components/CameraDropdownMenu";
import DateDropdownMenu from "./components/DateDropdownMenu";
import ModeToggle from "./components/ModeToggle";
import StreamDropdownMenu from "./components/StreamDropdownMenu";
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

  const [cameraId, setCameraId] = useLocalStorageState<string | undefined>(
    "cameraId",
    {
      defaultValue: undefined,
    },
  );

  const [startedAtGt, setStartedAtGt] = useLocalStorageState<
    string | undefined
  >("startedAtGt", {
    defaultValue: undefined,
  });

  const [startedAtLte, setStartedAtLte] = useLocalStorageState<
    string | undefined
  >("startedAtLte", {
    defaultValue: undefined,
  });

  useEffect(() => {
    const handleResize = () => {
      setResponsive(window.innerWidth < 992);
    };

    window.addEventListener("resize", () => {
      handleResize();
    });
  }, []);

  console.log(
    "state = ",
    JSON.stringify(
      {
        responsive: responsive,
        cameraId: cameraId,
        startedAtGt: startedAtGt,
        startedAtLte: startedAtLte,
      },
      null,
      2,
    ),
  );

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
        <Grid
          xs={11}
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "flex-start",
          }}
        >
          <Typography
            level="h4"
            component="h4"
            sx={{ pt: 0.1, pl: 0.75, pr: 1, textAlign: "center" }}
            color="neutral"
          >
            {responsive ? "C" : "Camry"}
          </Typography>
          <CameraDropdownMenu
            responsive={responsive}
            cameraId={cameraId}
            setCameraId={setCameraId}
          />
          <DateDropdownMenu
            responsive={responsive}
            startedAtGt={startedAtGt}
            setStartedAtGt={setStartedAtGt}
            startedAtLte={startedAtLte}
            setStartedAtLte={setStartedAtLte}
          />
          <StreamDropdownMenu responsive={responsive} />
          <Input size="sm" sx={{ mr: 1.5, width: "100%", maxWidth: 300 }} />
        </Grid>
        <Grid xs={1} sx={{ display: "flex", justifyContent: "end", pr: 0.5 }}>
          <ModeToggle responsive={responsive} />
        </Grid>
      </Grid>

      <VideoTable
        responsive={responsive}
        cameraId={cameraId}
        startedAtGt={startedAtGt}
        startedAtLte={startedAtLte}
      />
    </Sheet>
  );
}

export default App;
