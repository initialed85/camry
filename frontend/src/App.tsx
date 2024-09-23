import Grid from "@mui/joy/Grid";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";
import Typography from "@mui/joy/Typography";
import { useEffect, useState } from "react";
import CameraDropdownMenu from "./components/CameraDropdownMenu";
import DateSlider from "./components/DateSlider";
import ModeToggle from "./components/ModeToggle";
import StreamDropdownMenu from "./components/StreamDropdownMenu";
import { VideoTable } from "./components/VideoTable";
import { dateSliderStepMillis, responsiveWidth } from "./config";
import { parseDate } from "./helpers";

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

  const [responsive, setResponsive] = useState(
    window.innerWidth < responsiveWidth,
  );
  const [portrait, setPortrait] = useState(
    window.innerWidth < window.innerHeight,
  );
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  const [windowHeight, setWindowHeight] = useState(window.innerHeight);

  const [cameraId, setCameraId] = useState<string | undefined>(undefined);

  const [startedAtGt, setStartedAtGt] = useState<string | undefined>(undefined);

  const [startedAtLte, setStartedAtLte] = useState<string | undefined>(
    undefined,
  );

  const [classNameFilter, setClassNameFilter] = useState<string>("");

  useEffect(() => {
    if (startedAtLte) {
      const possibleStartedAtGt = new Date(
        parseDate(startedAtLte).getTime() - dateSliderStepMillis,
      ).toISOString();
      if (possibleStartedAtGt !== startedAtGt) {
        setStartedAtGt(possibleStartedAtGt);
      }
    }

    const handleResize = () => {
      const desiredResponsive = window.innerWidth < responsiveWidth;
      if (desiredResponsive !== responsive) {
        setResponsive(window.innerWidth < responsiveWidth);
      }

      const desiredPortrait = window.innerWidth < window.innerHeight;
      if (desiredPortrait !== portrait) {
        setPortrait(desiredPortrait);
      }

      const desiredWindowWidth = window.innerWidth;
      if (desiredWindowWidth !== windowWidth) {
        setWindowWidth(desiredWindowWidth);
      }

      const desiredWindowHeight = window.innerHeight;
      if (desiredWindowHeight !== windowHeight) {
        setWindowHeight(desiredWindowHeight);
      }
    };

    const eventListener = () => {
      handleResize();
    };

    window.addEventListener("resize", eventListener);

    return () => {
      window.removeEventListener("reisze", eventListener);
    };
  }, [
    portrait,
    responsive,
    startedAtGt,
    startedAtLte,
    windowHeight,
    windowWidth,
  ]);

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
          <StreamDropdownMenu responsive={responsive} />
          <Input
            size="sm"
            sx={{ mr: 1.5, width: "100%", maxWidth: 300 }}
            onChange={(e) => {
              setClassNameFilter((e.target.value || "").trim().toLowerCase());
            }}
          />
        </Grid>
        <Grid xs={1} sx={{ display: "flex", justifyContent: "end", pr: 0.5 }}>
          <ModeToggle responsive={responsive} />
        </Grid>
      </Grid>
      <Grid container sx={{ pb: 1, display: "flex", justifyContent: "center" }}>
        <DateSlider
          responsive={responsive}
          date={startedAtGt}
          setDate={setStartedAtLte}
        />
      </Grid>

      <VideoTable
        responsive={responsive}
        portrait={portrait}
        windowWidth={windowWidth}
        windowHeight={windowHeight}
        cameraId={cameraId}
        startedAtGt={startedAtGt}
        startedAtLte={startedAtLte}
        classNameFilter={classNameFilter}
      />
    </Sheet>
  );
}

export default App;
