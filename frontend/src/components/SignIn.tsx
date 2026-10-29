import Box from "@mui/joy/Box";
import Button from "@mui/joy/Button";
import Sheet from "@mui/joy/Sheet";
import Stack from "@mui/joy/Stack";
import Typography from "@mui/joy/Typography";
import { signInHost } from "../auth";

export interface SignInProps {
  onAuthenticated: () => void;
  // email is set when the cluster gate let this browser through but Camry itself answered 401, i.e.
  // the address is signed in but not in Camry's own ALLOWED_EMAILS.
  email?: string;
}

// There is no password form here any more: the gate in front of the host means an unauthenticated
// visitor never reaches this component in the common case. It shows when the session lapses mid-use
// (a 401 from the API) or when an address is signed in but not accepted by Camry.
export default function SignIn(props: SignInProps) {
  const host = signInHost();

  const signIn = () => {
    // A top-level navigation is the only thing the gate can intercept: reload the current URL, Traefik
    // redirects to Google, and the return_uri brings the browser back here with the session cookie set.
    window.location.assign(window.location.href);
  };

  return (
    <Box
      sx={{
        display: "grid",
        minHeight: "100vh",
        placeItems: "center",
        p: 3,
        bgcolor: "#fafafa",
      }}
    >
      <Sheet
        variant="plain"
        sx={{
          width: "min(100%, 360px)",
          p: "34px 32px 32px",
          border: "1px solid #dedede",
          borderRadius: 6,
          bgcolor: "#fff",
          boxShadow: "md",
        }}
      >
        <Stack
          direction="row"
          spacing={1.5}
          alignItems="center"
          sx={{ mb: 2.5 }}
        >
          <Box
            sx={{
              display: "grid",
              width: 42,
              height: 42,
              placeItems: "center",
              borderRadius: 7,
              bgcolor: "#222",
              color: "#fff",
              fontSize: 25,
              fontWeight: 700,
            }}
          >
            C
          </Box>
          <Typography
            level="body-xs"
            sx={{ color: "#888", letterSpacing: "0.5px", fontSize: "11px" }}
          >
            Camry
          </Typography>
        </Stack>

        {props.email ? (
          <Typography level="body-sm" sx={{ mb: 1 }}>
            Signed in as <strong>{props.email}</strong>, but Camry does not
            accept that address. Ask for it to be added to the sign-in
            allowlist.
          </Typography>
        ) : (
          <Typography level="body-sm" sx={{ mb: 1 }}>
            Your session has ended. Sign in again to keep watching.
          </Typography>
        )}

        {host ? (
          <Typography level="body-xs" sx={{ mb: 2, color: "#888" }}>
            Sign-in host: {host}
          </Typography>
        ) : null}

        <Button
          color="primary"
          onClick={signIn}
          sx={{
            width: "100%",
            fontSize: "11px",
            "--Button-radius": "3px",
            "--Button-paddingBlock": "7px",
            "--Button-paddingInline": "12px",
          }}
        >
          Sign in
        </Button>
      </Sheet>
    </Box>
  );
}
