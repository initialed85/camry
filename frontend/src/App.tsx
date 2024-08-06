import { CssVarsProvider, useColorScheme } from "@mui/joy/styles";

import Dropdown from "@mui/joy/Dropdown";
import Grid from "@mui/joy/Grid";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import Sheet from "@mui/joy/Sheet";
import Typography from "@mui/joy/Typography";

import "./App.css";

import Button from "@mui/joy/Button";
import React from "react";
import { useQuery } from "./api";

function ModeToggle() {
  const { mode, setMode } = useColorScheme();
  const [mounted, setMounted] = React.useState(false);

  // necessary for server-side rendering
  // because mode is undefined on the server
  React.useEffect(() => {
    setMounted(true);
  }, []);
  if (!mounted) {
    return null;
  }

  return (
    <Button
      variant="outlined"
      onClick={() => {
        setMode(mode === "light" ? "dark" : "light");
      }}
    >
      {mode === "light" ? "Dark" : "Light"}
    </Button>
  );
}

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

  const {
    data: camerasData,
    error: camerasError,
    isLoading: camerasIsLoading,
  } = useQuery("get", "/cameras", {}, { refetchInterval: 1_000 });

  const {
    data: videosData,
    error: videosError,
    isLoading: videosIsLoading,
  } = useQuery(
    "get",
    "/videos",
    { params: { query: { order_by__desc: "started_at", limit: 2 } } },
    { refetchInterval: 1_000 },
  );

  if (camerasIsLoading || videosIsLoading) {
    return <b>Loading...</b>;
  }

  if (camerasError) {
    return <h1>Failed to load cameras: ${camerasError.error}</h1>;
  }

  if (videosError) {
    return <h1>Failed to load videos: ${videosError.error}</h1>;
  }

  return (
    <CssVarsProvider>
      <Sheet
        variant="outlined"
        sx={{
          // width: "99%",
          mx: 1, // margin left & right
          my: 1, // margin top & bottom
          py: 1, // padding top & bottom
          px: 1, // padding left & right
          display: "flex",
          flexDirection: "column",
          gap: 2,
          borderRadius: "sm",
          boxShadow: "md",
        }}
      >
        <Grid container spacing={2} sx={{ flexGrow: 1 }}>
          <Grid xs={2}>
            <Dropdown>
              <MenuButton>Cameras</MenuButton>
              <Menu>
                {camerasData?.objects?.map((c) => {
                  return <MenuItem>{c.name}</MenuItem>;
                })}
              </Menu>
            </Dropdown>
          </Grid>
          <Grid xs={2}>
            <ModeToggle />
          </Grid>
        </Grid>

        <div>
          <Typography level="h4" component="h1">
            Camry
          </Typography>
          <div>
            <pre style={{ fontSize: 10 }}>
              {JSON.stringify(camerasData?.objects, null, 2)}
            </pre>
          </div>
          <div>
            <pre style={{ fontSize: 10 }}>
              {JSON.stringify(videosData?.objects, null, 2)}
            </pre>
          </div>
        </div>
      </Sheet>
    </CssVarsProvider>
  );
}

export default App;
